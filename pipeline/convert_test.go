package main

// Ported from tools/unit/converter.mjs (deleted with src/converter/index.mjs).
// The JS suite exercised the babel AST directly; these exercise the same
// semantics through the downgraded text — synthetic TSX goes through
// esbuildTsx exactly as the real convert verb's input does.

import (
	"os"
	"os/exec"
	"reflect"
	"sort"
	"strings"
	"testing"
)

func cvTransform(t *testing.T, src string) string {
	t.Helper()
	js, err := esbuildTsx(src)
	if err != nil {
		t.Fatalf("esbuildTsx: %v", err)
	}
	return js
}

func cvTableFixture() *cvTable {
	return &cvTable{
		base: "base-cls",
		axes: []cvaAxis{
			{axis: "size", values: []cvaKV{{"sm", "h-8"}, {"lg", "h-10"}, {"icon-xs", "size-8"}}},
			{axis: "variant", values: []cvaKV{{"outline", "border"}, {"ghost", "hover:bg"}}},
			{axis: "active", values: []cvaKV{{"on", "font-bold"}, {"off", ""}}},
		},
		defaults: []cvaKV{{"size", "sm"}},
	}
}

func TestUnitConverterTierOf(t *testing.T) {
	if got := tierOf("dialog", []string{"radix-ui"}); got != "kernel" {
		t.Errorf("tierOf kernel: got %q", got)
	}
	if got := tierOf("calendar", []string{"react-day-picker", "react"}); got != "external" {
		t.Errorf("tierOf external dep: got %q", got)
	}
	if got := tierOf("badge", []string{"react", "@/lib/utils"}); got != "static" {
		t.Errorf("tierOf static: got %q", got)
	}
}

func TestUnitConverterExportedNames(t *testing.T) {
	js := cvTransform(t, `import { x } from "p"
export function A() {}
export const B = () => null
const C = 1
export { C as D }
export default function E() {}`)
	names, err := cvExportedNames(js)
	if err != nil {
		t.Fatal(err)
	}
	got := []string{}
	for n := range names {
		got = append(got, n)
	}
	sort.Strings(got)
	// esbuild rewrites `export default function E` to `function E` +
	// `export { E as default }`; the declared fn name registers, matching
	// babel's ExportDefaultDeclaration branch.
	if want := []string{"A", "B", "D", "E"}; !reflect.DeepEqual(got, want) {
		t.Errorf("exports: got %v want %v", got, want)
	}
}

func TestUnitConverterCvaTables(t *testing.T) {
	js := cvTransform(t, `
const v = cva("base-cls", {
  variants: {
    orientation: { vertical: ["flex-col [&>*]:w-full", "gap-y-2"], horizontal: "flex-row" },
    variant: { icon: "", image: "bg-cover" },
    "quoted-axis": { "icon-xs": "size-8" },
  },
  defaultVariants: { orientation: "vertical" },
})`)
	tables := cvTablesOf(js)
	v := tables.get("v")
	if v == nil {
		t.Fatal("table v missing")
	}
	if v.base != "base-cls" {
		t.Errorf("cva base: got %q", v.base)
	}
	if cls, ok := v.valueFor("orientation", "vertical"); !ok || cls != "flex-col [&>*]:w-full gap-y-2" {
		t.Errorf("cva array value joined: got %q ok=%v", cls, ok)
	}
	if cls, ok := v.valueFor("variant", "icon"); !ok || cls != "" {
		t.Errorf("cva empty string value preserved: got %q ok=%v", cls, ok)
	}
	if cls, ok := v.valueFor("quoted-axis", "icon-xs"); !ok || cls != "size-8" {
		t.Errorf("cva quoted keys: got %q ok=%v", cls, ok)
	}
	if d, ok := v.hasDefault("orientation"); !ok || d != "vertical" {
		t.Errorf("cva defaults: got %q ok=%v", d, ok)
	}
}

