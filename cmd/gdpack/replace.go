package main

import "github.com/urfave/cli/v2"

// NewReplace replaces the specified dependency with another one.
func NewReplace() *cli.Command {
	return &cli.Command{
		Name:     "replace",
		Category: "Dependencies",

		Usage:     "replace a dependency with one at the provided 'URI'; can be a filepath or a URL to a git repository",
		UsageText: "gdpack replace [OPTIONS] <NAME> <URI>",

		Flags: append(
			[]cli.Flag{
				&cli.BoolFlag{
					Name:    flagDev,
					Aliases: []string{"d"},
					Usage:   "replace a development-only dependency (will not be propagated to dependents' installs)",
				},
				&cli.StringFlag{
					Name:    flagName,
					Aliases: []string{"n"},
					Usage: "replace with the addon named 'NAME' from a multi-addon dependency; " +
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
					Usage:   "replace the dependency only for `TARGET` (can be specified more than once)",
				},
			},

			gitRevisionFlags()...,
		),

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
