package main

import (
	"log"
	"reflect"

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

	log.Println(m)
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
		log.Println(parsed)
		log.Println("--")
		log.Println(&m)

		log.Fatal("mismatch")
	}

	log.Printf("%#v", parsed.List())
}
