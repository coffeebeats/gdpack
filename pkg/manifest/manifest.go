package manifest

import "errors"

// ErrMissingTarget is returned when no target is provided.
var ErrMissingTarget = errors.New("missing target")

/* -------------------------------------------------------------------------- */
/*                             Interface: Manifest                            */
/* -------------------------------------------------------------------------- */

/* ---------------------------- Interface: Adder ---------------------------- */

// Adder describes a type which can record dependencies.
type Adder interface {
	Add(name string, spec Spec) error
	AddDev(name string, spec Spec) error
	AddWithTarget(name string, spec Spec, target string) error
	AddDevWithTarget(name string, spec Spec, target string) error
}

/* ---------------------------- Interface: Lister --------------------------- */

// Lister describes a type which can list required dependencies.
type Lister interface {
	List() []Dependency
	ListDev() []Dependency
}

/* --------------------------- Interface: Remover --------------------------- */

// Remover describes a type which can remove required dependencies.
type Remover interface {
	Remove(name string) error
	RemoveWithTarget(name, target string) error
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

// Manifest is a mapping from environment/target to the list of direct
// dependencies.
type Manifest struct {
	// Embed production/development dependencies for the default target.
	*Dependencies

	Target map[string]Dependencies `json:"target,omitempty"`
}

var _ Adder = (*Manifest)(nil)
var _ Lister = (*Manifest)(nil)
var _ Remover = (*Manifest)(nil)

/* ------------------------------- Method: Add ------------------------------ */

// Add records the provided named specification as a production dependency in
// the 'Manifest'. If 'name' already exists then the 'Spec' will be overwritten.
func (m *Manifest) Add(name string, spec Spec) error {
	return m.Dependencies.add(name, spec)
}

/* ----------------------------- Method: AddDev ----------------------------- */

// AddDev records the provided named specification as a development-only
// dependency in the 'Manifest'. If 'name' already exists in either production
// or development dependencies then the 'Spec' will be overwritten.
func (m *Manifest) AddDev(name string, spec Spec) error {
	return m.Dependencies.addDev(name, spec)
}

/* ------------------------ Method: AddDevWithTarget ------------------------ */

// AddDevWithTarget records the provided named specification as a development-
// only dependency in the 'Manifest' for the specified target. If 'name' already
// exists within that target then the 'Spec' will be overwritten.
func (m *Manifest) AddDevWithTarget(name string, spec Spec, target string) error {
	if target == "" {
		return ErrMissingTarget
	}

	if m.Target == nil {
		m.Target = make(map[string]Dependencies)
	}

	deps := m.Target[target]

	return deps.addDev(name, spec)
}

/* -------------------------- Method: AddWithTarget ------------------------- */

// AddWithTarget records the provided named specification as a production
// dependency in the 'Manifest' for the specified target. If 'name' already
// exists within that target then the 'Spec' will be overwritten.
func (m *Manifest) AddWithTarget(name string, spec Spec, target string) error {
	if target == "" {
		return ErrMissingTarget
	}

	if m.Target == nil {
		m.Target = make(map[string]Dependencies)
	}

	deps := m.Target[target]

	return deps.add(name, spec)
}

/* ------------------------------ Method: List ------------------------------ */

// List returns the list of production dependencies for the default target.
func (m *Manifest) List() []Dependency {
	return m.Dependencies.list()
}

/* ----------------------------- Method: ListDev ---------------------------- */

// ListDev returns the list of development dependencies for the default target.
func (m *Manifest) ListDev() []Dependency {
	return m.Dependencies.listDev()
}

/* ------------------------- Method: ListDevWithTarget ------------------------- */

// ListDevWithTarget returns the list of development dependencies for the
// specified target. This list is build by combining the default set of
// development dependencies with the target-specific set. Any conflicting
// specifications will be overwritten by the target-specific 'Spec'.
func (m *Manifest) ListDevWithTarget(target string) ([]Dependency, error) {
	if target == "" {
		return nil, ErrMissingTarget
	}

	depset := make(map[string]Dependency)

	for _, d := range m.Dependencies.listDev() {
		depset[d.Name] = d
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

/* ------------------------- Method: ListWithTarget ------------------------- */

// ListWithTarget returns the list of production dependencies for the specified
// target. This list is build by combining the default set of production
// dependencies with the target-specific set. Any conflicting specifications
// will be overwritten by the target-specific 'Spec'.
func (m *Manifest) ListWithTarget(target string) ([]Dependency, error) {
	if target == "" {
		return nil, ErrMissingTarget
	}

	depset := make(map[string]Dependency)

	for _, d := range m.Dependencies.list() {
		depset[d.Name] = d
	}

	if m.Target != nil {
		targetDeps := m.Target[target]

		for _, d := range targetDeps.list() {
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
	return m.Dependencies.remove(name)
}

/* ------------------------ Method: RemoveWithTarget ------------------------ */

// RemoveWithTarget deletes the specified addon as a dependency, production or
// development, in the 'Manifest' for the specified target. This method is a no-
// op if the addon does not exist in either dependency set for that target.
func (m *Manifest) RemoveWithTarget(name, target string) error {
	if target == "" {
		return ErrMissingTarget
	}

	if m.Target == nil {
		return nil
	}

	deps := m.Target[target]

	return deps.remove(name)
}
