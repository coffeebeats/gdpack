package main

import (
	"log"

	"github.com/coffeebeats/gdpack/pkg/manifest"
)

func main() {
	m := manifest.Manifest{}

	a := manifest.Spec{Local: &manifest.Local{Path: "./pkg/manifest"}}
	if err := m.Add("manifest", a); err != nil {
		panic(err)
	}

	b := manifest.Spec{Git: &manifest.Git{Git: "git@github.com:coffeebeats/gdpack.git", Branch: "main"}}
	if err := m.AddDev("gdpack", b); err != nil {
		panic(err)
	}

	// c := manifest.Spec{Git: &manifest.Git{Git: "git@github.com:coffeebeats/gdpack.git", Branch: "next"}}
	// if err := m.AddDevWithTarget("gdpack", c, "x86_64"); err != nil {
	// 	panic(err)
	// }

	d := manifest.Spec{Local: &manifest.Local{Path: "./pkg/manifest/next"}}
	if err := m.AddWithTarget("gdpack", d, "x86_64"); err != nil {
		panic(err)
	}

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

	// if !reflect.DeepEqual(parsed, &m) {
	// 	log.Printf("%#v\n", parsed)
	// 	log.Println("--")
	// 	log.Printf("%#v\n", &m)

	// 	log.Fatal("mismatch")
	// }

	deps, err := parsed.ListWithTarget("x86_64")
	if err != nil {
		panic(err)
	}

	devDeps, err := parsed.ListDevWithTarget("x86_64")
	if err != nil {
		panic(err)
	}

	log.Printf("%#v", deps)
	log.Printf("%#v", devDeps)
}
