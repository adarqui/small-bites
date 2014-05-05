package main

import (
	"github.com/shavac/readline"
	"github.com/andrew-d/go-termutil"
	"os/signal"
	"os"
	"log"
)

const (
	HISTFILE = ".history.go"
)

func main() {
	prompt := "prompt> "

	println(os.Stdin, os.Stdout, os.Stderr)
	if termutil.Isatty(os.Stdin.Fd()) {
		println("TTY!")
	}

	readline.ReadHistoryFile(HISTFILE)

	sigch := make(chan os.Signal, 1)
	signal.Notify(sigch, os.Interrupt, os.Kill)
	go func() {
		_ = <-sigch
		println("the end!");
		err := readline.WriteHistoryFile(HISTFILE)
		log.Println(err)
		os.Exit(0)
	}()

	L:
	for {
		switch result := readline.ReadLine(&prompt); true {
			case result == nil:
				println()
				break L
			case *result != "":
				println(*result)
				readline.AddHistory(*result);
		}
	}

}
