use itertools::Itertools;

pub fn read_input() -> Vec<u32> {
    return include_str!("../day1")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
}

pub fn part1(input: &Vec<u32>) -> u32 {
    return input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

pub fn part2(input: &Vec<u32>) -> u32 {
    let mut sorted = input.clone();
    sorted.sort();

    for i in 0..sorted.len() - 2 {
        let mut j = i + 1;
        let mut k = sorted.len() - 1;

        let a = sorted[i];
        while j < k {
            let b = sorted[j];
            let c = sorted[k];

            if a + b + c == 2020 {
                return a * b * c;
            } else if a + b + c > 2020 {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
    return 0;
}
