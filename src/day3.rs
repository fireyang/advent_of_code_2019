mod day3 {

    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn part1(vec: Vec<String>) -> i32 {
        let v: Vec<Vec<Dirt>> = vec
            .iter()
            .map(|value| parse_vec(value.to_string()))
            // .map(|value| value /3 -2)
            .collect();

        let mut m = HashMap::new();
        let mut ret_v = Vec::new();
        let mut idx = 0;
        for i in &v {
            let mut p = (0, 0);
            for n in i {
                // println!("n out:{:?}", n);
                let (x, y, len) = match n {
                    Dirt::Left(inner) => (-1, 0, *inner),
                    Dirt::Right(inner) => (1, 0, *inner),
                    Dirt::Up(inner) => (0, 1, *inner),
                    Dirt::Down(inner) => (0, -1, *inner),
                    Dirt::None => (0, 0, 0),
                };
                for _j in 0..len {
                    p = (p.0 + x, p.1 + y);
                    if idx == 0 {
                        // println!("a:{:?}", &p);
                        m.insert(p, 1);
                    } else {
                        // println!("b:{:?}", &p);
                        if let Some(_) = m.get(&p) {
                            ret_v.push(p);
                        }
                    }
                }
            }
            idx += 1;
        }
        let mut min_size = 0;
        for (x, y) in ret_v {
            let v = (x as i32).abs() + (y as i32).abs();
            if min_size == 0 || v < min_size {
                min_size = v
            }
            // println!("{:?},{:?}", x, y);
        }
        // println!("{:?},{:?}", idx,ret_v);

        min_size
    }

    fn parse_vec(s: String) -> Vec<Dirt> {
        let v = s
            .split(",")
            .map(|v| {
                let (h, t) = v.split_at(1);
                let l = t.parse::<i32>().unwrap();
                match h {
                    "R" => Dirt::Right(l),
                    "L" => Dirt::Left(l),
                    "U" => Dirt::Up(l),
                    "D" => Dirt::Down(l),
                    _ => Dirt::None,
                }
            })
            .collect();
        // println!("out,{:?}", v);
        v
    }

    #[derive(Debug)]
    enum Dirt {
        None,
        Left(i32),
        Right(i32),
        Up(i32),
        Down(i32),
    }

    #[allow(dead_code)]
    pub fn part2(vec: Vec<String>) -> i32 {
        let v: Vec<Vec<Dirt>> = vec
            .iter()
            .map(|value| parse_vec(value.to_string()))
            // .map(|value| value /3 -2)
            .collect();

        let mut m = HashMap::new();
        let mut idx = 0;
        let mut min_size = 0;
        for i in &v {
            let mut p = (0, 0);
            let mut step = 0;
            for n in i {
                // println!("n out:{:?}", n);
                let (x, y, len) = match n {
                    Dirt::Left(inner) => (-1, 0, *inner),
                    Dirt::Right(inner) => (1, 0, *inner),
                    Dirt::Up(inner) => (0, 1, *inner),
                    Dirt::Down(inner) => (0, -1, *inner),
                    Dirt::None => (0, 0, 0),
                };
                for _j in 0..len {
                    p = (p.0 + x, p.1 + y);
                    step += 1;
                    if idx == 0 {
                        // println!("a:{:?}", &p);
                        m.insert(p, step);
                    } else {
                        // println!("b:{:?}", &p);
                        if let Some(k) = m.get(&p) {
                            if min_size == 0 || k + step < min_size {
                                min_size = k + step
                            }
                        }
                    }
                }
            }
            idx += 1;
        }

        min_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    #[test]
    fn day3_part1() {
        let input_str = r#"
            R75,D30,R83,U83,L12,D49,R71,U7,L72
            U62,R66,U55,R34,D71,R55,D58,R83
            "#;
        let input = common::parse_from_str(input_str);
        println!("input:{:?}", input);
        let ret = day3::part1(input);
        assert_eq!(ret, 159);
        let list = common::parse_from_file("./data/day3_part1.txt").unwrap();
        let ret2 = day3::part1(list);
        assert_eq!(ret2, 709);
    }

    #[test]
    fn day3_part2() {
        let list = common::parse_from_file("./data/day3_part2.txt").unwrap();
        let ret2 = day3::part2(list);
        assert_eq!(ret2, 13836);
    }
}
