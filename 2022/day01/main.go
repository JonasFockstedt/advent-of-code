package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

var elf_most_cal int = 0

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

func findElf(inventory []string) (int, []int) {
	min_index := 0
	max_index := 0
	largest_cal := 0
	summed_cal := 0
	elf_nr := 0
	summed_cal_arr := []int{}
	for i := 0; i < len(inventory); i++ {
		if inventory[i] == "" {
			elf_nr += 1
			max_index = i
			if max_index != 0 {
				slice_arr := inventory[min_index:max_index]
				summed_cal = sumArrayValues(slice_arr)
				summed_cal_arr = append(summed_cal_arr, summed_cal)
			} else {
				slice_arr := inventory[0:max_index]
				summed_cal = sumArrayValues(slice_arr)
				summed_cal_arr = append(summed_cal_arr, summed_cal)
			}
			min_index = i + 1
		}
		if i == len(inventory)-1 { // If the second last element of the array is an empty string, and the last element is a value
			elf_nr += 1
			last_element := inventory[len(inventory)-1]
			summed_cal, _ = strconv.Atoi(last_element) // Convert from string to int
			summed_cal_arr = append(summed_cal_arr, summed_cal)
		}
		if summed_cal > largest_cal {
			largest_cal = summed_cal
			elf_most_cal = elf_nr
		}
	}
	sort.Ints(summed_cal_arr[:])
	return largest_cal, summed_cal_arr
}

func topThreeElfes(cal_array []int) int {
	top_three := cal_array[len(cal_array)-3:]
	sum := 0
	for i := 0; i < len(top_three); i++ {
		sum += top_three[i]
	}
	return sum
}

func main() {
	input_file, err := os.Open("input.txt")
	inventory := []string{}
	if err != nil {
		log.Fatal(err)
	}
	fileScanner := bufio.NewScanner(input_file)
	for fileScanner.Scan() {
		inventory = append(inventory, fileScanner.Text())
	}
	most_cal, cal_array := findElf(inventory)
	total_top_three := topThreeElfes(cal_array)
	fmt.Printf("Elf number %d is carrying the most calories (%d)\nTe top three elfes are carrying a total of %d calories.", elf_most_cal, most_cal, total_top_three)
}
