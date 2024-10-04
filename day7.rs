

// +- collection types
// +- vectors
    // +- stores values of same type
    // +- a cool😎 dynamic arr. that can grow/shrink when needed  
// +- UTF-8 encoding
    // +- all about strings
// +- Sorting keys with associated values in Hash Map



#![allow(warnings)]

use std::collections::HashMap;   // it will remove the warings
fn main() {

//__VECTOR__________________________________________________________
    // let mut _v1: Vec<u16> = Vec::new();  // empty vector
    // _v1.push(12);
    // _v1.push(10);
    // _v1.push(25);
    // println!("Values of vector _V2: {:?}", _v1);
    
    let mut _my_vector: Vec<u8> = vec![12,45,79,19];
    _my_vector.push(10);
    _my_vector.push(34);
    println!("Values of Vector _my_vactor: {:?}", _my_vector);

    
    // printing 4th element in vector
    let forth: &u8 = &_my_vector[3];  //✅
    // ^^^ we use referencing here
    //      if we would have not used referencing (&var_name)- 
    //      -then ownership would have changed
    println!("The elem. at 4th position is: {forth}");

    // printing 4th element in vector
    let fo: Option<&u8> = _my_vector.get(3);  //✅
    match fo {
        Some(fo) => println!("The elem. using .get() method at 4th position is: {fo}"),
        None => println!("There is no fourth elem."),
    }


//__UTF-8 ENCODING____________________________________________________________  
    let s: String = "just a string".to_string();
    let s: String = String::from("String using another method.");
    let mut s: String = String::from("New string ");
    s.push_str("Pushed string");
    s.push('!');
    println!("\nThe modified string is: {s}");

    let s1: String = String::from("\nfirst string ");
    let s2: String = String::from("second string ");
    let s3: String = s1 + &s2;  //✅ 
    // <s1> value drops
    println!("\nFinal S3 string: {s3}");

    let salam: String = String::from("السلام عليكم");
    let namaste: String = String::from("नमस्ते");
    let full_msg: String = format!("\n{salam} {namaste}");
    println!("{full_msg}");
    
    //%%%%%%%%%%%%%%%%%% YET TO COVER %%%%%%%%%%%%%%%%%%%%
    //%%%%%%%%%%%%%%% SO MUCH IN STRING %%%%%%%%%%%%%%%%%%


//__Hash Maps_________________________________________________________
    // +- more like dictionary in python
    
    let mut scores:HashMap<String,u8> = HashMap::new();
    scores.insert(String::from("Blue"), 78);
    scores.insert(String::from("Red"), 23);
    println!("\nThe map of scores is:\n{:?}", scores);

    let team_name: String = String::from("Blue");
    let team_scores: u8 = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} = {value}");
    }
    //%%%%%%%%%%%%%%% YET SO MUCH TO COVER %%%%%%%%%%%%%%%%%%%%% 
    //%%%%%%%%%%%%%%% IN COMMON COLLECTION %%%%%%%%%%%%%%%%%%%%%
    //%%%%%%%%%%%%%%%%%% FROM RUST BOOK %%%%%%%%%%%%%%%%%%%%%%%%


}