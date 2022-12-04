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

func formatSectionPairs(first_section []string, second_section []string) ([]int, []int) {
	cast_first_1, _ := strconv.Atoi(first_section[0])
	cast_first_2, _ := strconv.Atoi(first_section[1])
	cast_sec_1, _ := strconv.Atoi(second_section[0])
	cast_sec_2, _ := strconv.Atoi(second_section[1])
	first_formatted := []int{cast_first_1, cast_first_2}
	second_formatted := []int{cast_sec_1, cast_sec_2}
	return first_formatted, second_formatted
}

func checkFullOverlapse(section_assignment [][][]string) int {
	overlapping_sections := 0
	for i := 0; i < len(section_assignment); i++ {
		section_first_elf, section_second_elf := strings.Split(section_assignment[i][0][0], "-"), strings.Split(section_assignment[i][1][0], "-")
		sec_first_elf, sec_sec_elf := formatSectionPairs(section_first_elf, section_second_elf)
		if (sec_first_elf[0] <= sec_sec_elf[0] && sec_first_elf[1] >= sec_sec_elf[1]) || (sec_first_elf[0] >= sec_sec_elf[0] && sec_first_elf[1] <= sec_sec_elf[1]) {
			overlapping_sections += 1
		}

	}
	return overlapping_sections
}

func checkPartialOverlapse(section_assignment [][][]string) int {
	overlapping_sections := 0
	for i := 0; i < len(section_assignment); i++ {
		section_first_elf, section_second_elf := strings.Split(section_assignment[i][0][0], "-"), strings.Split(section_assignment[i][1][0], "-")
		sec_first_elf, sec_sec_elf := formatSectionPairs(section_first_elf, section_second_elf)
		if (sec_first_elf[0] <= sec_sec_elf[0] && sec_first_elf[1] >= sec_sec_elf[1]) ||
			(sec_first_elf[0] >= sec_sec_elf[0] && sec_first_elf[1] <= sec_sec_elf[1]) ||
			(sec_first_elf[0] <= sec_sec_elf[0] && sec_sec_elf[0] <= sec_first_elf[1] || sec_first_elf[0] <= sec_sec_elf[1] && sec_sec_elf[1] <= sec_first_elf[1]) {
			overlapping_sections += 1
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
	input.Close()
	fully_overlapping_sections := checkFullOverlapse(formatted_input)
	partial_overlapses := checkPartialOverlapse(formatted_input)
	fmt.Printf("%d sections are fully overlapping one another\n", fully_overlapping_sections)
	fmt.Printf("%d sections are fully or partioally overlapping one another", partial_overlapses)
}
