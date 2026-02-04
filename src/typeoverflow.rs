pub fn run(){
    let x:u32=234567;
    let y:u16= x as u16;
    println!("y: {}",y);

     
//              INTEGER OVERFLOW HANDLING
 {



    // WRAPPING OPERATIONS (Always wrap around)


    println!(" WRAPPING OPERATIONS ");
    
    let max: u8 = 255;
    let min: u8 = 0;
    
    // wrapping_add: 255 + 1 = 0 (wraps)


    let wrapped_add = max.wrapping_add(1);
    println!("255u8.wrapping_add(1) = {}", wrapped_add);  // 0
    
    // wrapping_sub: 0 - 1 = 255 (wraps)

    let wrapped_sub = min.wrapping_sub(1);
    println!("0u8.wrapping_sub(1) = {}", wrapped_sub);  // 255
    
    // wrapping_mul: 200 * 2 = 144 (400 wraps)

    let wrapped_mul = 200u8.wrapping_mul(2);
    println!("200u8.wrapping_mul(2) = {}", wrapped_mul);  // 144
    
    // CHECKED OPERATIONS (Return Option)

    println!("\n CHECKED OPERATIONS ");
    
    // checked_add: Returns None on overflow

    let checked_add = 100u8.checked_add(50);
    let checked_overflow = 255u8.checked_add(1);
     
    println!("100u8.checked_add(50) = {:?}", checked_add);    // Some(150)
    println!("255u8.checked_add(1) = {:?}", checked_overflow); // None
    
    // Practical usage

    match 200u8.checked_add(18) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred!"),
    }
    
    // Using unwrap_or for default

    let result = 255u8.checked_add(1).unwrap_or(0);
    println!("With fallback: {}", result);  // 0
    
    // OVERFLOWING OPERATIONS (Return tuple)

    println!("\n OVERFLOWING OPERATIONS ");
    
    // Returns (result, did_overflow)

    let (result1, overflow1) = 100u8.overflowing_add(50);
    let (result2, overflow2) = 255u8.overflowing_add(1);
    
    println!("100u8.overflowing_add(50) = ({}, {})", result1, overflow1);
    // (150, false)
    
    println!("255u8.overflowing_add(1) = ({}, {})", result2, overflow2);
    // (0, true)
    
    // Check if overflow occurred

    if overflow2 {
        println!("Warning: Overflow detected!");
    }
    
    // SATURATING OPERATIONS (Clamp to bounds)

    println!("\n SATURATING OPERATIONS ");
    
    // saturating_add: Clamps to MAX instead of wrapping

    let sat_add = 250u8.saturating_add(10);
    println!("250u8.saturating_add(10) = {}", sat_add);  // 255 (clamped to max)
    
    let sat_add2 = 100u8.saturating_add(10);
    println!("100u8.saturating_add(10) = {}", sat_add2);  // 110 (no clamp needed)
    
    // saturating_sub: Clamps to MIN (0 for unsigned)

    let sat_sub = 5u8.saturating_sub(10);
    println!("5u8.saturating_sub(10) = {}", sat_sub);  // 0 (clamped to min)
    
    // Signed saturation

    let signed_sat = 120i8.saturating_add(20);
    println!("120i8.saturating_add(20) = {}", signed_sat);  // 127 (i8::MAX)
    
    let signed_sat_neg = (-120i8).saturating_sub(20);
    println!("(-120i8).saturating_sub(20) = {}", signed_sat_neg);  // -128 (i8::MIN)
    
    // PRACTICAL BLOCKCHAIN EXAMPLE

    println!("\n BLOCKCHAIN EXAMPLE ");
    
    // Token balance operations should use checked arithmetic!

    let balance: u64 = 1000000;
    let transfer_amount: u64 = 500000;
    
    // Safe subtraction

    match balance.checked_sub(transfer_amount) {
        Some(new_balance) => {
            println!("Transfer successful! New balance: {}", new_balance);
        }
        None => {
            println!("Error: Insufficient funds!");
        }
    }
    
   
    let small_balance: u64 = 100;
    let big_transfer: u64 = 1000;
    
    match small_balance.checked_sub(big_transfer) {
        Some(new_balance) => {
            println!("New balance: {}", new_balance);
        }
        None => {
            println!("REJECTED: Cannot transfer {} from balance of {}", 
                     big_transfer, small_balance);
        }
    }
}
}