package tsx

import "unicode/utf16"

// MapUTF16 converts a babel-style span (UTF-16 code-unit offsets) to byte
// offsets into s. Spans snapshot files (spans-snapshot.json, twmerge's
// config dump) share babel's convention; every position this scanner reports
// internally is in bytes, so tests map at the boundary.
func MapUTF16(s string, start, end int) (b0, b1 int) {
	runes := []rune(s)
	wide := utf16.Encode(runes)
	if start > len(wide) || end > len(wide) {
		return start, end
	}
	b0 = len(string(utf16.Decode(wide[:start])))
	b1 = len(string(utf16.Decode(wide[:end])))
	return b0, b1
}
