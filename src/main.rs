
mod giga_second;
mod type_input;


fn main() {
    let choice :i32; 
    choice = type_input::ip_main::int_input();
    println!("One month challenge of coding with RUST");
    println!("selected the {} choice", choice);
    match choice {
        1 => {
            println!("{}", choice);
            giga_second::gs_main::giga_second();
            let x = type_input::ip_main::string_input();
            println!("{}", x);

        }
        _ => {
            print!("{} is not a valid choice", choice);
        }
    }
}
