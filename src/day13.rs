
#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;

pub fn printResult()
{
    if let Ok(lines) = fileUtil::readLines("data/input13.txt")
    {
        let mut iter = lines.into_iter();
        let time: i64 = iter.next().unwrap().unwrap().parse().unwrap();
        println!("Time {}", time);

        let mut closest = i64::max_value();
        let mut id: i64 = 0;
        let secondLine = iter.next().unwrap().unwrap();
        let busses = secondLine.split(',');
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
                            break;
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
                    },
                Err(this) => {},
            }
        }
        println!("Bus {} is next. Result is {}", id, id*closest);
    }
}