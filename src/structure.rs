pub fn run (){
 /*  //BASIC OF STRUCT  Field-by-field initialization
    struct Person {
        name : String,
        id :i32,
        age:u32,
        salary:f32
 }

  {
    let person1 =Person{
      name: String::from ("Joe"),
    age: 20,
    id:888,
    salary:90000.0
    };
    println!("Name:{}",person1.name);
    println!("Age: {}",person1.age);
    println!("Id: {}",person1.id);
    println!("Salary: {}",person1.salary);


  }
  //NORMAL impl
 impl Person{
fn _print(&self){
  println!("{} is {}",self.age,self.salary);
}
 }

 //Trait 

 trait Speak{
  fn speak (&self);
 }
 

   struct Dog;
   struct Cat;
   
    
    // TRAIT impl
   impl Speak for Dog{
    fn speak(&self){
      println!("the Dog barks");
    }
   }
   impl Speak for Cat{
    fn speak(&self){
      println!("the Cat says meow");
    }
   }

 {
  let dog =Dog;
  let cat =Cat;
  dog.speak();
  cat.speak();


 }
 

 //Another example for trait 

 
{
 trait Transfer{
  fn transfer(&mut self, amount :u32);
 }

 struct Wallet{
  balance :u32,
 }

 impl Transfer for Wallet{
 fn transfer(&mut self ,amount:u32){
  self.balance +=amount;
 }
 }
 let mut to=Wallet {balance:100} ;
  to.transfer(5000);
  println!("The transfered amouny is : {} ",to.balance);
}




 // problems on trait 



 {
trait Banking{
  fn deposite(&mut self, amount:u32);
  fn withdrawl (&mut self ,amount :u32) -> bool;

}

struct Transaction {
balance :u32,
}

impl Banking for Transaction{
  fn deposite(&mut self,amount:u32){
 self.balance +=amount;

  }
  fn withdrawl (&mut self,amount :u32)->bool {
if amount <= self.balance{
  self.balance -=amount;
  true
}
else{
  false
}
  }
}
let mut _account =Transaction{balance :10000};
let mut _account =Transaction{balance:10000};
_account.deposite(10000);
_account.withdrawl(2000);

println!("The Deposited Amount is : {}",_account.balance);
println!("The Withdrawl Amount is : {}",_account.balance);




 }


 //TUPLE +STRUCT 

 struct Point (i32,i32);

 let p=(10,20);
 println!("x :{}",p.0);
 println!("y:{}",p.1);

 //UNIT STRUCT 

 {
  struct Logger;
  impl Logger{
  fn logger(&self){
    println!("Loading data ");
  }                                    //no fields ,no data , only behavior 
}

let log=Logger;
log.logger();


 }


 struct Age(u8);

impl Age {
    fn new(age: u8) -> Option<Age> {
        if age <= 120 {
            Some(Age(age))
        } else {
            None
        }
    }

    fn value(&self) -> u8 {
        self.0
    }
}
 {
    let age = Age::new(25).unwrap();
    println!("Age: {}", age.value());
}
*/
// GENERIC STRUCT SINGLE
{
  struct Holder<T>{
    item:T,
  }
  let a=Holder{item:10};
  let b=Holder{item:"hello"};
  println!("{}",a.item);
  println!("{}",b.item);
}

//GENERIC STRUCT MULTIPLE
{
  struct Holder<T,U>{
    item1:T,
    item2:U,
  }
  let t1=Holder{item1:1,item2:78.09};
  let u1=Holder{ item1:"id",item2:true};
println!("{}",t1.item1);
println!("{}",t1.item2);

println!("{}",u1.item1);
println!("{}",u1.item2);


  }

  //ADDING THE LIFETIME IN THE GENERIC STRUCT 
  {
    struct Holder<'a,T>{
      item:&'a T,
    }
    let x=10;
    let t1=Holder{item:&x};
    println!("{}",t1.item);
    }
  // DESTRUCTING 
  {
    struct User{
      id:u32,
      name:String,
    }
    //Destructure
    let user =User{
      id:1,
      name:String::from("JOE"),
    };
    let User {id , name }
 = user;

 println!("id : {}",id);
 println!("name : {}",name);
  }


}