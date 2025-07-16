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
    let add1 = File::open("special_char.txt")
        .expect("Failed to open file provide");
    let reader2 = BufReader::new(add1);
// change here to parse like at all. 
    let mut foundspec = false;
    for line in reader2.lines(){
        if let Ok(bad) = line{
            if bad.trim() == password{
                foundspec = true;
                break;
            }
        }
    }
// change for better logic 
    if found{
        println!("❌ This password is generic. Score = 0. \nPlease try again.");
    }else if password.len() >11 && foundspec{
            println!("Password accepted ✅! generating score..");
    } else {
    println!(" Password is weak but not generic. Not long enough or lacks special characters.");
}
}

