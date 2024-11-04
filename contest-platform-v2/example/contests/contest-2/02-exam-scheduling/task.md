+++
name = "Exam Scheduling"
difficulty = "Medium"
+++

In a university, there are $n$ exams scheduled throughout the day. You know the starting and ending time of each exam. What is the maximum number of exams you can take entirely?

### Input

The first input line has an integer $n$: the number of exams.
After this, there are $n$ lines that describe the exams. Each line has two integers $a$ and $b$: the starting and ending times of an exam.

### Output

Print one integer: the maximum number of exams you can take entirely.

### Example

Input:

```
3
3 5
4 9
5 8
```

Correct output:

```
2
```

### Constraints

**5 subtasks**

For all subtasks:

- $1 \lt n \le 10^5$
- $\forall a, b \; 1 \lt a \lt b \le 10^9$
