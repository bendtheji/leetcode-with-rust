#![allow(dead_code)]

pub fn find_judge(n: i32, trusts: Vec<Vec<i32>>) -> i32 {
    let mut trust_scores = vec![0; (n+1) as usize];

    for trust in &trusts {
        let a = trust[0] as usize;
        let b = trust[1] as usize;
        trust_scores[b] = trust_scores[b] + 1;
        trust_scores[a] = trust_scores[a] - 1;
    }

    for i in 1..=n {
        if trust_scores[i as usize] == n - 1 { return i as i32; }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::p997_find_the_town_judge::find_judge;

    #[test]
    fn test_one() {
        assert_eq!(find_judge(2, vec![vec![1,2]]), 2);
    }

    #[test]
    fn test_two() {
        assert_eq!(find_judge(3, vec![vec![1,3],vec![2,3]]), 3);
    }

    #[test]
    fn test_three() {
        assert_eq!(find_judge(3, vec![vec![1,3], vec![2,3], vec![3,1]]), -1);
    }

    #[test]
    fn test_four() {
        assert_eq!(find_judge(3, vec![vec![1,2],vec![2,3]]), -1);
    }

    #[test]
    fn test_five() {
        assert_eq!(find_judge(2, vec![]), -1);
    }

    #[test]
    fn test_six() {
        assert_eq!(find_judge(1, vec![]), 1);
    }
}