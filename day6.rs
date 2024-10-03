

// Enums


#![allow(warnings)]
#[allow(dead_code)]   //❗
fn main() {
    // enum Currency {
    //     INR,
    //     USD,
    // }
    // let _inr: Currency = Currency::INR;
    // let _usd: Currency = Currency::USD;
    // fn exchange(_cur: Currency) {}
    // exchange(Currency::INR);
    // exchange(Currency::USD);

    // struct Market {
    //     kind: Currency,
    //     name: String,
    // }

    // let pintu: Market = Market {
    //     kind: Currency::INR,
    //     name: String::from("pintu"),
    // };
    // let harry: Market = Market {
    //     kind: Currency::USD,
    //     name: String::from("Harry"),
    // };

//__another way of diong it_______________
    enum IPadd {
        v4(u8, u8, u8, u8),
        v6(String),
    } 
    let home_v4:IPadd = IPadd::v4(128, 56, 23, 0);
    let home_v6:IPadd = IPadd::v6("1a.sd.33.7n.f5.8j".to_string());
    match home_v4 {
        IPadd::v4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        _ => (),
    }
    match home_v6 {
        IPadd::v6(ref s) => println!("IPv6: {}", s),
        _ => (),
    } 
   

}