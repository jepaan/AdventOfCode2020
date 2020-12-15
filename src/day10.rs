#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use std::collections::VecDeque;

pub fn printResult()
{
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(0);
    if let Ok(lines) = fileUtil::readLines("data/input10.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    numbers.push(value.parse().unwrap());
                }
            }
        }
    }

    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);
    let mut ones = 0;
    let mut threes = 0;
    for index in 1..numbers.len()
    {
        let diff = numbers[index] - numbers[index - 1];
        match diff
        {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    println!("{} {} {}", ones, threes, ones*threes);

}

