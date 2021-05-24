use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", troutify(arg));
    }
}

fn troutify(s:String) -> String {
    let mut i:i32 = 0;
    let mut trout:String = "".to_string();

    for c in s.chars() {
        if !c.is_alphabetic() {
            trout.push(c);
        }
        else if i % 2 == 0 {
            trout.push_str(&c.to_uppercase().to_string());
        }
        else {
            trout.push_str(&c.to_lowercase().to_string());
        }

        i += 1;
    }

    return format!("{}", trout);
}
