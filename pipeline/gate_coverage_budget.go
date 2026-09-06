package main

// The coverage budget lives in gates/ledger.json, next to every other recorded
// difference. It fails both ways on purpose: over budget means a gate or a
// contract def was lost (or a new component landed unverified), and UNDER
// budget means coverage improved and must be re-recorded — slack that is not
// re-recorded gets silently re-spent.
//
// Read and written through ledgerFile (ledger.go), the one writer that keeps
// gates/ledger.json's committed key order stable — a second ad-hoc JSON round
// trip through generic maps would alphabetically resort the file on every
// write, turning a one-line change into a whole-file diff.

import (
	"fmt"
)

const (
	ledgerPath  = "gates/ledger.json"
	coverageKey = "coverage.uncovered-cells"
)

func coverageBudget(root string, uncovered, covered int, record, check bool) error {
	if !record && !check {
		return nil
	}
	l, err := readLedger(root)
	if err != nil {
		return fmt.Errorf("FAIL  coverage: %v", err)
	}

	if record {
		if _, ok := l.Budgets[coverageKey]; !ok {
			l.BudgetOrder = append(l.BudgetOrder, coverageKey)
		}
		l.Budgets[coverageKey] = &ledgerBudget{
			Max: uncovered, Target: 0, Class: "debt",
			Reason: "cells of the product matrix (component x path x theme x dir x state) no gate makes a computed-style or behavioral assertion about; see pipeline/gate_coverage.go",
		}
		if err := l.write(root); err != nil {
			return err
		}
		fmt.Printf("coverage: budget %s recorded = %d\n", coverageKey, uncovered)
		return nil
	}

	budget, ok := l.Budgets[coverageKey]
	if !ok {
		return fmt.Errorf("FAIL  coverage: no budget %s in %s — run ./build/pipeline coverage --record", coverageKey, ledgerPath)
	}
	if uncovered > budget.Max {
		return fmt.Errorf("FAIL  coverage: %d uncovered cells > budget %d — a gate or a contract def was lost, or a new component landed unverified", uncovered, budget.Max)
	}
	if uncovered < budget.Max {
		return fmt.Errorf("FAIL  coverage: %d uncovered cells < budget %d — coverage improved; record it: ./build/pipeline coverage --record", uncovered, budget.Max)
	}
	fmt.Printf("PASS  coverage (%d uncovered cells, at budget; %d covered)\n", uncovered, covered)
	return nil
}
