#![allow(dead_code)]

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = vec![0; nums.len()];
    let mut postfix = vec![0; nums.len()];

    for i in 0..nums.len() {
        if i == 0 { prefix[0] = nums[0]; } else { prefix[i] = nums[i] * prefix[i - 1]; }
    }

    for i in (0..nums.len()).rev() {
        if i == nums.len() - 1 { postfix[nums.len() - 1] = nums[nums.len() - 1]; } else { postfix[i] = nums[i] * postfix[i + 1]; }
    }


    let mut ans = vec![];

    for i in 0..nums.len() {
        if i == 0 { ans.push(postfix[i + 1]); } else if i == nums.len() - 1 { ans.push(prefix[i - 1]); } else { ans.push(prefix[i - 1] * postfix[i + 1]); }
    }

    ans
}

// one pass you get proudct from left to right prefix
// two pass you get product from right to left postfix
// to get product for i
// prefix[0..i] * postfix[i+1..]

#[cfg(test)]
mod tests {
    use crate::p238_product_of_array_except_self::product_except_self;

    #[test]
    fn test_one() {
        let expected = vec![24, 12, 8, 6];
        let output = product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = vec![0, 0, 9, 0, 0];
        let output = product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(expected, output);
    }
}