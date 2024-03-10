#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(start_gene.clone());
    seen.insert(start_gene);

    let mut steps = 0;

    while !queue.is_empty() {
        let no_of_nodes = queue.len();
        for _ in 0..no_of_nodes {
            let cur_node = queue.pop_front().unwrap();
            if cur_node == end_gene { return steps; }

            let available_chars = vec!['A', 'C', 'G', 'T'];

            for character in available_chars {
                for pos in 0..cur_node.len() {
                    let string = format!("{}{}{}", &cur_node[0..pos], character, &cur_node[pos + 1..]);
                    if !seen.contains(&string) && bank.contains(&string) {
                        queue.push_back(string.clone());
                        seen.insert(string);
                    }
                }
            }
        }
        steps = steps + 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::p433_minimum_genetic_mutation::min_mutation;

    #[test]
    fn test_one() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string()];
        let expected = 1;
        let output = min_mutation(start_gene, end_gene, bank);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string(), "AACCGCTA".to_string(), "AAACGGTA".to_string()];
        let expected = 2;
        let output = min_mutation(start_gene, end_gene, bank);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AATTCCGG".to_string();
        let bank = vec!["AATTCCGG".to_string(), "AACCTGGG".to_string(), "AACCCCGG".to_string(), "AACCTACC".to_string()];
        let expected = -1;
        let output = min_mutation(start_gene, end_gene, bank);
        assert_eq!(expected, output);
    }
}