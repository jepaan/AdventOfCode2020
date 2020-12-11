

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

                    //println!("{}", key);
                    elements.next();


                    let mut set = Vec::new();
                    while let Some(waste) = elements.next()
                    {
                        let count = elements.next();
                        let innerFirst = elements.next();
                        let innerSecond = elements.next();
                        if innerSecond.is_some()
                        {
                            //println!("{} {} {}", count.unwrap(), innerFirst.unwrap(), innerSecond.unwrap());
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
                }
            }
        }
        let bagName = String::from("shiny gold");
        let count = countBagOuter(&bagName, &rules);
        println!("{} is in {}", bagName, count);
    }

    fn hasBag(bagToFind: &String, currentBag: &String, rules: &HashMap<String, Vec<Bag>>) -> bool
    {
        let mut ret = false;
        if bagToFind == currentBag
        {
            return true;
        }

        let a = rules.get(currentBag);
        if a.is_none()
        {
            return false;
        }
        let b = a.unwrap();

        for element in &*b
        {
            ret = hasBag(bagToFind, &element.name, rules);
            if ret == true
            {
                break;
            }
        }
        return ret;
    }

    fn countBagOuter(name: &String, rules: &HashMap<String, Vec<Bag>>) -> i32
    {
        let mut count = 0;
        for element in rules
        {
            if element.0 == name //Has to be inside another
            {
                continue;
            }
            if hasBag(name, &element.0, rules)
            {
                println!("Found {} in {}", name, &element.0);
                count += 1;
            }
        }

        return count;
    }


}
