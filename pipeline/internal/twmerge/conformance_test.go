package twmerge

import (
	"encoding/json"
	"os"
	"testing"
)

// Conformance: twMerge's actual output on the repo's real class strings,
// snapshotted by the JS implementation (snapshot.json). If this diverges the
// port is wrong, not the snapshot — regenerate the snapshot only against a
// deliberate tailwind-merge version bump.
func TestMergeSnapshot(t *testing.T) {
	b, err := os.ReadFile("snapshot.json")
	if err != nil {
		t.Skip("snapshot not generated")
	}
	var cases [][2]string
	if err := json.Unmarshal(b, &cases); err != nil {
		t.Fatal(err)
	}
	bad := 0
	for _, c := range cases {
		if got := Merge(c[0]); got != c[1] {
			t.Errorf("Merge(%q)\n  = %q\n  want %q", c[0], got, c[1])
			if bad++; bad > 8 {
				t.Fatalf("more failures…")
			}
		}
	}
	t.Logf("%d / %d snapshot cases agree", len(cases)-bad, len(cases))
}
