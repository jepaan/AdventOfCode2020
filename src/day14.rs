
#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

pub fn printResult()
{
    if let Ok(lines) = fileUtil::readLines("data/input14.txt")
    {
        let mut andMask = 0;
        let mut orMask = 0;
        let mut memory: HashMap<usize, u64> = HashMap::new();
        for line in lines
        {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    if value.contains("mask")
                    {
                        if let Ok((mask)) = scan_fmt!(&value,
                        "mask = {}", String)
                        {
                            andMask = 0;
                            orMask = 0;
                            for character in mask.chars()
                            {
                                andMask <<= 1;
                                orMask <<= 1;
                                match character
                                {
                                    '1' => {orMask += 1; andMask += 1},
                                    '0' =>  {},
                                    'X' => andMask += 1,
                                    _ => {},
                                }

                            }
                            //println!("{:#038b}", andMask);
                            //println!("{:#038b}", orMask);
                        }
                    }
                    else
                    {
                        if let Ok((index, input)) = scan_fmt!(&value,
                        "mem[{}] = {}", usize, u64)
                        {
                            //println!("Read {} {}", index, input);
                            let actual = (input & andMask)  | orMask;
                            memory.insert(index,  actual);
                            //println!("Actual {}", actual);

                        }
                    }

                }
            }
        }
        let mut sum = 0;
        for value in memory
        {
            sum += value.1;
        }

        println!("The sum is {}", sum);
    }
}