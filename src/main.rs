use std::{fs::{File, self}, io::{Write, Read, Seek}, str::Chars};
use itertools::Itertools;

const w: usize = 32;
const n: usize = 624;
const m: usize = 397; // middle word
const r: usize = 31; // separation point
const a: usize = 0x990FB0DF; // coefficient of rational normal form matrix
const u: usize = 11;
const d: usize = 0xFFFFFFFF;
const l: usize = 18;
const s: usize = 7;
const b: usize = 0x9D2C5680;
const t: usize = 15;
const c: usize = 0xEFC60000;
const f: usize = 1812433253;

struct Twister {
    mt: [usize; n as usize],
    index: usize,
    lower_mask: usize,
    upper_mask: usize,
}

impl Twister {
    fn new(seed: usize) -> Self {
        let lower_mask = (1 << r) - 1;
        let mut twister = Self {
            mt: [0; n as usize],
            index: n - 1,
            lower_mask,
            upper_mask: ((1 << w) - 1) & !lower_mask,
        };
        twister.mt[0] = seed;

        for i in 1..n {
            twister.mt[i as usize] = ((1 << w) - 1)
                & (f * (twister.mt[(i - 1) as usize] ^ (twister.mt[(i - 1) as usize] >> (w - 2)))
                    + i);
        }
        twister
    }

    fn twist(&mut self) {
        for i in 0..n - 1 {
            let x = (self.mt[i as usize] & self.upper_mask)
                + (self.mt[((i + 1) % n) as usize] & self.lower_mask);
            let mut xA = x >> 1;
            if x % 2 != 0 {
                xA = xA ^ a
            }
            self.mt[i as usize] = self.mt[((i + m) % n) as usize] ^ xA
        }
        self.index = 1;
    }

    fn extract_number(&mut self) -> usize {
        if self.index == n - 1 {
            self.twist();
        }
        // int y := MT[index]
        let mut y = self.mt[self.index - 1];
        // y := y xor ((y >> u) and d)
        y = y ^ ((y >> u) & d);
        // y := y xor ((y << s) and b)
        y = y ^ ((y << u) & d);
        // y := y xor ((y << t) and c)
        y = y ^ ((y << t) & c);
        // y := y xor (y >> l)
        y = y ^ (y >> l);
        // index := index + 1
        self.index += 1;
        // return lowest w bits of (y)
        lowest(w, y)
    }
}

const fn lowest(shift: usize, x: usize) -> usize {
    ((1 << shift) - 1) & x
}

fn main() {
    let mut tw = Twister::new(4); // ISO certified random number
    let mut file = File::create("numbers").unwrap();

    for _ in 0..10000 {
        writeln!(file, "{:032b}", tw.extract_number()).unwrap();
    }

    let mut str = String::new();
    let mut file = File::open("numbers").unwrap();
    file.read_to_string(&mut str).unwrap();


    let mut file_post = File::create("numbers_post").unwrap();
    file_post.write_all(to_von_neumann(&str).as_bytes()).unwrap();
}

fn to_von_neumann(str: &str) -> String {
    let nums = str.split("\n").map(|line| von_neumann(line)).join("\n");
    return nums
}

fn von_neumann(line : &str) -> String{
    let mut res =   String::new();

    for chn in &line.chars().chunks(2) {
        let chunk: Vec<_> = chn.collect();
        match chunk.as_slice() {
            ['0', '1'] => res.push_str("0"),
            ['1', '0'] => res.push_str("1"),
            _ => {}
        }      
    }
    
    return res
}

fn hardware_combinatorial_trojan(idx: usize)  -> usize{
    let mut triggered = false;
    let mut mt_test = Twister::new(5489);
    let fake_mersenne = [2, 3, 4, 5, 6, 7];
    while !triggered {
        let new = mt_test.extract_number();
        //if the last digit of the generated number is 2, trigger the trojan
        //this is intentionally a relatively common event such that we can observe the behavior of the trojan
        //because this is a combinatorial trojan we want to monitor for a state rather than a sequence of inputs
        if new % 10 == 2 {
            dbg!("triggered trojan");
            triggered = true;
        }
    }
    //payload
    dbg!("deploying payload");
    return fake_mersenne[idx];
}

fn hardware_sequential_trojan(idx:usize) -> usize {
    let mut triggered = false;
    let mut mt_test = Twister::new(5489);
    let fake_mersenne = [2, 3, 4, 5, 6, 7];

    let mut rands = Vec::new();

    while !triggered {
        let new = mt_test.extract_number();
        rands.push(new);
        
        //if the last digit of the generated number is 2, trigger the trojan
        //this is intentionally a relatively common event such that we can observe the behavior of the trojan
        //this is a sequential trojan so we care about a sequence of inputs
        if rands.len() > 2{
            let mut temp = rands.clone();
            temp.reverse();
            let el1 = temp.get(0).unwrap();
            let el2 = temp.get(1).unwrap();
            if el1 % 2 == 0 && el2 % 2 == 0 {
                dbg!("triggered trojan");
                triggered = true;
            }
        }

    }
    //payload
    dbg!("deploying payload");
    return fake_mersenne[idx];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, 3 + 2)
    }

    #[test]
    fn generate_a_value() {
        let mut mt_test = Twister::new(5489);
        let mut last = Vec::new();
        for _ in 0..(3 * n) {
            let i = mt_test.extract_number();
            assert!(!last.contains(&i));
            last.push(i);
        }
    }

    #[test]
    fn attack(){
        dbg!(hardware_sequential_trojan(3));
    }
}
