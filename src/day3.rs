#![allow(non_snake_case)]

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


    fn countTrees( xStep: usize, yStep: usize,  map: &Vec<Vec<char> >) -> i64
    {
        let depth = map.len();
        let width = map[0].len();
        let mut y = 0;
        let mut treeCount1 = 0;
        for x in ((0+xStep)..depth).step_by(xStep)
        {
            y += yStep;
            if map[x][y % width] == '#'
            {
                treeCount1 += 1;
            }
        }
        println!("Returning {}", treeCount1);
        return treeCount1;
    }

    println!("Found {} rows and {} trees", map.len(), countTrees(1, 3, &map));


    let mut mult: i64 = countTrees(1, 1, &map);
    mult *= countTrees(1, 3, &map);
    mult *= countTrees(1, 5, &map);
    mult *= countTrees(1, 7, &map);
    mult *= countTrees(2, 1, &map);

    println!("Found {} mult", mult);
}
