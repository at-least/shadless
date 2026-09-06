package main

// parity-baseline, ported from gates/parity-baseline.mjs — the
// recorded-difference baseline shared by the three parity gates
// (style-parity, demo-parity, path-parity). Values are pinned too: a
// recorded cell that MOVES fails and has to be re-recorded — the same
// ratchet the ids already had.

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type parityCell struct {
	id       string
	oracle   string
	shadless string
}

var reParityCalc = regexp.MustCompile(`calc\([^)]*\)`)

// parityNormValue is the shared getComputedStyle() normalizer behind all
// three parity gates (style-parity, demo-parity, path-parity): round every
// embedded number to 2dp, using reParityNum/reParityOklab declared in
// demo_parity.go, and canonicalise axis-only oklab() to oklch() (Chrome
// serialises the same colour both ways). style-parity additionally
// canonicalizes calc(...) via canonicalizeCalc; demoParityNorm and ppNorm
// call this with it off, which demo-parity and path-parity do not do
// themselves.
func parityNormValue(v string, canonicalizeCalc bool) string {
	if v == "" {
		return v
	}
	v = reParityNum.ReplaceAllStringFunc(v, func(n string) string {
		r := parseFloat2dp(n)
		if r == 0 {
			return "0" // Object.is(-0) guard
		}
		return jsNumberString(r)
	})
	v = reParityOklab.ReplaceAllString(v, "oklch($1 0 0)")
	if canonicalizeCalc {
		v = reParityCalc.ReplaceAllString(v, "calc(…)")
	}
	return v
}

// cellMap: duplicate ids would silently drop a difference — rejected.
func cellMap(cells []parityCell) (map[string]parityCell, []string) {
	m := map[string]parityCell{}
	var order []string
	for _, c := range cells {
		if _, dup := m[c.id]; dup {
			panic(fmt.Sprintf("duplicate parity cell id: %s — the id is not unique enough to ratchet on", c.id))
		}
		m[c.id] = c
		order = append(order, c.id)
	}
	return m, order
}

type parityBaseline struct {
	Pin   string   `json:"pin"`
	Note  string   `json:"note"`
	Flaky []string `json:"flaky,omitempty"`
	Cells []struct {
		ID       string `json:"id"`
		Oracle   string `json:"oracle"`
		Shadless string `json:"shadless"`
	} `json:"cells"`
}

// loadParityBaseline errors on the pre-value format (bare cell ids).
func loadParityBaseline(path string) (*parityBaseline, map[string]parityCell, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return nil, nil, nil // absent → nil, caller records
	}
	var raw parityBaseline
	if err := json.Unmarshal(b, &raw); err != nil {
		return nil, nil, err
	}
	cells := map[string]parityCell{}
	for _, c := range raw.Cells {
		cells[c.ID] = parityCell{c.ID, c.Oracle, c.Shadless}
	}
	if err := detectPrevalue(b); err != nil {
		return nil, nil, err
	}
	return &raw, cells, nil
}

func detectPrevalue(b []byte) error {
	var probe struct {
		Cells []json.RawMessage `json:"cells"`
	}
	if err := json.Unmarshal(b, &probe); err != nil {
		return nil
	}
	for _, c := range probe.Cells {
		var s string
		if err := json.Unmarshal(c, &s); err == nil {
			_ = s
			return fmt.Errorf("pre-value format (bare cell ids). Re-record")
		}
	}
	return nil
}

// writeParityBaseline emits JSON.stringify(body, null, 1) + "\n".
func writeParityBaseline(path, note string, flaky []string, cells map[string]parityCell) error {
	pinB, _ := os.ReadFile("src/registry/pin.json")
	var pin struct {
		ShadcnUI struct {
			Tag string `json:"tag"`
		} `json:"shadcn_ui"`
	}
	json.Unmarshal(pinB, &pin)

	ids := make([]string, 0, len(cells))
	for id := range cells {
		ids = append(ids, id)
	}
	sort.Strings(ids)

	var b strings.Builder
	b.WriteString("{\n \"pin\": " + jsonString(pin.ShadcnUI.Tag) + ",\n \"note\": " + jsonString(note))
	if len(flaky) > 0 {
		sorted := append([]string(nil), flaky...)
		sort.Strings(sorted)
		var parts []string
		for _, f := range sorted {
			parts = append(parts, jsonString(f))
		}
		b.WriteString(",\n \"flaky\": [\n  " + strings.Join(parts, ",\n  ") + "\n ]")
	}
	if len(ids) > 0 {
		b.WriteString(",\n \"cells\": [")
		for i, id := range ids {
			if i > 0 {
				b.WriteString(",")
			}
			c := cells[id]
			b.WriteString("\n  {\n   \"id\": " + jsonString(id) +
				",\n   \"oracle\": " + jsonString(c.oracle) +
				",\n   \"shadless\": " + jsonString(c.shadless) + "\n  }")
		}
		b.WriteString("\n ]")
	} else {
		b.WriteString(",\n \"cells\": []")
	}
	b.WriteString("\n}\n")
	return os.WriteFile(path, []byte(b.String()), 0o644)
}

type parityDiff struct {
	appeared []string
	fixed    []string
	changed  []struct {
		id  string
		was parityCell
		now parityCell
	}
}

// diffParityBaseline: appeared / fixed / changed are all failures.
func diffParityBaseline(recorded, actual map[string]parityCell, actualOrder []string) parityDiff {
	var d parityDiff
	for _, id := range actualOrder {
		v := actual[id]
		was, ok := recorded[id]
		if !ok {
			d.appeared = append(d.appeared, id)
		} else if was.oracle != v.oracle || was.shadless != v.shadless {
			d.changed = append(d.changed, struct {
				id  string
				was parityCell
				now parityCell
			}{id, was, v})
		}
	}
	for id := range recorded {
		if _, ok := actual[id]; !ok {
			d.fixed = append(d.fixed, id)
		}
	}
	sort.Strings(d.appeared)
	sort.Strings(d.fixed)
	sort.Slice(d.changed, func(i, j int) bool { return d.changed[i].id < d.changed[j].id })
	return d
}

func trunc60(s string) string {
	if len(s) > 60 {
		return s[:60] + "…"
	}
	return s
}

func showCell(v parityCell) string {
	return "oracle=" + trunc60(v.oracle) + " shadless=" + trunc60(v.shadless)
}

func showChange(c struct {
	id  string
	was parityCell
	now parityCell
}) string {
	return c.id + "\n      recorded: " + showCell(c.was) + "\n      now:      " + showCell(c.now)
}

// jsNumberString mirrors JS String(number): decimal notation with the
// shortest round-trip, exponent only at >=1e21 (never for our values —
// %g would print 3.35544e+07 where the baseline records 33554400).
func jsNumberString(f float64) string {
	s := strconv.FormatFloat(f, 'f', -1, 64)
	return s
}
