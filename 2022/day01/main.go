package main

import "fmt"

// import "strconv"

var elf_nr int = 0

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
		temp := array[i]
		fmt.Println(temp)
		// sum += strconv.ParseInt(temp)
	}
	return sum
}

func splitInventory(inventory []string) {
	min_index := 0
	max_index := 0
	// return_array := []string{}

	largest_cal := 0
	summed_cal := 0
	for i := 0; i < len(inventory); i++ {
		if inventory[i] == "" {
			max_index = i
			if max_index != 0 {
				slice_arr := inventory[min_index:max_index]
				summed_cal = sumArrayValues(slice_arr)
			} else {
				slice_arr := inventory[0:max_index]
				summed_cal = sumArrayValues(slice_arr)
				// temp_arr := make([]string, max_index)
				// copy(temp_arr, slice_arr)
				// return_array = append(return_array, temp_arr)
			}
			min_index = i
		}
		if summed_cal > largest_cal {
			elf_nr = i
		}
	}
}

func main() {
	inventory := []string{"1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"}
	splitInventory(inventory)
}
