use std::io;
use std::thread;
use std::time::Duration;

struct Bankaccount{
    firstname:String,
    lastname:String,
    balance:f32,
}

impl Bankaccount {
    fn deposit(&mut self, amount:f32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount:f32) {
        self.balance -= amount;
    }

    fn display(&self) {
        println!("Account holder: {} {}", self.firstname, self.lastname);
        println!("Current balance: ${}", self.balance);
    }
}

fn main(){
    println!("Welcome to the Bank!");
    println!("Enter c to Create your bank account:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = user_input.trim().parse().expect("Please insert character!");
    
    if user_input=="c" || user_input=="C" {
        println!("we need your details to create your bank account");    
    } else {
        println!("Invalid input. Exiting.");
        return;
    }
    let mut account = create_account();
    account.display();
    println!("Creating your bank account...");
    thread::sleep(Duration::from_secs(2));
    println!("Account created successfully!");
    println!("\nManage your account");
    println!("Enter a to deposit to your bank account\nw to withdraw from your bank account\nd to display your bank account details:");
    let mut user_input1 = String::new();
    io::stdin()
        .read_line(&mut user_input1)
        .expect("Failed to read line");
    user_input1 = user_input1.trim().parse().expect("Please insert character!");

    if user_input1 != "a" || user_input1 != "w" || user_input1 !="d" || user_input1 != "A" || user_input1 != "W" || user_input1 !="D" {
        println!("Invalid input. Exiting.");
        return;
    }else{
        if user_input1 == "a" || user_input1 == "A" {
            println!("\nEnter amount to deposit:");
            let mut deposit_amount = String::new();
            io::stdin()
                .read_line(&mut deposit_amount)
                .expect("Failed to read line");
            let deposit_amount: f32 = deposit_amount.trim().parse().expect("Please type a number!");
            account.deposit(deposit_amount);
            account.display();
        } else if user_input1 == "w" || user_input1 == "W" {
            println!("\nEnter amount to withdraw:");
            let mut withdraw_amount = String::new();
            io::stdin()
                .read_line(&mut withdraw_amount)
                .expect("Failed to read line");
            let withdraw_amount: f32 = withdraw_amount.trim().parse().expect("Please type a number!");
            account.withdraw(withdraw_amount);
            account.display(); 
        } else {
            account.display();
        }
    }  
}

fn create_account() -> Bankaccount {
    println!("Enter Firstname:");
    
    let mut firstname = String::new();
    io::stdin()
        .read_line(&mut firstname)
        .expect("Failed to read line");
    let firstname = firstname.trim().parse().expect("Please type a number!");
    
    println!("Enter Surname:");
    let mut surname = String::new();
    io::stdin()
        .read_line(&mut surname)
        .expect("Failed to read line");
    let surname = surname.trim().parse().expect("Please type a number!");
    
    println!("Enter First deposit:");
    let mut balance = String::new();
    io::stdin()
        .read_line(&mut balance)
        .expect("Failed to read line");
    let balance: f32 = balance.trim().parse().expect("Please type a number!");

    Bankaccount {
        firstname: firstname,
        lastname: surname,
        balance: balance,
    }
}