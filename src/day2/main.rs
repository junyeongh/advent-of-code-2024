use std::{fs, io};
fn main() -> io::Result<()> {
    let contents = fs::read_to_string("src/day2/test.txt")?;
    // let contents = fs::read_to_string("src/day2/input.txt")?;

    let mut reports: Vec<Vec<i32>> = vec![];
    for line in contents.lines() {
        let report = line
            .split_ascii_whitespace()
            .flat_map(str::parse::<i32>)
            .collect();

        reports.push(report);
    }
    // println!("{:?}", reports);

    // Part 1
    let mut part_one = 0;

    for report in &reports {
        let levels: &Vec<i32> = &report[1..report.len()]
            .iter()
            .zip(&report[0..report.len() - 1])
            .map(|(x, y)| (x - y))
            .collect();
        // println!("{:?}", levels);

        if levels.iter().all(|&v| [1, 2, 3].contains(&v))
            || levels.iter().all(|&v| [-1, -2, -3].contains(&v))
        {
            part_one += 1;
        }
    }
    println!("{:?}", part_one);

    // Part 2
    let mut part_two = 0;
    for report in &reports {
        let levels: Vec<i32> = report[1..report.len()]
            .iter()
            .zip(&report[0..report.len() - 1])
            .map(|(x, y)| (x - y))
            .collect();

        // Check if already valid
        if levels.iter().all(|&v| [1, 2, 3].contains(&v))
            || levels.iter().all(|&v| [-1, -2, -3].contains(&v))
        {
            part_two += 1;
            continue;
        }

        // Try removing each number
        for skip_idx in 0..report.len() {
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

            if diffs.iter().all(|&v| [1, 2, 3].contains(&v))
                || diffs.iter().all(|&v| [-1, -2, -3].contains(&v))
            {
                part_two += 1;
                break;
            }
        }
    }
    println!("{:?}", part_two);

    Ok(())
}
