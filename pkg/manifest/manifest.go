package manifest

/* -------------------------------------------------------------------------- */
/*                             Struct: Dependency                             */
/* -------------------------------------------------------------------------- */

// Dependency is a named specification of an addon. The name should match the
// actual name of the addon as set by the addon.
type Dependency struct {
	Name string
	Spec Spec
}

/* -------------------------------------------------------------------------- */
/*                              Struct: Manifest                              */
/* -------------------------------------------------------------------------- */

// Manifest is a mapping from environment/target to the list of direct
// dependencies.
type Manifest struct {
	Dependencies    map[string]Spec `json:"dependencies,omitempty"`
	DevDependencies map[string]Spec `json:"devDependencies,omitempty"`
}

/* ------------------------------- Method: Add ------------------------------ */

// Add records the provided named specification as a production dependency in
// the 'Manifest'. If 'name' already exists, then the 'Spec' will be overwritten.
func (m *Manifest) Add(name string, spec Spec) error {
	delete(m.DevDependencies, name)
	m.set(&m.Dependencies, name, spec)

	return nil
}

/* ----------------------------- Method: AddDev ----------------------------- */

// AddDev records the provided named specification as a development-only
// dependency in the 'Manifest'. If 'name' already exists in either production
// or development dependencies, then the 'Spec' will be overwritten.
func (m *Manifest) AddDev(name string, spec Spec) error {
	delete(m.Dependencies, name)
	m.set(&m.DevDependencies, name, spec)

	return nil
}

/* ----------------------------- Method: Remove ----------------------------- */

// Remove deletes the specified addon as a production dependency in the
// 'Manifest'. This method is a no-op if the addon does not exist.
func (m *Manifest) Remove(name string) error {
	delete(m.Dependencies, name)
	delete(m.DevDependencies, name)

	return nil
}

/* ------------------------------- Method: set ------------------------------ */

// set adds or overwrites the named addon specification in the provided 'map',
// first ensuring the 'map' is non-'nil'.
func (m *Manifest) set(d *map[string]Spec, name string, spec Spec) {
	if d == nil {
		*d = make(map[string]Spec)
	}

	(*d)[name] = spec
}
