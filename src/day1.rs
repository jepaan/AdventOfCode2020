



pub mod day1
{
    pub use crate::fileUtil::fileUtil;

    pub fn printResult()
    {
        let mut values: Vec<i32> = Vec::new();
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = fileUtil::readLines("data/input1.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(value) = line
                {
                    if (value.len() > 1)
                    {
                        //println!("{}",  value);
                        let i = value.parse().unwrap();
                        values.push(i);
                    }
                }
            }
        }
        for i in 0..values.len() - 1
        {
            for j in i + 1..values.len()
            {
                //println!("{} {} {}", values[i], values[j],  values[i] + values[j]);
                if values[i] + values[j] == 2020
                {
                    println!("{} {} {} {}", values[i], values[j], values[i] + values[j], values[i] * values[j]);
                }
            }
        }
        for i in 0..values.len() - 1
        {
            for j in i + 1..values.len()
            {
                for k in j + 1..values.len()
                {
                    if values[i] + values[j] + values[k] == 2020
                    {
                        println!("{} {} {} {} {}", values[i], values[j], values[k], values[i] + values[j] + values[k], values[i] * values[j] * values[k]);
                    }
                }
                //println!("{} {} {}", values[i], values[j],  values[i] + values[j]);
            }
        }
    }
// 
}