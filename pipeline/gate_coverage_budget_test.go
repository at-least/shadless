package main

// coverageBudget's --record path must write gates/ledger.json through the
// ordered ledgerFile writer (ledger.go), not a second ad-hoc JSON round trip —
// a second writer would alphabetically resort the file on every write,
// turning a one-line change into a whole-file diff (see ledger.go's own
// comment on why ledgerFile exists). ledgerRecord only updates budgets
// already present in BudgetOrder and never appends one, so the coverage
// budget's own append-if-absent step is load-bearing on a fixture that starts
// without a coverage.uncovered-cells entry at all.

import (
	"os"
	"path/filepath"
	"testing"
)

func TestUnitCoverageBudgetRecordKeepsLedgerOrder(t *testing.T) {
	root := ledgerFixture(t, 0)
	before, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	wantOrder := append(append([]string(nil), before.BudgetOrder...), coverageKey)

	if err := coverageBudget(root, 7, 0, true, false); err != nil {
		t.Fatal(err)
	}
	after, err := readLedger(root)
	if err != nil {
		t.Fatal(err)
	}
	if len(after.BudgetOrder) != len(wantOrder) {
		t.Fatalf("BudgetOrder = %v, want %v", after.BudgetOrder, wantOrder)
	}
	for i, k := range wantOrder {
		if after.BudgetOrder[i] != k {
			t.Errorf("BudgetOrder[%d] = %q, want %q (a second JSON round trip "+
				"through generic maps would alphabetically resort this)", i, after.BudgetOrder[i], k)
		}
	}
	if after.Budgets[coverageKey] == nil || after.Budgets[coverageKey].Max != 7 {
		t.Fatalf("budget %s not recorded with max=7: %+v", coverageKey, after.Budgets[coverageKey])
	}

	// idempotent: recording the same value again must not reformat the file —
	// the alphabetical-resort bug did this on every single write.
	first, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if err := coverageBudget(root, 7, 0, true, false); err != nil {
		t.Fatal(err)
	}
	second, err := os.ReadFile(filepath.Join(root, ledgerPath))
	if err != nil {
		t.Fatal(err)
	}
	if string(first) != string(second) {
		t.Error("recording the same value twice changed the file's bytes")
	}

	// the check path, reading the same budget back through the same writer:
	// at budget passes, over budget fails, under budget fails (slack must be
	// re-recorded, not silently re-spent).
	if err := coverageBudget(root, 7, 0, false, true); err != nil {
		t.Errorf("at-budget check should pass: %v", err)
	}
	if err := coverageBudget(root, 8, 0, false, true); err == nil {
		t.Error("8 uncovered > recorded budget 7 should fail")
	}
	if err := coverageBudget(root, 6, 0, false, true); err == nil {
		t.Error("6 uncovered < recorded budget 7 should fail")
	}
}
