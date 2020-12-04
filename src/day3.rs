pub fn read_input() -> Vec<Vec<bool>> {
    return include_str!("../day3")
        .lines()
        .map(|line| parse_line(line))
        .collect();
}

fn parse_line(line: &str) -> Vec<bool> {
    return line.bytes().map(|symbol| symbol == '#' as u8).collect();
}

pub fn count_trees(map: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let line_len = map[0].len();
    let mut x_pos = 0;
    let mut trees_visited = 0;

    for line in map.iter().skip(dy).step_by(dy) {
        x_pos += dx;
        while x_pos >= line_len {
            x_pos -= line_len;
        }

        trees_visited += line[x_pos] as usize;
    }

    return trees_visited;
}

pub fn part1(map: &Vec<Vec<bool>>) -> usize {
    return count_trees(map, 3, 1);
}

pub fn part2(map: &Vec<Vec<bool>>) -> usize {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    return slopes
        .iter()
        .map(|(dx, dy)| count_trees(map, *dx, *dy))
        .fold(1, |result, val| result * val);
}
