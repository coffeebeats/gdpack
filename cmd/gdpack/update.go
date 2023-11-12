package main

import "github.com/urfave/cli/v2"

// NewUpdate updates one or more remote addon dependencies.
func NewUpdate() *cli.Command {
	return &cli.Command{
		Name:     "update",
		Category: "Dependencies",

		Usage:     "update one or more remote addon dependencies to their latest version",
		UsageText: "gdpack update [OPTIONS] [NAME]",

		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    flagPath,
				Aliases: []string{"p"},
				Usage:   "a 'PATH' to the Godot project containing the manifest",
			},
			&cli.StringSliceFlag{
				Name:    flagTarget,
				Aliases: []string{"t"},
				Usage:   "update dependencies only for 'TARGET' (can be specified more than once)",
			},
		},

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
