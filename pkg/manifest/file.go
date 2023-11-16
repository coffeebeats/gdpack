package manifest

import (
	"bytes"
	"io"
	"os"

	toml "github.com/pelletier/go-toml/v2"
)

func Write(m Manifest, out string) error {
	f, err := os.Create(out)
	if err != nil {
		return err
	}

	defer f.Close()

	bb, err := toml.Marshal(m)
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

	if err := toml.Unmarshal(contents, &m); err != nil {
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
