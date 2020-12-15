#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;

pub fn printResult()
{
    let size: usize = 25;
    let mut numbers: [i64; 25] = [0; 25];
    let mut i = 0;
    if let Ok(lines) = fileUtil::readLines("data/input9.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    let number: i64 = value.parse().unwrap();

                    if i >= size
                    {
                        //Evaluate
                        let mut foundMatch = false;
                        for x1 in 0 .. size-1
                        {
                            match foundMatch
                            {
                                false => for x2 in 1 .. size
                                {
                                    if numbers[x1] + numbers[x2] == number
                                    {
                                        //println!("{}: Number {} is sum of {} and {}", i, number,numbers[x1], numbers[x2]);
                                        foundMatch = true;
                                        break;
                                    }
                                },
                                true => break,
                            }

                        }
                        if !foundMatch
                        {
                            println!("{}: {} is wrong!", i, number);
                            break;
                        }
                    }
                    numbers[i % size] = number;
                    i += 1;
                }
            }
        }

    }
}
