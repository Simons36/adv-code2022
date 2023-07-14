use crate::util::read_lines;

pub fn function() -> Result<[String; 2], String>{

    //part 1

    let vect = &read_lines("input/input4.txt");

    let mut result = 0;
    let mut result2 = 0;

    for str in vect{
        let mut str_split = str.split(',');
        let mut range1_str = str_split.next().unwrap().split('-');
        let mut range2_str = str_split.next().unwrap().split('-');

        let range1 = [range1_str.next().unwrap().parse::<i32>().unwrap(), range1_str.next().unwrap().parse::<i32>().unwrap()];
        let range2 = [range2_str.next().unwrap().parse::<i32>().unwrap(), range2_str.next().unwrap().parse::<i32>().unwrap()];

        let range1_len = range1[1] - range1[0];
        let range2_len = range2[1] - range2[0];

        let is_1_smaller = range1_len < range2_len;

        if is_1_smaller{
            if (range1[0] >= range2[0]) && (range1[1] <= range2[1]){
                result += 1;
            }
        }else{
            if (range2[0] >= range1[0]) && (range2[1] <= range1[1]){
                result += 1;
            }
        }

        //part 2

        let end_2_after_1 = range2[1] >= range1[0]; //if end of 2 is "after" 1

        if end_2_after_1{
            if range2[0] <= range1[1]{
                result2 += 1;
            }
        }else{
            if range1[0] <= range2[1]{
                result2 += 1;
            }
        }
    }

    Ok([result.to_string(),result2.to_string()])
}