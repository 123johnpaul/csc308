use std::io;
fn main() {
    println!("Input your sentence");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = user_input.trim_end().to_string();
    //let x = user_input.len();

    let mut whitespace_position:u32 = 0;
    whitespace_position = identify_whitespace(&user_input,whitespace_position);

    let q = whitespace_position as usize;
    let a:String = (&user_input[q+1..]).to_string();
    println!("{}",a);

    return
}

fn identify_whitespace(s:&String,mut position:u32)->u32{
    for (index, c) in s.char_indices() {
        if c == ' '{
            position=index as u32;
        }
    }
    return position;
}