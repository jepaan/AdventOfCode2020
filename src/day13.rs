
#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;

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

        let mut otherTimestamp:i64 = 100000000000000; //100000000000000
        while otherTimestamp > 0
        {
            let mut isOk = true;
            for bus in &busList
            {
                let t2 = otherTimestamp + bus.offset;
                if t2 % bus.id != 0
                {
                    isOk = false;
                    break;
                }
            }
            if isOk
            {
                break;
            }
            else
            {
                otherTimestamp += 1;
            }
        }
        println!("Other time is {}", otherTimestamp);
    }
}