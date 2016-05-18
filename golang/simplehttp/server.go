package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func handleClient(conn net.Conn) error {
	r := bufio.NewReader(conn)
	_, err := r.ReadByte()
	if err != nil {
		return err
	}
	w := bufio.NewWriter(conn)
	_, err = w.WriteString("HTTP/1.0 200 OK\n")
	if err != nil {
		return err
	}
	w.Flush()

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

		conn.Close()
	}

	return 0
}
