use std::io::{self, BufRead};

fn main() {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for i in 0..list1.len() {
        let diff = list1[i] - list2[i];
        sum += diff.abs();
    }
    println!("{}", sum);
}
