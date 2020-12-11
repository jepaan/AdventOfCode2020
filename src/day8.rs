

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;
use std::ops::Add;

trait Instruction
{
    fn nextOffset(&self) -> i32;
    fn accumulatorOffset(&self) -> i32;
    fn visitCount(& mut self) -> usize;
}


struct Accumulator
{
    value: i32,
    visits: usize,
}

struct Jump
{
    value: i32,
    visits: usize,
}

struct NoOperation
{
    value: i32,
    visits: usize,
}

impl Instruction for Accumulator
{
    fn nextOffset(&self) -> i32 {
        return 1;
    }

    fn accumulatorOffset(&self) -> i32 {
        return self.value;
    }

    fn visitCount(& mut self) -> usize {
        self.visits += 1;
        return self.visits;
    }
}

impl Instruction for Jump
{
    fn nextOffset(&self) -> i32 {
        return self.value;
    }

    fn accumulatorOffset(&self) -> i32 {
        return 0;
    }

    fn visitCount(& mut self) -> usize {
        self.visits += 1;
        return self.visits;
    }
}

impl Instruction for NoOperation
{
    fn nextOffset(&self) -> i32 {
        return 1;
    }

    fn accumulatorOffset(&self) -> i32 {
        return 0;
    }

    fn visitCount(& mut self) -> usize {
        self.visits += 1;
        return self.visits;
    }
}


pub fn printResult()
{
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    if let Ok(lines) = fileUtil::readLines("data/input8.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    if let Ok((command, n)) = scan_fmt!(&value,
                        "{} {d}",
                        String, i32)
                    {
                        instructions.push( match command.as_str()
                        {
                            "acc" => Box::new(Accumulator{value:n, visits:0 }),
                            "jmp" => Box::new(Jump{value:n, visits:0 }),
                            "nop" => Box::new(NoOperation{value:n, visits:0 }),
                            _ => panic!("Unexpected input"),
                        }
                        )
                    }
                }
            }
        }
    }
    let mut accumulator = 0;
    let mut offset = 0;

    while instructions[offset].visitCount() < 2
    {
        accumulator += instructions[offset].accumulatorOffset();
        let next = instructions[offset].nextOffset();
        if next > 0
        {
            offset = offset.checked_add(i32::abs(next) as usize).unwrap();
        }
        else
        {
            offset = offset.checked_sub(i32::abs(next) as usize).unwrap();
        }
    }
    println!("Accumulator is {} before infinite loop!", accumulator);
}
