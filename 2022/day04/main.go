package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func format_input(input_file *os.File) [][][]string {
	formatted_input := [][][]string{}
	filescanner := bufio.NewScanner(input_file)
	filescanner.Split(bufio.ScanLines)
	for filescanner.Scan() {
		line := strings.Split(filescanner.Text(), ",")
		temp_arr := [][]string{{line[0]}, {line[1]}}
		formatted_input = append(formatted_input, temp_arr)
	}

	return formatted_input
}

func checkOverlapse(section_assignment [][][]string) int {
	overlapping_sections := 0
	for i := 0; i < len(section_assignment); i++ {
		section_first_elf, section_second_elf := strings.Split(section_assignment[i][0][0], "-"), strings.Split(section_assignment[i][1][0], "-")
		cast_first_1, _ := strconv.Atoi(section_first_elf[0])
		cast_first_2, _ := strconv.Atoi(section_first_elf[1])
		cast_sec_1, _ := strconv.Atoi(section_second_elf[0])
		cast_sec_2, _ := strconv.Atoi(section_second_elf[1])
		sec_first_elf := []int{cast_first_1, cast_first_2}
		sec_sec_elf := []int{cast_sec_1, cast_sec_2}
		if (sec_first_elf[0] <= sec_sec_elf[0] && sec_first_elf[1] >= sec_sec_elf[1]) || (sec_first_elf[0] >= sec_sec_elf[0] && sec_first_elf[1] <= sec_sec_elf[1]) {
			overlapping_sections += 1
			// fmt.Println("The sections", section_first_elf, "and", section_second_elf, " are overlapping")
		}

	}
	return overlapping_sections
}

func main() {
	input, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	formatted_input := format_input(input)
	overlapping_sections := checkOverlapse(formatted_input)
	fmt.Printf("%d sections are overlapping one another", overlapping_sections)
}
