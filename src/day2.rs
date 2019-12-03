mod day2 {
    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let mut v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            // .map(|value| value /3 -2)
            .collect();

        let mut idx = 0;
        while v[idx] != 99 {
            idx = match v.get(idx) {
                Some(1) => {
                    let mut l = v[idx + 1 as usize];
                    l = v[l as usize];
                    let mut r = v[idx + 2 as usize];
                    r = v[r as usize];
                    let result = l + r;
                    let r_idx = v[idx + 3] as usize;
                    v[r_idx] = result;
                    idx + 4
                }
                Some(2) => {
                    let mut l = v[idx + 1 as usize];
                    l = v[l as usize];
                    let mut r = v[idx + 2 as usize];
                    r = v[r as usize];
                    let result = l * r;
                    let r_idx = v[idx + 3] as usize;
                    v[r_idx] = result;
                    idx + 4
                }
                _ => idx,
            }
        }
        return v[0];
    }

    fn cal(noun: i32, verb: i32, mut v: Vec<i32>) -> i32 {
        v[1] = noun;
        v[2] = verb;
        let mut idx = 0;
        while v[idx] != 99 {
            idx = match v.get(idx) {
                Some(1) => {
                    let mut l = v[idx + 1 as usize];
                    l = v[l as usize];
                    let mut r = v[idx + 2 as usize];
                    r = v[r as usize];
                    let result = l + r;
                    let r_idx = v[idx + 3] as usize;
                    v[r_idx] = result;
                    idx + 4
                }
                Some(2) => {
                    let mut l = v[idx + 1 as usize];
                    l = v[l as usize];
                    let mut r = v[idx + 2 as usize];
                    r = v[r as usize];
                    let result = l * r;
                    let r_idx = v[idx + 3] as usize;
                    v[r_idx] = result;
                    idx + 4
                }
                _ => idx,
            }
        }
        return v[0];
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>, sum: i32) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        for i in 1..=99 {
            for j in 1..=99 {
                let d = cal(i, j, v.to_vec());
                if d == sum {
                    return i * 100 + j;
                }
            }
        }
        0
        //     let v: Vec<i32> = vec
        //         .iter()
        //         .map(|value| value.parse::<i32>().unwrap())
        //         .map(|value| cal(value))
        //         .collect();
        //     // println!("out:{:?}", v);
        //     let result = v.iter().sum();
        //     //
        //     // println!("out2:{:?}", result);
        //
        //     return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day2_part1() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50"
            .split(',')
            .map(|s| s.to_string())
            .collect();
        // println!("input:{:?}", input);
        let ret = day2::part1(input);
        assert_eq!(ret, 3500);
        let list = common::parse_from_file("./data/day2_part1.txt").unwrap();
        let mut input2: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        input2[1] = "12".to_string();
        input2[2] = "2".to_string();
        let ret2 = day2::part1(input2);
        // println!("out:{:?}", input);
        assert_eq!(ret2, 2842648);
    }

    #[test]
    fn day2_part2() {
        let list = common::parse_from_file("./data/day2_part1.txt").unwrap();
        let input2: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        let ret = day2::part2(input2, 19690720);
        assert_eq!(ret, 1);
        // let k = day2::cal(1969);
        // assert_eq!(k, 966);
        // let k2 = day2::cal(100756);
        // println!("out:{:?}", k2);
        // assert_eq!(k2, 50346);
        //
        // let list = common::parse_from_file("./data/day2_part2.txt");
        // let fuels = day2::part2(list.unwrap());
        // // println!("out:{:?}", fuels);
        // assert_eq!(fuels, 4870838);
    }
}
