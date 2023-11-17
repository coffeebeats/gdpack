package manifest

type Manifest struct {
	Dependencies map[string]Spec `toml:"dependencies"`
}

type Dependency struct {
	Name string
	Spec Spec
}

func (m *Manifest) Add(name string, spec Spec) error {
	if m.Dependencies == nil {
		m.Dependencies = make(map[string]Spec)
	}

	m.Dependencies[name] = spec

	return nil
}

func (m *Manifest) List() []Dependency {
	out := make([]Dependency, 0, len(m.Dependencies))

	for name, s := range m.Dependencies {
		out = append(out, Dependency{Name: name, Spec: s})
	}

	return out
}
