// current issue, only reads letters and not numbers 
use std::io::{self,BufRead, BufReader};
use std::fs::File; 
fn main() {
    let mut user_input = String::new(); 
    println!("Please enter password for testing:"); 

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read password");
    let password = user_input.trim();
    println!("The Password you entered is: '{}', ...Checking password strength...", user_input.trim());
    // fixed problem here , file wasnt opening from relative or complete path, not sure why this was the fix but it works for rn
    let file = File::open("attributes.txt")
        .expect("Failed to open file provided");
    let reader = BufReader::new(file);

    let mut found = false;
    for line in reader.lines(){
        if let Ok(bad) = line{
            if bad.trim() == password{
                found = true;
                break;
            }
        }
    }

    if found{
        println!("❌ This password is weak. Score = 0. \nPlease try again.");
    }else if password.len() <12 && !found {
        println!("😓Password is unique but is <12 characters.\nPlease try again.");
    }else{
        println!("✅Password accepted, generating strength score...");
    }

}

