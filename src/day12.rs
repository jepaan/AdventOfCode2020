#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;


//0 east
//1 north
//2 west
// south

pub fn printResult()
{
    let mut location: (i64, i64) = (0,0);
    let mut currentDirection = 0;
    if let Ok(lines) = fileUtil::readLines("data/input12.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line
            {
                if value.len() > 0
                {
                    //println!("{}", value);
                    let mut it = value.chars();
                    let c = it.next().unwrap();
                    let n = it.collect::<String>().parse().unwrap();;
                    //println!("{} {}", c, n);
                    location = match c
                    {
                        'F' => moveFerry(currentDirection, location, n),
                        'N'  => moveFerry(1, location, n),
                        'W'  => moveFerry(2, location, n),
                        'S'  => moveFerry(3, location, n),
                        'E'  => moveFerry(0, location, n),
                        'L' => {currentDirection = rotateFerryLeft(currentDirection, n); location},
                        'R' => {currentDirection = rotateFerryRight(currentDirection, n); location},

                        _ => panic!("Unexpected"),
                    };
                    //println!("Manhatten distance is {}. Location {} {}. Direction {}", location.0 + location.1, location.0, location.1, currentDirection);
                }
            }
        }
    }

    fn moveFerry(direction: i32, location: (i64, i64), distance: i64) -> (i64, i64)
    {
        //println!("Direction {} distance {}", direction, distance);
        return match direction
        {
            0 => (location.0 + distance, location.1),
            1 => (location.0, location.1 + distance),
            2 => (location.0 - distance, location.1),
            3 => (location.0, location.1 - distance),
            _ => panic!("Oh ne"),
        }
    }

    fn rotateFerryLeft(mut direction: i32, mut degrees: i64) -> i32
    {
        //println!("Left {}",degrees);
        while degrees > 0
        {
            direction += 1;
            degrees -= 90;
        }
        return direction % 4;
    }

    fn rotateFerryRight(mut direction: i32, mut degrees: i64) -> i32
    {
        //println!("Right {}",degrees);
        while degrees > 0
        {
            if direction == 0
            {
                direction = 4;
            }
            direction -= 1;
            degrees -= 90;
        }
        return direction;
    }

    println!("Manhatten distance is {}. Location {} {}", i64::abs(location.0) + i64::abs(location.1), location.0, location.1);

}

