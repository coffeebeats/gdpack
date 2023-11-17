package manifest

import toml "github.com/pelletier/go-toml/v2"

type Spec struct {
	Asset
	Git
	Local
}

type Asset struct {
	Asset uint `toml:"asset,omitempty"`
}

type Git struct {
	Git string `toml:"git"`

	Branch string `toml:"branch,omitempty"`
	Tag    string `toml:"tag,omitempty"`
	Rev    string `toml:"rev,omitempty"`
}

type Local struct {
	Path string `toml:"path,omitempty"`
}

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
