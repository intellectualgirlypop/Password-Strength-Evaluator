// current issue, only reads letters and not numbers 
fn main() {
    let mut user_input = String::new(); 
    println!("Please enter password for testing:"); 

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read password");
    println!("The Password you entered is: '{}', ...Checking password strength...", user_input.trim());
    let password = user_input.trim();
    let bad_password = r"password_project\attributes.txt";{
        if bad_password.contains(password){
        println!("❌ This password is too generic. Score = 0. 
        Please try again")
        }else{
        println!("✅ Password accepted, evaluating further ...")
    }
    }
}

