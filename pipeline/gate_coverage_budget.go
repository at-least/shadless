package main

// The coverage budget lives in gates/ledger.json, next to every other recorded
// difference. It fails both ways on purpose: over budget means a gate or a
// contract def was lost (or a new component landed unverified), and UNDER
// budget means coverage improved and must be re-recorded — slack that is not
// re-recorded gets silently re-spent.
//
// Read and written as a generic map so the rest of the ledger round-trips
// untouched: this gate owns one key, not the file's shape.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

const (
	ledgerPath  = "gates/ledger.json"
	coverageKey = "coverage.uncovered-cells"
)

func coverageBudget(root string, uncovered, covered int, record, check bool) error {
	if !record && !check {
		return nil
	}
	path := filepath.Join(root, ledgerPath)
	raw, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("FAIL  coverage: %v", err)
	}
	var ledger map[string]json.RawMessage
	if err := json.Unmarshal(raw, &ledger); err != nil {
		return fmt.Errorf("FAIL  coverage: %s: %v", ledgerPath, err)
	}
	var budgets map[string]json.RawMessage
	if err := json.Unmarshal(ledger["budgets"], &budgets); err != nil {
		return fmt.Errorf("FAIL  coverage: %s budgets: %v", ledgerPath, err)
	}

	if record {
		entry := map[string]any{
			"max": uncovered, "target": 0, "class": "debt",
			"reason": "cells of the product matrix (component x path x theme x dir x state) no gate makes a computed-style or behavioral assertion about; see pipeline/gate_coverage.go",
		}
		eb, err := json.Marshal(entry)
		if err != nil {
			return err
		}
		budgets[coverageKey] = eb
		bb, err := json.Marshal(budgets)
		if err != nil {
			return err
		}
		ledger["budgets"] = bb
		out, err := json.MarshalIndent(ledger, "", "  ")
		if err != nil {
			return err
		}
		if err := os.WriteFile(path, append(out, '\n'), 0o644); err != nil {
			return err
		}
		fmt.Printf("coverage: budget %s recorded = %d\n", coverageKey, uncovered)
		return nil
	}

	b, ok := budgets[coverageKey]
	if !ok {
		return fmt.Errorf("FAIL  coverage: no budget %s in %s — run ./build/pipeline gate coverage --record", coverageKey, ledgerPath)
	}
	var budget struct {
		Max int `json:"max"`
	}
	if err := json.Unmarshal(b, &budget); err != nil {
		return fmt.Errorf("FAIL  coverage: budget %s: %v", coverageKey, err)
	}
	if uncovered > budget.Max {
		return fmt.Errorf("FAIL  coverage: %d uncovered cells > budget %d — a gate or a contract def was lost, or a new component landed unverified", uncovered, budget.Max)
	}
	if uncovered < budget.Max {
		return fmt.Errorf("FAIL  coverage: %d uncovered cells < budget %d — coverage improved; record it: ./build/pipeline gate coverage --record", uncovered, budget.Max)
	}
	fmt.Printf("PASS  coverage (%d uncovered cells, at budget; %d covered)\n", uncovered, covered)
	return nil
}
