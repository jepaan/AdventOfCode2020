
#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

fn doPart1(line: &String, memory:& mut HashMap<usize, u64>, andMask: &mut u64, orMask: &mut u64)
{
    if line.contains("mask")
    {
        if let Ok((mask)) = scan_fmt!(&line,
                        "mask = {}", String)
        {
            *andMask =0;
            *orMask = 0;
            for character in mask.chars()
            {
                *andMask = (*andMask << 1);
                *orMask = (*orMask << 1);
                match character
                {
                    '1' => {*orMask = *orMask + 1; *andMask = *andMask + 1},
                    '0' =>  {},
                    'X' => *andMask = *andMask + 1,
                    _ => {},
                }

            }
            //println!("{:#038b}", andMask);
            //println!("{:#038b}", orMask);
        }
    }
    else
    {
        if let Ok((index, input)) = scan_fmt!(&line,
                        "mem[{}] = {}", usize, u64)
        {
            //println!("Read {} {}", index, input);
            let actual = (input & *andMask)  | *orMask;
            memory.insert(index,  actual);
            //println!("Actual {}", actual);

        }
    }
}

fn flunctuate(indexes: &Vec<usize>, currentIndex: usize, values: &mut Vec<usize>, value: usize)
{
    //println!("Index {} value {}", currentIndex, value);
    if currentIndex == indexes.len()
    {
        values.push(value)
    }
    else
    {
        let lowMask = usize::max_value() ^ (1 << indexes[currentIndex]);
        let valueLow = value & lowMask;
        let valueHigh = value | (1 << indexes[currentIndex]);
        flunctuate(indexes, currentIndex + 1, values, valueLow);
        flunctuate(indexes, currentIndex + 1, values, valueHigh);
    }
}

fn doPart2(line: &String, memory:& mut HashMap<usize, u64>, orMask: u64, flunctuations: &mut Vec<usize>)
{
    if line.contains("mask")
    {
        if let Ok((mask)) = scan_fmt!(&line,
                        "mask = {}", String)
        {
            flunctuations.clear();
            let mut index: usize = 36;
            for character in mask.chars()
            {
                match character
                {
                    'X' => {(*flunctuations).push(index - 1)},
                    _ => {},
                }
                index -= 1;

            }
            // for v in flunctuations
            // {
            //     print!("{} ", v);
            // }
            // println!("");

            //println!("{:#038b}", andMask);
            //println!("{:#038b}", orMask);
        }
    }
    else
    {
        if let Ok((index, input)) = scan_fmt!(&line,
                        "mem[{}] = {}", usize, u64)
        {
            //println!("Read {} {}", index, input);
            //println!("Actual {}", actual);
            let mut values: Vec<usize> = Vec::new();
            let vv = index | orMask as usize;
            flunctuate(flunctuations, 0, &mut values, vv);
            for v in values
            {
                memory.insert(v, input);
            }

        }
    }
}

pub fn printResult()
{
    if let Ok(lines) = fileUtil::readLines("data/input14.txt")
    {
        let mut andMask = 0;
        let mut orMask = 0;
        let mut memory: HashMap<usize, u64> = HashMap::new();
        let mut memory2: HashMap<usize, u64> = HashMap::new();
        let mut flunctuations: Vec<usize> = Vec::new();
        for line in lines
        {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    doPart1(&value, & mut memory, & mut andMask, & mut orMask);

                    //Part2 relies on orMask from part 1.
                    doPart2(&value, & mut memory2, orMask, &mut flunctuations);
                }
            }
        }
        let mut sum = 0;
        for value in memory
        {
            sum += value.1;
        }
        println!("The sum is {}", sum);

        sum = 0;
        for value in memory2
        {
            sum += value.1;
        }
        println!("The sum is {}", sum);
    }
}