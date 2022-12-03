package main

import (
	"fmt"
	"strings"
)

var priorities = map[string]int{
	"a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6, "g": 7, "h": 8, "i": 9, "j": 10, "k": 11, "l": 12, "m": 13, "n": 14, "o": 15, "p": 16, "q": 17, "r": 18, "s": 19, "t": 20, "u": 21, "v": 22, "w": 23, "x": 24, "y": 25, "z": 26, "A": 27, "B": 28, "C": 29, "D": 30, "E": 31, "F": 32, "G": 33, "H": 34, "I": 35, "J": 36, "K": 37, "L": 38, "M": 39, "N": 40, "O": 41, "P": 42, "Q": 43, "R": 44, "S": 45, "T": 46, "U": 47, "V": 48, "W": 49, "X": 50, "Y": 51, "Z": 52,
}

func splitRucksackContents(contents string) (string, string) {
	first_comp := contents[:len(contents)/2]
	sec_comp := contents[len(contents)/2:]

	return first_comp, sec_comp
}

func compareCompartments(first_comp string, sec_comp string) string {
	first_comp_rune := []rune(first_comp)
	sec_comp_rune := []rune(sec_comp)
	for i := 0; i < len(first_comp_rune); i++ {
		for j := 0; j < len(sec_comp_rune); j++ {
			if string(first_comp_rune[i]) == string(sec_comp[j]) {
				return string(first_comp[i])
			}
		}
	}
	return ""
}

func calculatePriorities(list_of_contents string) int {
	priority := 0
	sacks := strings.Split(list_of_contents, "\n")

	for _, element := range sacks { // For each loop that lets you iterate over index and element
		first_comp, sec_comp := splitRucksackContents(element)
		common_item := compareCompartments(first_comp, sec_comp)
		priority += priorities[common_item]
	}
	return priority
}

func main() {
	list_of_contents := "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
	priority := calculatePriorities(list_of_contents)

	fmt.Printf("Summed priority: %d", priority)
}
