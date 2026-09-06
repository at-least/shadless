// Package twmerge ports tailwind-merge v3.6.0's default-config merge.
//
// The converter, emitter, path-parity gate and resolve-skins all call
// twMerge because upstream's cn() is clsx+twMerge and every class string in
// the product has passed through it. While the pipeline still runs those
// tools under node, this package serves two purposes: (1) the Go
// implementation of tools which have been ported, and (2) a conformance
// harness — the tests pin this package against twMerge's own output on the
// real class strings the repo ships/ingests.
//
// The port is line-faithful to dist/bundle-cjs.js: mergeClassList walks
// right-to-left, parseClassName splits on ':' outside brackets, groups are
// looked up in a trie built from getDefaultConfig(), and arbitrary values
// ([...]) / variables ((...)) answer the group's validators. The config is
// embedded as config.json, dumped from node getDefaultConfig() with
// functions replaced by $fn/$theme tokens. Regenerate it (and
// snapshot.json) with tools/twmerge-dump.mjs: `node tools/twmerge-dump.mjs`.
package twmerge

import (
	"regexp"
	"slices"
	"sort"
	"strconv"
	"strings"
	"sync"
)

func mustRe(pattern string) *regexp.Regexp {
	// Go RE2 has no lookahead/lookbehind; the bundle's regexes below were
	// already RE2-safe. If a bump of the npm dep ever introduces an
	// unsupported construct this panic is on purpose.
	r, err := regexp.Compile(pattern)
	if err != nil {
		panic("twmerge regex: " + err.Error())
	}
	return r
}

// ---------------------------------------------------------------------------
// Validators (bundle-cjs.js:519-590)

var (
	arbitraryValueRe = mustRe(`^\[(?:(\w[\w-]*):)?(.+)\]$`)
	arbitraryVarRe   = mustRe(`^\((?:(\w[\w-]*):)?(.+)\)$`)
	fractionRe       = mustRe(`^\d+(?:\.\d+)?/\d+(?:\.\d+)?$`)
	tshirtRe         = mustRe(`^(\d+(\.\d+)?)?(xs|sm|md|lg|xl)$`)
	// The JS regex is UNANCHORED (a substring like "8rem" of "0.8rem"
	// satisfies it) except for its ^0$ branch. Copy it verbatim so the
	// validator's true/false boundary matches exactly.
	lengthUnitRe = mustRe(`\d+(%|px|r?em|[sdl]?v([hwib]|min|max)|pt|pc|in|cm|mm|cap|ch|ex|r?lh|cq(w|h|i|b|min|max))|\b(calc|min|max|clamp)\(.+\)|^0$`)
	colorFuncRe  = mustRe(`^(rgba?|hsla?|hwb|(ok)?(lab|lch)|color-mix)\(.+\)$`)
	shadowRe     = mustRe(`^(inset_)?-?((\d+)?\.?(\d+)[a-z]+|0)_-?((\d+)?\.?(\d+)[a-z]+|0)`)
	imageRe      = mustRe(`^(url|image|image-set|cross-fade|element|(repeating-)?(linear|radial|conic)-gradient)\(.+\)$`)
	numberLikeRe = mustRe(`^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$`)
)

func isFraction(v string) bool { return fractionRe.MatchString(v) }

// isNumber mirrors !!value && !Number.isNaN(Number(value)) — Number() also
// accepts "Infinity"/hex, but class bodies never carry those; the RE2
// decimal form is the faithful subset and tests pin the difference.
func isNumber(v string) bool {
	if v == "" {
		return false
	}
	return numberLikeRe.MatchString(v)
}

func isInteger(v string) bool {
	if v == "" {
		return false
	}
	// JS: Number.isInteger — integer means no fraction/exponent after parse
	if !numberLikeRe.MatchString(v) || strings.ContainsAny(v, ".eE") {
		return false
	}
	_, err := strconv.Atoi(v)
	return err == nil
}

func isPercent(v string) bool {
	return strings.HasSuffix(v, "%") && isNumber(v[:len(v)-1])
}

func isTshirtSize(v string) bool { return tshirtRe.MatchString(v) }
func isAny(string) bool          { return true }
func isNever(string) bool        { return false }

func isLengthOnly(v string) bool {
	return lengthUnitRe.MatchString(v) && !colorFuncRe.MatchString(v)
}
func isShadow(v string) bool { return shadowRe.MatchString(v) }
func isImage(v string) bool  { return imageRe.MatchString(v) }

