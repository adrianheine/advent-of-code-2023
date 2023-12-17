use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut galaxies = vec![];
    let mut column_empty = vec![];
    let mut lines_empty = vec![];
    for (y, line) in input.enumerate() {
        let line = line.as_ref().as_bytes();
        let mut line_empty = true;
        for (x, c) in line.iter().enumerate() {
            if column_empty.len() <= x {
                column_empty.push(true);
            }
            if *c == b'#' {
                galaxies.push((y, x));
                line_empty = false;
                column_empty[x] = false;
            }
        }
        lines_empty.push(line_empty);
    }
    let mut sum = 0;
    for (idx, galaxy) in galaxies.iter().enumerate() {
        for other in &galaxies[idx + 1..] {
            let (min, max) = (other.0.min(galaxy.0), other.0.max(galaxy.0));
            sum += max - min + lines_empty[min..max].iter().filter(|x| **x).count() * 999_999;
            let (min, max) = (other.1.min(galaxy.1), other.1.max(galaxy.1));
            sum += max - min + column_empty[min..max].iter().filter(|x| **x).count() * 999_999;
        }
    }
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"
            .lines()
        ),
        82_000_210
    );
}
