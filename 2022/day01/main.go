package main

import (
	"fmt"
	"strconv"
)

var elf_most_cal int = 0

func findCalories(args ...[]string) (int, int) {
	max := 0
	index := 0
	for i := 0; i < len(args); i++ {
		temp_array := sumArrayValues(args[i])
		if temp_array > max {
			max = temp_array
			index = i
		}
	}
	return max, index
}

func sumArrayValues(array []string) int {
	sum := 0
	integer := 0
	for i := 0; i < len(array); i++ {
		temp := array[i]
		integer, _ = strconv.Atoi(temp)
		sum += integer
	}
	return sum
}

func splitInventory(inventory []string) {
	min_index := 0
	max_index := 0
	largest_cal := 0
	summed_cal := 0
	elf_nr := 0
	for i := 0; i < len(inventory); i++ {
		if inventory[i] == "" {
			elf_nr += 1
			max_index = i
			if max_index != 0 {
				slice_arr := inventory[min_index:max_index]
				// fmt.Println(slice_arr)
				summed_cal = sumArrayValues(slice_arr)
				fmt.Println(summed_cal)
			} else {
				slice_arr := inventory[0:max_index]
				// fmt.Println(slice_arr)
				summed_cal = sumArrayValues(slice_arr)
				fmt.Println(summed_cal)

			}
			min_index = i
		}
		if summed_cal > largest_cal {
			elf_most_cal = elf_nr
		}
	}
}

func main() {
	inventory := []string{"1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"}
	splitInventory(inventory)
	fmt.Printf("Elf number %d is carrying the most calories", elf_most_cal)
}
