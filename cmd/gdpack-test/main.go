package main

import (
	"context"
	"log"
	"os"
	"path/filepath"
	"reflect"

	"github.com/coffeebeats/gdpack/internal/git"
	"github.com/coffeebeats/gdpack/pkg/manifest"
)

func main() { //nolint:funlen
	m := manifest.Manifest{}

	a := manifest.Spec{Git: &manifest.Git{Git: "https://github.com/coffeebeats/gdenv.git", Tag: "v0.6.6"}}
	if err := m.Add("gdenv", a); err != nil {
		panic(err)
	}

	b := manifest.Spec{Git: &manifest.Git{Git: "git@github.com:coffeebeats/gdpack.git", Rev: "47d63298ef2134797792c6bd65e1b7ce122779d1"}}
	if err := m.Add("gdpack", b, manifest.WithDevEnvironment()); err != nil {
		panic(err)
	}

	// c := manifest.Spec{Git: &manifest.Git{Git: "git@github.com:coffeebeats/gdpack.git", Branch: "next"}}
	// if err := m.Add("gdpack", c, manifest.WithDevEnvironment(), manifest.WithTarget("aarch64")); err != nil {
	// 	panic(err)
	// }

	// d := manifest.Spec{Local: &manifest.Local{Path: "./pkg/manifest/next"}}
	// if err := m.Add("gdpack", d, manifest.WithTarget("x86_64")); err != nil {
	// 	panic(err)
	// }

	log.Printf("%#v\n", m.Target)
	log.Println("--")

	if err := manifest.Write(&m, "./manifest.json"); err != nil {
		panic(err)
	}

	log.Println("Wrote manifest contents to './manifest.json'")

	parsed, err := manifest.ParseFile("./manifest.json")
	if err != nil {
		panic(err)
	}

	log.Println("Parsed manifest from './manifest.json'")

	if !reflect.DeepEqual(parsed, &m) {
		log.Printf("%#v\n", parsed)
		log.Println("--")
		log.Printf("%#v\n", &m)

		log.Fatal("mismatch")
	}

	deps := parsed.List(manifest.WithTarget("x86_64"))
	devDeps := parsed.List(manifest.WithTarget("x86_64"), manifest.WithDevEnvironment())

	log.Printf("%v", deps)
	log.Printf("%v", devDeps)
	log.Println("--")

	tmp, err := os.MkdirTemp("", "gdpack-*")
	if err != nil {
		panic(err)
	}

	log.Println("Created temp dir:", tmp)

	for _, d := range append(deps, devDeps...) {
		if d.Spec.Git == nil {
			continue
		}

		spec := d.Spec.Git
		log.Println("Installing dependency:", spec.Git, "at branch", spec.Branch)

		path := filepath.Join(tmp, d.Name)

		err := git.Clone(context.Background(), *spec, path)
		if err != nil {
			panic(err)
		}
	}
}
