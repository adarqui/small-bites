package main

import (
	"fmt"
	"reflect"
)

func printit(x ...interface{}) {
	a := x
	for i := range a {
		t := reflect.TypeOf(a[i])
		fmt.Printf("{type:%v,value:%v}\n", t, a[i])
	}
}

func local_variables() {

	fmt.Println("go: Variables, values, and data types")

	/*
	 * Variables
	 */

	var country string = "USA"

	var _int8 int8 = 5
	var _int16 int16 = 6
	var _int32 int32 = 7
	var _int64 int64 = 8

	var _byte byte = 'c'
	var _bool bool = true

	/*
	 * Typo casting
	 */
	result := int64(_int8) + int64(_int16) + int64(_int32) + _int64

	printit(country, result, _int8, _int16, _int32, _int64, _byte, _bool)

	/*
	 * Group of declarations
	 */

	var (
		name string = "bob"
		age  int    = 33
	)

	printit(name, age)

	/*
	 * Const
	 */
	const pi = 3.14
	printit(pi)

	/*
	 * Arrays
	 */

	_bytes := []byte{1, 2, 3, 4}
	_strings := []string{"hey", "whats", "up"}
	printit(_bytes, _strings)
}



func structures() {
	type Message struct{
		x int
		y int
	}
	var m Message
	m.x = 1
	m.y = 2
}



func type_switch(t interface{}) {
	fmt.Println("type_switch:")
	switch t := t.(type) {
		default: fmt.Printf("\tunexptected type %T\n", t)
		case bool: fmt.Printf("\tbool %t\n", t)
		case int: fmt.Printf("\tint %d\n", t)
		case string: fmt.Printf("\tstring %s\n", t)
	}
}


type Buf struct {
	len int
}

func new_example() {
	/* allocates memory, zero's it: returns a ptr to newly allocated zero value of type T */
	fmt.Println("new_example:")
	buf := new(Buf) // *Buf
	var buf2 Buf // Buf
	fmt.Println("\t",buf,buf2)
}

func make_example() {
	/* creates initialized slices, channels, or maps only */
	a := make([]int,10,100)
	fmt.Println("make_example:")
	fmt.Println("\t",a)
}


func composite_literal() (*Buf) {
	var b Buf
	return &b
}


func main() {
	local_variables()
	structures()
	type_switch(1)
	type_switch(true)
	type_switch("hi")
	new_example()
	make_example()
	b := composite_literal()
	b.len = 0
}
