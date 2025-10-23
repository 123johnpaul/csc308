fn main(){
        let mut name = String::from("Johnpaul");
        add_surname_to_firstname(&mut name);
        println!("Full name: {}", name);
}

fn add_surname_to_firstname(n: &mut String){
        n.push_str(" Emmanuel");
}
