use crate::util::{self, read_lines};

pub fn function() -> Result<[String; 2], String>{
    
    //part 1
    let vect1 = read_lines("input/input3.txt");

    let mut total_sum = 0;

    for str in vect1{
        let str_size = str.len();

        let part_size = str_size / 2;

        let str1 = &str[..part_size];
        let str2 = &str[part_size..];

        let chars1 = str1.chars();
        let chars2: Vec<char> = str2.chars().collect();

        let mut char_equal : char = '\0';

        
'outer: for char1 in chars1{
            let chars2clone = chars2.clone();
            for char2 in chars2clone{
                if char1 == char2{
                    char_equal = char1;
                    break 'outer;
                }
            }
        }

        if char_equal.is_ascii_uppercase(){
            total_sum += (char_equal as u32) - 38;
        }else{
            total_sum += (char_equal as u32) - 96;
        }

    }

    Ok([total_sum.to_string(), "not implemented yet".to_string()])
}