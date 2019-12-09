mod day8 {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(w: i32, h: i32, vec: &Vec<i32>) -> i32 {
        // let mut layers = vec![];
        let mut digit_num = vec![];
        let mut idx = 0;
        for i in 0..vec.len() {
            let cur = i as usize % (w * h) as usize;
            if cur == 0 {
                // layers[idx] = vec![];
                digit_num.push(vec![0, 0, 0]);
                // idx +=1;
            }
            let mut last = digit_num.last_mut().unwrap();
            // layers[idx][cur] = vec[i];
            match vec[i] {
                0 => last[0] += 1,
                1 => last[1] += 1,
                2 => last[2] += 1,
                _ => {}
            }
        }
        let mut max = 0;
        let mut min_idx = 0;
        for (idx, it) in digit_num.iter().enumerate() {
            if max == 0 || it[0] < max {
                min_idx = idx;
                max = it[0];
            }
        }
        // println!("aaa:{:?}", (digit_num.len(), min_idx));
        // println!("aaa:{:?}", digit_num[min_idx]);
        digit_num[min_idx][1] * digit_num[min_idx][2]
    }

    #[allow(dead_code)]
    pub fn part2(vec: &Vec<String>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day8_part1() {
        let input = common::read_from_file("./data/day8_part1.txt").unwrap();
        println!("out:{:?}", input.len());

        // let list = common::parse_from_str(&input);
        // let ret2 = day8::part1(&list);
        // assert_eq!(ret2, 417916);
        let v = input
            .chars()
            // .map(|value| value.to_string().parse::<i32>().unwrap())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        let ret = day8::part1(25, 6, &v);
        assert_eq!(ret, 1677);
    }

    // #[test]
    fn day8_part2() {}
}
