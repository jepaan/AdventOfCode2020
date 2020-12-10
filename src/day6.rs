

pub use crate::fileUtil::fileUtil;
use std::collections::BTreeSet;

pub fn printResult()
{

    let mut sum = 0;
    let mut found: BTreeSet<char> = BTreeSet::new();
    if let Ok(lines) = fileUtil::readLines("data/input6.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    for c in value.chars()
                    {
                        found.insert(c);
                    }

                }
                else
                {
                    sum += found.len();
                    found.clear();
                }
            }
        }
    }
    sum += found.len();
    println!("The sum is {}", sum);


}
