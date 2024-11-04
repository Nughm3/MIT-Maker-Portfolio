+++
name = "Example Contest 2"
duration = 3600 # seconds
submission-cooldown = 60 # seconds

[scoring]
answer-score = 100
task-score = 100
subtask-score = 50
test-score = 5

[judge]
skip-count = 3 # how many TLE/MLE tests to tolerate before skipping the rest of the subtask

[judge.resource-limits]
cpu = 1 # seconds
cpu-tolerance = 0.1 # seconds
memory = 512_000_000 # bytes
memory-tolerance = 1000 # bytes

[[judge.languages]]
name = "Python 3"
filename = "submission.py"
# Python is interpreted so there is no `compile` command
run = ["python3", "./submission.py"]
+++

Welcome to another example contest!

## Rules

### Languages

Contestants shall answer in Python 3.

### Allowed

- [Python 3 documentation](https://docs.python.org/3)
- Built-in search functionality on the above sites
- Your editor (with non-AI autocomplete)
- Python interpreter

### Disallowed

- Search engines (e.g. Google)
- Other websites (e.g. Stack Overflow)
- Generative AI (e.g. ChatGPT, Poe, GitHub Copilot, etc.)
- Communication with other contestants or third parties