func isAnyNonArbitrary(v string) bool { return !isArbitraryValue(v) && !isArbitraryVariable(v) }
func isArbitraryValue(v string) bool  { return arbitraryValueRe.MatchString(v) }
func isArbitraryVariable(v string) bool {
	return arbitraryVarRe.MatchString(v)
}

func isArbitraryLength(v string) bool {
	return getIsArbitraryValue(v, isLabelLength, isLengthOnly)
}
func isArbitraryNumber(v string) bool {
	return getIsArbitraryValue(v, isLabelNumber, isNumber)
}
func isArbitraryWeight(v string) bool { return getIsArbitraryValue(v, isLabelWeight, isAny) }
func isArbitrarySize(v string) bool   { return getIsArbitraryValue(v, isLabelSize, isNever) }
func isArbitraryFamilyName(v string) bool {
	return getIsArbitraryValue(v, isLabelFamilyName, isNever)
}
func isArbitraryPosition(v string) bool {
	return getIsArbitraryValue(v, isLabelPosition, isNever)
}
func isArbitraryImage(v string) bool  { return getIsArbitraryValue(v, isLabelImage, isImage) }
func isArbitraryShadow(v string) bool { return getIsArbitraryValue(v, isLabelShadow, isShadow) }

func isArbitraryVariableLength(v string) bool {
	return getIsArbitraryVariable(v, isLabelLength, false)
}
func isArbitraryVariableFamilyName(v string) bool {
	return getIsArbitraryVariable(v, isLabelFamilyName, false)
}
func isArbitraryVariablePosition(v string) bool {
	return getIsArbitraryVariable(v, isLabelPosition, false)
}
func isArbitraryVariableSize(v string) bool {
	return getIsArbitraryVariable(v, isLabelSize, false)
}
func isArbitraryVariableImage(v string) bool {
	return getIsArbitraryVariable(v, isLabelImage, false)
}
func isArbitraryVariableShadow(v string) bool {
	return getIsArbitraryVariable(v, isLabelShadow, true)
}
func isArbitraryVariableWeight(v string) bool {
	return getIsArbitraryVariable(v, isLabelWeight, true)
}

func getIsArbitraryValue(value string, testLabel func(string) bool, testValue func(string) bool) bool {
	m := arbitraryValueRe.FindStringSubmatch(value)
	if m == nil {
		return false
	}
	if m[1] != "" {
		return testLabel(m[1])
	}
	return testValue(m[2])
}

// getIsArbitraryVariable's third parameter is shouldMatchNoLabel: a
// label-less (…) variable counts ONLY for shadow and weight (bundle lines
// 558-559 pass true; everything else passes undefined → false).
func getIsArbitraryVariable(value string, testLabel func(string) bool, shouldMatchNoLabel bool) bool {
	m := arbitraryVarRe.FindStringSubmatch(value)
	if m == nil {
		return false
	}
	if m[1] != "" {
		return testLabel(m[1])
	}
	return shouldMatchNoLabel
}

func isLabelPosition(label string) bool { return label == "position" || label == "percentage" }
func isLabelImage(label string) bool    { return label == "image" || label == "url" }
func isLabelSize(label string) bool {
	return label == "length" || label == "size" || label == "bg-size"
}
func isLabelLength(label string) bool     { return label == "length" }
func isLabelNumber(label string) bool     { return label == "number" }
func isLabelFamilyName(label string) bool { return label == "family-name" }
func isLabelWeight(label string) bool     { return label == "number" || label == "weight" }
func isLabelShadow(label string) bool     { return label == "shadow" }

// isNamedContainerQuery mirrors the bundle's offset checks verbatim.
// '@container' (bare) does NOT match (offset 10 must carry '/' or a unit
// prefix), consistent with the JS `value[10] === '/'` short-circuit.
func isNamedContainerQuery(v string) bool {
	if !strings.HasPrefix(v, "@container") {
		return false
	}
	if len(v) > 11 && v[10] == '/' && v[11] != 0 {
		return true
	}
	if len(v) > 16 && v[11] == 's' && v[16] != 0 && strings.HasPrefix(v[10:], "-size/") {
		return true
	}
	if len(v) > 18 && v[11] == 'n' && v[18] != 0 && strings.HasPrefix(v[10:], "-normal/") {
		return true
	}
	return false
}

