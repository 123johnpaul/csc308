use std::io;
use std::thread;
use std::time::Duration;

struct Studentevaluation{
    firstname:String,
    lastname:String,
    score:f32,
    evaluation:bool,
}

impl Studentevaluation {

    fn evaluate(&mut self,) {
        if self.score >= 40.00{
            self.evaluation= false;
            println!("Student has failed the evaluation");
        }else{
            self.evaluation = true;
            println!("Student has passed the evaluation");
        }

    }

    fn display(&self) {
        println!("Student: {} {}", self.firstname, self.lastname);
        println!("Score: {}", self.score);
    }
}

fn main(){
    println!("Welcome to the Student Evaluation!");
    println!("Enter c to begin registration:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = user_input.trim().parse().expect("Please insert character!");
    
    if user_input=="c" || user_input=="C" {
        println!("Enter requred details");    
    } else {
        println!("Invalid input. Exiting.");
        return;
    }
    let mut student = create_student(); 
    
    println!("Evaluating Student");
    thread::sleep(Duration::from_secs(2));
    println!("Student Evaluated successfully");
    student.display();
    student.evaluate();
}

fn create_student() -> Studentevaluation {
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
    
    println!("Enter Student Score:");
    let mut score = String::new();
    io::stdin()
        .read_line(&mut score)
        .expect("Failed to read line");
    let score: f32 = score.trim().parse().expect("Please type a number!");

    Studentevaluation {
        firstname: firstname,
        lastname: surname,
        score: score,
        evaluation: false,
    }
}