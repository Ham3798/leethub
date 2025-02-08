impl Solution {
    fn score(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!()
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().rev().collect();
        let mut total = 0;
        let mut prev = 0;
        for i in chars {
            let cur = Self::score(i);
            if prev <= cur {
                total += cur;
                prev = cur;
            }
            else {
                total -= cur
            }
        }
        total
    }
}