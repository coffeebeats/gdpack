package manifest

import "errors"

// ErrMissingTarget is returned when no target is provided.
var ErrMissingTarget = errors.New("missing target")

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

// Manifest is a mapping from environment/target to the list of direct
// dependencies.
type Manifest struct {
	// Embed production/development dependencies for the default target.
	*Dependencies

	Target map[string]*Dependencies `json:"target,omitempty"`
}

/* ------------------------------- Method: Add ------------------------------ */

// Add records the provided named specification as adependency in the manifest.
// If 'name' already exists within the specified target then the 'Spec' will be
// overwritten.
func (m *Manifest) Add(name string, spec Spec, opts ...Option) error {
	q := query{env: production, target: ""}
	for _, opt := range opts {
		opt(&q)
	}

	deps := m.dependencies(q)

	switch q.env {
	case production:
		return deps.add(name, spec)
	case development:
		return deps.addDev(name, spec)
	}

	return nil
}

/* ------------------------------ Method: List ------------------------------ */

// List returns the list of production dependencies for the default target.
func (m *Manifest) List(opts ...Option) []Dependency {
	q := query{env: production, target: ""}
	for _, opt := range opts {
		opt(&q)
	}

	deps := m.dependencies(q)

	depset := make(map[string]Dependency)

	switch q.env {
	case production:
		for _, d := range m.dependencies(query{env: production, target: ""}).list() {
			depset[d.Name] = d
		}

		for _, d := range deps.list() {
			depset[d.Name] = d
		}

	case development:
		for _, d := range m.dependencies(query{env: development, target: ""}).listDev() {
			depset[d.Name] = d
		}

		for _, d := range deps.listDev() {
			depset[d.Name] = d
		}
	}

	out := make([]Dependency, 0, len(depset))
	for _, d := range depset {
		out = append(out, d)
	}

	return out
}

func (m *Manifest) ListDevWithTarget(target string) ([]Dependency, error) {
	if target == "" {
		return nil, ErrMissingTarget
	}

	depset := make(map[string]Dependency)

	if m.Dependencies != nil {
		for _, d := range m.Dependencies.listDev() {
			depset[d.Name] = d
		}
	}

	if m.Target != nil {
		targetDeps := m.Target[target]

		for _, d := range targetDeps.listDev() {
			depset[d.Name] = d
		}
	}

	deps := make([]Dependency, 0, len(depset))

	for _, d := range depset {
		deps = append(deps, d)
	}

	return deps, nil
}

/* ----------------------------- Method: Remove ----------------------------- */

// Remove deletes the specified addon as a dependency, production or
// development, in the 'Manifest'. This method is a no-op if the addon does not
// exist in either dependency set.
func (m *Manifest) Remove(name string) error {
	if m.Dependencies == nil {
		m.Dependencies = &Dependencies{}
	}

	return m.Dependencies.remove(name)
}

/* -------------------------- Method: dependencies -------------------------- */

func (m *Manifest) dependencies(q query) *Dependencies {
	if m.Dependencies == nil {
		m.Dependencies = &Dependencies{}
	}

	deps := m.Dependencies

	if q.target != "" {
		if m.Target == nil {
			m.Target = make(map[string]*Dependencies)
		}

		if m.Target[q.target] == nil {
			m.Target[q.target] = &Dependencies{}
		}

		deps = m.Target[q.target]
	}

	return deps
}
