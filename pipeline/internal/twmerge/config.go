package twmerge

import (
	_ "embed"
	"encoding/json"
)

//go:embed config.json
var configJSON []byte

// config.json was dumped from `node getDefaultConfig()` (tailwind-merge
// v3.6.0, dist/bundle-cjs.js) with every function replaced by a token:
//
//	{"$theme": "color"}    — fromTheme('color'): expand the theme list in place
//	{"$fn": "isFraction"}  — a validator by bundle identifier
//
// Strings are trie endpoints; {"prefix": […]} nests under that path
// component; null never survives the dump (it is absent from the v3 dump).
type rawConfig struct {
	Theme       map[string][]any `json:"theme"`
	ClassGroups map[string][]any `json:"classGroups"`
	// ClassGroupOrder preserves JS Object.keys order: groups that SHARE a
	// trie prefix (text-*, border-*) add validators to the same node, and
	// the first match wins. Go maps iterate in random order, so without
	// this list a catch-all validator (color's isAny) can be probed before
	// the group-specific one and the wrong group claims the class.
	ClassGroupOrder           []string            `json:"classGroupOrder"`
	ConflictingGroups         map[string][]string `json:"conflictingClassGroups"`
	ConflictingGroupModifiers map[string][]string `json:"conflictingClassGroupModifiers"`
	PostfixLookupGroups       []string            `json:"postfixLookupClassGroups"`
	OrderSensitiveModifiers   []string            `json:"orderSensitiveModifiers"`
}

var (
	rawCfg                    rawConfig
	conflictingGroups         map[string][]string
	conflictingGroupModifiers map[string][]string
	postfixLookupGroups       = map[string]bool{}
	orderSensitiveModifiers   []string
)

func loadConfig() (map[string][]any, map[string][]any) {
	if rawCfg.ClassGroups != nil {
		return rawCfg.ClassGroups, rawCfg.Theme
	}
	if err := json.Unmarshal(configJSON, &rawCfg); err != nil {
		panic("twmerge: config.json: " + err.Error())
	}
	conflictingGroups = rawCfg.ConflictingGroups
	conflictingGroupModifiers = rawCfg.ConflictingGroupModifiers
	orderSensitiveModifiers = rawCfg.OrderSensitiveModifiers
	for _, g := range rawCfg.PostfixLookupGroups {
		postfixLookupGroups[g] = true
	}
	return rawCfg.ClassGroups, rawCfg.Theme
}

// processDefinition dispatches the JSON-encoded definition onto the trie.
//
//	"literal"                → endpoint
//	{"$fn": name}            → validator at the current node
//	{"$theme": key}          → theme list expanded in place
//	{"prefix": [defs]}       → nest
func processDefinition(def any, parent *classPart, groupID string, theme map[string][]any) {
	switch d := def.(type) {
	case string:
		target := parent
		if d != "" {
			target = parent.part(d)
		}
		target.groupID = groupID
	case map[string]any:
		if name, ok := d["$fn"].(string); ok {
			fn, ok := validatorsByName[name]
			if !ok {
				panic("twmerge: unknown validator " + name)
			}
			parent.validators = append(parent.validators, validatorEntry{groupID, fn})
			return
		}
		if key, ok := d["$theme"].(string); ok {
			for _, sub := range theme[key] {
				processDefinition(sub, parent, groupID, theme)
			}
			return
		}
		for prefix, sub := range d {
			list, _ := sub.([]any)
			for _, s := range list {
				processDefinition(s, parent.part(prefix), groupID, theme)
			}
		}
	default:
		panic("twmerge: unhandled definition shape in config.json")
	}
}