// resolveCvaArgs: the bounded value shapes measured in the pinned tree.
func TestUnitConverterResolveCvaArgs(t *testing.T) {
	newCtx := func() *cvCtx {
		return &cvCtx{file: &cvFile{cva: newCvTables(), metaImport: map[string]string{}},
			reg: newCvReg(), refKeys: map[string]bool{}}
	}
	lit := func(v string) string { return `"` + v + `"` }

	{
		acc := []string{}
		err := newCtx().resolveCvaArgs(nil, cvTableFixture(), map[string]string{"size": lit("lg")}, &acc, "t", false)
		if err != nil {
			t.Fatal(err)
		}
		want := []string{"base-cls", "h-10"}
		if !reflect.DeepEqual(acc, want) {
			t.Errorf("resolve literal value: got %v want %v", acc, want)
		}
	}
	{
		ctx := newCtx()
		ctx.paramDefaults = map[string]string{"size": "icon-xs"}
		acc := []string{}
		err := ctx.resolveCvaArgs(nil, cvTableFixture(), map[string]string{"size": "size"}, &acc, "t", false)
		if err != nil {
			t.Fatal(err)
		}
		want := []string{"base-cls", "size-8"}
		if !reflect.DeepEqual(acc, want) {
			t.Errorf("resolve param default: got %v want %v", acc, want)
		}
	}
	{
		ctx := newCtx()
		ctx.paramDefaults = map[string]string{"size": "sm"}
		acc := []string{}
		err := ctx.resolveCvaArgs(nil, cvTableFixture(), map[string]string{"size": "x ?? size"}, &acc, "t", false)
		if err != nil {
			t.Fatal(err)
		}
		want := []string{"base-cls", "h-8"}
		if !reflect.DeepEqual(acc, want) {
			t.Errorf("resolve x ?? param: got %v want %v", acc, want)
		}
	}
	{
		// dynamic axis: no arg → default merges into base + dynAxes records
		ctx := newCtx()
		el := &cvElCtx{}
		acc := []string{}
		table := cvTableFixture()
		err := ctx.resolveCvaArgs(el, table, map[string]string{}, &acc, "t", true)
		if err != nil {
			t.Fatal(err)
		}
		want := []string{"base-cls", "h-8"}
		if !reflect.DeepEqual(acc, want) {
			t.Errorf("resolve dynamic axis merges default: got %v want %v", acc, want)
		}
	}
	{
		// attr-driven ident-ternary: data-active={isActive} + isActive ? "on" : "off"
		ctx := newCtx()
		ctx.file = &cvFile{cva: newCvTables(), metaImport: map[string]string{}}
		// recordCvaRef needs a slot; the JS suite stubbed the recorder, this
		// asserts the recorded entry instead
		el := &cvElCtx{props: []cvProp{{key: "data-slot", val: `"x"`}, {key: "data-active", val: "isActive"}}}
		acc := []string{}
		table := &cvTable{axes: []cvaAxis{{axis: "active", values: []cvaKV{{"on", "font-bold"}, {"off", ""}}}}}
		err := ctx.resolveCvaArgs(el, table, map[string]string{"active": `isActive ? "on" : "off"`}, &acc, "t", true)
		if err != nil {
			t.Fatal(err)
		}
		if len(acc) != 0 {
			t.Errorf("resolve ternary alternate with empty value not pushed: got %v", acc)
		}
		if len(ctx.file.cvaRefs) != 1 {
			t.Fatalf("ternary dyn via data-attr: no cvaRef recorded")
		}
		ref := ctx.file.cvaRefs[0]
		dyn, _ := ref[3].V.([]any)
		want := jsonObj{}.add("attr", "data-active").add("when", "true").add("classes", "font-bold")
		if !reflect.DeepEqual(dyn[0], any(want)) {
			t.Errorf("ternary dyn: got %v want %v", dyn[0], want)
		}
	}
	// loud failures: unknown variant value + ternary without attr binding
	{
		err := newCtx().resolveCvaArgs(nil, cvTableFixture(), map[string]string{"size": lit("xl")}, &[]string{}, "t", false)
		if err == nil || !strings.Contains(err.Error(), "unknown variant value") {
			t.Errorf("unknown value: got %v", err)
		}
	}
	{
		ctx := newCtx()
		ctx.file = &cvFile{cva: newCvTables(), metaImport: map[string]string{}}
		el := &cvElCtx{}
		table := &cvTable{axes: []cvaAxis{{axis: "v", values: []cvaKV{{"a", "x"}, {"b", ""}}}}}
		err := ctx.resolveCvaArgs(el, table, map[string]string{"v": `nope ? "a" : "b"`}, &[]string{}, "t", false)
		if err == nil || !strings.Contains(err.Error(), "without data-* attr") {
			t.Errorf("ternary without data-attr binding: got %v", err)
		}
	}
}

