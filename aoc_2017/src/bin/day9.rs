use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("day9.input").unwrap();
    let (_, scores) = count_and_score_groups(&input);
    println!("part 1: {}", scores);
}

fn to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn remove_bang(input: Vec<char>) -> Vec<char> {
    let n = input.len();

    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        if input[i] == '!' {
            i += 2
        } else {
            result.push(input[i]);
            i += 1;
        }
    }

    result
}

fn remove_garbages(input: Vec<char>) -> Vec<char> {
    let n = input.len();
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        if input[i] == '<' {
            while i < n && input[i] != '>' {
                i += 1;
            }
        } else {
            result.push(input[i]);
        }

        i += 1;
    }

    result
}

fn count_and_score_groups(input: &str) -> (usize, usize) {
    let input = remove_garbages(remove_bang(to_chars(input)));

    let mut total_scores = 0;
    let mut local_score = 0;
    let mut groups = 0;
    let mut left_parens = 0;
    for c in input {
        if c == '{' {
            left_parens += 1;
            local_score += 1;
        } else if c == '}' {
            if left_parens > 0 {
                left_parens -= 1;
                groups += 1;

                total_scores += local_score;
                local_score -= 1;
            }
        }
    }

    (groups, total_scores)
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn remove_garbages_tests() {
        assert_eq!(remove_garbages(remove_bang(to_chars("<>"))), vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<random characters>"))),vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<>"))), vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<{!>}>"))), vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<!!>"))), vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<!!!>>"))), vec![]);
        assert_eq!(remove_garbages(remove_bang(to_chars("<{o\"i!a,<{i<a>"))),vec![]);
    }

    #[test]
    fn count_groups_tests() {
        assert_eq!(count_and_score_groups("{}").0, 1);
        assert_eq!(count_and_score_groups("{{{}}}").0, 3);
        assert_eq!(count_and_score_groups("{{},{}}").0, 3);
        assert_eq!(count_and_score_groups("{{{},{},{{}}}}").0, 6);
        assert_eq!(count_and_score_groups("{<{},{},{{}}>}").0, 1);
        assert_eq!(count_and_score_groups("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(count_and_score_groups("{{<a>},{<a>},{<a>},{<a>}}").0, 5);
        assert_eq!(count_and_score_groups("{{<!>},{<!>},{<!>},{<a>}}").0, 2);
    }

    #[test]
    fn scores_groups_tests() {
        assert_eq!(count_and_score_groups("{}").1, 1);
        assert_eq!(count_and_score_groups("{{{}}}").1, 6);
        assert_eq!(count_and_score_groups("{{},{}}").1, 5);
        assert_eq!(count_and_score_groups("{{{},{},{{}}}}").1, 16);
        assert_eq!(count_and_score_groups("{<a>,<a>,<a>,<a>}").1, 1);
        assert_eq!(count_and_score_groups("{{<ab>},{<ab>},{<ab>},{<ab>}}").1, 9);
        assert_eq!(count_and_score_groups("{{<!!>},{<!!>},{<!!>},{<!!>}}").1, 9);
        assert_eq!(count_and_score_groups("{{<a!>},{<a!>},{<a!>},{<ab>}}").1, 3);
    }
}

