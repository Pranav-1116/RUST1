pub fn run(){

//ENUMERATION +MATCH METHOD 

    #[derive(Debug)]
enum Direction {
    Left(i32),
    Right(&'static str),
    _Up,
    _Down,
}

 
    let d = Direction::Left(200);
    let e =Direction::Right("printed");
    match d{
        Direction::Left(a)=>println!("{:?}", a), _=>{}
    }
    match e{
        Direction::Right(b)=> println!("{:?}",b), _=>{}
    
    }
   
     //ENUMERATION +DATA

     {
        #[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
    Quit,
}


    let m1 = Message::Text("Hello".to_string());
    let m2 = Message::Number(10);
    let _m3 = Message::Quit;
    match m1{
        Message::Text(c)=>println!("{:?}",c), _=>{}
    }
    match m2 {
                Message::Number(f)=>println!("{:?}",f), _=>{}
}    
    }

 //ENUMERATION +DESTRUCTURING

 {
#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
    Quit,
}

fn process(msg: Message) {
    match msg {
        Message::Text(s) => println!("Text: {}", s),
        Message::Number(n) => println!("Number: {}", n),
        Message::Quit => println!("Exit"),
    }
}


    let m1 = Message::Text("Hello".to_string());
    let m2 = Message::Number(10);
    let m3 = Message::Quit;

    process(m1);
    process(m2);
    process(m3);


 }

}