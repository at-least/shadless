package main

// The gates, as Go tests.
//
// A gate asserts and produces nothing, which is what a test is. Before this
// file each ported gate was a `pipeline gate <name>` subcommand dispatched
// through a name->func map: a second, hand-maintained registry that the
// compiler never checked, and a gate could go missing from it silently. As
// tests they are found by the toolchain — `go test -C pipeline` runs every
// one, and a gate that nobody wired up still runs.
//
// Only gates that are pure file and process I/O live here. A gate that
// imports a module from src/ reads the pipeline's own definitions as data —
// porting one would mean a SECOND implementation of an emitter or converter
// rule, and two implementations that must agree is the failure mode this repo
// spends most of its effort on. Those stay in JS.
//
// Each keeps its predecessor's exact verdict and message shape, because the
// mutation that proves it (gates/mutations/) asserts a non-zero exit and the
// meta-gate is the only thing standing between "this gate runs" and "this
// gate can fail".
//
//   go test -C pipeline -count=1 -v                     every gate
//   go test -C pipeline -count=1 -v -run '^TestPack$'   one gate
//
// -count=1 is not optional in the pipeline: the test cache would replay a
// stale verdict over a tree that has since changed.

import (
	"os"
	"path/filepath"
	"testing"
)

// repoRoot resolves the tree a gate judges. Gates run as `go test -C
// pipeline`, so the working directory is pipeline/ — but rather than hardcode
// "..", walk up to the marker so `go test ./...` from anywhere inside the
// repo lands on the same root.
func repoRoot(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	for {
		if _, err := os.Stat(filepath.Join(dir, "gates", "registry.mjs")); err == nil {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			t.Fatal("repo root not found above the working directory")
		}
		dir = parent
	}
}

// gate bridges a gate function to `go test`. The error text IS the report —
// verbatim what the JS gate printed to stderr before exiting 1 — so a
// mutation's evidence still reads the same.
func gate(t *testing.T, fn func(root string) error) {
	t.Helper()
	if err := fn(repoRoot(t)); err != nil {
		t.Fatal(err)
	}
}

func TestPin(t *testing.T) {
	if code := runPin(repoRoot(t), true, false); code != 0 {
		t.Fatal("FAIL  pin (see PIN FAIL above)")
	}
}

func TestDistComplete(t *testing.T) { gate(t, gateDistComplete) }

func TestPack(t *testing.T) { gate(t, gatePack) }

func TestCoverage(t *testing.T) {
	gate(t, func(root string) error { return gateCoverage(root, []string{"--check"}) })
}

func TestReproducible(t *testing.T) { gate(t, gateReproducible) }
