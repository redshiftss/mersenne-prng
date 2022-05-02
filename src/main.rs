use std::{fs::File, io::Write};

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
        file.write_all(&tw.extract_number().to_le_bytes()).unwrap();
    }
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
}
