#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use std::collections::BTreeSet;

pub fn printResult()
{

    let mut sum = 0;
    let mut sum2 = 0;
    let mut found: BTreeSet<char> = BTreeSet::new();
    let mut found2: BTreeSet<char> = BTreeSet::new();
    let mut first = true;
    if let Ok(lines) = fileUtil::readLines("data/input6.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    let mut thisLine: BTreeSet<char> = BTreeSet::new();
                    for c in value.chars()
                    {
                        thisLine.insert(c);
                        found.insert(c);

                    }
                    if first
                    {
                        found2 = thisLine.clone();
                        first = false;
                    }
                    else
                    {
                        let d :Vec<char>= found2.difference(&thisLine).cloned().collect();
                        for x in d
                        {
                            found2.remove(&x);
                        }

                    }
                }
                else
                {
                    sum += found.len();
                    sum2 += found2.len();
                    found.clear();
                    found2.clear();
                    first = true;
                }
            }
        }
    }
    sum += found.len();
    sum2 += found2.len();
    println!("The sum is {} {}", sum, sum2);


}
