mod day6 {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: &Vec<String>) -> i32 {
        let v: Vec<_> = vec
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
            let c = get_num(key.to_string(), &m);
            sum += c;
            // println!("{:?}", (key, c));
        }
        // println!("{:?}", sum);

        sum
    }

    #[allow(dead_code)]
    pub fn part2(vec: &Vec<String>) -> i32 {
        let v: Vec<_> = vec
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

        let you = "YOU".to_string();
        let san = "SAN".to_string();
        let you_list = get_list(you, &m);
        let san_list = get_list(san, &m);
        // println!("{:?}", you_list);
        // println!("{:?}", san_list);
        let mut found = 0;
        for idx in 0..you_list.len() {
            if you_list[idx] != san_list[idx] {
                found = idx;
                break;
            }
        }
        // println!("{:?}", found);
        let sum = you_list.len() - found + san_list.len() - found;

        sum as i32
    }

    fn get_num(mut key: String, m: &HashMap<String, String>) -> i32 {
        let mut count = 0;
        while let Some(v) = m.get(&key) {
            key = v.to_string();
            count += 1;
        }
        count
    }

    fn get_list(mut key: String, m: &HashMap<String, String>) -> Vec<String> {
        let mut list = Vec::<String>::new();
        while let Some(v) = m.get(&key) {
            key = v.to_string();
            list.push(key.clone());
        }
        list.reverse();
        list
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
        let list: Vec<String> = common::parse_from_file("./data/day6_part1.txt").unwrap();
        // let list = common::parse_from_str(&input);
        let ret2 = day6::part1(&list);
        assert_eq!(ret2, 417916);
    }

    #[test]
    fn day6_part2() {
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
        K)YOU
        I)SAN
        "#;
        let list = common::parse_from_str(i_str);
        let ret2 = day6::part2(&list);
        assert_eq!(ret2, 4);
        let list = common::parse_from_file("./data/day6_part1.txt").unwrap();
        let ret2 = day6::part2(&list);
        assert_eq!(ret2, 523);
    }
}
