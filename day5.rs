

/*
 -- controll flow statement (cfl)
 -- loop {}
 -- while loop {}
 -- for loop {}
 -- tuple ()
 -- struct {}
*/

#![allow(warnings)]   // ir will remove the warings
#[allow(dead_code)]   // ❗
fn main() {
    let age: u16 = 19;
    if age > 18 {
        println!("\nYou can drive a car.");
    } else if age == 18 {
        println!("Wait for just 1 year to pass.");
    } else {
        println!("You can't drive a car.");
    }

//__using if in let statement_______________________________
//__short hand______________________________________________
    let condition: bool = false;
    let number: u16 = if condition {15} else {9};   // ✅
    // ^^^if condition is TRUE
    //        let number = 15
    //        else number = 9

    println!("The number is: {}", number);
    let num: u16 = if condition {
        100
    } else {
        11
    };
    println!("The value of NUM: {}", num);

    // let jf = if condition {56} else {"seventy"};   ❌
    // println!("The value of jf: {jf}");
    // ^^^ we cannot have 2 diff. d_types in same cfl. 

//__LOOPS__________________________________________________
    let mut count: usize = 0;
    let sum: usize = loop {
        if count == 10 {
            println!("{count}");
            break count;
        }
        println!("{count}");
        count += 1;
    }; 

    // __loop lable____________________________________________
    'outer: loop {
        let mut i: i16 = 0;
        loop {
            let mut j:i16 = 0;
            loop {
                println!("i = {}, j = {}", i, j);
                j += 1; // increment j
                if j == 3 {
                    break; // exit inner loop when j == 3
                }
            }
            i += 1; // increment i
            if i == 3 {
                break 'outer; // exit the labeled outer loop when i == 3
            }
        }
    }
//__WHILE LOOP_________________________________________________________
    let mut n: i16 = 0;
    while n != 20 {
        n += 1;
        println!("{n}");
    }

//__FOR LOOP________________________________________________________
    let arr:[i16; 6] = [1, 34, 56, 88, 18, 29];
    for elem in arr {  // don't need to specify d_type of 'elem'
        print!("{elem}\t"); // it wont go to new line   ✅
    }

//__TUPLE______________________________________________________________
    let tp:(i16, &str) = (45, "zoolpher");
    println!("\n{:?}", tp);
    println!("{}", tp.0);
    println!("{}", tp.1);
   
//__STRUCT_______________________________________________________
    struct Book {
        name: String,
        author: String,
        page_no: i32,
        availability: bool,
    };

    struct user {
        active: bool,
        name: String,
        email_id: String,
    }

    let mut user1: user = user {
        active: true,
        name: String::from("zoolpher"),
        email_id: String::from("zoolpher@pagal.com"),
    };
    user1.active = false;   // canging user1 <active> instance
    println!("\n{}", user1.active);
    println!("{}", user1.name);
    println!("{}", user1.email_id);

    //__returning struct from func.________________
    fn get_user(e: String, n: String, a: bool) -> user {
        user {
            active: a,
            name: n,
            email_id: e,           
        }
    }
    let new_user: user = get_user("any@gogo.com".to_string(), "No name".to_string(), false);
    println!("\n{} \n{} \n{}", new_user.active, new_user.name, new_user.email_id);
    
    //__creating instance from other instances__________
    let user3: user = user {
        email_id: user1.email_id,  // user1's email ID
        ..new_user   // remaining values similar to <new_user>
    };
    println!("\n{} \n{} \n{}", user3.active, user3.email_id, user3.name);

    //__Tuple struct____________________________
    struct color(i16, i32, i64);
    struct tag(i16, i32, i64);

    let black: color = color(12,45,7);
    let black_tag:tag = tag(100, 200, 300);
    println!("\n{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", black_tag.0, black_tag.1, black_tag.2);

    //__unit-like struct__________________________________
    /*
    - don't have any fields
    - used when we need to implement a trait but don't want to store data
    */
    struct not_decided;
    let nn: not_decided = not_decided; 


}