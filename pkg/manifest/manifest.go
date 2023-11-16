package manifest

import "encoding/json"

type Manifest struct {
	dependencies map[string]map[string]string `toml:"dependencies"`
}

func (m *Manifest) Add(name string, spec Spec) error {
	bb, err := json.Marshal(spec)
	if err != nil {
		return err
	}

	s := make(map[string]string)
	if err := json.Unmarshal(bb, &s); err != nil {
		return err
	}

	m.dependencies[name] = s

	return nil
}

func (m *Manifest) List() []Spec {
	out := make([]Spec, len(m.dependencies))

	for name, d := range m.dependencies {
		if a := (Asset{}); As(d, &a) {
			out[i] = a
			continue
		}
	}

	return out
}