func TestUnitConverterClassStrings(t *testing.T) {
	ctx := &cvCtx{file: &cvFile{cva: newCvTables(), metaImport: map[string]string{}}, reg: newCvReg()}
	{
		var acc []string
		ctx.classStrings(`cn("a-1", "b-2", isActive ? "on-x" : "off-x")`, &acc, nil)
		sort.Strings(acc)
		want := []string{"a-1", "b-2", "off-x", "on-x"}
		if !reflect.DeepEqual(acc, want) {
			t.Errorf("classStrings cn literal args + both ternary branches: got %v want %v", acc, want)
		}
	}
	{
		var acc []string
		ctx.classStrings("`flex`", &acc, nil)
		if want := []string{"flex"}; !reflect.DeepEqual(acc, want) {
			t.Errorf("classStrings single-quasi template: got %v want %v", acc, want)
		}
		var acc2 []string
		ctx.classStrings("`flex ${y}`", &acc2, nil)
		if len(acc2) != 0 {
			t.Errorf("classStrings interpolated template skipped (documented limit): got %v", acc2)
		}
	}
}

func TestUnitConverterTagVars(t *testing.T) {
	js := cvTransform(t, `function F() {
  const Comp = asChild ? Slot.Root : "span"
  return React.createElement(Comp, { "data-slot": "x" }, "t")
}`)
	top, err := scanTopJs(js)
	if err != nil {
		t.Fatal(err)
	}
	if len(top.decls) != 1 {
		t.Fatalf("decls: %d", len(top.decls))
	}
	vars, err := cvTagVarsOf(js, top.decls[0].body)
	if err != nil {
		t.Fatal(err)
	}
	if vars["Comp"] != "span" {
		t.Errorf("tagVars Comp alternate: got %v", vars)
	}
}

