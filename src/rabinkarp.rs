const MODS: i64 = 1e9 as i64 + 7;
const BASE: i64 = 37;
pub struct RabinKarpHash {
    hash: Vec<i64>,
    power: Vec<i64>,
}

impl RabinKarpHash {
    pub fn new(s: String) -> Self {
        let n = s.len();
        let chars = s.chars().collect::<Vec<char>>();
        let mut hash = vec![0; n];
        let mut power = vec![0; n];
        hash[0] = char_to_int(chars[0]);
        power[0] = 1;
        for i in 1..n {
            hash[i] = add(mul(hash[i - 1], BASE), char_to_int(chars[i]));
            power[i] = mul(power[i - 1], BASE)
        }
        RabinKarpHash { hash, power }
    }
    pub fn get_sub_hash(&self, l: usize, r: usize) -> i64 {
        let mut h = self.hash[r];
        if l > 0 {
            h = sub(h, mul(self.hash[l - 1], self.power[r - l + 1]))
        }
        h
    }
}
pub fn search_for_pattern(text: String, pattern: String) -> Vec<i32> {
    let n = text.len();
    let m = pattern.len();
    let mut result = vec![];
    let text_hash = RabinKarpHash::new(text);
    let pattern_hash = RabinKarpHash::new(pattern);
    let pattern_sub_hash = pattern_hash.get_sub_hash(0, m - 1);
    for i in 0..(n - m) {
        let sub_hash = text_hash.get_sub_hash(i, i + m - 1);
        if sub_hash == pattern_sub_hash {
            result.push(i as i32);
        }
    }
    result
}
fn add(a: i64, b: i64) -> i64 {
    let c = a + b;
    if c >= MODS {
        return c - MODS;
    }
    c
}
fn sub(a: i64, b: i64) -> i64 {
    let c = a - b;
    if c < 0 {
        return c + MODS;
    }
    c
}
fn mul(a: i64, b: i64) -> i64 {
    a * b % MODS
}

fn char_to_int(c: char) -> i64 {
    c as i64 - 'a' as i64 + 1
}
