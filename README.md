# 42-Ready-Set-boole
Discovering the basics of computer-related mathematics with Boolean Algebra and Sets Theory - Hive Helsinki project under 42 curriculum.

# Functions
Detailed instructions and constraints can be found in the [project's instructions](resources/ready_set_boole.en.pdf).
## 00 Adder
Sum of two natural numbers computed using only bitwise operators, assignment and comparison operators.

*Note: The incrementation operator (++ or += 1) is allowed only to increment the index of
a loop and must not be used to compute the result itself.*

| Adder           | `fn adder(a: u32, b: u32) -> u32;` |
|-----------------------|----------------------------------|
| Max Time Complexity:  | O(1)                             |
| Max Space Complexity: | O(1)                             |

## 01 Multiplier
Multiplication of two natural numbers computed using only bitwise operators, assignment and comparison operators.

*Note: The incrementation operator (++ or += 1) is allowed only to increment the index of
a loop and must not be used to compute the result itself.*

| Multiplier           | `fn multiplier(a: u32, b: u32) -> u32;` |
|-----------------------|----------------------------------|
| Max Time Complexity:  | O(1)                             |
| Max Space Complexity: | O(1)                             |

## 02 Gray code

Returns value of given integer in Gray code.

| Gray code           | `fn gray_code(n: u32) -> u32;` |
|-----------------------|----------------------------------|
| Max Time Complexity:  | N/A                             |
| Max Space Complexity: | N/A                             |


## 03 Boolean evaluation

Evaluates supplied propositional formula in reversed polish notation.

| Boolean evaluation           | `fn eval_formula(formula: &str) -> bool;` |
|-----------------------|----------------------------------|
| Max Time Complexity:  | O(n)                             |
| Max Space Complexity: | N/A                             |

## 04 Truth table

Prints truth table for supplied propositional formula in reversed polish notation.

| Boolean evaluation           | `fn print_truth_table(formula: &str);` |
|-----------------------|----------------------------------|
| Max Time Complexity:  | O(2^n)                             |
| Max Space Complexity: | N/A                             |