pub fn knot_hash(seed:&str) -> String {
    let result = knot_hash_u8(seed);
    let result: Vec<_> = result.into_iter().map(|b| format!("{:02x}", b)).collect();
    result.join("")
}

pub fn knot_hash_u8(seed:&str) -> Vec<u8> {
    let mut lengths = numbers_to_ascii(seed);
    lengths.extend_from_slice(&[17,31,73,47,23]);

    let mut numbers: Vec<_> = (0..=255).collect();

    reverse_and_skip(&mut numbers, &lengths, 64);

    (0..=255).step_by(16)
            .map(|start| compute_hash(&numbers, start, 16))
            .collect()
}

fn numbers_to_ascii(input: &str) -> Vec<u8> {
    input.as_bytes().into_iter().map(|b| *b).collect()
}

fn compute_hash(numbers: &Vec<u8>, start: usize, length: usize) -> u8 {
    numbers.iter().skip(start).take(length).fold(0, |result, n| result ^ *n)
}

pub fn reverse_and_skip(numbers: &mut Vec<u8>, lengths: &Vec<u8>, round: usize) {
    let n = numbers.len();
    let mut current_position = 0usize;
    let mut skip_size = 0usize;

    for _ in 0..round {
        for length in lengths {
            //reverse current_position until current_position + length
            let length = *length as usize;
            let mut i = current_position;
            let mut j = current_position + length - 1;
            while i < j {
                numbers.swap(i % n, j % n);
                i += 1;
                j -= 1;
            }

            current_position = (current_position + length + skip_size) % n;
            skip_size += 1;
        }
    }
}

#[cfg(test)]
mod knot_hash_tests {
    use super::*;

    #[test]
    fn numbers_to_ascii_tests() {
        let input = "1,2,3";
        assert_eq!(numbers_to_ascii(input), vec![49,44,50,44,51]);
    }

    #[test]
    fn computert_hash_tests() {
        let numbers = vec![65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22];
        assert_eq!(compute_hash(&numbers, 0, 16), 64);
    }

}