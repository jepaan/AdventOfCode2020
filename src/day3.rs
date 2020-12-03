

pub use crate::fileUtil::fileUtil;

pub fn printResult()
{
    let treeString= "#" ;
    let free : u8 = '.' as u8;
    let tree = &treeString.as_bytes()[0];
    let mut map : Vec<Vec<char> > = Vec::new();
    if let Ok(lines) = fileUtil::readLines("data/input3.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 1
                {
                    let mut row : Vec<char> = Vec::new();
                    for element in value.as_bytes(){
                        if element  == tree
                        {
                            row.push('#');
                        }
                        else
                        {
                            row.push('.');
                        }

                    }
                    map.push(row);
                }
                //println!("Found {} cols", value.len());

            }
        }
    }


    let depth = map.len();
    let width = map[0].len();
    let mut treeCount1 = 0;
    for x in 1..depth
    {
        if map[x][(x*3) % width] == '#'
        {
            treeCount1 += 1;
        }
    }
    println!("Found {} rows and {} trees", map.len(), treeCount1);
}
