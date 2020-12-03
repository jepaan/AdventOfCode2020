

    pub use crate::fileUtil::fileUtil;
    use scan_fmt::scan_fmt;

    pub fn printResult()
    {
        let mut result = 0;
        if let Ok(lines) = fileUtil::readLines("data/input2.txt") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(value) = line
                {
                    if let Ok((min, max, c, password)) = scan_fmt!(&value,
                        "{d}-{d} {}: {}",
                        usize, usize, char, String)
                    {
                        //let (mut min, mut max, c, password); (i32, i32, char, string);
                        //scan_fmt!(, value, min, max, c, password);
                        let count = password.matches(c).count();
                        if count >= min && count <= max
                        {
                            //println!("{} Count was {} min {} max {}", value, count, min, max);
                            result += 1;
                        }


                    }
                }
            }
        }
        println!("Found {}", result);
    }
