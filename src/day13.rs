
#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use std::any::Any;

struct Bus
{
    id: i64,
    offset: i64
}



pub fn printResult()
{
    if let Ok(lines) = fileUtil::readLines("data/input13.txt")
    {
        let mut iter = lines.into_iter();
        let time: i64 = iter.next().unwrap().unwrap().parse().unwrap();

        let mut closest = i64::max_value();
        let mut id: i64 = 0;
        let secondLine = iter.next().unwrap().unwrap();
        let busses = secondLine.split(',');
        let mut currentOffset = 0;
        let mut busList : Vec<Bus> = Vec::new();
        for bus in busses
        {
            let this = String::from(bus).parse();
            match this
            {
                Ok(this) =>
                    {
                        let modResult = time % this;
                        if modResult == 0
                        {
                            //Perfect match
                            id = this;
                            closest = 0;
                        }
                        else
                        {
                            //Bus arrived result minutes before
                            let diff =  this - modResult;
                            if diff < closest
                            {
                                closest = diff;
                                id = this;
                            }
                        }
                        busList.push(Bus{ id: this, offset: currentOffset });
                    },
                Err(this) => {},
            }
            currentOffset += 1;
        }
        println!("Bus {} is next. Result is {}", id, id*closest);

        //Brute force guessing is too slow even with hint.
        let mut otherTimestamp  = 0;

        let mut stepSize = busList[0].id;
        let mut isOk = true;
        for bus in &busList[1..]
        {
            isOk = false;
            while !isOk
            {
                //println!("Trying {}", otherTimestamp);
                let t2 = otherTimestamp + bus.offset;
                if t2 % bus.id == 0
                {
                    isOk = true;
                    stepSize *= bus.id;
                    //println!("Match for {} at {} (offset {})", bus.id, otherTimestamp, bus.offset);
                }
                else
                {
                    otherTimestamp += stepSize;
                }

            }

        }

        if isOk
        {
            println!("Other time is {}", otherTimestamp);
        }
        else
        {
            println!("Did not work!");
        }
    }
}