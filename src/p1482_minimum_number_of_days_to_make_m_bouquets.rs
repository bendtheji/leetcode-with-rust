#![allow(dead_code)]

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let mut right = *bloom_day.iter().max().unwrap();
    let mut left = *bloom_day.iter().min().unwrap();
    let mut result = -1;

    while left <= right {
        let mid = ((right - left) / 2) + left;
        if can_form_enough_bouquets(&bloom_day, m, k, mid) {
            result = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    result
}

fn can_form_enough_bouquets(bloom_day: &Vec<i32>, m: i32, k: i32, day: i32) -> bool {
    // keep no_of_bouquets formed so far (no_of_bouquets)
    let mut no_of_bouquets = 0;
    // keep current count of flowers in bouquet (cur_count)
    let mut cur_count = 0;
    // loop through each flower
    for flower in bloom_day {
        // if increase cur_count if cur flower <= day
        if flower <= &day {
            cur_count += 1;
            if cur_count == k {
                //   if cur_count == k, then increase no_of_bouquets
                no_of_bouquets += 1;
                //.     if no_of_bouquets == m , return true
                if no_of_bouquets == m {
                    return true;
                }
                cur_count = 0;
            }
        } // else drop cur_count to 0
        else {
            cur_count = 0;
        }
        // exit loop
    }
    // return false
    false
}

#[cfg(test)]
mod tests {
    use crate::p1482_minimum_number_of_days_to_make_m_bouquets::min_days;

    #[test]
    fn test_one() {
        let bloom_day = vec![1, 10, 3, 10, 2];
        let m = 3;
        let k = 1;
        let expected = 3;
        let output = min_days(bloom_day, m, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let bloom_day = vec![1, 10, 3, 10, 2];
        let m = 3;
        let k = 2;
        let expected = -1;
        let output = min_days(bloom_day, m, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
        let m = 2;
        let k = 3;
        let expected = 12;
        let output = min_days(bloom_day, m, k);
        assert_eq!(expected, output);
    }
}