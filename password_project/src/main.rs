use std::io;

fn main() {
    let mut user_input = String::new(); 
    println!("Please enter password for testing:"); 

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read password"); 
        
    println!("The Password you entered is: '{}', ...Checking password strength...", user_input.trim());
}
