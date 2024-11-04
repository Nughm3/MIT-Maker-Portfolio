---
name: Repetitions
examples:
- input: "ATTCGGGA\n"
  output: "3\n"
subtasks:
- tests: 4
- tests: 4
- tests: 4
constraints:
- $1 \le n \le 10^6$
difficulty: Hard
---

You are given a DNA sequence: a string consisting of characters A, C, G, and T. Your task is to find the longest repetition in the sequence. This is a maximum-length substring containing only one type of character.

## Input

The only input line contains a string of $n$ characters.

## Output

Print one integer: the length of the longest repetition.

[Source](https://cses.fi/problemset/task/1069)
