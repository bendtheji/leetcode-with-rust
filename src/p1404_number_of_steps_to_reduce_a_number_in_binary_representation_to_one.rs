#![allow(dead_code)]

pub fn num_steps(s: String) -> i32 {
    let mut is_one = false;
    let mut one_count = 0;
    let mut index = s.len() as i32 - 1;
    let mut s = s.chars().collect::<Vec<char>>();
    let mut result = 0;

    while index >= 0 {
        if index == 0 && s[index as usize] == '1' && !is_one {
            return result;
        } else if s[index as usize] == '1' {
            is_one = true;
            one_count += 1;
            index -= 1;
        } else if s[index as usize] == '0' && is_one {
            result += one_count;
            result += 1;
            is_one = false;
            s[index as usize] = '1';
            one_count = 0;
        } else if s[index as usize] == '0' {
            result += 1;
            index -= 1;
        }
    }

    if is_one {
        result += one_count + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::p1404_number_of_steps_to_reduce_a_number_in_binary_representation_to_one::num_steps;

    #[test]
    fn test_one() {
        let s = String::from("1101");
        let expected = 6;
        let output = num_steps(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = String::from("10");
        let expected = 1;
        let output = num_steps(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = String::from("1");
        let expected = 0;
        let output = num_steps(s);
        assert_eq!(expected, output);
    }
}