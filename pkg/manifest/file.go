package manifest

import (
	"bytes"
	"encoding/json"
	"io"
	"os"
)

func Write(m *Manifest, out string) error {
	f, err := os.Create(out)
	if err != nil {
		return err
	}

	defer f.Close()

	bb, err := json.MarshalIndent(m, "", "  ")
	if err != nil {
		return err
	}

	if _, err := io.Copy(f, bytes.NewReader(bb)); err != nil {
		return err
	}

	return nil
}

func Parse(contents []byte) (*Manifest, error) {
	var m Manifest

	if err := json.Unmarshal(contents, &m); err != nil {
		return nil, err
	}

	return &m, nil
}

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
