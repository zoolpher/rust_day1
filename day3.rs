

// OWNERSHIP
// BORROWING
// REFERENCING (mutable or immutable)

//=====================

// - each value in rust has an OWNERSHIP
// - only one owner at a time
// - when owner goes out of scope -> value is dropped

#[allow(dead_code)]   // ❗

fn main() {
//__OWNERSHIP_____________________________________________
    let str1: String = String::from("Rusty!");
    println!("{}", calculate_len(&str1));
    let str2: String = str1;    // ownership (str1 -> str2)
    println!("{}\n", str2);
    // println!("{}", str1);     ❌  
    // ^^^str1 value is dropped

    // changing owner in <&str>_____________________
    let s1: &str = "a string";
    let s2: &str = s1;
    println!("S2 value: {}", s2);   // ✅
    println!("S1 value: {}\n", s1);   // ✅

//__REFERENCING & DEREFENENCING_______________________________
    let mut _x: u16 = 40;
    let _y: &mut u16 = &mut _x;
    // _y * 2;
    // println!("Double value of Y: {}", _y);
    // println!("Double value of X: {}\n", _x);
    *_y = *_y * 2;

    println!("Address of Y: {:p}", &_y); // add. of _y
    println!("Address of X: {:p}", _y);  // add. of _x
    // println!("Address of X: {:p}\n", &_x);  // add of _x ❗❗
    println!("Double value of Y: {}", &_y);
    println!("Double value of X: {}", _y);   // auto-refenencing
    // println!("Double value of X: {}", _x);    ❌
    // ^^^immutable borrow occur here
    println!("Double value of X: {}\n", *_y);  // manual-referencing


//__BORROWING________________________________________ 
    let a1: i32 = 87;
    let a2: i32 = a1.clone() + 10;  // we didn't add any <mut> in a2
    println!("A1 value: {}", a1);
    println!("A2 value: {}", a2);
    println!("A2 value + ten: {}", a2 + 10);   // we didn't add any <mut> in a2


}

fn calculate_len(s1: &String) -> usize {
    return s1.len();
}