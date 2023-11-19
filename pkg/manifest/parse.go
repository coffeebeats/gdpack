package manifest

import (
	"encoding/json"
	"io"
	"os"
)

// Parse parses the provided byte slice into a 'Manifest'.
func Parse(contents []byte) (*Manifest, error) {
	var m Manifest

	if err := json.Unmarshal(contents, &m); err != nil {
		return nil, err
	}

	return &m, nil
}

// ParseFile parses the contents of the file specified by 'path' into a
// 'Manifest'.
func ParseFile(path string) (*Manifest, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}

	defer f.Close()

	bb, err := io.ReadAll(f)
	if err != nil {
		return nil, err
	}

	return Parse(bb)
}
