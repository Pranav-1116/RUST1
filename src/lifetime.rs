pub fn run() {

 //SINGLE LIFETIME PARAMETER 
println!("SINGLE LIFETIME PARAMETER ");
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let s1 =String::from("Rust");
    let s2 = String::from("programming");
 //   let s3 =20;
   let  result=longest(s1.as_str(),s2.as_str());
   println!("{}",result );

   {
    fn short<'a>(x: &'a str, y: &'a str, n:&'a i32) -> &'a str {
        println!(" Numbers: {}",n);
        if x.len() < y.len() {
            x
        } else {
            y
        }
    }
let s1 =String::from("Rust");
    let s2 = String::from("programming");
 let s3:i32 =20;
   let  result=short(s1.as_str(),s2.as_str(),& s3);
   println!("{}",result );

   }

//MULTIPLE LIFETIME PARAMETER 
{
    println!("MULTIPLE LIFETIME PARAMETER ");
    fn select <'a ,'b>(x:&'a str, _y:&'b str,first:bool) -> &'a str{
        if first{
            x 
        }
        else{
            panic!("mustchoose first");
        }
    }
    let s1 =String::from("hello");
    let s2=String::from("world");
    let result=select(&s1.as_str(),&s2.as_str(),true);
    println!("RESULT :  {}",result);
}

   
}

