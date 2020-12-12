

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;
use std::ops::Add;

trait Instruction
{
    fn nextOffset(&self) -> i32;
    fn accumulatorOffset(&self) -> i32;
    fn visitCount(& mut self) -> usize;
    fn mayHaveError(&self) -> bool;
    fn attemptErrorFix(&self) -> Box<dyn Instruction>;
    fn clone(&self) -> Box<dyn Instruction>;
    fn reset(&mut self);
}


struct Accumulator
{
    value: i32,
    visits: usize,
}

#[derive(Clone)]
struct Jump
{
    value: i32,
    visits: usize,
}

#[derive(Clone)]
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

    fn mayHaveError(&self) -> bool {
        return false;
    }

    fn attemptErrorFix(&self) -> Box<dyn Instruction> {
        return Box::new(Accumulator { value: self.value, visits: self.visits });
    }

        fn clone(&self) -> Box<dyn Instruction>{
        return Box::new(Accumulator{value:self.value, visits:self.visits});
    }

    fn reset(& mut self)
    {
        self.visits = 0;
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

    fn mayHaveError(&self) -> bool {
        return true;
    }

    fn attemptErrorFix(&self) -> Box<dyn Instruction> {
        return Box::new(NoOperation { value: self.value, visits: self.visits });
    }

    fn clone(&self) -> Box<dyn Instruction>{
        return Box::new(Jump{value:self.value, visits:self.visits});
    }

    fn reset(& mut self)
    {
        self.visits = 0;
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

    fn mayHaveError(&self) -> bool {
        return false;
    }

    fn attemptErrorFix(&self) -> Box<dyn Instruction> {
        return Box::new(Jump { value: self.value, visits: self.visits });
    }

    fn clone(&self) -> Box<dyn Instruction> {
        return Box::new(NoOperation{value:self.value, visits:self.visits});
    }

    fn reset(& mut self)
    {
        self.visits = 0;
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
                        instructions.push(createInstruction(command, n));
                    }
                }
            }
        }
    }

    fn createInstruction(command: String, n: i32) -> Box<dyn Instruction>
    {
        return match command.as_str()
        {
            "acc" => Box::new(Accumulator{value:n, visits:0 }),
            "jmp" => Box::new(Jump{value:n, visits:0 }),
            "nop" => Box::new(NoOperation{value:n, visits:0 }),
            _ => panic!("Unexpected input"),
        }
    }

    let mut accumulator = 0;
    let mut offset = 0;

    while instructions[offset].visitCount() < 2
    {
        accumulator += instructions[offset].accumulatorOffset();
        let next = instructions[offset].nextOffset();
        let prev = offset;
        if next > 0
        {
            offset = offset.checked_add(i32::abs(next) as usize).unwrap();
        }
        else
        {
            offset = offset.checked_sub(i32::abs(next) as usize).unwrap();
        }
        //println!("Navigating from {} to {}", prev, offset);
    }
    println!("Accumulator is {} before infinite loop at {}!", accumulator, offset);

    //Just redo ccde with a loop
    let mut modifiedOffset = 0;

    while true
    {
        accumulator = 0;
        offset = 0;
        for element in & mut instructions
        {
            element.reset();
        }

        let mut didChange = false;
        while !didChange
        {
            if instructions[modifiedOffset].mayHaveError()
            {

                instructions[modifiedOffset] = instructions[modifiedOffset].attemptErrorFix();
                didChange = true;
            }
            else
            {
                modifiedOffset += 1;
            }
        }

        while instructions[offset].visitCount() < 2
        {



            accumulator += instructions[offset].accumulatorOffset();
            let next = instructions[offset].nextOffset();
            let prev = offset;
            if next > 0
            {
                offset = offset.checked_add(i32::abs(next) as usize).unwrap();
            } else {
                offset = offset.checked_sub(i32::abs(next) as usize).unwrap();
            }

            if offset >= instructions.len()
            {
                println!("Accumulator is {} at exit!", accumulator);
                return;
            }
        }


        //Restore
        instructions[modifiedOffset] = instructions[modifiedOffset].attemptErrorFix();
        modifiedOffset += 1;
    }

}
