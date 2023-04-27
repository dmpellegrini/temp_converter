use std::io;

fn main() {

    println!("Welcome to temperature converter");

    println!("Easily convert Celcius to Farenheight and vice versa");

    println!("First enter the temperature degrees");

    let mut temp_value = String::new();

    io::stdin()
        .read_line(&mut temp_value)
        .expect("Failed to read line");

    let temp_value: i32 = temp_value
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("You entered {temp_value}");

    println!("Now enter the temperature unit (C/F)");

    let mut temp_unit = String::new();

    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Failed to read line");

    let temp_unit: String = temp_unit
        .trim()
        .parse()
        .expect("Please enter a unit");

    println!("You entered {temp_unit}");


    if temp_unit != "F" && temp_unit != "C" {
        println!("I'm sorry that's not a valid unit");
    }else if temp_unit == "F" {
        let temp_unit: i32 = (temp_value - 32) * 5/9;
        println!("That is {temp_unit} degrees celcius");
    }else if temp_unit == "C" {
        let temp_unit: i32 = (temp_value * 9/5) + 32;
        println!("That is {temp_unit} degrees farenheight");
    }

}
