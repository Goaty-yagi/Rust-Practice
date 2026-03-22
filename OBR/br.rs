// > **At any given time, you can have:**
// > 
// - **ONE mutable reference (`&mut`)**
// - OR **ANY NUMBER of immutable references (`&`)**
// - **BUT NOT BOTH at the same time**

fn main(){
    let _x = 5; // type is i32
    let _r = &_x; // pass references so type is &i32
    // let _val: i32 = _r; // can't do like this cause it _r is &i32 

    println!("Value of _x : {}", _x); // Owner
    println!("Value of _r : {}", _r); // Reference

    let mut _xx = 5;
    let _rr = &mut _xx; // mutable borrowing

    *_rr += 1; // * access the value of the reference
    *_rr -= 3;

    // cant access here
    // println!("Mutable Value: {}", _xx);
    println!("Mutable Value: {}", _rr); // 3

    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(50.5);
    account.check_balance();

}

struct BankAccount {
    owner: String,
    balance: f64,
}
impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}