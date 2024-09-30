use std::io;

fn main() {
    println!("We speak pig-latin here, what are you trying to say? In English first...");

    let mut sentence_vec = Vec::new();
    manual_entry(&mut sentence_vec);
    
    let split_sentence = split_string_on_space_return_multi_part_vec(sentence_vec[0].clone());
    let mut piggified_sentence = String::new();
    for word in split_sentence {
        let word = convert_to_pig_latin(word);
        let space = " ";
        for letter in word {
            piggified_sentence = piggified_sentence + &letter;
        }
        piggified_sentence = piggified_sentence + space;
    }
    println!("{:?}", piggified_sentence.trim());
}

fn convert_to_pig_latin(s: String) -> Vec<String> {
    let mut clean_float = Vec::new();
    let mut clean_float_check = Vec::new();

    // Clean up input for manipulation
    for char in s.chars() {
        if char.is_ascii(){
            clean_float.push(String::from(char));
            clean_float_check.push(char);
        }
    }
    
    // Check for symbols and log them with index info
    let mut temp_symbol_hold = Vec::new();
    for i in 0..clean_float_check.len() {
        if clean_float_check[i].is_ascii_punctuation() {
            let mut symbol = Vec::new();
            let vec_data = clean_float_check[i].to_string();
            let vec_index = i.to_string();
            symbol.push(vec_data);
            symbol.push(vec_index);
            temp_symbol_hold.push(symbol);
            clean_float.remove(i);
        }
    }

    // Check for uppercase and log index info
    let mut uppercase_tracker = Vec::new();
    for i in 0..clean_float_check.len() {
        if clean_float_check[i].is_ascii_uppercase() {
            let vec_index = i.to_string();
            uppercase_tracker.push(vec_index);
        }
    }
    
    // Set all characters to lowercase
    for i in 0..clean_float.len() {
        clean_float[i] = clean_float[i].to_lowercase();
    }

    // Match begining char to vowel and -hey if so, otherwise normal manipulation, also regulates symbol placement post transmutation
    if matches!(clean_float[0].as_str(), "a" | "e" | "i" | "o" | "u" | "y" | "A" | "E" | "I" | "O" | "U" | "Y" ) {
        clean_float.push("-".to_string());
        clean_float.push("h".to_string());
        clean_float.push("a".to_string());
        clean_float.push("y".to_string());
        if temp_symbol_hold.len() > 0 {
            for symbol in temp_symbol_hold {
                let mut symbol_index: usize  = symbol[1].parse().expect("Failed to parse...");
                if symbol[0] != "'" {
                    symbol_index += 4;
                }
                clean_float.insert(symbol_index, symbol[0].clone());
            }
        }
    } else {
        clean_float.push(clean_float[0].clone());
        clean_float.remove(0);
        if temp_symbol_hold.len() > 0 {
            for symbol in temp_symbol_hold {
                clean_float.insert(symbol[1].parse().expect("Failed to parse..."), symbol[0].clone());
            }
        }
    }

    // Re-establish capitals in the updated order.    
    if uppercase_tracker.len() > 0 {
        for i in 0..uppercase_tracker.len() {
            let uppercase_index_string = uppercase_tracker[i].clone();
            let index: usize = uppercase_index_string.parse().expect("Failed to parse index");
            clean_float[index] = clean_float[index].to_uppercase();
        }
    }

    clean_float
}

fn split_string_on_space_return_multi_part_vec(s: String) -> Vec<String> {
    let parts: Vec<String> = s
        .split(" ")
        .map(|part| part.to_string())
        .collect();
    parts
}

fn manual_entry(v: &mut Vec<String>) {
    println!("Please enter your phrase to convert to pig latin: 'X' to Exit");
    
    let mut user_entry = String::new();
    let mut clean_float = Vec::new();
    
    io::stdin()
        .read_line(&mut user_entry)
        .expect("Failed to read line");
    
    user_entry = user_entry.trim().to_string();

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
            println!("Exiting...");
        }
    }
}