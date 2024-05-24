#![allow(dead_code)]

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut letters: Vec<i32> = letters.into_iter().fold(vec![0; 26], |mut m, c| {
        m[c as usize - 'a' as usize] += 1;
        m
    });

    let words = words.into_iter().map(|s| s.chars().fold(vec![0; 26], |mut m, c| {
        m[c as usize - 'a' as usize] += 1;
        m
    })).collect::<Vec<Vec<i32>>>();

    backtracking(&words, &mut letters, &score, 0, 0, &mut max);
    max
}

fn backtracking(words: &Vec<Vec<i32>>, letters: &mut Vec<i32>, score: &Vec<i32>, index: usize, curr: i32, max: &mut i32) {
    if index == words.len() {
        *max = std::cmp::max(curr, *max);
        return;
    }

    let curr_word = &words[index];

    if can_form_word(curr_word, letters) {
        remove_word_letters(curr_word, letters);
        backtracking(words, letters, score, index + 1, curr + get_word_score(curr_word, score), max);
        add_word_letters(curr_word, letters);
    }

    backtracking(words, letters, score, index + 1, curr, max);
}

// take or don't take the word
// can only take word if there are enough letters for it
// need a function to get score as well
fn can_form_word(word: &Vec<i32>, letters: &Vec<i32>) -> bool {
    word.iter().zip(letters.iter()).all(|(w_char_count, letters_char_count)| letters_char_count >= w_char_count)
}

fn remove_word_letters(word: &Vec<i32>, letters: &mut Vec<i32>) {
    for i in 0..letters.len() {
        letters[i] -= word[i]
    }
}

fn add_word_letters(word: &Vec<i32>, letters: &mut Vec<i32>) {
    for i in 0..letters.len() {
        letters[i] += word[i]
    }
}

fn get_word_score(word: &Vec<i32>, score: &Vec<i32>) -> i32 {
    word.iter().zip(score.iter()).map(|(char_count, char_score)| char_count * char_score).sum()
}

#[cfg(test)]
mod tests {
    use crate::p1255_maximum_score_words_formed_by_letters::max_score_words;

    #[test]
    fn test_one() {
        let words = vec![String::from("dog"), String::from("cat"), String::from("dad"), String::from("good")];
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        let score = vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = 23;
        let output = max_score_words(words, letters, score);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let words = vec![String::from("xxxz"), String::from("ax"), String::from("bx"), String::from("cx")];
        let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10];
        let expected = 27;
        let output = max_score_words(words, letters, score);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let words = vec![String::from("leetcode")];
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let expected = 0;
        let output = max_score_words(words, letters, score);
        assert_eq!(expected, output);
    }
}