use std ::collections::HashMap;
pub fn run (){

    {
        let mut scores =HashMap::new();
        scores.insert("CSK",10);
        scores.insert("MI",8);
        println!("{:?}",scores);
    }
}