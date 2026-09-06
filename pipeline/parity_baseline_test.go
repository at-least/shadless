package main

// The recorded-difference ratchet shared by all three parity gates
// (style-parity, demo-parity, path-parity), tested directly rather than only
// through the JSON-fixture mutation harness in mutations_test.go.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// cellMap's whole reason to exist is refusing a duplicate id — a collision
// would silently drop one side's recorded difference.
func TestUnitCellMapRejectsDuplicateID(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Fatal("a duplicate cell id did not panic")
		}
	}()
	cellMap([]parityCell{{id: "x", oracle: "1"}, {id: "x", oracle: "2"}})
}

func TestUnitCellMapPreservesInsertionOrder(t *testing.T) {
	m, order := cellMap([]parityCell{
		{id: "b/comp", oracle: "1", shadless: "2"},
		{id: "a/comp", oracle: "3", shadless: "4"},
	})
	if len(m) != 2 || m["a/comp"].shadless != "4" {
		t.Fatalf("map not built correctly: %+v", m)
	}
	if strings.Join(order, ",") != "b/comp,a/comp" {
		t.Errorf("order = %v, want insertion order [b/comp a/comp]", order)
	}
}

func TestUnitDetectPrevalue(t *testing.T) {
	if err := detectPrevalue([]byte(`{"cells":["bare-id"]}`)); err == nil {
		t.Error("bare-string cells (the pre-value format) should be rejected")
	}
	if err := detectPrevalue([]byte(`{"cells":[{"id":"x","oracle":"1","shadless":"2"}]}`)); err != nil {
		t.Errorf("value-shaped cells should be accepted directly, got %v", err)
	}
}

func TestUnitLoadParityBaseline(t *testing.T) {
	dir := t.TempDir()

	// absent file: nil baseline, nil cells, nil error — the caller records
	rawAbs, cellsAbs, err := loadParityBaseline(filepath.Join(dir, "missing.json"))
	if err != nil || rawAbs != nil || cellsAbs != nil {
		t.Errorf("absent baseline: got raw=%v cells=%v err=%v, want nil,nil,nil", rawAbs, cellsAbs, err)
	}

	// value-shaped baseline decodes into the id->cell map
	valuePath := filepath.Join(dir, "value.json")
	if err := os.WriteFile(valuePath, []byte(
		`{"pin":"v1.2.3","cells":[{"id":"alert/root/color","oracle":"red","shadless":"blue"}]}`,
	), 0o644); err != nil {
		t.Fatal(err)
	}
	raw, cells, err := loadParityBaseline(valuePath)
	if err != nil {
		t.Fatal(err)
	}
	if raw.Pin != "v1.2.3" {
		t.Errorf("pin not decoded: %+v", raw)
	}
	c, ok := cells["alert/root/color"]
	if !ok || c.oracle != "red" || c.shadless != "blue" {
		t.Errorf("cell not decoded: %+v", cells)
	}

	// pre-value (bare id) baseline errors instead of silently decoding empty
	bareVal := filepath.Join(dir, "bare.json")
	if err := os.WriteFile(bareVal, []byte(`{"cells":["alert/root/color"]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if _, _, err := loadParityBaseline(bareVal); err == nil {
		t.Error("pre-value baseline should error, not decode silently")
	}
}

func TestUnitDiffParityBaseline(t *testing.T) {
	recorded := map[string]parityCell{
		"m": {id: "m", oracle: "1", shadless: "1"}, // unchanged
		"z": {id: "z", oracle: "9", shadless: "9"}, // changed below
		"y": {id: "y", oracle: "8", shadless: "8"}, // fixed: missing from actual
	}
	// actualOrder is deliberately NOT alphabetical, to prove appeared/fixed
	// come back SORTED rather than dependent on iteration/insertion order.
	actual := map[string]parityCell{
		"m": {id: "m", oracle: "1", shadless: "1"},       // unchanged
		"z": {id: "z", oracle: "9", shadless: "CHANGED"}, // changed value
		"w": {id: "w", oracle: "new", shadless: "new"},   // appeared
		"a": {id: "a", oracle: "new2", shadless: "new2"}, // appeared
	}
	actualOrder := []string{"z", "w", "m", "a"} // not alphabetical

	d := diffParityBaseline(recorded, actual, actualOrder)

	if strings.Join(d.appeared, ",") != "a,w" {
		t.Errorf("appeared = %v, want sorted [a w]", d.appeared)
	}
	if strings.Join(d.fixed, ",") != "y" {
		t.Errorf("fixed = %v, want [y] (only the recorded id absent from actual)", d.fixed)
	}
	if len(d.changed) != 1 || d.changed[0].id != "z" || d.changed[0].now.shadless != "CHANGED" {
		t.Errorf("changed = %+v, want one entry for z", d.changed)
	}
}
