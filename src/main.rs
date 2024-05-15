use std::io;

fn user_input() -> i32{
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

fn main() {
    let choice :i32; 
    choice = user_input();
    println!("One month challenge of coding with RUST");
    println!("selected the {} choice", choice);
    match choice {
        1 => {
            println!("{}", choice);
        }
        _ => {
            print!("{} is not a valid choice", choice);
        }
    }
}
