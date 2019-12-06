mod day1 {
    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .map(|value| value / 3 - 2)
            .collect();
        // println!("out:{:?}", v);
        let result = v.iter().sum();
        // println!("out2:{:?}", result);

        return result;
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v: Vec<i32> = vec
            .iter()
            .map(|value| value.parse::<i32>().unwrap())
            .map(|value| cal(value))
            .collect();
        // println!("out:{:?}", v);
        let result = v.iter().sum();
        //
        // println!("out2:{:?}", result);

        return result;
    }

    pub fn cal(mut val: i32) -> i32 {
        let mut v = 0;
        val = val / 3 - 2;
        while val > 0 {
            // println!("v:{:?}", val);
            v += val;
            val = val / 3 - 2;
        }
        return v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day1_part1() {
        let list = common::parse_from_file("./data/day1_part1.txt");
        let fuels = day1::part1(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 3249140);
    }

    #[test]
    fn day1_part2() {
        let k = day1::cal(1969);
        assert_eq!(k, 966);
        let k2 = day1::cal(100756);
        println!("out:{:?}", k2);
        assert_eq!(k2, 50346);

        let list = common::parse_from_file("./data/day1_part2.txt");
        let fuels = day1::part2(list.unwrap());
        // println!("out:{:?}", fuels);
        assert_eq!(fuels, 4870838);
    }
}
