package main

// Gates ported from gates/*.mjs. Each keeps its predecessor's exact verdict
// and message shape, because the mutation that proves it (gates/mutations/)
// asserts a non-zero exit and the meta-gate is the only thing standing between
// "this gate runs" and "this gate can fail".
//
// Only gates that are pure file and process I/O live here. A gate that imports
// a module from src/ reads the pipeline's own definitions as data — porting
// one would mean a SECOND implementation of an emitter or converter rule, and
// two implementations that must agree is the failure mode this repo spends
// most of its effort on. Those stay in JS.

import (
	"fmt"
	"os"
)

type gateFn func(root string) error

var gates = map[string]gateFn{}

func runGate(root, name string) {
	fn, ok := gates[name]
	if !ok {
		fmt.Fprintf(os.Stderr, "unknown gate: %s\n", name)
		os.Exit(2)
	}
	if err := fn(root); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
