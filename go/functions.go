package main

import (
	"errors"
	"fmt"
)

func nothing() {
}

type Fn func(int) int

func inc(x int) int {
	return x + 1
}

func dec(x int) int {
	return x - 1
}

func first_class_fn(x int, f Fn) int {
	return f(x)
}

func variable_length_parameters(x ...string) {
	strings := x
	fmt.Printf("%v\n", strings)
}

func multiple_return_values(x, y int) (int, error) {
	if x > y {
		return x, nil
	} else if x < y {
		return y, nil
	}
	return 0, errors.New("arguments are equal")
}

func interface_param(x interface{}) interface{} {
	return x
}

func main() {

	fmt.Println("go: Functions")
	nothing()

	/*
	 * variable length parameters
	 */

	variable_length_parameters("hi", "yo", "bye")

	/*
	 * functions as first class citizens
	 */

	res := first_class_fn(5, inc)
	fmt.Printf("result: %v, %v, %v\n", res, first_class_fn(99, inc), first_class_fn(100, dec))

	max, err := multiple_return_values(3, 3)
	fmt.Printf("max %v, err %v\n", max, err)

	max, err = multiple_return_values(3, 1)
	fmt.Printf("max %v, err %v\n", max, err)

	val := interface_param(5)
	fmt.Printf("interface param: %v\n", val)
}
