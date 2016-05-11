package main

import (
	"flag"
	"os"
)

func main() {
	var host string
	var port int
	flag.StringVar(&host, "host", "127.0.0.1", "bind hostname or ipaddr")
	flag.IntVar(&port, "port", 10020, "bind port")

	flag.Parse()

	os.Exit(Run(host, port))
}
