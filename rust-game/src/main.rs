use std::io;
use rand::seq::IndexedRandom; 


fn UserPassword() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut passwordGuess = (input.trim().to_string());
    return passwordGuess;
}

fn PasswordFn(passwords: &[&str]) -> String {
    let password = passwords.choose(&mut rand::rng()).unwrap();
    return password.to_string(); 
    /*use random to pick a password from the list
    and assign that to password */ 
}

fn main() {
let passwords = vec!["tryhackme", "P@ssw0rd", "password", "s4feP@s5word", "n0h4ck1ng"];
let mut attempts = 0;
let mut score = 0;
let i = rand::random_range(0.. passwords.len());
let secret = passwords[i];
let maxAttempts = 7;
let password = PasswordFn(&passwords);

loop {
    let passwordGuess = UserPassword(); 
     if passwordGuess == password() {
        println!("YOU WIN!!!");
        println!("your score is + {score}" );
        println!("took {attempts} attempts");
    break;

        elif attempts == maxAttempts{
    println!("bounce back!!");
    break;
    }

    else { 
    println!("incorrect {attempts} / 7 attempts");
    attempts += 1
    //score -= 7 
    UserPassword();
    }
}
   
}

}


