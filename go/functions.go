package main

import (
	"errors"
	"fmt"
	"os"
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


func lots_of_return_values(a int, b int, c int, d int, e int) (int, int, int, int, int) {
	return a,b,c,d,e
}

func named_result_parameters() (key, value int) {
	value = 1
	key = 1
	return key,value
}


func defer_example(a string) {
	fmt.Println("defer_example:")
	f, err := os.Open(a)

	if err != nil {
		fmt.Println("\terr=",err)
		return
	}

	defer func() {
		fmt.Println("\tdeferred close")
		f.Close()
	}()

	defer func() {
		fmt.Println("\textra defer")
		a = "hi"
	}()
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


	a,b,c,d,e := lots_of_return_values(1,2,3,4,5)
	fmt.Println("lots_of_return_values:\n\t",a,b,c,d,e)

	key,value := named_result_parameters()
	fmt.Println("named_result_parameters:\n\t",key,value)

	defer_example("/tmp/t")
}
