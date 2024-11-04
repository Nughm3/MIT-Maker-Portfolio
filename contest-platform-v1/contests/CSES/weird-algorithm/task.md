---
name: Weird Algorithm
examples:
- input: "3\n"
  output: "3 10 5 16 8 4 2 1\n"
  comment: "Make sure to include the initial $N$ and $1$"
subtasks:
- tests: 5
  constraints:
  - $N \le 100$
- tests: 5
  constraints:
  - $N \le 10^5$
- tests: 4
constraints:
- $1 \le N \le 10^6$
difficulty: Easy
---

Consider an algorithm that takes as input a positive integer $n$. If $n$ is even, the algorithm divides it by two, and if $n$ is odd, the algorithm multiplies it by three and adds one. The algorithm repeats this, until $n$ is one. For example, the sequence for $n = 3$ is as follows:

$$
3 \rightarrow 10 \rightarrow 5 \rightarrow 16 \rightarrow 8 \rightarrow 4 \rightarrow 2 \rightarrow 1
$$

Your task is to simulate the execution of the algorithm for a given value of $n$.

## Input

The only input line contains an integer $n$.

## Output

Print a line that contains all values of $n$ during the algorithm.

[Source](https://cses.fi/problemset/task/1068)
