

pub use crate::fileUtil::fileUtil;
use std::borrow::Borrow;


struct Pair
(
    String,
    String
);

pub fn printResult()
{
    let mut values : Vec<Vec<Pair> > = Vec::new();
    let t = "hello world!".split(" ");

    for s in t
    {
        println!("{}", s);
    }
    values.push(Vec::new());
    if let Ok(lines) = fileUtil::readLines("data/input4.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(value) = line
            {
                if(value.len() == 0)
                {
                    values.push(Vec::new());
                }
                else
                {
                    for pair in value.split(" ")
                    {
                        //println!("{}", (*pair).next());

                        let p = (*pair).split(":");
                        let vec:Vec<&str>  = p.collect::<Vec<&str>>();
                        if vec.len() != 2
                        {
                            panic!("Bad number");
                        }
                        values.last_mut().unwrap().push(Pair(String::from(vec[0]),String::from(vec[1])));
                    }
                }
            }
        }
    }

    //let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];




    fn validate(found: &Vec<Pair>, toFind: &[&str], index: usize ) -> bool
    {
        for pair in found
        {
            if pair.0 == toFind[index]
            {
                if index == 0
                {
                    return true;
                }
                return true && validate(found, toFind, index - 1);
            }
        }
        return false;
    }

    let mut numberOfValid = 0;
    for pairs in values
    {
        if validate(&pairs, required.borrow(), required.len() - 1)
        {
            numberOfValid += 1;
        }
    }

    println!("Found {} valid", numberOfValid);
}