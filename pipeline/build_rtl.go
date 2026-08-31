package main

// build-rtl — multi-language RTL demo emission, ported from
// tools/build-rtl.mjs. Reads src/registry/rtl-translations.json, substitutes
// each ar value with the target language's, patches lang/dir attributes,
// and injects the theme pre-paint script (the same one src/docs/theme-prepaint.mjs
// ships — ported into prepaint.go for the Go tree).
//
// Output: docs/demos/<name>-{he,en,fa}.html + dist/components/<name>-… + the
// language manifest build/rtl-langs.json. Nothing is written on any failure.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// substituteAndPatch mirrors tools/rtl-lib.mjs. Order of the two phases is
// load-bearing: ar→X substitution first (longest-value ordering avoids a
// "Loading" / "Loading…" prefix collision), then the lang/dir attribute
// patch.
func substituteAndPatch(arabicHTML string, translations map[string]struct {
	Dir    string            `json:"dir"`
	Values map[string]string `json:"values"`
}, fromLang, toLang string, toValues map[string]string, toDirOverride string) string {
	type langEnt struct {
		Dir    string
		Values map[string]string
	}
	// typed view
	tr := map[string]langEnt{}
	for k, v := range translations {
		tr[k] = langEnt{Dir: v.Dir, Values: v.Values}
	}
	from := tr[fromLang].Values
	// keys sorted by descending from-value length so longer strings match
	// first (longest-prefix problem); missing from-side sorts last
	keys := make([]string, 0, len(toValues))
	for k := range toValues {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool {
		return len(from[keys[j]]) < len(from[keys[i]])
	})
	var unmatched []string
	out := arabicHTML
	for _, key := range keys {
		fromVal := from[key]
		toVal := toValues[key]
		if fromVal != "" && toVal != "" && fromVal != toVal {
			if !strings.Contains(out, fromVal) {
				unmatched = append(unmatched, key)
				continue
			}
			out = strings.ReplaceAll(out, fromVal, toVal)
		} else if toVal != "" && fromVal == "" {
			unmatched = append(unmatched, key+"(no "+fromLang+" source)")
		}
	}
	if len(unmatched) > 0 {
		fmt.Fprintf(os.Stderr, "warn %s: translation keys not found in HTML (left as %s): %s\n",
			toLang, fromLang, strings.Join(unmatched, ", "))
	}
	langDir := toDirOverride
	if langDir == "" {
		langDir = tr[toLang].Dir
	}
	if langDir == "" {
		langDir = "ltr"
	}
	// <html lang>: replace or inject
	if regexp.MustCompile(`<html[^>]*\slang="`).MatchString(out) {
		out = regexp.MustCompile(`(<html[^>]*\slang=")[^"]*(")`).
			ReplaceAllString(out, `${1}`+toLang+`${2}`)
	} else {
		out = regexp.MustCompile(`<html(\s[^>]*)?>`).
			ReplaceAllStringFunc(out, func(m string) string {
				if !strings.HasSuffix(m, ">") {
					return m
				}
				return m[:len(m)-1] + ` lang="` + toLang + `">`
			})
	}
	// every dir attribute (attribute-boundary anchored: a bare global would
	// also rewrite data-dir="ltr")
	out = regexp.MustCompile(`([\s"'])dir="(rtl|ltr)"`).
		ReplaceAllString(out, `${1}dir="`+langDir+`"`)
	return out
}


