use std::collections::HashMap;
use std::iter::zip;
use std::{fs, io};

fn main() -> io::Result<()> {
    // let contents = fs::read_to_string("src/day1/test.txt")?;
    let contents = fs::read_to_string("src/day1/input.txt")?;

    let (left_list, right_list): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_ascii_whitespace();
            if let (Some(left), Some(right)) = (nums.next(), nums.next()) {
                if let (Ok(left_num), Ok(right_num)) = (left.parse::<i32>(), right.parse::<i32>()) {
                    return Some((left_num, right_num));
                }
            }
            None
        })
        .unzip();

    let mut left_sorted_list = left_list.clone();
    left_sorted_list.sort();
    let mut right_sorted_list = right_list.clone();
    right_sorted_list.sort();

    let pair: Vec<(i32, i32)> = zip(left_sorted_list, right_sorted_list).collect();

    // part 1
    let distance: i32 = pair.iter().map(|(l, r)| (l - r).abs()).sum();
    println!("{:?}", distance);

    // part 2
    let counter: HashMap<i32, i32> = right_list.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let score: i32 = left_list
        .iter()
        .map(|&v| v * counter.get(&v).unwrap_or(&0))
        .sum();
    println!("{:?}", score);

    Ok(())
}
