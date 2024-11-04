+++
name = "Example Contest 1"
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
name = "C 99"
filename = "submission.c"
compile = ["gcc", "./submission.c", "-std=c99", "-O3", "-o", "./submission"]
run = ["./submission"]

[[judge.languages]]
name = "C++ 17"
filename = "submission.cpp"
compile = ["g++", "./submission.cpp", "-std=c++17", "-O3", "-o", "./submission"]
run = ["./submission"]
+++

Welcome to an example contest!

## Rules

### Languages

Contestants shall answer in C 99 or C++ 17.

### Allowed

- [C/C++ reference](https://cppreference.com/)
- Built-in search functionality on the above sites
- Your editor (with non-AI autocomplete)
- C/C++ compilers

### Disallowed

- Search engines (e.g. Google)
- Other websites (e.g. Stack Overflow)
- Generative AI (e.g. ChatGPT, Poe, GitHub Copilot, etc.)
- Communication with other contestants or third parties
