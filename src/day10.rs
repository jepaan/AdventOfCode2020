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
    println!("{} {} {}", ones, threes, ones * threes);

    let finalValue = *numbers.last().unwrap();

    //Make new vector and push in active routes
    let mut routes: Vec<i32> = Vec::new();
    routes.push(0);
    while !routes.is_empty()
    {
        let value = routes.pop().unwrap();
        //println!("{} value", value);
        if value == finalValue
        {
            routes.push(value);
            break;
        }



        let plus1=value+1;
        let plus2 = value+2;
        let plus3 = value+3;
        if numbers.contains(&plus1)
        {
            //println!("{} can go to {}", value, plus1);
            routes.push(plus1);
        }
        if numbers.contains(&plus2)
        {
            //println!("{} can go to {}", value, plus2);
            routes.push(plus2);
        }
        if numbers.contains(&plus3)
        {
            //println!("{} can go to {}", value, plus3);
            routes.push(plus3);
        }
        routes.sort_by(|a, b| b.cmp(a));
        //for element in &routes
        //{
        //    print!("{} ", element);
        //}
        //print!("\n");

    }
    println!("Found {} ways!", routes.len());


}

