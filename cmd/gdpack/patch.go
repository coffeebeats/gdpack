package main

import "github.com/urfave/cli/v2"

const flagDir = "directory"

// NewPatch simplifies patching an installed dependency in a 'gdpack.toml' manifest.
func NewPatch() *cli.Command {
	return &cli.Command{
		Name:     "patch",
		Category: "Dependencies",

		Usage:     "create a directory in which to edit a dependency (changes can be applied via 'gdpack patch-commit')",
		UsageText: "gdpack patch [OPTIONS] <NAME>",

		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    flagDir,
				Aliases: []string{"d", "dir"},
				Usage:   "a directory 'DIR' to edit the dependency in",
			},
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

// NewPatchCommit applies the edits made to a dependency via 'gdpack patch'.
func NewPatchCommit() *cli.Command {
	return &cli.Command{
		Name:     "patch-commit",
		Category: "Dependencies",

		Usage:     "applies the edits made via 'gdpack patch'",
		UsageText: "gdpack patch-commit <DIR>",

		Action: func(c *cli.Context) error {
			return nil
		},
	}
}
