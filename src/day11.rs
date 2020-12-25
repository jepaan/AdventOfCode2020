#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::Entry;
use std::any::Any;


//#[derive(PartialEq)]
#[derive(Copy, Clone, PartialEq)]
enum Tile
{
    Empty = 1,
    Occupied = 2,
    Floor = 3,
}

fn tileToChar(tile: &Tile) -> char
{
    return match tile
    {
        Tile::Occupied => '#',
        Tile::Empty => 'L',
        Tile::Floor => '.',
    };
}

pub fn printResult()
{
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    if let Ok(lines) = fileUtil::readLines("data/input11.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    let mut row = Vec::new();
                    for c in value.chars()
                    {
                        let cc = c;
                        let tile = match c
                        {
                            'L' => Tile::Empty,
                            '.' => Tile::Floor,
                            _ => panic!("Unexpected!"),
                        };
                        row.push(tile);
                    }
                    tiles.push(row);

                }
            }
        }
    }

    fn countOccupiedAround(tiles: &Vec<Vec<Tile>>, row: usize, column: usize) -> usize
    {
        let rowBegin = if row == 0 {0} else {row-1};
        let rowEnd = if row == tiles.len()-1 {row+1}else {row+2};
        let columnBegin = if column == 0 {0} else {column-1};
        let columnEnd = if column == tiles[0].len() -1 {column+1} else {column+2};

        let mut returnValue = 0;
        for x in rowBegin..rowEnd
        {
            for y in columnBegin..columnEnd
            {
                let v: &Tile = &tiles[x][y];
                if *v == Tile::Occupied
                {
                    returnValue += 1;
                }
            }
        }
        return returnValue;
    }

    fn countType(tiles: &Vec<Vec<Tile>>, typeToCount: &Tile) -> usize
    {
        let mut count = 0;
        for row in 0..tiles.len()
        {
            for column in 0..tiles[0].len()
            {
                if &tiles[row][column] == typeToCount
                {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn doRound(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>>
    {
        let mut newTiles = Vec::new();
        for row in 0..tiles.len()
        {
            let mut newRow = Vec::new();
            for column in 0..tiles[0].len()
            {
                newRow.push(  match tiles[row][column]
                {//4 because the centre is counted as well
                    Tile::Occupied => if countOccupiedAround(tiles, row, column) > 4 {Tile::Empty} else {Tile::Occupied},
                    Tile::Empty => if countOccupiedAround(tiles, row, column) > 0 {Tile::Empty} else {Tile::Occupied},
                    Tile::Floor => Tile::Floor,
                });
            }
            newTiles.push(newRow);
        }
        return newTiles;
    }

    fn print(tiles: &Vec<Vec<Tile>>)
    {
        for row in 0..tiles.len()
        {
            for column in 0..tiles[0].len()
            {
                let c = tileToChar(&tiles[row][column]);
                print!("{}", c);
            }
            print!("\n");
        }

    }

    fn compareTiles(set1: &Vec<Vec<Tile>>, set2: &Vec<Vec<Tile>>) -> bool
    {
        for row in 0..set1.len()
        {
            for column in 0..set1[0].len()
            {
                if set1[row][column] != set2[row][column]
                {
                    return false;
                }
            }
        }
        return true;
    }

    //print(&tiles);
    let mut newTiles = doRound(&tiles);
    //println!("Doing one round!");
    //print(&newTiles);
    let mut roundCount = 1;
    while true
    {
        //println!("Doing one more round!");
        let newTiles2 = doRound(&newTiles);
        if roundCount > 2 || compareTiles(&newTiles, &newTiles2)
        {
            break;
        }
        else
        {
            newTiles = newTiles2;
            //roundCount += 1;
            //print(&newTiles);
        }
    }
    println!("done {}", countType(&newTiles, &Tile::Occupied));

    fn getFirstSeatInDirection(tiles: &Vec<Vec<Tile>>, mut row: usize, mut column: usize, rowStep: i32, columnStep: i32) -> Tile
    {
        let mut moving = true;

        let mut usedRow: i32 = row as i32;
        let mut usedColumn: i32 = column as i32;
        while moving
        {
            row = ((row as i32) + rowStep) as usize;
            column = ((column as i32) + columnStep) as usize;
            if tiles.get(row).is_none()
            {
                return Tile::Floor;
            }
            if tiles[row].get(column).is_none()
            {
                return Tile::Floor;
            }
            if tiles[row][column] != Tile::Floor
            {
                return tiles[row][column];
            }
        }
        return Tile::Floor;
    }

    fn countOccupiedAroundVisible(tiles: &Vec<Vec<Tile>>, row: usize, column: usize) -> usize
    {
        let mut returnValue = 0;
        for rowStep in -1..=1
        {
            for columnStep in -1..=1
            {
                if(rowStep == 0 && columnStep == 0)
                {
                    continue;
                }
                if getFirstSeatInDirection(tiles, row, column, rowStep, columnStep) == Tile::Occupied
                {
                    returnValue += 1;
                }
            }
        }
        return returnValue;
    }

    fn doRound2(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>>
    {
        let mut newTiles = Vec::new();
        for row in 0..tiles.len()
        {
            let mut newRow = Vec::new();
            for column in 0..tiles[0].len()
            {
                newRow.push(  match tiles[row][column]
                {
                    Tile::Occupied => if countOccupiedAroundVisible(tiles, row, column) > 4 {Tile::Empty} else {Tile::Occupied},
                    Tile::Empty => if countOccupiedAroundVisible(tiles, row, column) > 0 {Tile::Empty} else {Tile::Occupied},
                    Tile::Floor => Tile::Floor,
                });
            }
            newTiles.push(newRow);
        }
        return newTiles;
    }

    let mut newTiles = doRound2(&tiles);
    //println!("Doing one round!");
    //print(&newTiles);
    let mut roundCount = 1;
    //println!("ddd {}", countOccupiedAroundVisible(&newTiles, 0, 2));
    while true
    {
        //println!("Doing one more round!");
        let newTiles2 = doRound2(&newTiles);
        if roundCount > 2 || compareTiles(&newTiles, &newTiles2)
        {
            break;
        }
        else
        {
            newTiles = newTiles2;
            //roundCount += 1;
            //print(&newTiles);
        }
    }
    println!("done {}", countType(&newTiles, &Tile::Occupied));
}

