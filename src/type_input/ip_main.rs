use std::io;


pub fn int_input() -> i32 {
    let mut input:String = String::new();
    let output:i32 ;
    match io::stdin().read_line(&mut input){
        Ok(n) => {
            println!("{n}");
            output = input.trim().parse::<i32>().unwrap();
            return output
        }
        Err(error)=>{
            panic!("Expected Integers, but got String!{}", error);
        }

    }
}

pub fn string_input() -> String{
    let mut input:String = String::new();
    match io::stdin().read_line(&mut input){
        Ok(n) =>{
            println!("{n}");
            return input
        }
        Err(error)=>{
            panic!("Expected String, {}", error);
        }
    }


}