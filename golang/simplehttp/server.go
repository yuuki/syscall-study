package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"os/signal"
	"syscall"
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

	sigs := make(chan os.Signal, 1)

	signal.Notify(sigs, syscall.SIGCHLD)
	go func() {
		<-sigs
		var status syscall.WaitStatus
		var rusage syscall.Rusage
		_, err := syscall.Wait4(-1, &status, 0, &rusage)
		if err != nil {
			fmt.Fprintf(os.Stderr, "%s\n", err)
			return
		}
	}()

	for {
		conn, err := ln.Accept()
		if err != nil {
			fmt.Fprintf(os.Stderr, "%s\n", err)
			continue
		}
		fmt.Printf(".")

		ret, _, errno := syscall.Syscall(syscall.SYS_FORK, 0, 0, 0)
		if errno != 0 {
			return int(errno)
		}
		if ret != 0 {
			conn.Close() // parent process close
			continue
		}

		if err := handleClient(conn); err != nil {
			fmt.Fprintf(os.Stderr, "%s\n", err)
			continue
		}

		conn.Close() // child process close
		os.Exit(0) // child process exit
	}

	return 0
}
