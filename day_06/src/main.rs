/*
https://adventofcode.com/2025/day/6
--- Day 6: Trash Compactor ---
 */
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

fn sum_problems(col_numbers: &Vec<Vec<u64>>, operators: &Vec<Operator>) -> u64 {
    let problems = operators.len();
    assert!(col_numbers.len() == problems);

    let mut sum: u64 = 0;
    for (p, &op) in operators.into_iter().enumerate() {
        let problem;
        match op {
            Operator::Add => {
                problem = col_numbers[p].iter().fold(0, |acc, num| acc + num);
            }
            Operator::Mul => {
                problem = col_numbers[p].iter().fold(1, |acc, num| acc * num);
            }
        }
        sum += problem;
    }

    sum
}

fn main() {
    let mut row_numbers = Vec::<Vec<u64>>::new();
    let mut operators = Vec::<Operator>::new();

    let mut lines = io::stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        // split_whitespace() correcly returns real element even if multiple spaces separate them.
        // contrary to split(' ') which would return many empty strings when padding is used.
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Final line ?
        if tokens[0] == "+" || tokens[0] == "*" {
            // final row with operators
            operators = tokens
                .iter()
                .map(|o| {
                    if *o == "+" {
                        Operator::Add
                    } else if *o == "*" {
                        Operator::Mul
                    } else {
                        panic!("Unknown operator {o}")
                    }
                })
                .collect();
            break;
        }
        // Normal line of numbers
        let numbers = tokens.iter().map(|n| u64::from_str(n).unwrap()).collect();
        row_numbers.push(numbers);
    }

    // Transpose the numbers matrix to represent columns.
    let mut col_numbers = Vec::<Vec<u64>>::new();
    let rows = row_numbers.len();
    let problems = row_numbers[0].len();

    for p in 0..problems {
        let mut one_column = Vec::<u64>::new();
        for r in 0..rows {
            one_column.push(row_numbers[r][p]);
        }
        col_numbers.push(one_column);
    }

    let grand_total = sum_problems(&col_numbers, &operators);
    println!("Part 1 = {grand_total}");
}
