package main

import (
	"fmt"
	"reflect"
)

func printit(x ... interface{}) {
	a := x
	for i := range a {
		t := reflect.TypeOf(a[i])
		fmt.Printf("{type:%v,value:%v}\n",t,a[i])
	}
}

func main() {

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

	printit(country,result,_int8,_int16,_int32,_int64,_byte,_bool)


	/*
	 * Group of declarations
	 */

	var (
		name string = "bob"
		age int = 33
	)

	printit(name,age)


	/*
	 * Const
	 */
	 const pi = 3.14
	 printit(pi)


	/*
	 * Arrays
	 */

	_bytes := []byte{1,2,3,4}
	_strings := []string{"hey","whats","up"}
	printit(_bytes,_strings)
}
