mod day5 {

    #[derive(Debug, Clone)]
    pub enum Mode {
        Position,
        Immediate,
    }

    #[allow(dead_code)]
    pub fn part1(input: i32, vec: &Vec<String>) -> i32 {
        let mut v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            // .map(|value| value /3 -2)
            .collect();
        // println!("len,{:?}", (input, v.len()));

        let mut idx = 0usize;
        let mut mode_list = vec![Mode::Position, Mode::Position, Mode::Position];
        // let mut mode_list = mode_init;
        let mut out = Vec::new();
        while v[idx] != 99 {
            // println!("len,{:?}", v);
            let val = v[idx];
            let pcode = val % 100;
            let mut _mode = val / 100;
            for i in 0..3 {
                if _mode % 10 == 1 {
                    mode_list[i] = Mode::Immediate;
                } else {
                    mode_list[i] = Mode::Position;
                }
                _mode = _mode / 10;
            }
            idx = match pcode {
                1 | 2 => {
                    let l = get_param(idx + 1, &v, &mode_list[0]);
                    let r = get_param(idx + 2, &v, &mode_list[1]);
                    let r_idx = v[idx + 3] as usize;
                    let result = match pcode {
                        1 => l + r,
                        _ => l * r,
                    };
                    // println!("eeee,{:?}", (val, l, r, result));
                    v[r_idx as usize] = result;
                    idx + 4
                }
                3 => {
                    let pos = v[idx + 1 as usize] as usize;
                    v[pos] = input;
                    idx + 2
                }
                4 => {
                    let first = get_param(idx + 1, &v, &mode_list[0]);
                    // println!("set:{:?}", (idx, pcode, first));
                    out.push(first);
                    idx + 2
                }
                5 => {
                    let first = get_param(idx + 1, &v, &mode_list[0]);
                    let second = get_param(idx + 2, &v, &mode_list[1]);
                    // println!("set:{:?}", (idx, pcode, first, second));
                    if first != 0 {
                        second as usize
                    } else {
                        idx + 3
                    }
                }
                6 => {
                    let first = get_param(idx + 1, &v, &mode_list[0]);
                    let second = get_param(idx + 2, &v, &mode_list[1]);
                    // println!("set:{:?}", (idx, pcode, first, second));
                    if  first == 0 {
                        second as usize
                    } else {
                        idx + 3
                    }
                }
                7 => {
                    let first = get_param(idx + 1, &v, &mode_list[0]);
                    let second = get_param(idx + 2, &v, &mode_list[1]);
                    let third = v[idx + 3 as usize] as usize;
                    if first < second {
                        v[third] = 1;
                    } else {
                        v[third] = 0;
                    }
                    // println!("set:{:?}", (idx,pcode, first, second, third, v[third]));
                    idx + 4
                }
                8 => {
                    let first = get_param(idx + 1, &v, &mode_list[0]);
                    let second = get_param(idx + 2, &v, &mode_list[1]);
                    let third = v[idx + 3 as usize] as usize;
                    if first == second {
                        v[third] = 1;
                    } else {
                        v[third] = 0;
                    }
                    // println!("set:{:?}", (idx, pcode,first, second, third, v[third]));
                    idx + 4
                }
                _ => idx,
            }
        }
        // println!("out:{:?}", out);
        return *out.last().unwrap();
    }

    fn get_param(idx: usize, v: &Vec<i32>, mode: &Mode) -> i32 {
        match mode {
            Mode::Position => v[v[idx] as usize],
            _ => v[idx],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day5_part1() {
        let list = common::parse_from_file("./data/day5_part1.txt").unwrap();
        let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        let ret2 = day5::part1(1, &input);
        assert_eq!(ret2, 16225258);
    }

    #[test]
    fn day5_part2() {
        // let _str ="3,9,8,9,10,9,4,9,99,-1,8";
        // let list = common::parse_from_str(_str);
        // let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        // let ret2 = day5::part2(8, &input);
        // assert_eq!(ret2, 1);
        // let ret2 = day5::part2(7, &input);
        // assert_eq!(ret2, 0);
        // let _str = "3,3,1108,-1,8,3,4,3,99";
        // let list = common::parse_from_str(_str);
        // let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        // let ret2 = day5::part2(7, &input);
        // assert_eq!(ret2, 0);
        // let _str ="3,3,1107,-1,8,3,4,3,99";
        // let list = common::parse_from_str(_str);
        // let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        // let ret2 = day5::part2(7, &input);
        // assert_eq!(ret2, 0);
        // let _str ="3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        // let _str ="3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        // let _str ="3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        // let list = common::parse_from_str(_str);
        // let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        // let ret2 = day5::part1(7, &input);
        // assert_eq!(ret2, 0);

        let list = common::parse_from_file("./data/day5_part1.txt").unwrap();
        let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        let ret2 = day5::part1(5, &input);
        assert_eq!(ret2, 2808771);
    }
}
