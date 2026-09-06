package main

// The oracle's esbuild bundling ran with the Go API's Write zero value —
// FALSE — so bundles existed only in memory and every render ERR_FILE_NOT_FOUND
// went unseen for the whole life of the Go port: the pre-port JS tool had left
// bundles in node_modules/.cache and the .key cache hit skipped rebuilding
// them. Any cold cache (fresh clone, CI, the dagger cache volume) rendered
// nothing. This test builds one oracle bundle with a cold cache and demands
// the file on disk, so the class cannot come back.

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestUnitOracleColdCacheBundle(t *testing.T) {
	// under the runner, every open is audited against the node's declared
	// inputs (run.go's -test.testlogfile) and this test's esbuild resolution
	// reads node_modules/**, which is undeclarable — so it only runs on
	// direct `go test` invocations
	for _, a := range os.Args {
		if strings.HasPrefix(a, "-test.testlogfile=") {
			t.Skip("skipped under the pipeline runner: its opens are audited and node_modules is undeclarable")
		}
	}
	root := repoRoot(t)
	// gitignored prerequisites; absent on a fresh clone (jsbuild_test.go's
	// convention for environment-dependent checks)
	for _, rel := range []string{"node_modules", ".upstream/shadcn-ui/apps/v4/examples/radix/alert-demo.tsx", "build/resolved-ui/ui"} {
		if _, err := os.Stat(filepath.Join(root, rel)); err != nil {
			t.Skipf("%s missing (unbuilt tree)", rel)
		}
	}
	wd, _ := os.Getwd()
	if err := os.Chdir(root); err != nil {
		t.Fatal(err)
	}
	defer os.Chdir(wd)

	// the probe cache sits under node_modules/.cache so esbuild's
	// node_modules walk-up from the entry file resolves react the same way
	// the real cache dir does
	probe := filepath.Join("node_modules", ".cache", "probe-cold-cache")
	os.RemoveAll(probe)
	defer os.RemoveAll(probe)
	os.Setenv("SHADLESS_CACHE", filepath.Join(probe, "oracle"))
	defer os.Unsetenv("SHADLESS_CACHE")

	// the scratch dir must be relative too — buildOracleGo computes the
	// page's bundle src with filepath.Rel against it — and lives under the
	// probe dir so the cleanup takes it
	tmp := filepath.Join(probe, "scratch")
	html, err := buildOracleGo("alert-demo", tmp)
	if err != nil {
		t.Fatalf("buildOracleGo: %v", err)
	}
	if _, err := os.Stat(html); err != nil {
		t.Fatalf("oracle page: %v", err)
	}
	bundle := filepath.Join(os.Getenv("SHADLESS_CACHE"), "bundle-alert-demo.js")
	if _, err := os.Stat(bundle); err != nil {
		t.Fatalf("bundle missing after a successful build — esbuild wrote nothing to disk: %v", err)
	}
}
