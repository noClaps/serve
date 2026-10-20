package main

import (
	"fmt"
	"log"
	"mime"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"slices"
)

func main() {
	args := os.Args[1:]
	if slices.Contains(args, "-h") || slices.Contains(args, "--help") {
		fmt.Println(`Usage: serve <directory>

A simple HTTP server for serving a directory.

Arguments:
  <directory>  The directory to serve.

Options:
  -h, --help   Show this help and exit.`)
		return
	}
	if len(args) == 0 {
		log.Fatalln("Missing required argument: <directory>")
		os.Exit(1)
	}

	directory := args[0]
	port := 3000

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		reqPath := r.URL.Path
		filePath := filepath.Join(directory, reqPath)
		log.Println("Serving ", filePath)
		f, err := os.ReadFile(filePath)
		if err == nil {
			w.Header().Add("Content-Type", mime.TypeByExtension(filePath))
			w.Write(f)
			return
		}
		f, err = os.ReadFile(filePath + ".html")
		if err == nil {
			w.Header().Add("Content-Type", "text/html")
			w.Write(f)
			return
		}
		f, err = os.ReadFile(filepath.Join(filePath, "index.html"))
		if err == nil {
			w.Header().Add("Content-Type", "text/html")
			w.Write(f)
			return
		}
		f, err = os.ReadFile(filepath.Join(directory, "404.html"))
		if err == nil {
			w.Header().Add("Content-Type", "text/html")
			w.WriteHeader(404)
			w.Write(f)
			return
		}
		w.WriteHeader(404)
		w.Write([]byte("Not found"))
	})

	for {
		conn, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
		if err == nil {
			conn.Close()
			break
		}
		port += 1
	}

	fmt.Printf("Serving %s at http://localhost:%d\n", directory, port)

	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		log.Fatalln(err)
	}
}
