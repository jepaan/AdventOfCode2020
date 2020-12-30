#![allow(non_snake_case)]

pub use crate::fileUtil::fileUtil;
use scan_fmt::scan_fmt;


//0 east
//1 north
//2 west
// south


fn directionToValue(input: char) -> i32
{
    return match input
    {
        'E' => 0,
        'N' => 1,
        'W' => 2,
        'S' => 3,
        _ => panic!("Wrong input"),
    }
}

trait Ferry
{
    fn forward(&mut self, distance: i64);
    fn changeDirection(&mut self, direction: char, value: i64);
    fn rotateLeft(&mut self, degrees: i64);
    fn rotateRight(&mut self, degrees: i64);
}

struct Ferry1
{
    location: (i64, i64),
    direction: i32,
}

impl Ferry for Ferry1
{
    fn forward(&mut self, distance: i64)
    {
        match self.direction
        {
            0 => self.location.0 += distance,
            1 => self.location.1 += distance,
            2 => self.location.0 -= distance,
            3 => self.location.1 -= distance,
            _ => panic!("Oh ne"),
        }
    }

    fn changeDirection(&mut self, direction: char, value: i64)
    {
        let oldDirection = self.direction;
        self.direction = directionToValue(direction);
        self.forward(value);
        self.direction = oldDirection;
    }

    fn rotateLeft(&mut self, mut degrees: i64)
    {
        while degrees > 0
        {
            self.direction += 1;
            degrees -= 90;
        }
        self.direction  %= 4;
    }

    fn rotateRight(&mut self, mut degrees: i64)
    {
        while degrees > 0
        {
            if self.direction == 0
            {
                self.direction = 4;
            }
            self.direction -= 1;
            degrees -= 90;
        }
    }
}

struct Ferry2
{
    location: (i64, i64),
    waypoint: (i64, i64),
}

impl Ferry for Ferry2
{
    fn forward(&mut self, mut distance: i64) {
        while distance > 0
        {
            self.location.0 += self.waypoint.0;
            self.location.1 += self.waypoint.1;
            distance -= 1;
        }
    }

    fn changeDirection(&mut self, direction: char, value: i64) {
        return match direction
        {
            'E' => self.waypoint.0 += value,
            'N' => self.waypoint.1 += value,
            'W' => self.waypoint.0 -= value,
            'S' => self.waypoint.1 -= value,
            _ => panic!("Wrong input"),
        }
    }

    fn rotateLeft(&mut self, mut degrees: i64)
    {
        while degrees > 0
        {
            let tmp = self.waypoint.0;
            self.waypoint.0 = -self.waypoint.1;
            self.waypoint.1 = tmp;

            degrees -= 90;
        }
    }
    //East 10, North 4 = (10, 4)
    //East 4, South 10 = (4, -10)
    //West 10, South 4 = (-10, -4)
    //North 10, West 4 = (-4, 10)
    //East 10, North 4 = (10, 4)

    fn rotateRight(&mut self, mut degrees: i64) {
        while degrees > 0
        {
            let tmp = self.waypoint.0;
            self.waypoint.0 = self.waypoint.1;
            self.waypoint.1 = -tmp;

            degrees -= 90;
        }
    }
}

pub fn printResult()
{
    let mut ferry1 = Ferry1 {location: (0, 0), direction: 0 };
    let mut ferry2 = Ferry2 {location: (0, 0), waypoint: (10, 1) };

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

                    //Ferry 1
                    match c
                    {
                        'F' => {ferry1.forward(n); ferry2.forward(n)},
                        'N'  => {ferry1.changeDirection(c, n); ferry2.changeDirection(c, n)},
                        'W'  => {ferry1.changeDirection(c, n); ferry2.changeDirection(c, n)},
                        'S'  => {ferry1.changeDirection(c, n); ferry2.changeDirection(c, n)},
                        'E'  => {ferry1.changeDirection(c, n); ferry2.changeDirection(c, n)},
                        'L' => {ferry1.rotateLeft(n); ferry2.rotateLeft(n)},
                        'R' => {ferry1.rotateRight(n); ferry2.rotateRight(n)},

                        _ => panic!("Unexpected"),
                    };
                    //println!("Manhatten distance is {}. Location {} {}. Direction {}", location.0 + location.1, location.0, location.1, currentDirection);
                }
            }
        }
    }


    println!("Manhatten distance is {}. Location {} {}", i64::abs(ferry1.location.0) + i64::abs(ferry1.location.1), ferry1.location.0, ferry1.location.1);
    println!("Manhatten distance is {}. Location {} {}", i64::abs(ferry2.location.0) + i64::abs(ferry2.location.1), ferry2.location.0, ferry2.location.1);
}

