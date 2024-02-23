#![allow(dead_code)]

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        match token.parse() {
            Ok(num) => stack.push(num),
            Err(_) => {
                let operand_two = stack.pop().unwrap();
                let operand_one = stack.pop().unwrap();
                let ans = match token.as_str() {
                    "*" => operand_one * operand_two,
                    "/" => operand_one / operand_two,
                    "+" => operand_one + operand_two,
                    "-" => operand_one - operand_two,
                    _ => unreachable!()
                };
                stack.push(ans);
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::p150_evaluate_reverse_polish_notation::eval_rpn;

    fn convert_to_vec_of_string(input: Vec<&str>) -> Vec<String> {
        input.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_one() {
        let input = vec!["4", "13", "5", "/", "+"];
        let input = convert_to_vec_of_string(input);
        assert_eq!(eval_rpn(input), 6);
    }

    #[test]
    fn test_two() {
        let input = vec!["2", "1", "+", "3", "*"];
        let input = convert_to_vec_of_string(input);
        assert_eq!(eval_rpn(input), 9);
    }

    #[test]
    fn test_three() {
        let input = vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"];
        let input = convert_to_vec_of_string(input);
        assert_eq!(eval_rpn(input), 22);
    }
}