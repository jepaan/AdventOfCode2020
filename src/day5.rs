

pub use crate::fileUtil::fileUtil;

use std::cmp;

pub fn printResult()
{
    let mut max: i32 = 0;
    if let Ok(lines) = fileUtil::readLines("data/input5.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 1
                {
                    let mut row = 0;
                    let mut col = 0;
                    for x in value.chars()
                    {
                        match x
                        {

                            'B' => row = ((row) << 1)  + 1,
                            'F' => row = (row << 1),
                            'R' => col = ((col) << 1)  + 1,
                            'L' => col = (col << 1),
                            _ => panic!("Bad input {}", x ),

                        }
                    }
                    max = cmp::max(max, row*8+col);
                    //println!("Row {} Col {} id {}", row, col, row*8+col);
                }
            }
        }
    }

    println!("The max value is {}", max);
}
