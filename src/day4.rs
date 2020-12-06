

pub use crate::fileUtil::fileUtil;


struct Pair
{
    key: String,
    value: String,
}

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
                if(value.len() == 1)
                {
                    values.push(Vec::new());
                }
                else
                {
                    for pair in value.split(" ")
                    {
                        //println!("{}", (*pair).next());

                        let mut p = (*pair).split(":");
                        if p.count() != 2
                        {
                            panic!("Bad number");
                        }
                        for x in p {
                            let y = x.to_string();
                            let p: Pair = Pair{y, "sss".to};
                            (*values.last()).push({y, "sss"});
                        }

                        //let k: p.next();
                        //let v: p.next();
                        //


                    }
                }
            }
        }
    }
    println!("{}", values.len() );
}