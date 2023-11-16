package manifest

import "github.com/pelletier/go-toml/v2"

type Spec interface {
	spec()
}

type Asset struct {
	Asset uint `toml:"asset"`
}

func (a Asset) spec() {}

type Git struct {
	Git string `toml:"git"`

	Branch string `toml:"branch"`
	Tag    string `toml:"tag"`
	Rev    string `toml:"rev"`
}

func (g Git) spec() {}

type Local struct {
	Path string `toml:"path"`
}

func (l Local) spec() {}

func As(data map[string]string, target Spec) bool {
	bb, err := toml.Marshal(data)
	if err != nil {
		return false
	}

	if err := toml.Unmarshal(bb, &target); err != nil {
		return false
	}

	return true
}
