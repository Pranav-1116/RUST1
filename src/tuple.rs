pub fn run(){

     //SIMPLE TUPLE STRUCT 

     // Definition
struct Color(u8, u8, u8);

 {
    // Creation
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    
    // Access using dot notation with index
    println!("Red component: {}", red.0);
    println!("Green component: {}", green.1);
    println!("Blue component: {}", blue.2);
    
    // Destructuring
    let Color(r, g, b) = red;
    println!("RGB: ({}, {}, {})", r, g, b);
}

 //TUPLE STRUCTS WITH METHODS

 struct Point3D(f64, f64, f64);

impl Point3D {
    // Associated function (constructor)
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D(x, y, z)
    }
    
    // Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    
    // Method to scale the point
    fn scale(&mut self, factor: f64) {
        self.0 *= factor;
        self.1 *= factor;
        self.2 *= factor;
    }
    

    // Method for dot product
    fn dot(&self, other: &Point3D) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

{
    let mut point = Point3D::new(3.0, 4.0, 0.0);
    println!("Distance from origin: {}", point.distance_from_origin());
    
    point.scale(3.0);
    println!("After scaling: ({}, {}, {})", point.0, point.1, point.2);
    
    let other = Point3D::new(1.0, 0.0, 0.0);
    println!("Dot product: {}", point.dot(&other));
}


//SMART CONTRACT TOKEN EXAMPLE 

// Simulating a smart contract token structure

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TokenAmount(u128);

#[derive(Debug, Clone, PartialEq, Eq)]
struct AccountId(String);

#[derive(Debug)]
struct Balance(AccountId, TokenAmount);

impl TokenAmount {
    fn new(amount: u128) -> Self {
        TokenAmount(amount)
    }
    
    fn checked_add(&self, other: TokenAmount) -> Option<TokenAmount> {
        self.0.checked_add(other.0).map(TokenAmount)
    }
    
    fn checked_sub(&self, other: TokenAmount) -> Option<TokenAmount> {
        self.0.checked_sub(other.0).map(TokenAmount)
    }
    
  
}

impl AccountId {
    fn new(id: &str) -> Self {
        AccountId(id.to_string())
    }
    
    fn validate(&self) -> bool {
        !self.0.is_empty() && self.0.len() <= 64
    }
}

impl Balance {
    fn new(account: AccountId, amount: TokenAmount) -> Option<Self> {
        if account.validate() {
            Some(Balance(account, amount))
        } else {
            None
        }
    }
    
    fn credit(&mut self, amount: TokenAmount) -> Result<(), String> {
        match self.1.checked_add(amount) {
            Some(new_amount) => {
                self.1 = new_amount;
                Ok(())
            }
            None => Err("Overflow error".to_string())
        }
    }
    
    fn debit(&mut self, amount: TokenAmount) -> Result<(), String> {
        match self.1.checked_sub(amount) {
            Some(new_amount) => {
                self.1 = new_amount;
                Ok(())
            }
            None => Err("Insufficient balance".to_string())
        }
    }
}

 {
    let account = AccountId::new("alice.near");
    let initial_amount = TokenAmount::new(1000);
    
    let mut balance = Balance::new(account, initial_amount)
        .expect("Invalid account");
    
    println!("Initial: {:?}", balance);
    
    // Credit operation
    balance.credit(TokenAmount::new(500)).unwrap();
    println!("After credit: {:?}", balance);
    
    // Debit operation
    match balance.debit(TokenAmount::new(20)) {
        Ok(_q) => println!("Debit successful"),
        Err(e) => println!("Debit failed: {}", e),
    }
    
    // Valid debit
    balance.debit(TokenAmount::new(500)).unwrap();
    println!("After debit: {:?}", balance);
}


//TYPE TAGGING

{
    struct TokenAmount(u64);
    struct Age(u64);
    
    fn transfer(amount:TokenAmount){
        println!("Transfering the amount : {}",amount.0);
    }

    fn age(age:Age){
        println!("Age is :{}",age.0);
    }
    {
        transfer(TokenAmount(26));
        age(Age(26)); 
    }
}


// Tuple struct with no fields (unit-like)

// Single-field tuple struct
struct Wrapper(i32);

impl Wrapper {
    fn unwrap(self) -> i32 {
        self.0
    }
}

// Used for type tagging
struct Public(String);
struct Private(String);

fn publish_data(data: Public) {
    println!("Publishing: {}", data.0);
}
fn publish_data1(data1:Private){
    println!("Publishing :{}",data1.0);
}
 {
    let public_msg = Public("Hello World".to_string());
    publish_data(public_msg);
    
    let private_msg = Private("Secret".to_string());
     publish_data1(private_msg); 
    let wrapped = Wrapper(42);
    let value = wrapped.unwrap();
    println!("Unwrapped: {}", value);
}

//TUPLE WITHH THE GENERICS 

struct Stat<T>( T);

impl <T> Stat<T>{
    fn get (&self) -> &T{
        &self.0
    }
}

struct Teams{
    name:String,
    runs:Stat<u32>,
    wickets:Stat<u8>
} 

impl Teams{
    fn display(&self){
        println!("TEAMS :{} ,RUNS :{}, WICKETS : {}",self.name,self.runs.0,self.wickets.0);
    }
}

{
    let csk =Teams{
        name:String::from("CSK"),
        runs:Stat(200),
        wickets:Stat(5),

    };
    csk.display();
}


}