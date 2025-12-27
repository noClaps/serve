package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
)

func main() {
	var port uint = 3000
	flag.UintVar(&port, "port", 3000, "The port to serve at.")

	flag.Parse()

	directory := flag.Arg(0)
	server := http.Server{Addr: fmt.Sprintf(":%d", port)}

	fs := http.FileServer(http.Dir(directory))
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		path := directory + r.URL.Path + ".html"
		if _, err := os.Stat(path); err == nil {
			http.ServeFile(w, r, path)
			return
		}
		fs.ServeHTTP(w, r)
	})

	fmt.Printf("Server started at http://localhost%s\n", server.Addr)
	if err := server.ListenAndServe(); err != nil {
		log.Fatalln(err)
	}
}
