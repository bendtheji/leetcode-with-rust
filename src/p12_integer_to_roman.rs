#![allow(dead_code)]

pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut ans = String::new();

    let thousands = num / 1000;

    for _ in 0..thousands {
        ans.push('M');
    }

    num = num % 1000;

    let hundreds = num / 100;

    match hundreds {
        4 => ans.push_str("CD"),
        9 => ans.push_str("CM"),
        x if x < 4 => {
            for _ in 0..x { ans.push('C'); }
        }
        x if x >= 5 && x < 9 => {
            ans.push('D');
            for _ in 0..x - 5 { ans.push('C'); }
        }
        _ => unreachable!()
    }

    num = num % 100;

    let tens = num / 10;

    match tens {
        4 => ans.push_str("XL"),
        9 => ans.push_str("XC"),
        x if x < 4 => {
            for _ in 0..x { ans.push('X'); }
        }
        x if x >= 5 && x < 9 => {
            ans.push('L');
            for _ in 0..x - 5 { ans.push('X'); }
        }
        _ => unreachable!()
    }
    num = num % 10;

    let ones = num;
    match ones {
        4 => ans.push_str("IV"),
        9 => ans.push_str("IX"),
        x if x < 4 => {
            for _ in 0..x { ans.push('I'); }
        }
        x if x >= 5 && x < 9 => {
            ans.push('V');
            for _ in 0..x - 5 { ans.push('I'); }
        }
        _ => unreachable!()
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::p12_integer_to_roman::int_to_roman;

    #[test]
    fn test_one() {
        let expected = "III".to_string();
        let output = int_to_roman(3);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = "LVIII".to_string();
        let output = int_to_roman(58);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let expected = "MCMXCIV".to_string();
        let output = int_to_roman(1994);

        assert_eq!(expected, output);
    }
}
