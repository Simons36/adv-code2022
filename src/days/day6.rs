use crate::util::read_lines;

pub fn function() -> Result<[String; 2], String>{
    let str_input = read_lines("input/input6.txt");

    

    let marker1 = get_marker_for_x_distinct_chars(str_input.get(0).unwrap(), 4); //part 1
    let marker2 = get_marker_for_x_distinct_chars(str_input.get(0).unwrap(), 14); //part 2

    

    Ok([marker1.to_string(), marker2.to_string()])
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

fn get_marker_for_x_distinct_chars(str : &String, nr_chars : usize) -> u32{
    let mut marker = 0;

    let mut str_4_ch = String::new();

    for ch in str.chars(){
        str_4_ch.insert(str_4_ch.len(), ch);
        marker += 1;
        if str_4_ch.len() > nr_chars{
            str_4_ch.remove(0);
        }

        if str_4_ch.len() == nr_chars{
            if is_string_chars_unique(&str_4_ch){
                break;
            }
        }
    }

    marker
}