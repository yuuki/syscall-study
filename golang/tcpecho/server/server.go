package main

import (
	"fmt"
	"net"
	"os"
	"io"
)

func handleClient(conn net.Conn) error {
	if _, err := io.Copy(conn, conn); err != nil {
		return err
	}
	return nil
}

func Run(host string, port int) int {
	fmt.Printf("--> binding to %s:%d\n", host, port)

	addr, err := net.ResolveTCPAddr("tcp", fmt.Sprintf("%s:%d", host, port))
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		return -1
	}

	ln, err := net.ListenTCP("tcp", addr)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		return -1
	}

	for {
		conn, err := ln.Accept()
		if err != nil {
			fmt.Fprintf(os.Stderr, "%s\n", err)
			continue
		}
		fmt.Printf(".")

		if err := handleClient(conn); err != nil {
			fmt.Fprintf(os.Stderr, "%s\n", err)
			continue
		}
	}

	return 0
}
