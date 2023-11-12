package main

import "github.com/urfave/cli/v2"

const flagForce = "force"

// NewVendor vendors remote addon dependencies to the specified directory.
func NewVendor() *cli.Command {
	return &cli.Command{
		Name:     "vendor",
		Category: "Install",

		Usage:     "vendor remote addon dependencies into the specified directory",
		UsageText: "gdpack vendor [OPTIONS] <OUT>",

		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    flagForce,
				Aliases: []string{"f"},
				Usage:   "forcibly overwrite the contents of 'OUT'",
			},
			&cli.StringFlag{
				Name:    flagPath,
				Aliases: []string{"p"},
				Usage:   "a 'PATH' to the Godot project containing the manifest",
			},
			&cli.BoolFlag{
				Name:    flagProd,
				Aliases: []string{"prod"},
				Usage:   "don't vendor development dependencies",
			},
			&cli.StringSliceFlag{
				Name:    flagTarget,
				Aliases: []string{"t"},
				Usage:   "vendor dependencies only for 'TARGET' (can be specified more than once)",
			},
		},

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
