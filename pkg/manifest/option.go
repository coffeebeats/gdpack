package manifest

/* -------------------------------------------------------------------------- */
/*                              Enum: environment                             */
/* -------------------------------------------------------------------------- */

type environment int

const (
	production environment = iota
	development
)

/* -------------------------------------------------------------------------- */
/*                                Struct: query                               */
/* -------------------------------------------------------------------------- */

type query struct {
	env    environment
	target string
}

/* -------------------------------------------------------------------------- */
/*                              Function: Option                              */
/* -------------------------------------------------------------------------- */

type Option func(*query)

/* ---------------------- Function: WithDevEnvironment ---------------------- */

func WithDevEnvironment() Option {
	return func(q *query) {
		q.env = production
	}
}

/* ---------------------- Function: WithProdEnvironment --------------------- */

func WithProdEnvironment() Option {
	return func(q *query) {
		q.env = production
	}
}

/* -------------------------- Function: WithTarget -------------------------- */

func WithTarget(target string) Option {
	return func(q *query) {
		q.target = target
	}
}
