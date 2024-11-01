# 42-Ready-Set-boole
Discovering the basics of computer-related mathematics with Boolean Algebra
and Sets Theory - Hive Helsinki project under 42 curriculum.

# Exercises
Below is a high level description of implemented exercises. Detailed
instructions and constraints can be found in the
[project's instructions](resources/ready_set_boole.en.pdf).

## Usage
Each of the binaries can be run using `cargo run --bin exNN` where NN is
the exercise number. Or with `--release` to use the optimizations, but it
mostly doesn't matter :) Implemented tests can be run with `cargo test`.

## 00 Adder
Sum of two natural numbers computed using only bitwise operators,
assignment and comparison operators.

## 01 Multiplier
Multiplication of two natural numbers computed using only bitwise
operators, assignment and comparison operators.

## 02 Gray code
Returns value of given integer in Gray code.

## 03 Boolean evaluation
Evaluates supplied propositional formula in reverse polish notation.

## 04 Truth table
Prints truth table for supplied propositional formula
in reverse polish notation.

## 05 Negation Normal Form
Takes a propositional formula in reverse polish notation, and returns
an equivalent formula in Negation Normal Form (NNF), meaning that every
negation operators must be located right after a variable.

## 06 Conjunctive Normal Form
Takes a propositional formula in reverse polish notation, and returns
an equivalent formula in Conjunctive Normal Form (CNF). This means that
in the output, every negation must be located right after a variable
and every conjunction must be located at the end of the formula.

## 07 SAT
Takes a propositional formula in reverse polish notation
and tells whether it is satisfiable.

## 08 Powerset
Takes as input a set of integers, and returns its powerset.

## 09  Set evaluation (TODO)
Takes a propositional formula in reverse polish notation,
and a list of sets (each containing numbers), then evaluates this list
and returns the resulting set.

## 10 Curve
An inverse of a space-filling curve, used to encode spatial data into a line.
It takes a pair of coordinates in two dimensions and assigns a unique
value in the closed interval [0; 1] ∈ R.

## 11 Inverse function
This is the inverse function `f^(-1)` of the function `f` from the previous
exercise (so this time, this is a space-filling curve, used to decode data
from a line into a space).
