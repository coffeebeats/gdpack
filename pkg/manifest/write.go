package manifest

import (
	"bytes"
	"encoding/json"
	"io"
	"os"
)

// Write persists the provided manifest to the file specified by 'out'.
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