// convertFile + buildTagHints end-to-end over a synthetic 2-file registry.
func TestUnitConverterEndToEnd(t *testing.T) {
	btnSrc := `import * as React from "react"
import { Slot } from "radix-ui"
import { cva } from "class-variance-authority"
const buttonVariants = cva("btn-base", { variants: { size: { sm: "h-8" } }, defaultVariants: { size: "sm" } })
function Button({ asChild }) {
  const Comp = asChild ? Slot.Root : "button"
  return <Comp data-slot="button" className="inline-flex btn-base">Go</Comp>
}
export { Button, buttonVariants }`
	pagerSrc := `import * as React from "react"
import { ChevronLeftIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
function PaginationPrevious() {
  return <Button data-slot="pagination-previous" className="gap-1"><ChevronLeftIcon /><span className="sr-only">Previous</span></Button>
}
function Provider() { return <ExtPrimitive.Viewport data-slot="vp" /> }
export { PaginationPrevious, Provider }`

	files := []string{"button", "pagination-previous"}
	srcs := map[string]string{"button": btnSrc, "pagination-previous": pagerSrc}
	jsOf := map[string]string{}
	for n, s := range srcs {
		jsOf[n] = cvTransform(t, s)
	}
	reg, _ := cvBuildReg(files, jsOf)

	btn, err := convertFile("button", btnSrc, jsOf["button"], reg)
	if err != nil {
		t.Fatal(err)
	}
	pager, err := convertFile("pagination-previous", pagerSrc, jsOf["pagination-previous"], reg)
	if err != nil {
		t.Fatal(err)
	}
	if err := buildTagHints([]*cvFile{btn, pager}); err != nil {
		t.Fatal(err)
	}
	if got := hintsIndexOf(btn.tagHints, "Comp"); got < 0 || btn.tagHints[got].V != "button" {
		t.Errorf("tagHints Comp via local ternary var: got %v", btn.tagHints)
	}
	if got := hintsIndexOf(pager.tagHints, "ChevronLeftIcon"); got < 0 || pager.tagHints[got].V != "svg" {
		t.Errorf("tagHints icon → svg: got %v", pager.tagHints)
	}
	if got := hintsIndexOf(pager.tagHints, "Button"); got < 0 || pager.tagHints[got].V != "button" {
		t.Errorf("tagHints imported component root (cross-file): got %v", pager.tagHints)
	}
	if got := hintsIndexOf(pager.tagHints, "ExtPrimitive.Viewport"); got < 0 || pager.tagHints[got].V != "div" {
		t.Errorf("tagHints external member suffix Viewport → div: got %v", pager.tagHints)
	}
	// class attribution: Button-wrap classes resolved onto the element
	var prev *cvComponent
	for i := range pager.components {
		if pager.components[i].fn == "PaginationPrevious" {
			prev = &pager.components[i]
		}
	}
	if prev == nil || len(prev.elements) == 0 {
		t.Fatal("PaginationPrevious element missing")
	}
	joined := strings.Join(cvJSONStrings(prev.elements[0][2].V), " ")
	if !strings.Contains(joined, "btn-base") {
		t.Errorf("convert: wrapped classes recorded: got %q", joined)
	}
	// wrap recorded as cross-file cvaRef
	found := false
	for _, r := range pager.cvaRefs {
		if s, _ := r[0].V.(string); s == "pagination-previous" {
			found = true
		}
	}
	if !found {
		t.Errorf("convert: wrap recorded as cross-file cvaRef: got %d refs", len(pager.cvaRefs))
	}
}

func TestUnitConverterTagHintsLoudFailure(t *testing.T) {
	weird := `function W() { return <Mystery data-slot="m" /> }
export { W }`
	js := cvTransform(t, weird)
	reg := newCvReg()
	ir, err := convertFile("weird", weird, js, reg)
	if err != nil {
		t.Fatal(err)
	}
	if err := buildTagHints([]*cvFile{ir}); err == nil || !strings.Contains(err.Error(), "unresolved") {
		t.Errorf("tagHints unresolvable fails loud: got %v", err)
	}
}

// The convert verb's output over the pinned tree must equal the committed IR
// byte for byte — the same guarantee `reproducible` gives, scoped to this
// step so a port regression shows here and not three gates downstream.
func TestUnitConverterIrParity(t *testing.T) {
	root := ".."
	if _, err := os.Stat(root + "/build/resolved-ui/ui"); err != nil {
		t.Skip(err)
	}
	bin := t.TempDir() + "/pipeline"
	if out, err := exec.Command("go", "build", "-o", bin, ".").CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, out)
	}
	cmd := exec.Command(bin, "convert")
	cmd.Dir = root
	if out, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("convert: %v\n%s", err, out)
	}
	out, err := exec.Command("git", "-C", root, "status", "--porcelain", "generated/ir").Output()
	if err != nil {
		t.Fatal(err)
	}
	if len(strings.TrimSpace(string(out))) > 0 {
		t.Errorf("convert output drifted from the committed IR:\n%s", out)
	}
}
