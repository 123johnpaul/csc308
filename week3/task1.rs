fn main() {
	let mut message = String::from("Hello");
	show_message(&message);
	add_note(&mut message);
	println!("Final Message: {}",message);
}

fn show_message(msg:&String) {
        println!("Current Message: {}",msg);
}

fn add_note(msg:&mut String) {
       msg.push_str(", world!");
}

