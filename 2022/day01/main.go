package main

import "strconv"

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
	for i := 0; i < len(array); i++ {
		sum += strconv.ParseInt(array[i])
	}
	return sum
}

func splitInventory(inventory []string) {
	min_index := 0
	max_index := 0
	return_array := []string{}
	for i := 0; i < len(inventory); i++ {
		if inventory[i] == "" {
			max_index = i
			if max_index != 0 {
				for j := min_index; j < max_index; j++ {
					append(return_array, inventory[min_index:max_index])
				}
			} else {
				for j := 0; j < max_index; j++ {
					append(return_array, inventory[0:max_index])
				}
			}

		}
	}
}

func main() {
	inventory := []string{"1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"}
}
