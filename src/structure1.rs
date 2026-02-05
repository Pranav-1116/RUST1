pub fn run(){

    {
    // Wrapping primitive types for type safety
struct AccountId(String);
struct TokenAmount(u64);
struct _Timestamp(u64);

fn transfer(from: AccountId, to: AccountId, amount: TokenAmount) {
    println!("Transferring {} tokens from {} to {}", 
             amount.0, from.0, to.0);
}
 let sender = AccountId("alice.near".to_string());
    let receiver = AccountId("bob.near".to_string());
    let amount = TokenAmount(1000);
    
    transfer(sender, receiver, amount);

}
}