package main

import "github.com/urfave/cli/v2"

const (
	flagDev  = "development"
	flagName = "name"
)

// NewAdd adds the specified dependency to a 'gdpack.toml' manifest.
func NewAdd() *cli.Command {
	return &cli.Command{
		Name:     "add",
		Category: "Dependencies",

		Usage:     "add the dependency at the provided 'URI' (can be a filepath or a URL to a git repository)",
		UsageText: "gdpack add [OPTIONS] <URI>",

		Flags: append(
			[]cli.Flag{
				&cli.BoolFlag{
					Name:    flagDev,
					Aliases: []string{"d", "dev"},
					Usage:   "add a development-only dependency (will not be propagated to dependents' installs)",
				},
				&cli.StringFlag{
					Name:    flagName,
					Aliases: []string{"n"},
					Usage: "install the addon named 'NAME' from a multi-addon dependency; " +
						"if omitted, assumed to be repository name or filepath base name",
				},
				&cli.StringFlag{
					Name:    flagPath,
					Aliases: []string{"p"},
					Usage:   "a 'PATH' to the Godot project containing the manifest",
				},
				&cli.StringSliceFlag{
					Name:    flagTarget,
					Aliases: []string{"t"},
					Usage:   "add the dependency only for 'TARGET' (can be specified more than once)",
				},
			},

			// Git version specifications
			gitRevisionFlags()...,
		),

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}

/* ----------------------- Function: gitRevisionFlags ----------------------- */

// gitRevisionFlags returns the set of options for specifying a git revision.
func gitRevisionFlags() []cli.Flag {
	return []cli.Flag{
		&cli.StringFlag{
			Name:  "branch",
			Usage: "use a git 'BRANCH' version (only used with a git repository 'URI')",
		},
		&cli.StringFlag{
			Name:  "commit",
			Usage: "use a git 'COMMIT' version (only used with a git repository 'URI')",
		},
		&cli.StringFlag{
			Name:  "tag",
			Usage: "use a git 'TAG' version (only used with a git repository 'URI')",
		},
	}
}
