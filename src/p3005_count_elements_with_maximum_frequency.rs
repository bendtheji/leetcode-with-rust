#![allow(dead_code)]

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut frequencies = vec![0; 101];
    for num in nums {
        frequencies[num as usize] += 1;
    }

    let mut max = 0;
    let mut count = 0;

    for frequency in frequencies {
        if frequency > max {
            max = frequency;
            count = frequency;
        } else if frequency == max {
            count += frequency;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::p3005_count_elements_with_maximum_frequency::max_frequency_elements;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 2, 3, 1, 4];
        let expected = 4;
        let output = max_frequency_elements(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 5;
        let output = max_frequency_elements(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![81, 81, 81, 81, 81, 81, 81, 17, 28, 81, 81, 56, 81, 54, 81, 81, 81, 81, 81, 60, 81, 28, 81, 81, 81, 81, 81, 54, 81, 81, 81, 81, 100, 28];
        let expected = 25;
        let output = max_frequency_elements(nums);
        assert_eq!(expected, output);
    }
}