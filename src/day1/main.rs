use std::collections::HashMap;
use std::fs;
use std::io;
use std::iter::zip;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("src/day1/test")?;
    // let contents = fs::read_to_string("src/day1/input")?;

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for line in contents.lines() {
        let mut nums = line.split_ascii_whitespace();
        if let (Some(left), Some(right)) = (nums.next(), nums.next()) {
            if let Ok(left_num) = left.parse::<i32>() {
                left_list.push(left_num);
            }
            if let Ok(right_num) = right.parse::<i32>() {
                right_list.push(right_num);
            }
        }
    }

    let mut left_sorted_list = left_list.clone();
    left_sorted_list.sort();
    let mut right_sorted_list = right_list.clone();
    right_sorted_list.sort();

    let pair: Vec<(i32, i32)> = zip(left_sorted_list, right_sorted_list)
        .into_iter()
        .collect();

    // part 1
    let sum: i32 = pair.iter().map(|v| (v.0 - v.1).abs()).sum();
    println!("{:?}", sum);

    // part 2
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for number in right_list {
        *counter.entry(number).or_insert(0) += 1;
    }
    let score: i32 = left_list
        .iter()
        .fold(0, |acc, &v| acc + v * counter.get(&v).unwrap_or(&0));
    println!("{:?}", score);

    Ok(())
}
