package main

import (
	"fmt"
)

func if_else(a int) (int) {
	if(a < 5) {
		fmt.Println("\t<5")
	} else {
		fmt.Println("\t>= 5")
	}
	return a
}


func switch_statement(a int, b string) {
	fmt.Println("switch_statement:")
	/* There's no fall through */
	switch a {
		case 0: fmt.Println("\t0")
	}
	switch b {
		case "hi": fmt.Println("\thi")
	}
	switch {
		case 0 == a : fmt.Println("\t... 0")
		case "hi" == b : fmt.Println("\t... hi")
	}
}


func main() {
	if_else(2)
	if_else(10)
	switch_statement(1,"hey")
	switch_statement(0,"hi")
}
