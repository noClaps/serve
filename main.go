package main

import (
	"flag"
	"fmt"
	"net/http"
	"os"
)

func main() {
	var port uint = 3000
	flag.UintVar(&port, "port", 3000, "The port to serve at.")

	flag.Parse()

	directory := flag.Arg(0)

	fs := http.FileServer(http.Dir(directory))
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		path := directory + r.URL.Path + ".html"
		if _, err := os.Stat(path); err == nil {
			http.ServeFile(w, r, path)
			return
		}
		fs.ServeHTTP(w, r)
	})

	fmt.Printf("Server started at http://localhost:%d\n", port)
	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
