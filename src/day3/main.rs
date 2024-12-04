use regex::Regex;
fn main() {
    // let input = std::fs::read_to_string("src/day3/test1.txt").unwrap();
    // let input = std::fs::read_to_string("src/day3/test2.txt").unwrap();
    let input = std::fs::read_to_string("src/day3/input.txt").unwrap();

    let result = parser(&input);
    println!("Part 1: {:?}", result.0);
    println!("Part 2: {:?}", result.1);
}

fn parser(input: &String) -> (i32, i32) {
    // Part 1
    let pattern1 = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    let mut capture1: Vec<(i32, i32)> = pattern1
        .captures_iter(input)
        .into_iter()
        .map(|cap| {
            (
                cap.get(0).unwrap().start().clone() as i32,
                &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect();
    println!("{:?}", capture1);
    let result1 = capture1.iter().map(|(_, x)| x).sum();

    // Part 2
    let pattern2 = Regex::new(r"do(?:n't)?\(\)").unwrap();
    let mut capture2: Vec<(i32, &str)> = pattern2
        .captures_iter(input)
        .into_iter()
        .map(|cap| {
            (
                cap.get(0).unwrap().start().clone() as i32,
                cap.get(0).unwrap().as_str(),
            )
        })
        .collect();
    println!("{:?}", capture2);

    let mut result2 = 0;
    let mut disabled = false; // not disabled by default
    while !capture1.is_empty() {
        if capture2.is_empty() || capture1[0].0 < capture2[0].0 {
            let removed = capture1.remove(0);
            if !disabled {
                result2 += removed.1
            }
        } else {
            // enable
            let removed = capture2.remove(0);
            println!("{:?}", removed);
            match removed.1 {
                "do()" => disabled = false,
                _ => disabled = true,
            }
        }
    }

    (result1, result2)
}
