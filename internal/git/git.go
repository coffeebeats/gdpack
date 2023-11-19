package git

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"os/exec"

	"github.com/coffeebeats/gdpack/pkg/manifest"
)

var (
	// ErrMissingGit is returned when 'gdpack' cannot find a 'git' executable.
	ErrMissingGit = errors.New("missing 'git' command")
	// ErrMissingRepository is returned when the repository specification is
	// missing a URI.
	ErrMissingRepository = errors.New("missing repository")
)

// Clone downloads a working copy of the specified git repository. Depending on
// the constraints specified in 'spec' a different type of clone is used:
//
//   - Rev: A full clone is downloaded and then the revision checked out.
//   - Branch,Tag: The 'HEAD' commit with a depth of '1'.
//   - None: The 'HEAD' commit on the default branch with a depth of '1'.
func Clone(ctx context.Context, spec manifest.Git, out string) error {
	if spec.Git == "" {
		return ErrMissingRepository
	}

	git, err := exec.LookPath("git")
	if err != nil {
		return err
	}

	args := []string{"clone"}

	switch {
	case spec.Rev != "": // Full clone is required to check out specific rev.
	case spec.Branch != "":
		args = append(args, "--depth", "1", "--branch", spec.Branch)
	case spec.Tag != "":
		args = append(args, "--depth", "1", "--branch", spec.Tag)
	default: // No specification - get latest commit on default branch.
		args = append(args, "--depth", "1")
	}

	args = append(args, spec.Git, out)

	cmd := exec.CommandContext(ctx, git, args...)
	if err := execute(cmd); err != nil {
		return err
	}

	// Check out the specified git ref if required.
	if spec.Rev != "" {
		cmd := exec.CommandContext(ctx, git, "checkout", spec.Rev)
		cmd.Dir = out

		if err := execute(cmd); err != nil {
			return err
		}
	}

	return nil
}

func execute(cmd *exec.Cmd) error {
	var stderr bytes.Buffer
	cmd.Stderr = &stderr

	if err := cmd.Run(); err != nil {
		return fmt.Errorf("%w: %s", err, stderr.String())
	}

	return nil
}