// validatorsByName is keyed by the bundle's identifiers; config.json carries
// those names in its $fn slots.
var validatorsByName = map[string]func(string) bool{
	"isAny":                         isAny,
	"isAnyNonArbitrary":             isAnyNonArbitrary,
	"isArbitraryFamilyName":         isArbitraryFamilyName,
	"isArbitraryImage":              isArbitraryImage,
	"isArbitraryLength":             isArbitraryLength,
	"isArbitraryNumber":             isArbitraryNumber,
	"isArbitraryPosition":           isArbitraryPosition,
	"isArbitraryShadow":             isArbitraryShadow,
	"isArbitrarySize":               isArbitrarySize,
	"isArbitraryValue":              isArbitraryValue,
	"isArbitraryVariable":           isArbitraryVariable,
	"isArbitraryVariableFamilyName": isArbitraryVariableFamilyName,
	"isArbitraryVariableImage":      isArbitraryVariableImage,
	"isArbitraryVariableLength":     isArbitraryVariableLength,
	"isArbitraryVariablePosition":   isArbitraryVariablePosition,
	"isArbitraryVariableShadow":     isArbitraryVariableShadow,
	"isArbitraryVariableSize":       isArbitraryVariableSize,
	"isArbitraryVariableWeight":     isArbitraryVariableWeight,
	"isArbitraryWeight":             isArbitraryWeight,
	"isFraction":                    isFraction,
	"isInteger":                     isInteger,
	"isNumber":                      isNumber,
	"isPercent":                     isPercent,
	"isTshirtSize":                  isTshirtSize,
	"isNamedContainerQuery":         isNamedContainerQuery,
	"isNever":                       isNever,
	"isLengthOnly":                  isLengthOnly,
	"isShadow":                      isShadow,
	"isImage":                       isImage,
}

// ---------------------------------------------------------------------------
// Class map (bundle createClassMap → processClassGroups)

type classPart struct {
	next       map[string]*classPart
	groupID    string
	validators []validatorEntry
}

type validatorEntry struct {
	groupID   string
	validator func(string) bool
}

func newClassPart() *classPart { return &classPart{next: map[string]*classPart{}} }

func (cp *classPart) part(path string) *classPart {
	cur := cp
	for _, p := range strings.Split(path, "-") {
		n, ok := cur.next[p]
		if !ok {
			n = newClassPart()
			cur.next[p] = n
		}
		cur = n
	}
	return cur
}

var (
	classMap     *classPart
	classMapOnce sync.Once
)

// buildClassMap is the package's one lazy-init closure, guarded by
// classMapOnce via ensureClassMap: it builds the trie AND the
// order-sensitive-modifier weights together so there is no way to run one
// half without the other (see ensureClassMap).
func buildClassMap() *classPart {
	root := newClassPart()
	groups, theme := loadConfig()
	initModifierWeights()
	for _, id := range rawCfg.ClassGroupOrder {
		for _, def := range groups[id] {
			processDefinition(def, root, id, theme)
		}
	}
	return root
}

// ensureClassMap lazily builds classMap (and, via buildClassMap,
// modifierWeight) exactly once. Merge and getClassGroupID both call this
// same function rather than each guarding their own classMapOnce.Do with
// a different closure, so there is only one way this package initializes.
func ensureClassMap() {
	classMapOnce.Do(func() {
		classMap = buildClassMap()
	})
}

// ---------------------------------------------------------------------------
// Group lookup (getGroupRecursive)

func getGroupRecursive(parts []string, start int, node *classPart) string {
	if len(parts)-start == 0 {
		return node.groupID
	}
	if next, ok := node.next[parts[start]]; ok {
		if r := getGroupRecursive(parts, start+1, next); r != "" {
			return r
		}
	}
	rest := strings.Join(parts[start:], "-")
	for _, v := range node.validators {
		if v.validator(rest) {
			return v.groupID
		}
	}
	return ""
}

// getClassGroupID mirrors the bundle: an arbitrary-property class
// ([margin:0]) gets a synthetic "$prop" group; real classes split on '-'
// and a leading empty part (negative values) is skipped.
func getClassGroupID(className string) string {
	ensureClassMap()
	if strings.HasPrefix(className, "[") && strings.HasSuffix(className, "]") {
		content := className[1 : len(className)-1]
		i := strings.IndexByte(content, ':')
		if i == -1 {
			return ""
		}
		if prop := content[:i]; prop != "" {
			return "$" + prop
		}
		return ""
	}
	parts := strings.Split(className, "-")
	start := 0
	if parts[0] == "" && len(parts) > 1 {
		start = 1
	}
	return getGroupRecursive(parts, start, classMap)
}

