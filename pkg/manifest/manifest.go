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

	return m.dependencies(q).add(name, spec, q.env)
}

/* ------------------------------ Method: List ------------------------------ */

// List returns the list of production dependencies for the default target.
func (m *Manifest) List(opts ...Option) []Dependency {
	q := query{env: production, target: ""}
	for _, opt := range opts {
		opt(&q)
	}

	depset := make(map[string]Dependency)

	for _, d := range m.dependencies(query{env: production, target: ""}).list(q.env) {
		depset[d.Name] = d
	}

	for _, d := range m.dependencies(q).list(q.env) {
		depset[d.Name] = d
	}

	out := make([]Dependency, 0, len(depset))
	for _, d := range depset {
		out = append(out, d)
	}

	return out
}

/* ----------------------------- Method: Remove ----------------------------- */

// Remove deletes the specified addon as a dependency, production or
// development, in the 'Manifest'. This method is a no-op if the addon does not
// exist in either dependency set.
func (m *Manifest) Remove(name string, opts ...Option) error {
	q := query{env: production, target: ""}
	for _, opt := range opts {
		opt(&q)
	}

	return m.dependencies(q).remove(name)
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
