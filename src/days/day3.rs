use crate::util::read_lines;

pub fn function() -> Result<[String; 2], String>{
    
    //part 1
    let vect1 = read_lines("input/input3.txt");

    let mut total_sum1 = 0;

    for str in &vect1{
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
            total_sum1 += (char_equal as u32) - 38;
        }else{
            total_sum1 += (char_equal as u32) - 96;
        }
        
    }
    
    
    //part 2
    let mut group_vec: Vec<String> = Vec::new();
    let mut total_sum2 = 0;

    for str in &vect1{
        group_vec.push(str.to_string());

        if group_vec.len() == 3{
            let c = get_letter_equal(&group_vec);

            if c.is_ascii_uppercase(){
                total_sum2 += (c as u32) - 38;
            }else{
                total_sum2 += (c as u32) - 96;
            }

            group_vec.clear();
        }
    }


    Ok([total_sum1.to_string(), total_sum2.to_string()])
}

fn get_letter_equal(group_vector : &Vec<String>) -> char{

    let chars1 = group_vector.get(0).map(|string| string.chars()).unwrap_or_else(|| "".chars());
    let chars2 : Vec<char> = group_vector.get(1).map(|string| string.chars()).unwrap_or_else(|| "".chars()).collect();
    let chars3 : Vec<char> = group_vector.get(2).map(|string| string.chars()).unwrap_or_else(|| "".chars()).collect();

    let mut char_return = '\0';


'outer: for c1 in chars1{
            let chars2clone = chars2.clone();
            for c2 in chars2clone{
                if c1 == c2{
                    let chars3clone = chars3.clone(); 
                    for c3 in chars3clone{
                        if c3 == c1{
                            char_return = c3;
                            break 'outer;
                        }
                    }
                }
            }
        }

        return char_return;
    
}