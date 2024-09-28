use std::io;

fn main() {
    println!("We speak pig-latin here, what are you trying to say? In English first...");

    let mut storage_vec = Vec::new();
    manual_entry(&mut storage_vec);
}


fn manual_entry(v: &mut Vec<String>) {
    let mut loop_check = true;

    while loop_check {
        println!("Please enter your phrase to convert to pig latin: 'X' to Exit");
    
        let mut user_entry = String::new();
        let mut clean_float = Vec::new();
    
        io::stdin()
            .read_line(&mut user_entry)
            .expect("Failed to read line");
    
        for char in user_entry.chars() {
            if char.is_ascii(){
                clean_float.push(char as u8)
            }
        }
        
        if !clean_float.is_empty() {
            v.push(String::from_utf8(clean_float).expect("Our bytes should be valid utf8"));
        }
    
        for char in user_entry.chars() {
            if char == 'x' || char == 'X'{
                loop_check = false;
                println!("Exiting...");
                break;
            }
        }
    }
}
