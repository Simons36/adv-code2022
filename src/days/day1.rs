use crate::util;

pub fn function() -> Result<[String; 2], String>{
    let mut vect : Vec<String> = util::read_lines("input/input1.txt");

    vect.push("".to_string());
    let vect2 = vect.clone();
    
    //part 1

    let mut max1 = 0;
    let mut count1 = 0;

    for str in vect{
        if str.eq(""){
            if count1 > max1{
                max1 = count1;
            }

            count1 = 0;
        }else {
            count1 += str.parse::<i32>().unwrap();
        }
    }

    //part 2

    let mut max = 0;
    let mut count = 0;

    let mut max_vec : Vec<i32> = Vec::new();

    for str in vect2{
        if str.eq(""){
            
            max_vec.push(count);

            // let mut sorted_vec = max_vec.clone();
            // sorted_vec.sort();

            max_vec.sort();
            max_vec.reverse();

            max_vec.truncate(3);

            count = 0;

        }else{
            count += str.parse::<i32>().unwrap();
        }
    }

    for int in max_vec{
        max += int;
    }

    return Ok([max1.to_string(), max.to_string()]);
}