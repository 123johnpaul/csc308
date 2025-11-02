use std::io;
fn main() {
    println!("Input your sentence");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input = user_input.trim_end().to_string();
    //let x = user_input.len();

    
    let words = identify_whitespace(&user_input);
    println!("Returned vector: {:?}", words);
    let (longest, shortest) = find_longest_and_shortest_strings(&words);
    match longest {
        Some(s) => println!("Longest string: \"{}\" (length: {})", s, s.len()),
        None => println!("No longest string found."),
    }

    match shortest {
        Some(s) => println!("Shortest string: \"{}\" (length: {})", s, s.len()),
        None => println!("No shortest string found."),
    }
    return
}

fn identify_whitespace(s:&String)->Vec<String>{
    let mut count = 0;
    let mut words:Vec<String> = Vec::new();
    let mut a:String = String::from("wosnsk");

    for (index, c) in s.char_indices() {
        if c == ' '{
            count += 1;
        }
        
    }

    for (index,c) in s.char_indices(){
        let mut increment = 0;
        let mut t = 0;
        if c == ' '{
            let mut q = index as usize;
            let mut y = t as usize;
            increment += 1;
            if increment == count {
                a = (&s[q+1..]).to_string();
                words.push(a);
                println!("{}",words[count]);
            }
            a = (&s[y..q]).to_string();
            println!("{}",a);
            words.push(a);
            t+=index;
        }
    }

    words
}

fn find_longest_and_shortest_strings(strings: &[String]) -> (Option<&String>, Option<&String>) {
    if strings.is_empty() {
        return (None, None); // Return None if the vector is empty
    }

    let mut longest_string: Option<&String> = None;
    let mut shortest_string: Option<&String> = None;

    for s in strings {
        // Initialize if first element or update longest
        if longest_string.is_none() || s.len() > longest_string.unwrap().len() {
            longest_string = Some(s);
        }
        // Initialize if first element or update shortest
        if shortest_string.is_none() || s.len() < shortest_string.unwrap().len() {
            shortest_string = Some(s);
        }
    }

    (longest_string, shortest_string)
}