// ---------------------------------------------------------------------------
// Modifier handling (createSortModifiers)

var modifierWeight = map[string]int{}

func initModifierWeights() {
	for i, m := range orderSensitiveModifiers {
		modifierWeight[m] = 1000000 + i
	}
}

// sortModifiers: predefined modifiers sorted alphabetically, order-
// sensitive and arbitrary ones keep their relative position (segments are
// flushed in place).
func sortModifiers(modifiers []string) []string {
	if len(modifiers) < 2 {
		return append([]string(nil), modifiers...)
	}
	var out []string
	var segment []string
	flush := func() {
		sort.Strings(segment)
		out = append(out, segment...)
		segment = nil
	}
	for _, mod := range modifiers {
		isArb := strings.HasPrefix(mod, "[")
		_, sensitive := modifierWeight[mod]
		if isArb || sensitive {
			flush()
			out = append(out, mod)
		} else {
			segment = append(segment, mod)
		}
	}
	flush()
	return out
}

// parseClassName mirrors bundle-cjs.js:253-291: split at ':' when both
// bracketDepth and parenDepth are zero; track the last top-level '/'.
type parsedClass struct {
	modifiers       []string
	important       bool
	base            string
	postfixPosition int // -1 when none
}

func parseClassName(name string) parsedClass {
	var modifiers []string
	bracketDepth, parenDepth := 0, 0
	modifierStart := 0
	postfix := -1
	for i := 0; i < len(name); i++ {
		c := name[i]
		if bracketDepth == 0 && parenDepth == 0 {
			switch c {
			case ':':
				modifiers = append(modifiers, name[modifierStart:i])
				modifierStart = i + 1
			case '/':
				postfix = i
			}
		}
		switch c {
		case '[':
			bracketDepth++
		case ']':
			bracketDepth--
		case '(':
			parenDepth++
		case ')':
			parenDepth--
		}
	}
	base := name[modifierStart:]
	important := false
	if strings.HasSuffix(base, "!") {
		base = base[:len(base)-1]
		important = true
	} else if strings.HasPrefix(base, "!") {
		base = base[1:]
		important = true
	}
	pp := -1
	if postfix != -1 && postfix > modifierStart {
		pp = postfix - modifierStart
	}
	return parsedClass{modifiers: modifiers, important: important, base: base, postfixPosition: pp}
}

var whitespaceRe = mustRe(`\s+`)

// Merge implements twMerge("…") over the embedded default config.
func Merge(classList string) string {
	ensureClassMap()
	classes := whitespaceRe.Split(strings.TrimSpace(classList), -1)
	var conflictIDs []string
	var result []string

	for i := len(classes) - 1; i >= 0; i-- {
		original := classes[i]
		p := parseClassName(original)
		hasPostfix := p.postfixPosition != -1
		var groupID string
		if hasPostfix {
			groupID = getClassGroupID(p.base[:p.postfixPosition])
			if groupID != "" && postfixLookupGroups[groupID] {
				if withPostfix := getClassGroupID(p.base); withPostfix != "" && withPostfix != groupID {
					groupID = withPostfix
					hasPostfix = false
				}
			}
		} else {
			groupID = getClassGroupID(p.base)
		}
		if groupID == "" {
			if hasPostfix {
				groupID = getClassGroupID(p.base)
				if groupID != "" {
					hasPostfix = false
				}
			}
			if groupID == "" {
				result = append(result, original)
				continue
			}
		}
		modifier := ""
		if len(p.modifiers) == 1 {
			modifier = p.modifiers[0]
		} else if len(p.modifiers) > 1 {
			modifier = strings.Join(sortModifiers(p.modifiers), ":")
		}
		if p.important {
			modifier += "!"
		}
		classID := modifier + groupID
		if slices.Contains(conflictIDs, classID) {
			continue
		}
		conflictIDs = append(conflictIDs, classID)
		conflicting := conflictingGroups[groupID]
		if hasPostfix {
			if mods := conflictingGroupModifiers[groupID]; len(mods) > 0 {
				conflicting = append(append([]string(nil), conflicting...), mods...)
			}
		}
		for _, g := range conflicting {
			conflictIDs = append(conflictIDs, modifier+g)
		}
		result = append(result, original)
	}
	for i, j := 0, len(result)-1; i < j; i, j = i+1, j-1 {
		result[i], result[j] = result[j], result[i]
	}
	return strings.Join(result, " ")
}
