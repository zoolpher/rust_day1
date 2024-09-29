


// FUNCTIONS
    // |- func. name should have snake case
// EXPRESSIONS - anything that returns value
    // |- 5 or 234 or23.6
    // |- boolean (true / false)
    // |- func(a, b) -> <return>
    // |- conditional statenents 
    //     |-if (condition) {code1} else {code2}
    // |- ({block of code})
    //     |- Ex-> my_store()vinside main func.
// STATEMENT - anything that does not return a value 
    // |- var. decleration
    //     |- let var: u64 = 90; 
    // |- func. defination
    //     |- fn my_func() { }
    // |-controlflow statements
    //     |- if else condition, while loop
    
    
    
// .into() => used in case of over flow
// ^^^ when (u8) * (u8) > (u8)
// use half::f16;


#[allow(dead_code)]   // ❗
// ^^^ was throwinng error   
// ^^^ coz main() is not used in thes prog.
fn main() {
    let a: String = say_hi();
    println!("{}", a);

    let p: u8 = product(12, 9);
    println!("{}", p);

    my_data("Krito", 5.9, 27);

    let my_bmi: f64 = bmi(1.6, 250.8);
    println!("My BMI: {:.2}", my_bmi);  // float till 2 decimal places
 
    let my_store = {  
        // expression
        let _store_name: String = String::from("\n---ROAD SIDE STORE---");
        let price: u8 = 9;
        let item_qty: u8 =20;
        let _total_worth: u8 = product(price, item_qty);
        // return price * item_qty;     // ❌
        price * item_qty    // ✅
    };
    println!("\nTotal worth of my store is: {}\n", my_store);
}

//_____________________________________________________________
fn say_hi() -> String {
    return "Hello rust".to_string();   // ✅
    // typecasting to String()
}

//_____________________________________________________________
fn product(i: u8, j: u8) -> u8 {
    // d_type of <args.> and <return> d_type should be same.     // ✅
    return i * j;
}

//_____________________________________________________________
fn my_data(name: &str, height: f32, age: u8) {
// fn my_data(name: &str, height: f32, age: u8) {    // ❌ 
    println!("\nName: {},\nHeight: {},\nAge: {}", name, height, age);
    // f8 and f16 are not default d_types of rust
    // ^^^we need to use some dependencues to use them   
}

//________________________________________________________________
fn bmi(height: f64, weight: f64) -> f64 {
    return weight / (height * height);
}
