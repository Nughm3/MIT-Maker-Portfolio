---
name: Sports Day
examples:
- input: "5\nXPPXGDDG\nGGGGDDDD\nDPDPGXXG\nXGXPDPDG\nPGXDDXPG\n"
  output: "Pegasus\nPhoenix\nDragon\nGriffin\n"
  comment: "The 2nd line of results has been tampered with and should be ignored.\nNote: example data was randomly generated."
subtasks:
- tests: 10
  constraints:
  - $n \le 10$, and the results have not been tampered with at all. 
- tests: 10
  constraints:
  - $n \le 100$ 
- tests: 10
  constraints:
  - $n \le 1000$ 
- tests: 10
  constraints:
  - $n \le 10^4$ 
- tests: 10
  constraints:
  - $n \le 10^5$ 
constraints:
- There are no ties
difficulty: Easy
---

The Sports Day race results have been collected, and you've been tasked with calculating who's won! There are $n$ race records in the data, with eight runners in each race, two from each of the four houses. House A beats House B if House A's runners have a **higher ranking in more races** in comparison to House B.

However, upon inspecting the results manually, you suspect that the results have been tampered with. Some races appear to have more runners from some houses (for example 3 Dragons and only 1 Pegasus). **These results must not be counted**.

### Input

The first line contains an integer $n$. The following $n$ lines each contain a string of 8 characters, containing the rankings for an individual race. The 1st character corresponds to the house in 1st place, the 2nd corresponds to the house in 2nd place, and so on.

In the data, the houses are abbreviated to their house letters: G for Griffin, D for Dragon, P for Pegasus, and X for Phoenix.

### Output 

Output the four houses, from 1st place to 4th place.
