// T9 probe: prove sync's diffFile detects real drift (not just always-zero).
// Seeds three synthetic modifications against button.tsx source and asserts
// each is flagged with the expected drift kind. Exits 1 on any miss.
import { diffFile, extractSig } from "../../tools/sync.mjs"
import { readFileSync } from "node:fs"
import { execSync } from "node:child_process"

const UP = ".upstream/shadcn-ui"
const REG = "apps/v4/registry/new-york-v4/ui"
const pin = JSON.parse(readFileSync("src/registry/pin.json", "utf8"))
const A = pin.shadcn_ui.commit
const base = execSync(`git -C ${UP} show ${A}:${REG}/button.tsx`, { encoding: "utf8" })

let fail = false
const assert = (cond, msg) => {
  if (cond) console.log(`  ok  ${msg}`)
  else { console.error(`  FAIL  ${msg}`); fail = true }
}

// 1. class drift: change a token inside a cva variant class string
const c1 = base.replace("bg-primary text-primary-foreground", "bg-primary-CHANGED text-primary-foreground")
const d1 = diffFile("button", base, c1)
console.log("case 1: class string modified in cva")
assert(d1 !== null, "detected (non-null)")
assert(d1 && d1.classRemoved.some((c) => c.includes("bg-primary text-primary-foreground")), "removed old variant string flagged")
assert(d1 && d1.classAdded.some((c) => c.includes("bg-primary-CHANGED")), "added modified variant string flagged")
assert(d1 && !d1.cosmetic, "not cosmetic")

// 1b. cva surface drift: rename a variant key (xs -> xsmall) — no class string
//     changes, but the public API (size="xs") changes. Converter records variant
//     keys; sync must too.
const c1b = base.replace('xs: "h-6 gap-1', 'xsmall: "h-6 gap-1')
const d1b = diffFile("button", base, c1b)
console.log("case 1b: cva variant key renamed (xs -> xsmall)")
assert(d1b !== null, "detected")
assert(d1b && d1b.cvaRemoved.some((s) => s.includes("|xs")), 'old key "xs" in cvaRemoved')
assert(d1b && d1b.cvaAdded.some((s) => s.includes("|xsmall")), 'new key "xsmall" in cvaAdded')
assert(d1b && d1b.classAdded.length === 0 && d1b.classRemoved.length === 0, "no class-string drift (pure cva-surface)")

// 1c. cva surface drift: change a defaultVariants value (default size=default -> xs)
const c1c = base.replace('size: "default",', 'size: "xs",')
const d1c = diffFile("button", base, c1c)
console.log("case 1c: defaultVariants value changed (size default -> xs)")
assert(d1c !== null, "detected")
assert(d1c && d1c.cvaRemoved.some((s) => s === "buttonVariants|default|size=default"), 'old default "size=default" in cvaRemoved')
assert(d1c && d1c.cvaAdded.some((s) => s === "buttonVariants|default|size=xs"), 'new default "size=xs" in cvaAdded')

// 2. structure drift: change a data-slot value
const c2 = base.replace('data-slot="button"', 'data-slot="button-CHANGED"')
const d2 = diffFile("button", base, c2)
console.log("case 2: data-slot renamed")
assert(d2 !== null, "detected")
assert(d2 && d2.structAdded.some((s) => s.includes("|button-CHANGED")), "new slot sig in structAdded")
assert(d2 && d2.structRemoved.some((s) => s.includes("|button")), "old slot sig in structRemoved")

// 3. cosmetic-only: append a trailing comment, no class/struct change
const c3 = base + "\n// trailing sync-drill comment\n"
const d3 = diffFile("button", base, c3)
console.log("case 3: cosmetic-only change")
assert(d3 !== null, "detected (raw text differs)")
assert(d3 && d3.cosmetic === true, "flagged cosmetic")
assert(d3 && d3.classAdded.length === 0 && d3.classRemoved.length === 0, "no class drift")
assert(d3 && d3.structAdded.length === 0 && d3.structRemoved.length === 0, "no struct drift")

// 4. identical source → null
const d4 = diffFile("button", base, base)
console.log("case 4: identical source")
assert(d4 === null, "null (no drift)")

// sanity: extractSig returns non-empty classes for button
const sig = extractSig(base)
assert(sig.classes.some((c) => c.includes("bg-primary")), "extractSig finds bg-primary variant string")
assert(sig.struct.some((s) => s === "Button|Comp|button"), "extractSig finds Button|Comp|button slot sig")
assert(sig.cva.some((s) => s.startsWith("buttonVariants|variant|default")), "extractSig records cva variant key")
assert(sig.cva.some((s) => s === "buttonVariants|default|size=default"), "extractSig records defaultVariants")

// 5. array-valued cva variant (field.tsx orientation): a string inside the
//    array changes. extractSig must descend into ArrayExpression elements.
const fieldPath = `${REG}/field.tsx`
const fieldSrc = execSync(`git -C ${UP} show ${A}:${fieldPath}`, { encoding: "utf8" })
const c5 = fieldSrc.replace("flex-col [&>*]:w-full [&>.sr-only]:w-auto", "flex-col-DRILLED [&>*]:w-full [&>.sr-only]:w-auto")
const d5 = diffFile("field", fieldSrc, c5)
console.log("case 5: array-valued cva variant string changed (field.tsx)")
assert(d5 !== null, "detected")
assert(d5 && d5.classRemoved.some((c) => c.includes("flex-col [&>*]:w-full")), "removed old array string flagged")
assert(d5 && d5.classAdded.some((c) => c.includes("flex-col-DRILLED")), "added modified array string flagged")
const sigF = extractSig(fieldSrc)
assert(sigF.classes.some((c) => c.includes("flex-col [&>*]:w-full")), "extractSig descends into cva ArrayExpression")

// 6. non-exported top-level arrow component (sonner.tsx Toaster): a class in
//    its JSX changes. extractSig must walk non-exported VariableDeclaration
//    arrow functions, not only ExportNamedDeclaration ones.
const sonnerPath = `${REG}/sonner.tsx`
const sonnerSrc = execSync(`git -C ${UP} show ${A}:${sonnerPath}`, { encoding: "utf8" })
const c6 = sonnerSrc.replace("toaster group", "toaster-DRILLED group")
const d6 = diffFile("sonner", sonnerSrc, c6)
console.log("case 6: non-exported arrow component class changed (sonner.tsx Toaster)")
assert(d6 !== null, "detected")
assert(d6 && d6.classRemoved.includes("toaster group"), "removed old Toaster class flagged")
assert(d6 && d6.classAdded.includes("toaster-DRILLED group"), "added modified Toaster class flagged")
const sigS = extractSig(sonnerSrc)
assert(sigS.struct.some((s) => s.startsWith("Toaster|")), "extractSig walks non-exported arrow fn Toaster")

if (fail) { console.log("\nFAIL  t9 detect probe"); process.exit(1) }
console.log("\nPASS  t9 detect probe (8/8 cases)")
