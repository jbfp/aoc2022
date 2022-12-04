use std::collections::BinaryHeap;

static FILE: &str = include_str!("../inputs/day01.txt");

pub fn part1() {
    let mut max = 0;
    let mut acc = 0;

    for line in FILE.lines() {
        if line.is_empty() {
            if acc > max {
                max = acc;
            }

            acc = 0;
        } else {
            acc += line.parse::<usize>().unwrap();
        }
    }

    println!("{}", max);
}

pub fn part2() {
    let mut calories = BinaryHeap::new();
    let mut acc = 0;

    for line in FILE.lines() {
        if line.is_empty() {
            calories.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<usize>().unwrap();
        }
    }

    let sum = calories.into_iter_sorted().take(3).sum::<usize>();

    println!("{}", sum);
}
