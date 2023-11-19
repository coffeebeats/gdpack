package manifest

/* -------------------------------------------------------------------------- */
/*                              Enum: environment                             */
/* -------------------------------------------------------------------------- */

// environment is the set of possible dependency installation environments.
type environment int

const (
	production environment = iota
	development
)

/* -------------------------------------------------------------------------- */
/*                                Struct: query                               */
/* -------------------------------------------------------------------------- */

// query is the configuration allowed by 'Manifest' operations.
type query struct {
	env    environment
	target string
}

/* -------------------------------------------------------------------------- */
/*                              Function: Option                              */
/* -------------------------------------------------------------------------- */

// Option defines a functional option for 'Manifest' operatons.
type Option func(*query)

/* ---------------------- Function: WithDevEnvironment ---------------------- */

// WithDevEnvironment specifies development-only dependencies.
func WithDevEnvironment() Option {
	return func(q *query) {
		q.env = development
	}
}

/* ---------------------- Function: WithProdEnvironment --------------------- */

// WithProdEnvironment specifies production dependencies.
func WithProdEnvironment() Option {
	return func(q *query) {
		q.env = production
	}
}

/* -------------------------- Function: WithTarget -------------------------- */

// WithTarget specifies a target under which 'Manifest' operations apply.
func WithTarget(target string) Option {
	return func(q *query) {
		q.target = target
	}
}
