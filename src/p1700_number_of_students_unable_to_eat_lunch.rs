#![allow(dead_code)]

use std::collections::VecDeque;

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut students = VecDeque::from(students);
    let mut sandwiches = VecDeque::from(sandwiches);

    'outer: while !sandwiches.is_empty() {
        let cur_sandwich = sandwiches.front().unwrap();
        let no_of_students = students.len();
        for _ in 0..no_of_students {
            let curr_student = students.pop_front().unwrap();
            if curr_student == *cur_sandwich {
                sandwiches.pop_front();
                continue 'outer;
            } else {
                students.push_back(curr_student);
            }
        }
        return no_of_students as i32;
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::p1700_number_of_students_unable_to_eat_lunch::count_students;

    #[test]
    fn test_one() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        let expected = 0;
        let output = count_students(students, sandwiches);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        let expected = 3;
        let output = count_students(students, sandwiches);
        assert_eq!(expected, output);
    }
}