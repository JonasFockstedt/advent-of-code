package main

import "fmt"

/*
A - Opponent Rock
B - Opponent Paper
C - Opponent Scissor
X - Me Rock
Y - Me Paper
Z - Me Scissor

1 point for Rock, 2 points for Paper, 3 points for Scissors
0 points for loss, 3 points for draw, 6 points for win

Input of game:
|A | Y|
|B | X|
|C | Z|
*/

func decideWinner(round [3][2]string) {
	points := 0
	for i := 0; i < len(round); i++ {
		if round[i][0] == "A" && round[i][1] == "X" { // Opponent rock, me rock
			points += 4
		} else if round[i][0] == "A" && round[i][1] == "Y" { // Opponent rock, me paper
			points += 8
		} else if round[i][0] == "A" && round[i][1] == "Z" { // Opponent rock, me scissor
			points += 3
		} else if round[i][0] == "B" && round[i][1] == "X" { // Opponent paper, me rock
			points += 1
		} else if round[i][0] == "B" && round[i][1] == "Y" { // Opponent paper, me paper
			points += 5
		} else if round[i][0] == "B" && round[i][1] == "Z" { // Opponent paper, me scissor
			points += 9
		} else if round[i][0] == "C" && round[i][1] == "X" { // Opponent scissor, me rock
			points += 7
		} else if round[i][0] == "C" && round[i][1] == "Y" { // Opponent scissor, me paper
			points += 2
		} else if round[i][0] == "C" && round[i][1] == "Z" { // Opponent scissor, me scissor
			points += 6
		}
	}
	fmt.Printf("Estimated points from this match are %d", points)
}

func main() {
	strategy_guide := [3][2]string{{"A", "Y"}, {"B", "X"}, {"C", "Z"}}
	decideWinner(strategy_guide)
}
