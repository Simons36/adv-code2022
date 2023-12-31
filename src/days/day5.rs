use crate::util::read_lines;

use sscanf::sscanf;

pub fn function() -> Result<[String; 2], String>{
    //part 1

    let vect = &read_lines("input/input5.txt");
    let mut vect_iter = vect.iter();

    let mut vect_crates_raw = Vec::new();

    for str in &mut vect_iter{

        if str.chars().nth(1).unwrap() == '1'{
            break;
        }else{
            vect_crates_raw.push(str.to_string());
        }
    }

    let mut vec_crates : Vec<Vec<char>> = Vec::new();

    for _ in 0..9{
        vec_crates.push(Vec::new());
    }

    for str in vect_crates_raw{ //process input of crates
        let str_chars = str.chars();
        let mut i = 1;
        for ch in str_chars{
            if ch.is_ascii_uppercase(){
                let crate_row = ((i + 2) / 4) - 1;
                vec_crates[crate_row].push(ch);
            }
            i += 1;
        }
    }

    vect_iter.next();
    let mut vect_iter2 =  vect_iter.clone();
    let mut vec_crates2 = vec_crates.clone();

    for move_ in vect_iter{
        let res = sscanf!(move_, "move {} from {} to {}", usize, usize, usize).unwrap();

        let amount = res.0;
        let from = res.1;
        let to = res.2;

        let mut new_vec_from = vec_crates[from - 1].clone();
        let mut new_vec_to = vec_crates[to - 1].clone();

        for _ in 0..amount{
            new_vec_to.insert(0,new_vec_from.remove(0));
        }

        vec_crates[from - 1] = new_vec_from;
        vec_crates[to - 1] = new_vec_to;

    }

    let mut str_final = String::new();

    for vec in vec_crates{
        if vec.len() != 0{
            str_final.insert(str_final.len(), *vec.get(0).unwrap());
        }
    }

    //part 2

    for move_ in vect_iter2{
        let res = sscanf!(move_, "move {} from {} to {}", usize, usize, usize).unwrap();

        let amount = res.0;
        let from = res.1;
        let to = res.2;

        let mut new_vec_from = vec_crates2[from - 1].clone();
        let mut new_vec_to = vec_crates2[to - 1].clone();

        let mut crates_to_transfer : Vec<char> = Vec::new();

        for _ in 0..amount{
            crates_to_transfer.push(new_vec_from.remove(0));
        }

        for elem in crates_to_transfer.iter().rev(){
            new_vec_to.insert(0, *elem);
        }

        vec_crates2[from - 1] = new_vec_from;
        vec_crates2[to - 1] = new_vec_to;

    }

    let mut str_final2 = String::new();
    for vec in vec_crates2{
        if vec.len() != 0{
            str_final2.insert(str_final2.len(), *vec.get(0).unwrap());
        }
    }

    Ok([str_final.to_string(), str_final2.to_string()])
}