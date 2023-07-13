use crate::util::read_lines;

pub fn function() -> Result<[String; 2], String>{

    //part 1

    let vect = &read_lines("input/input4.txt");

    let mut result = 0;

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
    }

    Ok([result.to_string(),"not implemented yet".to_string()])
}