func runBuildRtl() int {
	dictB, err := os.ReadFile("src/registry/rtl-translations.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "build-rtl:", err)
		return 1
	}
	var dict map[string]map[string]struct {
		Dir    string            `json:"dir"`
		Values map[string]string `json:"values"`
	}
	if err := json.Unmarshal(dictB, &dict); err != nil {
		fmt.Fprintln(os.Stderr, "build-rtl: dict:", err)
		return 1
	}
	tiersB, err := os.ReadFile("src/registry/tiers.json")
	if err != nil {
		fmt.Fprintln(os.Stderr, "build-rtl:", err)
		return 1
	}
	var tiers map[string]struct {
		Tier string `json:"tier"`
		Emit bool   `json:"emit"`
	}
	json.Unmarshal(tiersB, &tiers)
	shipped := func(name string) bool {
		t, ok := tiers[strings.TrimSuffix(name, "-rtl")]
		return ok && (t.Emit || t.Tier == "static" || t.Tier == "kernel" || t.Tier == "trivial-js")
	}

	for _, d := range []string{"docs/demos", "dist/components", "build"} {
		if err := os.MkdirAll(d, 0o755); err != nil {
			fmt.Fprintln(os.Stderr, "build-rtl:", err)
			return 1
		}
	}

	persian := map[string]string{
		"paymentTitle":       "پرداخت موفق",
		"paymentDescription": "پرداخت حضرت به مبلغ ۲۹.۹۹ دلار با موفقیت انجام شد. رسید نیز به نشانی پست الکترونیکی شما ارسال گردید.",
		"featureTitle":       "ویژگی جدید موجود است",
		"featureDescription": "ما پشتیبانی از حالت تیره را به سیستم افزوده‌ایم. می‌توانید این قابلیت را از بخش تنظیمات حساب کاربری خود فعال نمایید.",
	}

	var names []string
	for n := range dict {
		names = append(names, n)
	}
	sort.Strings(names)

	type pending struct{ path, html string }
	var pendingWrites []pending
	var failures, skipped []string
	manifest := map[string][]string{}
	emitted := 0

	for _, name := range names {
		existing := "docs/demos/" + name + ".html"
		b, err := os.ReadFile(existing)
		if err != nil {
			if shipped(name) {
				failures = append(failures, name+": no "+existing+" to translate from, but the component ships")
			} else {
				skipped = append(skipped, name)
			}
			continue
		}
		ar := dict[name]
		translations := map[string]struct {
			Dir    string            `json:"dir"`
			Values map[string]string `json:"values"`
		}{}
		for k, v := range ar {
			translations[k] = v
		}

		langs := []string{"ar"}
		for _, lang := range []string{"he", "en"} {
			entry, ok := ar[lang]
			if !ok {
				continue
			}
			html := injectPrePaint(substituteAndPatch(string(b), translations, "ar", lang, entry.Values, ""))
			for _, dst := range []string{
				filepath.Join("docs/demos", name+"-"+lang+".html"),
				filepath.Join("dist/components", name+"-"+lang+".html"),
			} {
				pendingWrites = append(pendingWrites, pending{dst, html})
			}
			emitted++
			langs = append(langs, lang)
		}
		if name == "alert-rtl" {
			html := injectPrePaint(substituteAndPatch(string(b), translations, "ar", "fa", persian, "rtl"))
			for _, dst := range []string{
				filepath.Join("docs/demos", name+"-fa.html"),
				filepath.Join("dist/components", name+"-fa.html"),
			} {
				pendingWrites = append(pendingWrites, pending{dst, html})
			}
			emitted++
			langs = append(langs, "fa")
		}
		manifest[name] = langs
	}

	if len(failures) > 0 {
		for _, f := range failures {
			i := strings.Index(f, ": ")
			fmt.Fprintf(os.Stderr, "FAIL [%s]: %s\n", f[:i], f[i+2:])
		}
		fmt.Fprintf(os.Stderr, "FAIL  build-rtl (%d previews could not be built) — nothing written\n", len(failures))
		return 1
	}
	for _, p := range pendingWrites {
		if err := os.WriteFile(p.path, []byte(p.html), 0o644); err != nil {
			fmt.Fprintln(os.Stderr, "build-rtl:", err)
			return 1
		}
	}
	mb, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		fmt.Fprintln(os.Stderr, "build-rtl:", err)
		return 1
	}
	if err := os.WriteFile("build/rtl-langs.json", mb, 0o644); err != nil {
		fmt.Fprintln(os.Stderr, "build-rtl:", err)
		return 1
	}
	fmt.Printf("build-rtl: %d language variants emitted (excluding ar default) + manifest for %d previews", emitted, len(manifest))
	if len(skipped) > 0 {
		fmt.Printf(" (%d dictionaries have no shipped page: %s)", len(skipped), strings.Join(skipped, ", "))
	}
	fmt.Println()
	return 0
}
