---
name: Missing Number
examples:
- input: "5\n2 3 1 5"
  output: "4\n"
subtasks:
- tests: 5
  constraints:
  - $N \le 100$
- tests: 5
  constraints:
  - $N \le 10^5$
- tests: 4
constraints:
- $2 \le n \le 2 \cdot 10^5$
difficulty: Medium
---

You are given all numbers between $1, 2, \dots, n$ except one. Your task is to find the missing number.

## Input

The first input line contains an integer $n$.

The second line contains $n − 1$ numbers. Each number is distinct and between $1$ and $n$ (inclusive).

## Output

Print the missing number.

[Source](https://cses.fi/problemset/task/1083)
