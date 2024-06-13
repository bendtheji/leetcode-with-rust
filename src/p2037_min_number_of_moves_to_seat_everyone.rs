#![allow(dead_code)]

pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    seats.iter().zip(students.iter()).map(|(i, j)| (i - j).abs()).sum()
}

#[cfg(test)]
mod tests {
    use crate::p2037_min_number_of_moves_to_seat_everyone::min_moves_to_seat;

    #[test]
    fn test_one() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        let expected = 4;
        let output = min_moves_to_seat(seats, students);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        let expected = 7;
        let output = min_moves_to_seat(seats, students);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        let expected = 4;
        let output = min_moves_to_seat(seats, students);
        assert_eq!(expected, output);
    }
}