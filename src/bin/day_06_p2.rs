use std::{fs, mem};

type Operand = u64;

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Multiply,
}

impl Operator {
    fn from_char(s: &char) -> Self {
        match s {
            '+' => Operator::Sum,
            '*' => Operator::Multiply,
            _ => panic!("failed to load operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct MathProblem {
    operator: Operator,
    operands: Vec<Operand>,
}

impl MathProblem {
    fn resolve(&self) -> Operand {
        match self.operator {
            Operator::Sum => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().product(),
        }
    }
}

fn main() {
    let problems = read_input("res/day_06_input.txt");
    let grand_total: Operand = problems.iter().map(MathProblem::resolve).sum();
    println!("Grand total is {grand_total}");
}

fn read_input(path: &str) -> Vec<MathProblem> {
    let input_file_content = fs::read_to_string(path).expect("failed to read file input");
    // Using char as the input file only contains ascii.
    let mut input_file_lines = input_file_content
        .lines()
        .rev() // Reverse so it's easier to read operands.
        .map(|line| line.chars().collect());

    let operators_line: Vec<char> = input_file_lines
        .next()
        .expect("failed to get operator line");

    let operands_lines: Vec<Vec<char>> = input_file_lines
        .rev() // Put back to original order.
        .collect();

    to_cephalopod_math_problems(operators_line, operands_lines)
}

fn to_cephalopod_math_problems(
    mut operators_lines: Vec<char>,
    mut operands_lines: Vec<Vec<char>>,
) -> Vec<MathProblem> {
    // Reverse everything to ease analysis (so encountering the operand means end of block).
    operators_lines.reverse();
    for operands_line in operands_lines.iter_mut() {
        operands_line.reverse();
    }

    let mut problems = Vec::new();
    let mut operands: Vec<Operand> = Vec::new();

    for (i, operator_char) in operators_lines.iter().enumerate() {
        let mut operand: Operand = 0;
        for (j, operand_chars) in operands_lines.iter().enumerate() {
            if operand_chars[i].is_whitespace() {
                // There is no whitespace in the middle, but if there is,
                // it'll be needed to think how to handle them.
                operand /= 10;
            } else {
                let digit = operand_chars[i]
                    .to_digit(10)
                    .expect("failed to convert operand_char to digit")
                    as u64;
                operand += 10_u64.pow((operands_lines.len() - j - 1) as u32) * digit;
            }
        }
        if operand != 0 {
            operands.push(operand);
        }

        if !operator_char.is_whitespace() {
            // End of block.
            problems.push(MathProblem {
                operator: Operator::from_char(operator_char),
                operands: mem::take(&mut operands),
            });
        }
    }

    // Put back to the original order.
    problems.reverse();

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_from_char_sum() {
        assert_eq!(Operator::from_char(&'+'), Operator::Sum);
    }

    #[test]
    fn operator_from_str_err() {
        assert_eq!(Operator::from_char(&'+'), Operator::Sum);
    }

    #[test]
    fn resolve_on_examples() {
        assert_eq!(
            MathProblem {
                operator: Operator::Multiply,
                operands: vec![123, 45, 6]
            }
            .resolve(),
            33210
        );

        assert_eq!(
            MathProblem {
                operator: Operator::Sum,
                operands: vec![328, 64, 98]
            }
            .resolve(),
            490
        );

        assert_eq!(
            MathProblem {
                operator: Operator::Multiply,
                operands: vec![51, 387, 215]
            }
            .resolve(),
            4243455
        );

        assert_eq!(
            MathProblem {
                operator: Operator::Sum,
                operands: vec![64, 23, 314]
            }
            .resolve(),
            401
        );
    }

    #[test]
    fn to_cephalopod_math_problems_on_example() {
        let example_operators_lines = "*   +   *   +  ".chars().collect();
        let example_operands_lines = vec![
            "123 328  51 64 ".chars().collect(),
            " 45 64  387 23 ".chars().collect(),
            "  6 98  215 314".chars().collect(),
        ];
        let math_problems =
            to_cephalopod_math_problems(example_operators_lines, example_operands_lines);
        assert_eq!(
            math_problems,
            vec![
                MathProblem {
                    operator: Operator::Multiply,
                    operands: vec![356, 24, 1],
                },
                MathProblem {
                    operator: Operator::Sum,
                    operands: vec![8, 248, 369]
                },
                MathProblem {
                    operator: Operator::Multiply,
                    operands: vec![175, 581, 32]
                },
                MathProblem {
                    operator: Operator::Sum,
                    operands: vec![4, 431, 623]
                }
            ]
        );
    }
}
