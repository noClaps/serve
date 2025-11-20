package main

import (
	"flag"
	"fmt"
	"net/http"
	"os"
	"strings"
)

func main() {
	var port uint = 3000
	flag.UintVar(&port, "port", 3000, "The port to serve at.")

	path := strings.Split(os.Getenv("PWD"), "/")
	cwd := path[len(path)-1]
	hostname := fmt.Sprintf("%s.localhost", cwd)
	flag.StringVar(&hostname, "host", hostname, "The hostname to serve at.")

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

	fmt.Printf("Server started at http://%s:%d\n", hostname, port)
	if err := http.ListenAndServe(fmt.Sprintf("%s:%d", hostname, port), nil); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
