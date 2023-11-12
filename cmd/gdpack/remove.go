package main

import "github.com/urfave/cli/v2"

// NewRemove removes the specified dependency from a 'gdpack.toml' manifest.
func NewRemove() *cli.Command {
	return &cli.Command{
		Name:     "remove",
		Category: "Dependencies",

		Usage:     "remove the specified dependency",
		UsageText: "gdpack remove [OPTIONS] <NAME>",

		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    flagPath,
				Aliases: []string{"p"},
				Usage:   "a 'PATH' to the Godot project containing the manifest",
			},
			&cli.StringSliceFlag{
				Name:    flagTarget,
				Aliases: []string{"t"},
				Usage:   "remove the dependency only for 'TARGET' (can be specified more than once)",
			},
		},

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
