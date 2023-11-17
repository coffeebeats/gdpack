package main

import (
	"log"
	"reflect"

	"github.com/coffeebeats/gdpack/pkg/manifest"
)

func main() {
	m := manifest.Manifest{}

	a := manifest.Spec{Local: manifest.Local{Path: "./pkg/manifest"}}
	if err := m.Add("manifest", a); err != nil {
		panic(err)
	}

	log.Println(m)
	log.Println("--")

	if err := manifest.Write(&m, "./manifest.toml"); err != nil {
		panic(err)
	}

	log.Println("Wrote manifest contents to './manifest.toml'")

	parsed, err := manifest.ParseFile("./manifest.toml")
	if err != nil {
		panic(err)
	}

	log.Println("Parsed manifest from './manifest.toml'")

	if !reflect.DeepEqual(parsed, &m) {
		log.Println(parsed)
		log.Println("--")
		log.Println(&m)

		log.Fatal("mismatch")
	}

	log.Printf("%#v", parsed.List())
}
