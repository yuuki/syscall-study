package main

import (
	"fmt"
	"net"
	"os"
)

func handleClient(net.Conn) error {
	if _, err := io.Copy(conn, conn); err != nil {
		return err
	}
	return nil
}

func Run(host string, port int) int {
	fmt.Printf("--> binding to %s:%d\n", host, port)

	ln, err := net.TCPListen("tcp", fmt.Sprintf("%s:%d", host, port))
	if err != nil {
		fmt.Fprintf(os.Stderr, err)
		return -1
	}

	for {
		conn, err := ln.Accept()
		if err != nil {
			fmt.Fprintf(os.Stderr, err)
			continue
		}
		if err := handleClient(conn); err != nil {
			fmt.Fprintf(os.Stderr, err)
			continue
		}
	}

	return 0
}
