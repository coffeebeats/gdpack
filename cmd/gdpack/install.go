package main

import "github.com/urfave/cli/v2"

const (
	flagProd   = "production"
	flagPath   = "path"
	flagTarget = "target"
)

// NewInstall installs addon dependencies to a Godot project.
func NewInstall() *cli.Command {
	return &cli.Command{
		Name:     "install",
		Category: "Install",

		Aliases: []string{"i"},

		Usage:     "install addon dependencies into the Godot project's 'addons' directory",
		UsageText: "gdpack install [OPTIONS]",

		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    flagPath,
				Aliases: []string{"p"},
				Usage:   "a 'PATH' to the Godot project containing the manifest",
			},
			&cli.BoolFlag{
				Name:    flagProd,
				Aliases: []string{"prod"},
				Usage:   "don't install development dependencies",
			},
			&cli.StringSliceFlag{
				Name:    flagTarget,
				Aliases: []string{"t"},
				Usage:   "install dependencies only for 'TARGET' (can be specified more than once)",
			},
		},

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
