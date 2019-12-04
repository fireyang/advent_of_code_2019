mod day4 {

    #[allow(dead_code)]
    pub fn part1(start: i32, end: i32) -> i32 {
        let mut sum = 0;
        for _i in start..end {
            // println!("{}", _i);
            if cal(_i) {
                sum += 1;
            }
        }
        sum
    }

    #[allow(dead_code)]
    pub fn part2(start: i32, end: i32) -> i32 {
        let mut sum = 0;
        for _i in start..end {
            // println!("{}", _i);
            if cal2(_i) {
                sum += 1;
            }
        }
        sum
    }

    pub fn cal(mut v: i32) -> bool {
        // let v2 = v;
        let mut max_len = 0;
        let mut adjacent = 0;
        let mut d = v % 10;
        v = v / 10;
        for _i in 0..6 {
            let n = v % 10;
            v = v / 10;
            if n > d {
                return false;
            }
            if n == d {
                adjacent += 1;
            } else {
                if adjacent > 0 {
                    max_len += 1;
                }
                adjacent = 0
            }
            d = n;
        }
        if max_len > 0 {
            // println!("{}", v2);
            return true;
        }
        false
    }

    pub fn cal2(mut v: i32) -> bool {
        // let v2 = v;
        let mut max_len = 0;
        let mut adjacent = 0;
        let mut d = v % 10;
        v = v / 10;
        for _i in 0..6 {
            let n = v % 10;
            v = v / 10;
            if n > d {
                return false;
            }
            if n == d {
                adjacent += 1;
            } else {
                if adjacent == 1 {
                    max_len += 1;
                }
                adjacent = 0
            }
            d = n;
        }
        if max_len > 0 {
            // println!("{}", v2);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_part1() {
        let ret = day4::part1(124075, 580769);
        assert_eq!(ret, 2150);
    }

    #[test]
    fn day4_part2() {
        let ret = day4::part2(124075, 580769);
        assert_eq!(ret, 1462);
    }
}
