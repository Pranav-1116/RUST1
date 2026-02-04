pub fn run (){
      //GETTING THE INPUT FROM THE USER 
      use std::io;
      {
        let mut input =String::new();
        println!("Enter your name");

        io::stdin()
        .read_line (&mut input)
        .expect("Failed to read input");
    println!("You entered the name as :{}",input);
      }

      {
        let mut input = String::new();
        println!("Enter a number");
        io::stdin().read_line(&mut input).expect("faild to add the number");

        let _x:u32=input.trim().parse().expect("Enter the valid number");
        println!("The number you have enter is : {}",input);


      }
}
