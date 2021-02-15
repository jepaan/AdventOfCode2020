
#![allow(non_snake_case)]


use std::collections::HashMap;

struct Number
{
    last: usize,
    lastLast: usize
}

fn speak(last: &usize, round: &usize, numbers: &mut HashMap<usize, Number>) -> usize
{
    //First decide what to say
    let mut lastNumber = numbers.get_mut(&last).unwrap();
    let mut returnValue;
    if lastNumber.lastLast == 0
    {
        returnValue = 0;
    }
    else
    {
        returnValue = lastNumber.last - lastNumber.lastLast;
    }

    //Now save what is going to be said
    let mut saidNumber = numbers.get_mut(&returnValue);
    if saidNumber.is_none()
    {
        numbers.insert(returnValue, Number { last: *round, lastLast: 0 });
    }
    else
    {
        let mut number = saidNumber.unwrap();
        number.lastLast = number.last;
        number.last = *round;
    }

    return returnValue;
}



    // let mut element = numbers.get_mut(value);
    // element
    // match(element)
    // {
    //     Some(element) => {println!("old")},
    //     None => {numbers.insert(*value, Number{last:0, current:0})}
    // }
    //


pub fn printResult()
{
    //let input = "0,3,6";
    let input ="0,13,1,16,6,17";
    let mut numbers: HashMap<usize, Number> = HashMap::new();

    //First do input
    let mut splittedInput = input.split(',');
    let mut last = 0;
    let mut round = 1;
    for element in splittedInput
    {
        last = element.parse::<usize>().unwrap();
        numbers.insert(last, Number{last:round, lastLast:0});
        round += 1;
    }
    println!("{} was spoken", last);

    let mut stopAt = 2021;
    while(stopAt != round)
    {
        last = speak(&last, &round, &mut numbers);
        //println!("{}: {} was spoken", round, last);
        round += 1;
    }
    println!("{}: {} was spoken", round-1, last);
}