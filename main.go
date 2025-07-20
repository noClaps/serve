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
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		if _, err := os.Stat(args.Directory + path + ".html"); err == nil {
			http.ServeFile(w, r, args.Directory+path+".html")
			return
		}
		fs.ServeHTTP(w, r)
	})

	fmt.Printf("Server started at http://localhost:%d\n", args.Port)
	if err := http.ListenAndServe(fmt.Sprintf(":%d", args.Port), nil); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
