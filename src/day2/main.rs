use std::{fs, io};

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("src/day2/test.txt")?;
    // let contents = fs::read_to_string("src/day2/input.txt")?;

    let reports: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(str::parse::<i32>)
                .collect()
        })
        .collect();

    // Part 1
    let part_one = reports
        .iter()
        .filter(|report| {
            let levels: Vec<i32> = report[1..]
                .iter()
                .zip(&report[..report.len() - 1])
                .map(|(x, y)| x - y)
                .collect();

            levels.iter().all(|&v| [1, 2, 3].contains(&v))
                || levels.iter().all(|&v| [-1, -2, -3].contains(&v))
        })
        .count();

    println!("{:?}", part_one);

    // Part 2
    let part_two = reports
        .iter()
        .filter(|report| {
            let levels: Vec<i32> = report[1..]
                .iter()
                .zip(&report[..report.len() - 1])
                .map(|(x, y)| x - y)
                .collect();

            if levels.iter().all(|&v| [1, 2, 3].contains(&v))
                || levels.iter().all(|&v| [-1, -2, -3].contains(&v))
            {
                return true;
            }

            (0..report.len()).any(|skip_idx| {
                let filtered: Vec<i32> = report
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != skip_idx)
                    .map(|(_, &x)| x)
                    .collect();

                let diffs: Vec<i32> = filtered[1..]
                    .iter()
                    .zip(&filtered[..filtered.len() - 1])
                    .map(|(x, y)| x - y)
                    .collect();

                diffs.iter().all(|&v| [1, 2, 3].contains(&v))
                    || diffs.iter().all(|&v| [-1, -2, -3].contains(&v))
            })
        })
        .count();

    println!("{:?}", part_two);

    Ok(())
}
