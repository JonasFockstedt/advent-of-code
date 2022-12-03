package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

/*
A - Opponent Rock
B - Opponent Paper
C - Opponent Scissor
X - I need to lose
Y - Needs to be a draw
Z - I need to win

1 point for Rock, 2 points for Paper, 3 points for Scissors
0 points for loss, 3 points for draw, 6 points for win

Input of game:
|A | Y|
|B | X|
|C | Z|
*/

func decideWinner(round [][]string) {
	points := 0
	for i := 0; i < len(round); i++ {
		if round[i][0] == "A" && round[i][1] == "X" { // Opponent rock, I need to lose, so I pick scissors
			points += 3
		} else if round[i][0] == "A" && round[i][1] == "Y" { // Opponent rock, needs to be a draw, I pick rock
			points += 4
		} else if round[i][0] == "A" && round[i][1] == "Z" { // Opponent rock, I need to win, so I pick papers
			points += 8
		} else if round[i][0] == "B" && round[i][1] == "X" { // Opponent paper, I need to lose, so I pick rock
			points += 1
		} else if round[i][0] == "B" && round[i][1] == "Y" { // Opponent paper, needs to be a draw, so I pick papers
			points += 5
		} else if round[i][0] == "B" && round[i][1] == "Z" { // Opponent paper, I need to win, so I pick scissors
			points += 9
		} else if round[i][0] == "C" && round[i][1] == "X" { // Opponent scissor, I need to lose, so I pick papers
			points += 2
		} else if round[i][0] == "C" && round[i][1] == "Y" { // Opponent scissor, needs to be a draw, so I pick scissors
			points += 6
		} else if round[i][0] == "C" && round[i][1] == "Z" { // Opponent scissor, I need to win, so I pick rock
			points += 7
		}
	}
	fmt.Printf("Estimated points from this match are %d", points)
}

func main() {
	inputs := [][]string{}
	input_file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	fileScanner := bufio.NewScanner(input_file)
	fileScanner.Split(bufio.ScanLines)
	for fileScanner.Scan() {
		line := strings.Split(fileScanner.Text(), " ")
		temp_arr := []string{line[0], line[1]}
		inputs = append(inputs, temp_arr)

	}
	input_file.Close()

	decideWinner(inputs)
}
