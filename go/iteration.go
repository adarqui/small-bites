package main

import (
	"fmt"
)


func for_loop(a int) {
	fmt.Println("for_loop:")
	sum := 0
	for i := 0; i < a ; i++ {
		sum += i
	}
	fmt.Printf("sum: %v\n", sum)
}



func for_loop_over_array(a []int) {
	fmt.Println("for_loop_over_array:")
	for x:= range(a) {
		fmt.Println("\t=",x,a[x])
	}
}

func for_loop_over_array_delete_members(a []int, b []int) {
	fmt.Println("for_loop_over_array_delete_members:")
	for key, value := range b {
		fmt.Println("\tdeleting",key,value,a)
//		delete(a,key)
		// ?
	}

	for _, value := range b {
		fmt.Println("\tvalue=",value)
	}
	for_loop_over_array(a)
}


func main() {
	for_loop(10)
	numbers := []int{1,2,3,4,5}
	for_loop_over_array(numbers)
	indexes := []int{1,3}
	for_loop_over_array_delete_members(numbers, indexes)
}
