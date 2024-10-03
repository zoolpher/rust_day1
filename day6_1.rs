

// -- Error Handeling
    // +- OPTION<T> {}
    // +- RESULT<T,E> {}




#![allow(warnings)]

fn div(num: f32, deno: f32) -> Option<f32> {
    if deno == 0.0 {
        None   // to return the value we did not use ';' here  ✅
    } else {
        Some(num / deno)   // to return the value we did not use ';' here  ✅
    }
}
fn divide(num: f32, deno: f32) -> Result<f32, String> {
    if deno == 0.0 {
        Err("Denominator is ZERO, can not divide.".to_string())   // to return the value we did not use ';' here  ✅
    } else {
        Ok(num / deno)   // to return the value we did not use ';' here  ✅
    } 
}



fn main() { 
//__option________________________________________________________
    // enum Option<T> {   // basically an enum
    //     Some(T),  // represents a value we look for is found
    //     None(),  //represents a value we look for is not found
    // }
    let result: Option<f32> = div(12.0, 6.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: Dividing by 0.0"),
    }


//__result________________________________________________________
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // } 
    let result2: Result<f32, String> = divide(90.0, 0.0);
    match result2 {
        Ok(val) => println!("Result: {val}"),
        Err(e) => println!("Error is: {e}"),
    }





}