//
// Program to determine how desireable a planet is in Sword of the Stars
//

use std::io;


fn main() {
    loop {
        let mut input = String::new();
        println!("Enter System Size:");
        io::stdin()
            .read_line(&mut input)
            .expect("Please Retry Value");
        let mut size: f32 = input.trim().parse().unwrap();
        println!("");

        let mut input = String::new();
        println!("Enter System Resources:");
        io::stdin()
            .read_line(&mut input)
            .expect("Please Retry Value");
        let mut resources: f32 = input.trim().parse().unwrap();
        println!("");

        let mut input = String::new();
        println!("Enter System Infrastructure:");
        io::stdin()
            .read_line(&mut input)
            .expect("Please Retry Value");
        let mut infrastructure: f32 = input.trim().parse().unwrap();
        println!("");

        let mut input = String::new();
        println!("Enter System Climate Hazard:");
        io::stdin()
            .read_line(&mut input)
            .expect("Please Retry Value");
        let mut climatehazard: f32 = input.trim().parse().unwrap();
        println!("");

        let score = size * 10.0 + resources / 100.0 + infrastructure - climatehazard / 10.0;
        if score > 110.0 {
            println!("\nSystem Score is {}", score);
            println!("System is a Peach!!!");
        } else if score > 80.0 {
            println!("\nSystem Score is {}", score);
            println!("System is a Favored World");
        } else if score > 60.0 {
            println!("\nSystem Score is {}", score);
            println!("System is a Secondary Target");
        } else if score > 30.0 {
            println!("\nSystem Score is {}", score);
            println!("System is an Unfavored World");
        } else {
            println!("\nSystem Score is {}", score);
            println!("System is a Rock :(");
        }       
        println!("");
    }
}
