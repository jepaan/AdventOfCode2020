

pub use crate::fileUtil::fileUtil;

use std::cmp;

pub fn printResult()
{
    let maxValue = 127*8+5;
    let mut max: usize = 0;
    let mut ids: Vec<usize> = (0..(maxValue)).collect();
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
                    let id: usize = row*8+col;
                    ids[id] = 0;
                    max = cmp::max(max, id);
                    //println!("Row {} Col {} id {}", row, col, row*8+col);
                }
            }
        }
    }

    println!("The max value is {}", max);


    ids.retain(|&x| x != 0);

    for x in 1..ids.len()-1
    {
        if  ids[x] - ids[x-1] > 1  && ids[x+1] - ids[x] > 1
        {
            println!("Seat id is {}", ids[x]);
        }
    }
}
