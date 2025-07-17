package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/noclaps/applause"
)

type Args struct {
	Directory string `help:"The directory to serve."`
	Port      int    `type:"option" short:"p" help:"The port to serve at."`
}

func main() {
	args := Args{Port: 3000}
	if err := applause.Parse(&args); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fs := http.FileServer(http.Dir(args.Directory))
	http.Handle("/", fs)

	fmt.Printf("Server started at http://localhost:%d\n", args.Port)
	if err := http.ListenAndServe(fmt.Sprintf(":%d", args.Port), nil); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
