

pub use crate::fileUtil::fileUtil;
use std::collections::HashMap;

struct Bag
{
    name: String,
    count: i32,
}

pub fn printResult()
{
    let mut rules: HashMap<String, Vec<Bag>> = HashMap::new();
    if let Ok(lines) = fileUtil::readLines("data/input7.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    let mut elements = value.split_whitespace();


                    //let key: String;
                    let first = elements.next().unwrap();
                    let second = elements.next().unwrap();
                    let key = String::from(first) + " " + second;

                    println!("{}", key);
                    elements.next();


                    let mut set = Vec::new();
                    while let Some(waste) = elements.next()
                    {
                        let count = elements.next();
                        let innerFirst = elements.next();
                        let innerSecond = elements.next();
                        if innerSecond.is_some()
                        {
                            println!("{} {} {}", count.unwrap(), innerFirst.unwrap(), innerSecond.unwrap());
                            let name = String::from(innerFirst.unwrap()) + " " + innerSecond.unwrap();
                            if count.unwrap() != "no"
                            {
                                let v = count.unwrap().parse().unwrap();
                                let b = Bag{count:v, name:name};
                                set.push(b);
                            }
                        }
                    }
                    let mut someSet = rules.insert(key, set);
                    if someSet.is_some()
                    {
                        panic!("Oh no");
                    }
                    //First identify
                }
            }
        }
    }

    fn hasBag(name: String, rules: &HashMap<String, Vec<Bag>>) -> bool
    {
        let a = rules.get(&name);
        let b = a.unwrap();

        for element in (*b)
        {
            let b = element.name;
            println!("{}", b);
        }
    }

    fn countBagOuter(name: String, rules: &HashMap<String, Vec<Bag>>) -> i32
    {
        let mut count = 0;
        for element in rules
        {
            if hasBag(*(element.0), rules)
            {
                count += 1;
            }
        }
        let a = rules.get(&name);
        let b = a.unwrap();

        for element in (*b)
        {
            let b = element.name;
            println!("{}", b);
        }
    }


}
