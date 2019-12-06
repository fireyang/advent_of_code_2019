mod day6 {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: &Vec<String>) -> i32 {
        let mut v: Vec<_> = vec
            .iter()
            .map(|value| value.split(")").collect())
            .map(|value: Vec<&str>| (value[0], value[1]))
            .collect();

        let mut m: HashMap<String, String> = HashMap::new();
        for (ref a, ref b) in v.iter() {
            // println!("{:?}){:?}", a, b);
            m.insert(b.to_string(), a.to_string());
        }
        // println!("{:?}", m);

        let mut sum = 0;
        for key in m.keys() {
            let c = getNum(key.to_string(), &m);
            sum += c;
            // println!("{:?}", (key, c));
        }
        // println!("{:?}", sum);

        sum
    }

    fn getNum(mut key: String, m: &HashMap<String, String>) -> i32 {
        let mut count = 0;
        while let Some(v) = m.get(&key) {
            key = v.to_string();
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day6_part1() {
        let i_str = r#"
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        "#;
        // let list = common::parse_from_file("./data/day6_part1.txt").unwrap();
        // let input: Vec<String> = list[0].split(',').map(|s| s.to_string()).collect();
        let list = common::parse_from_str(i_str);
        let ret2 = day6::part1(&list);
        assert_eq!(ret2, 42);
        let list:Vec<String> = common::parse_from_file("./data/day6_part1.txt").unwrap();
        // let list = common::parse_from_str(&input);
        let ret2 = day6::part1(&list);
        assert_eq!(ret2, 0);
    }

    // #[test]
    fn day6_part2() {}
}
