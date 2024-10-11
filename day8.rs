
/*
    binding refers to the process of associating a name with a value. 
        This is also known as variable binding or pattern binding. When 
        you bind a value to a name, you can then use that name to refer 
        to the value in your code.
    Taking user input.
*/


#![allow(warnings)]   // it will remove the warings
#[allow(dead_code)]   // ❗

use std::io;

fn add(x: i32, y: i32) -> i32 { x + y } // binds the function add to the name add

fn main() {

//___Binding_______________________________________________________________________
    let x = 5;                // variable binging
    println!("{x}");
    let (x, y) = (1, 2); // pattern binding
    println!("{x} {y}");
    let sum = add(x,y);       // func. binding
    println!("{sum}");


    // mod my_module {             // module binding
    //     pub fn hello() {
    //         println!("Hello, world!");
    //     }
    // }

//__Takig user input_______________________________________________________________

    let mut input = String::new();
    println!("Please enter some text:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input.trim());


}