#![allow(dead_code)]

pub fn check_record(n: i32) -> i32 {
    let mut dp = vec![vec![vec![-1; 3]; 2]; n as usize + 1];
    get_number_of_combinations(n, 0, 0, &mut dp) as i32
}

const MOD: i64 = 1000000007;

fn get_number_of_combinations(n: i32, total_absences: i64, consecutive_lates: i64, dp: &mut Vec<Vec<Vec<i64>>>) -> i64 {
    if total_absences >= 2 || consecutive_lates >= 3 {
        return 0;
    }

    if n == 0 {
        return 1;
    }

    if dp[n as usize][total_absences as usize][consecutive_lates as usize] != -1 {
        return dp[n as usize][total_absences as usize][consecutive_lates as usize];
    }

    let mut count = get_number_of_combinations(n - 1, total_absences, 0, dp) % MOD;
    count = (count + get_number_of_combinations(n - 1, total_absences + 1, 0, dp)) % MOD;
    count = (count + get_number_of_combinations(n - 1, total_absences, consecutive_lates + 1, dp)) % MOD;

    dp[n as usize][total_absences as usize][consecutive_lates as usize] = count;
    count
}

#[cfg(test)]
mod tests {
    use crate::p552_student_attendance_record_ii::check_record;

    #[test]
    fn test_one() {
        let n = 2;
        let expected = 8;
        let output = check_record(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let n = 1;
        let expected = 3;
        let output = check_record(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let n = 10101;
        let expected = 183236316;
        let output = check_record(n);
        assert_eq!(expected, output);
    }
}