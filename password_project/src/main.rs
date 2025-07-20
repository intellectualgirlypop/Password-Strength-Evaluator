// current issue, only reads letters and not numbers 
use std::io::{self,BufRead, BufReader};
use std::fs::File; 
use std::thread::sleep;
use std::time::Duration;
fn main() {
    loop{
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
   let special_file = File::open("special_characters.txt")
    .expect("Failed to open file provided");
   let reader_spec = BufReader:: new(special_file);
   let special_chars: String = reader_spec
   .lines()
   .filter_map(Result::ok)
   .collect::<Vec<String>>()
   .join("");


   let foundspec = password.chars().any(|c| special_chars.contains(c));
// change for better logic 
    if found{
        println!("❌ This password is generic. generating score ...");
        sleep(Duration::from_secs(3));
    }else if
        password.len() >=12 && foundspec{
            println!("Password accepted ✅! generating score..");
            sleep(Duration::from_secs(3));
    };

    let mut score =0;
    if password.len() >= 12{
        score+=1;
    }
    if foundspec{
        score += 1;
    }
    if !found{
        score += 1;
    }

    match score{
        3 => {
            println!("✅ strong password, you are good to go!.\nSCORE = 100%");
            break;
        },
        2 => {
            println!("🟡 password is medium strength, try again!.\nSCORE = 60% ");
            continue;
        },
        1 => {
            println!("🔴 very weak password. Boo.\nSCORE = 30%");
            continue;
        },
        _ => {
            println! ("❌ absolutely not. L password. \nSCORE = 0% ");
            continue;
        },
    }
}
}


