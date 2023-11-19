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

// add records the provided named specification as a dependency for the
// environment. If 'name' already exists then the 'Spec' will be overwritten.
func (d *Dependencies) add(name string, spec Spec, env environment) error {
	addTo := &d.Prod
	removeFrom := d.Dev

	if env == development {
		addTo = &d.Dev
		removeFrom = d.Prod
	}

	delete(removeFrom, name)

	if *addTo == nil {
		*addTo = make(map[string]Spec)
	}

	(*addTo)[name] = spec

	return nil
}

/* ------------------------------ Method: list ------------------------------ */

// list returns the set of dependencies for the specified environment.
func (d *Dependencies) list(env environment) []Dependency {
	m := d.Prod
	if env == development {
		m = d.Dev
	}

	deps := make([]Dependency, 0, len(m))

	for name, spec := range m {
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
