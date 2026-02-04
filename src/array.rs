pub fn run(){
    // Arrays
    println!("{}","Arrays");
    let  mut num =[10,20,20];
    let zeros =[0;5];
    num [2] =40;
    println!("{}",num[2]);
    println!("{:?}",zeros);
//Strings 

println!("{}","Strings");
 let name :&str="rust";
 println!("{}",name);
  //string slice (&str)
  println!("{}","String slice");
     let s=String::from("hello world ");
     let sli =&s [0..5];
     println!("{}",sli);

     //Heap allocated string
     println!("{}","Heap allocated string");
     let mut lang =String::from("hello");
     lang.push_str("programming");
     println!("{}",lang);


     let mut l =String::new();
    l.push_str("hello");
    l.push_str(" ");
    l.push_str("world");
    println!("{}",l);

    //Typecasting
    println!("{}","Tyoecasting");
    let a: i32=10;
    let b: f64= a as f64;
    println!(
"{}", b
    );

}