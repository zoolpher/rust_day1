// array
// arrat of &str
// tuple
// slice
// String & &str

fn main() {
    println!("Hello Rust ");
    let x:i16 = -65;
    let y:u16 = 54;
    println!("Signed integer = {}",x);
    println!("Unsigned integer = {}",y);
    

    let pi:f32 = 3.147;
    println!("Value of pi is = {}", pi);


    // there is difference b/w 'True' & 'true'
    let is_raining:bool = true;
    println!("It's raining outside = {}", is_raining);


    let character: char = 'j';
    println!("The characters are = {}", character);


//__ARRAY______________________________
    // array_name : [d_type ; size] = [arr...,...,...,...]
    let ar: [i32; 5] = [-23, -89, 63, 87, -4];
    println!("The typed aray is = {:?}", ar);


//__ARRAY OF &str__________________________________


    let a1: [&str; 4] = ["Hello ", "there ", "I am ", "Ark"];
    println!("The aray of string is = {:?}\n", a1); 
    println!("The first element in the string array is = {:?}", a1[0]); 
    println!("The second element in the string array is = {:?}", a1[1]); 
    println!("The third element in the string array is = {}", a1[2]); 
    println!("The fourth element in the string array is = {}\n", a1[3]); 


// __TUPLE___________________________________


    // .to_string()

    // let tup:(String, i16, bool, char) = ("YeahME", 90, false, 't');  
    // ^^^ it will throw error   ❌
    // let tup = ("YeahME", 90, false, 't'); 
    // ^^^ it will NOT throw error  ✅
    let tup:(String,String, i16, bool, char) = ("YeahME".to_string(),"🛩️".to_string(), 90, false, 't');  
    // ^^^ it will NOT throw error  ✅
    // "YeahME" is not a String so we need to typecast it to String 
    // we use .to_string() function for this
    println!("The elements of tuple are = {:?}", tup); 
    println!("The 1st element of tuple is = {}", tup.0);
    println!("The 2nd element of tuple is = {}", tup.1);
    println!("The 3rd element of tuple is = {}", tup.2);
    println!("The 4th element of tuple is = {}", tup.3);
    println!("The 5th element of tuple is = {}", tup.4);


// __SLICE_______________________________________
    

    let num_slice_1 = &[23,45,78,16,90];   // ✅
    println!("\nNum slice 1 = {:?}", num_slice_1);
    let num_slice_2: &[u16] = &[23,45,78,16,90];   // ✅
    println!("Num slice 1 = {:?}", num_slice_2);
    let animal_slice: &[&str] = &["Lion", "Elephant", "Croc"];   // ✅
    println!("The animals in slice are = {:?}", animal_slice);
    let book_slice: &[&String] = &[&"IT".to_string(), &"Arihant".to_string()];
    println!("The books in slice are = {:?}", book_slice);   // ✅


//__String_________________________


    // heap_(String)
    // [mutable, slow, dynamic storage]

    //.push_str(arg)

    let mut my_laptop:String = String::from("\nHow are u ?");   // ✅ 
    println!("My laptop says, {}", my_laptop);
    my_laptop.push_str("Hope good!!");
    println!("My laptop says, {}", my_laptop);


//__&str (string slice)______________________________


    // stack_(&str)
    // [immutable, fast, static storage, by dafault]

    let mut st1:String = String::from("\nThis is a string");
    println!("The original string is: {}", st1);
    let st2:&str = &st1;  // ✅
    println!("The reference string is: {}", st2);
    let st3:&str = &st2[1..6];   // ✅
    println!("The reference string is: {}", st3);
    // st3.push_str("Reference push");   ❌
    st1.push_str("Pushed in <String>");    // ✅
    println!("The pushed st1 <String> = {}", st1);


}
