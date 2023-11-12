package main

import "github.com/urfave/cli/v2"

// NewInit initializes a Godot project with a new 'gdpack.toml' manifest.
func NewInit() *cli.Command {
	return &cli.Command{
		Name:     "init",
		Category: "Init",

		Usage:     "create a new 'gdpack.toml' manifest for the Godot project",
		UsageText: "gdpack init [OPTIONS]",

		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    flagPath,
				Aliases: []string{"p"},
				Usage:   "the 'PATH' to the Godot project containing the manifest",
			},
		},

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
