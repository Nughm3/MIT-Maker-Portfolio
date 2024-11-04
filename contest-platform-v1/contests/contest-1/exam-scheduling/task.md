---
name: Exam Scheduling
examples:
- input: "3\n3 5\n4 9\n5 8\n"
  output: "2\n"
  comment: "In this example, the starting and ending times of the exams are:\n```\n3 5\n4 9\n5 8\n```\nYou can take the first and third exams entirely, as they do not overlap. The first exam is from 3 to 5, and the third exam is from 5 to 8. Thus, the maximum number of exams you can take entirely is 2.\n"
subtasks:
- tests: 10
- tests: 10
- tests: 10
- tests: 10
- tests: 10
constraints:
- $1 \lt n \le 10^5$
- $\forall a, b \; 1 \lt a \lt b \le 10^9$
difficulty: Medium
---

In a university, there are $n$ exams scheduled throughout the day. You know the starting and ending time of each exam. What is the maximum number of exams you can take entirely?

### Input

The first input line has an integer $n$: the number of exams.
After this, there are $n$ lines that describe the exams. Each line has two integers $a$ and $b$: the starting and ending times of an exam.

### Output

Print one integer: the maximum number of exams you can take entirely.
