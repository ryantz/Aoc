fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u8 {
    1
}

// test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("input1.txt");
        let result = part1(input);
        let answer = 1;
        assert_eq!(result, answer);
    }
}
