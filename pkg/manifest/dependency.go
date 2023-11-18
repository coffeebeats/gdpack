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

/* -------------------------- Struct: Dependencies -------------------------- */

// Dependencies is a set of required addons. Addons can be registered as either
// a production or development dependency, but not both.
type Dependencies struct {
	Prod map[string]Spec `json:"dependencies,omitempty"`
	Dev  map[string]Spec `json:"devDependencies,omitempty"`
}

/* ------------------------------- Method: add ------------------------------ */

// add records the provided named specification as a production dependency in
// the 'Dependencies' struct. If 'name' already exists then the 'Spec' will be
// overwritten.
func (d *Dependencies) add(name string, spec Spec) error {
	delete(d.Dev, name)

	if d.Prod == nil {
		d.Prod = make(map[string]Spec)
	}

	d.Prod[name] = spec

	return nil
}

/* ----------------------------- Method: addDev ----------------------------- */

// addDev records the provided named specification as a development dependency
// in the 'Dependencies' struct. If 'name' already exists then the 'Spec' will
// be overwritten.
func (d *Dependencies) addDev(name string, spec Spec) error {
	delete(d.Prod, name)

	if d.Dev == nil {
		d.Dev = make(map[string]Spec)
	}

	d.Dev[name] = spec

	return nil
}

/* ------------------------------ Method: list ------------------------------ */

// list returns the set of production dependencies in 'Dependencies'.
func (d *Dependencies) list() []Dependency {
	deps := make([]Dependency, 0, len(d.Prod))

	for name, spec := range d.Prod {
		deps = append(deps, Dependency{Name: name, Spec: spec})
	}

	return deps
}

/* ----------------------------- Method: listDev ---------------------------- */

// listDev returns the set of development dependencies in 'Dependencies'.
func (d *Dependencies) listDev() []Dependency {
	deps := make([]Dependency, 0, len(d.Prod))

	for name, spec := range d.Prod {
		deps = append(deps, Dependency{Name: name, Spec: spec})
	}

	return deps
}

/* ----------------------------- Method: remove ----------------------------- */

// remove deletes the specified addon as a dependency, production or
// development, in the 'Dependencies' struct. This method is a no-op if the
// addon does not exist in either dependency set.
func (d *Dependencies) remove(name string) error {
	delete(d.Prod, name)
	delete(d.Dev, name)

	return nil
}
