package main

// Pure normalization/decoding helpers behind the contract fixture comparison,
// tested apart from the browser-driving CLI driver (runContract /
// runContractsAll), the same split product_css_test.go uses for extractTokens.

import (
	"encoding/json"
	"testing"
)

func TestUnitCNormVal(t *testing.T) {
	// style branch: drop --radix-* vars, outline:none and pointer-events:auto
	got := cNormVal("color: red; --radix-foo: 1px; outline: none; pointer-events: auto; display:block", "style")
	if want := "color: red; display:block"; got != want {
		t.Errorf("style normalization: got %q, want %q", got, want)
	}
	// a single radix-shaped auto id
	if got := cNormVal("radix-:r1:", "id"); got != "<auto-id>" {
		t.Errorf("single auto id: got %q, want <auto-id>", got)
	}
	// an all-auto-id token list
	if got := cNormVal("radix-:r1: radix-_r_ab", "class"); got != "<auto-id> <auto-id>" {
		t.Errorf("all-auto token list: got %q", got)
	}
	// NOT all-auto-shaped: passes through unchanged, token by token
	if got := cNormVal("radix-:r1: plainclass", "class"); got != "radix-:r1: plainclass" {
		t.Errorf("mixed token list should pass through unchanged: got %q", got)
	}
}

func TestUnitCRawToJsonable(t *testing.T) {
	if v, err := cRawToJsonable(json.RawMessage("null")); err != nil {
		t.Errorf("null: unexpected error %v", err)
	} else if _, ok := v.(jsonNull); !ok {
		t.Errorf("null: got %#v, want jsonNull{}", v)
	}
	if v, err := cRawToJsonable(json.RawMessage(`"hello"`)); err != nil || v != "hello" {
		t.Errorf("string: got %#v, %v, want \"hello\", nil", v, err)
	}
	// The recorder's wire format only ever emits null, string or object
	// (attrs() reads getAttribute() results and textContent, always strings) —
	// a bare number is a shape it never produces, and cRawToJsonable errors
	// on it rather than silently accepting something outside that contract.
	if _, err := cRawToJsonable(json.RawMessage("42")); err == nil {
		t.Error("a bare number should error: not a shape the recorder wire format produces")
	}
	nested := json.RawMessage(`{"tag":"button","aria-x":null,"child":{"a":"1"}}`)
	v, err := cRawToJsonable(nested)
	if err != nil {
		t.Fatal(err)
	}
	obj, ok := v.(jsonObj)
	if !ok || len(obj) != 3 {
		t.Fatalf("nested object: got %#v", v)
	}
	if obj[0].K != "tag" || obj[0].V != "button" {
		t.Errorf("nested object: key order/value wrong at 0: %+v", obj[0])
	}
	if _, isNull := obj[1].V.(jsonNull); !isNull || obj[1].K != "aria-x" {
		t.Errorf("nested object: null value not preserved at 1: %+v", obj[1])
	}
	child, ok := obj[2].V.(jsonObj)
	if !ok || len(child) != 1 || child[0].K != "a" || child[0].V != "1" {
		t.Errorf("nested object: child object not decoded: %+v", obj[2])
	}
}
