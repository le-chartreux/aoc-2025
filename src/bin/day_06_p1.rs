use std::{fs, str::FromStr};

type Operand = u64;

#[derive(Debug, PartialEq)]
enum Operator {
    Sum,
    Multiply,
}

#[derive(Debug)]
struct ParseOperatorError {}
impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Sum),
            "*" => Ok(Operator::Multiply),
            _ => Err(ParseOperatorError {}),
        }
    }
}

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
    let mut input_file_lines = input_file_content.lines().peekable();

    let number_of_columns = input_file_lines
        .peek()
        .expect("can't read first line")
        .split_whitespace()
        .count();
    let mut all_operands: Vec<Vec<Operand>> = vec![Vec::new(); number_of_columns];

    while let Some(line) = input_file_lines.peek()
        && !(line.contains('+') || line.contains('*'))
    {
        for (i, number) in line.split_whitespace().enumerate() {
            all_operands[i].push(Operand::from_str(number).expect("can't parse number"));
        }
        input_file_lines.next();
    }

    let line = input_file_lines.next().expect("last line should exist");
    let operators = line
        .split_whitespace()
        .map(|operator| Operator::from_str(operator).expect("can't parse operator"));

    operators
        .zip(all_operands.drain(..))
        .map(|(operator, operands)| MathProblem { operator, operands })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_from_str_sum() {
        assert_eq!(Operator::from_str("+").unwrap(), Operator::Sum);
    }

    #[test]
    fn operator_from_str_err() {
        assert_eq!(Operator::from_str("+").unwrap(), Operator::Sum);
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
}
