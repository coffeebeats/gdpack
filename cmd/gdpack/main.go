package main

import (
	"context"
	"errors"
	"math"
	"os"
	"os/signal"

	"github.com/charmbracelet/lipgloss"
	"github.com/charmbracelet/log"
	"github.com/urfave/cli/v2"
)

const (
	envLogLevel = "GDPACK_LOG"

	lenLevelLabel = 5

	colorCyanBright    = 14
	colorGreenBright   = 10
	colorMagentaBright = 13
	colorRedBright     = 9
	colorWhiteBright   = 15
	colorYellowBright  = 11
)

func main() {
	cli.VersionPrinter = versionPrinter

	app := &cli.App{
		Name:    "gdpack",
		Version: "v0.0.1", // x-release-please-version

		Suggest:                true,
		UseShortOptionHandling: true,

		Commands: []*cli.Command{},
	}

	// Call 'os.Exit' as the first-in/last-out defer; ensures an exit code is
	// returned to the caller.
	var exitCode int
	defer func() {
		if err := recover(); err != nil {
			exitCode = 1

			log.Error(err)
		}

		os.Exit(exitCode)
	}()

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	// Ensure that the signal handler is removed after first interrupt.
	go func() {
		<-ctx.Done()
		stop()
	}()

	if err := setUpLogger(); err != nil {
		panic(err)
	}

	if err := app.RunContext(ctx, os.Args); err != nil {
		var usageErr UsageError
		if errors.As(err, &usageErr) {
			usageErr.PrintUsage()
		}

		panic(err)
	}
}

/* -------------------------------------------------------------------------- */
/*                              Type: UsageError                              */
/* -------------------------------------------------------------------------- */

// UsageError is any error returned from a subcommand implementation that should
// have subcommand usage instructions printed.
type UsageError struct {
	ctx *cli.Context
	err error
}

/* -------------------------- Function: PrintUsage -------------------------- */

// PrintUsage prints the usage associated with the subcommand that failed.
func (e UsageError) PrintUsage() {
	// NOTE: This never returns a meaningful error so ignore it.
	cli.ShowSubcommandHelp(e.ctx) //nolint:errcheck
}

/* ------------------------------- Impl: Error ------------------------------ */

func (e UsageError) Error() string {
	return e.err.Error()
}

/* -------------------------------------------------------------------------- */
/*                            Function: setUpLogger                           */
/* -------------------------------------------------------------------------- */

// setUpLogger configures the package-level charm.sh 'log' logger.
func setUpLogger() error {
	// Configure timestamp reporting.
	log.SetReportTimestamp(false)

	// Configure styles for each log level.
	s := log.DefaultStyles()
	s.Levels[log.DebugLevel] = newStyleWithColor("debug", colorCyanBright)
	s.Levels[log.InfoLevel] = newStyleWithColor("info", colorGreenBright)
	s.Levels[log.WarnLevel] = newStyleWithColor("warn", colorYellowBright)
	s.Levels[log.ErrorLevel] = newStyleWithColor("error", colorRedBright)
	s.Levels[log.FatalLevel] = newStyleWithColor("fatal", colorMagentaBright)

	log.SetStyles(s)

	// Try to parse a log level override.
	if envLevel := os.Getenv(envLogLevel); envLevel != "" {
		level, err := log.ParseLevel(envLevel)
		if err != nil {
			return err
		}

		// Configure the default logging level.
		log.SetLevel(level)
	}

	return nil
}

/* ----------------------- Function: newStyleWithColor ---------------------- */

// newStyleWithColor creates a new 'lipgloss.Style' for the given log level and
// ANSI escape color.
//
// NOTE: This function assumes that the width of the level strings is '5'.
func newStyleWithColor(name string, ansiColor int) lipgloss.Style {
	if name == "" {
		panic("missing style name")
	}

	return lipgloss.NewStyle().
		SetString(name + ":").
		PaddingRight(int(math.Max(float64(lenLevelLabel-len(name)), 0))).
		Bold(true).
		Foreground(lipgloss.ANSIColor(ansiColor))
}

/* -------------------------------------------------------------------------- */
/*                          Function: versionPrinter                          */
/* -------------------------------------------------------------------------- */

// versionPrinter prints a 'gdpack' version string to the terminal.
func versionPrinter(cCtx *cli.Context) {
	log.Printf("gdpack %s\n", cCtx.App.Version)
}
