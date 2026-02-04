pub fn run(){

    //IMMUTABLE VARIABLES 
   const MAX_SUPPLY : u64 =100000;
   {
    let user_balance :u64 =0;
    let deposite :u64 =50;
    if user_balance +deposite <=MAX_SUPPLY{
        let result= user_balance+deposite;
         println!("transaction done {}",result);
    }
    else{
        println!("the transaction is failed due to amount fund ");
    }
   }

   // THE CONCEPT OF TYPE OVERFLOW USING THE checked_add(),Some and None 
   {
    println!("THE CONCEPT OF TYPE OVERFLOW");
    let private_key:u8=255;
    match private_key.checked_add(0){
        Some(s) =>{
            println!("THERE IS NO OVERFLOW : {}",s);

        }
        None =>{
            println!("there is a typeoverflow");
        }
    }


    //Transferring Assets
    println!("Transferring Assets");
    let token_data= String::from(" BTC:0990");
    let transfering_assts_to_mark= token_data;
    println!("NEW OWNER : {}",transfering_assts_to_mark);
   
}

   // MUTABLE REFERENCES 
   {
    
   }
}