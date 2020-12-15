
#![allow(non_snake_case)]
pub use crate::fileUtil::fileUtil;


use scan_fmt::scan_fmt;


trait Valid
{
    fn valid(&self) -> bool;
    fn required(&self) -> bool;
}

//let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
struct BirthYear
{
    value: String,
}

struct IssueYear
{
    value: String,
}

struct ExpirationYear
{
    value: String,
}

struct Height
{
    value: String,
}

struct HairColour
{
    value: String,
}

struct EyeColour
{
    value: String,
}

struct PassportId
{
    value: String,
}

struct CountryId
{
    value: String,
}

impl Valid for BirthYear
{
    fn valid(&self) -> bool
    {
        return match self.value.parse::<i32>()
        {
            Ok(n) => n >= 1920 && n <= 2002,
            Err(E) => false,
        }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for IssueYear
{
    fn valid(&self) -> bool
    {
        match self.value.parse::<i32>()
        {
            Ok(n) => return n >= 2010 && n <= 2020,
            Err(E) => return false,
        }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for ExpirationYear
{
    fn valid(&self) -> bool
    {
        match self.value.parse::<i32>()
        {
            Ok(n) => return n >= 2020 && n <= 2030,
            Err(E) => return false,
        }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for Height
{
    fn valid(&self) -> bool
    {
        if let Ok((number, unit)) = scan_fmt!(&self.value,
                        "{d}{}",
                        usize, String)
        {
            return match unit.trim()
            {
                "cm" => number >= 150 && number <=193,
                "in" => number >= 59 && number <=76,
                _ => false,
            }
        }
        else { return false; }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for HairColour
{
    fn valid(&self) -> bool
    {
        if self.value.len() != 7
        {
            return false;
        }
        let mut it = self.value.chars();
        if  it.next() == Some('#')
        {
            for rest in it
            {
                if rest > 'f' ||   rest < '0'
                {
                    return false;
                }
                if( rest < 'a' && rest > '9')
                {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for EyeColour
{
    fn valid(&self) -> bool
    {
        return match self.value.trim()
        {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for PassportId
{
    fn valid(&self) -> bool
    {
        if self.value.len() != 9
        {
            return false;
        }
        match self.value.parse::<i64>()
        {
            Ok(n) => return true,
            Err(E) => return false,
        }
    }
    fn required(&self) -> bool
    {
        return true;
    }
}

impl Valid for CountryId
{
    fn valid(&self) -> bool
    {
        return true;
    }
    fn required(&self) -> bool
    {
        return false;
    }
}

enum Value
{
    BirthYear(BirthYear),
    ExpirationYear(ExpirationYear),
    IssueYear(IssueYear),
    Height(Height),
    EyeColour(EyeColour),
    HairColour(HairColour),
    PassportId(PassportId),
    CountryId(CountryId),
}

struct Pair
(
    String,
    String,
);

pub fn printResult()
{
    let mut values: Vec<Vec<Box<Valid>>> = Vec::new();
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
                if (value.len() == 0)
                {
                    values.push(Vec::new());
                } else {
                    for pair in value.split(" ")
                    {
                        //println!("{}", (*pair).next());

                        let p = (*pair).split(":");
                        let vec: Vec<&str> = p.collect::<Vec<&str>>();
                        if vec.len() != 2
                        {
                            panic!("Bad number");
                        }
                        match vec[0]
                        {
                            "byr" => values.last_mut().unwrap().push(Box::new(BirthYear{value: String::from(vec[1])})),
                            "iyr" => values.last_mut().unwrap().push(Box::new(IssueYear{value: String::from(vec[1])})),
                            "eyr" => values.last_mut().unwrap().push(Box::new(ExpirationYear{value: String::from(vec[1])})),
                            "hgt" => values.last_mut().unwrap().push(Box::new(Height{value: String::from(vec[1])})),
                            "hcl" => values.last_mut().unwrap().push(Box::new(HairColour{value: String::from(vec[1])})),
                            "ecl" => values.last_mut().unwrap().push(Box::new(EyeColour{value: String::from(vec[1])})),
                            "pid" => values.last_mut().unwrap().push(Box::new(PassportId{value: String::from(vec[1])})),
                            "cid" => values.last_mut().unwrap().push(Box::new(CountryId{value: String::from(vec[1])})),
                            _ => print!("Unhandled value {}", vec[0]),
                        }
                        //values.last_mut().unwrap().push(Pair(String::from(vec[0]), String::from(vec[1])));
                    }
                }
            }
        }
    }

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
    let mut numberOfValid2 = 0;
    for pairs in values
    {
        if pairs.len() > 6
        {
            if pairs.len() == 7 && pairs.iter().any(|x| !x.required())
            {
                continue;
            }
            numberOfValid += 1;
            if pairs.iter().all(|x|x.valid())
            {
                numberOfValid2 += 1;
            }

        }
    }

    println!("Found {} valid", numberOfValid);
    println!("Found {} really valid", numberOfValid2);
}