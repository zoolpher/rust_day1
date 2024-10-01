
/*
 -- struct {}
 -- var./val. as d_type
 -- variables & immitability
 -- multiline comments
*/


// can declare const. in a global scope
const PI:f64 = 3.14;


#[allow(dead_code)]   // ❗
fn main() {
//__structures________________________________________
    let mut acc1: BankAcc = BankAcc {
        owner: "Doom".to_string(),
        balance: 1800.122,
    };
    acc1.check_balance();  // immutable to check balance
    //__typecasting____________________________________________
    acc1.withdraw(100 as f64); // mutable to withdrawal
    // converting int -> f64
    acc1.check_balance();

//__var. & mutablility______________________________________________
    // constants & var. are different
    let mut _var: u8 = 90;
    println!("{}", _var);
     _var += 10;
    println!("{}", _var);

//__constants______________________________________________________
    // they are immutabel bydefault
    // var. name should be all caps.  ✅
    // never use <mut> with const.  ❌
    // const mut h:u8 = 16;  ❌
    const H: u8 = 16;
    println!("\n{}", H);
    println!("PI value = {}", PI);


//__SHADOWING_____________________________________________
    let v1: u8 = 6;
    let v1: u8 = v1 + 2;   // 8
    // value of 1st <v1> dropped here
    println!("\nValue of v1 = {}", v1);
    {
        let v1: u8 = v1 + 2;   // 10
        println!("Vlaue of v1 inside a block = {}", v1);
        //value of inner <v1> dropped
    }
    println!("\nValue of v1 out of the block = {}", v1);

}

struct BankAcc {
    owner : String,
    balance : f64,
}
impl BankAcc {
    // seld -> refers to the fields of structure
    // self.owner   ✅
    // self.balance    ✅
    // < &mut self > will make mutable reference for us--
    // -- but for user it remains imutable by < &self >
    fn withdraw(&mut self, amt: f64) {
        println!("Withdrawing {} from acc. owner {}", amt,self.owner);
        self.balance -= amt;
    }
    fn check_balance(&self) {
        println!("{} owns {} amount in the bank.", self.owner, self.balance);
    }
}