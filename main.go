package main

import (
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strings"

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
		absPath, err := filepath.Abs(filepath.Join(args.Directory, r.URL.Path, ".html"))
		if err != nil || !strings.HasPrefix(absPath, args.Directory) {
			http.Error(w, "Invalid path", http.StatusBadRequest)
			return
		}
		if _, err := os.Stat(absPath); err == nil {
			http.ServeFile(w, r, absPath)
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
