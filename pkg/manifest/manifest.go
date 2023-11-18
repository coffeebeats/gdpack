package manifest

type Manifest struct {
	Dependencies    map[string]Spec `json:"dependencies,omitempty"`
	DevDependencies map[string]Spec `json:"devDependencies,omitempty"`
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

func (m *Manifest) AddDev(name string, spec Spec) error {
	if m.DevDependencies == nil {
		m.DevDependencies = make(map[string]Spec)
	}

	m.DevDependencies[name] = spec

	return nil
}

func (m *Manifest) List() []Dependency {
	out := make([]Dependency, 0, len(m.Dependencies))

	for name, s := range m.Dependencies {
		out = append(out, Dependency{Name: name, Spec: s})
	}

	return out
}
