use crate::util::read_lines;

pub fn function() -> Result<[String; 2], String>{
    let str_input = read_lines("input/input6.txt");

    let mut str_4_ch = String::new();

    let mut marker = 0;

    for ch in str_input.get(0).unwrap().chars(){
        str_4_ch.insert(str_4_ch.len(), ch);
        marker += 1;
        if str_4_ch.len() > 4{
            str_4_ch.remove(0);
        }

        if str_4_ch.len() == 4{
            if is_string_chars_unique(&str_4_ch){
                break;
            }
        }

    }

    println!("{}", marker);

    Ok(["not implemented".to_string(), "not implemented".to_string()])
}

fn is_string_chars_unique(str : &String) -> bool{
    let mut str_cmp = String::new();

    

    for ch in str.chars(){
        for ch_cmp in str_cmp.chars(){
            if ch == ch_cmp{
                return false;
            }
        }
        str_cmp.insert(str_cmp.len(), ch);
    }

    return true;
}