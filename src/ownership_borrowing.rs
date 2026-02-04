pub fn run (){
    //OWNERSHIP
    let x = String::from("hello");
    let x1=20; 
    println!("{}",x+&x1.to_string()); //x is the owner

    //OWNERSHIP MOVE and RULES
    let a=String::from("hello world ");
    let b =&a;
    println!("{}",b); //changing the owner 


    //STACK AND HEAP
    
    let c =10; //x->stack 
    let d =String::from("heap"); //d->heap
    println!("{}",d+&c.to_string());

    //VARIABLE SCOPE
    {
        let x = 9.0;
        println!("{}",x); // variable x is in the scope 
    } //here the variable x is out of scope 

    //MOVE SEMANTICS

    let s1=String::from("RUST");
    let s2 =&s1;
    println!("{}",s2);
    
    //CLONE
    let c1=String::from("Clone me ");
    let c2=c1.clone();
    println!("{}",c1);
    println!("{}",c2);

    //COPY TRATIT -> taking the value of other variable not the ownership
    let f=10;
    let e=f;
    println!("{}",f); //{only taking the value of f not the ownership

    println!("{}",e);     //  }

    //IMMUTABLE REFERENCE 
    let s1=String::from("RUST");
    let s2 =&s1;
    println!("{}",s2);      // we can read the value cant change the value of s1 

    //MUTABLE REFERENCE define in a scope
    {
        let  mut x =String::from("RUST");
        let  y =&mut x;
        y.push_str("IS A PROGRAMMING LANGUAGE ");
        println!("{}",y);
    }
    
    



    
}