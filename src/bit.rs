#[inline]
pub fn r(x: u64) -> u64 {
    x & x.wrapping_sub(1)
}

#[inline]
pub fn e(x: u64) -> u64 {
    x & x.wrapping_neg()
}

#[inline]
pub fn s(x: u64) -> u64 {
    x ^ x.saturating_sub(1)
}

#[inline]
pub fn and(s: &Vec<u64>, d: &mut Vec<u64>) {
    for i in 0..s.len() {
        d[i] &= s[i];
    }
}

#[inline]
pub fn shift_right_by_one(v: &mut Vec<u64>) {
    let mut pre = 0u64;
    for i in (0..v.len()).rev() {
        let t = v[i];
        v[i] = (v[i] >> 1) | (pre << 63);
        pre = t;
    }
}

#[inline]
pub fn shift_left_by_one(v: &mut Vec<u64>) {
    let mut pre = 0u64;
    for i in 0..v.len() {
        let t = v[i];
        v[i] = (v[i] << 1) | (pre >> 63);
        pre = t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r() {
        struct TestCase {
            x: u64,
            want: u64,
        }

        let test_cases = vec![
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010u64,
            },
            TestCase {
                x:    0b10000000_00100000_00000000_00000000_00000000_00000000_00000100_00000000u64,
                want: 0b10000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
             TestCase {
                x:    0b00000000_00010000_00110010_00000000_00000000_00001100_00000000_00000000u64,
                want: 0b00000000_00010000_00110010_00000000_00000000_00001000_00000000_00000000u64,
            },
             TestCase {
                x:    0b11100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
             TestCase {
                x:    0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64,
                want: 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111110u64,
            },
        ];

        for test_case in test_cases {
            assert_eq!(test_case.want, r(test_case.x));
        }
    }

    #[test]
    fn test_e() {
        struct TestCase {
            x: u64,
            want: u64,
        }

        let test_cases = vec![
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
            TestCase {
                x:    0b10000000_00100000_00000000_00000000_00000000_00000000_00000100_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000100_00000000u64,
            },
            TestCase {
                x:    0b00000000_00010000_00110010_00000000_00000000_00001100_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000u64,
            },
            TestCase {
                x:    0b11100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
        ];

        for test_case in test_cases {
            assert_eq!(test_case.want, e(test_case.x));
        }
    }

    #[test]
    fn test_s() {
        struct TestCase {
            x: u64,
            want: u64,
        }

        let test_cases = vec![
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000001_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000001_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000001_11111111_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000000_00000001_11111111_11111111_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000000_00000001_11111111_11111111_11111111_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00000001_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
            TestCase {
                x:    0b10000000_00100000_00000000_00000000_00000000_00000000_00000100_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000111_11111111u64,
            },
            TestCase {
                x:    0b00000000_00010000_00110010_00000000_00000000_00001100_00000000_00000000u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000111_11111111_11111111u64,
            },
            TestCase {
                x:    0b11100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64,
                want: 0b00111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64,
            },
            TestCase {
                x:    0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111u64,
                want: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001u64,
            },
        ];

        for test_case in test_cases {
            println!("{:b} {:b} {:b}", test_case.x, test_case.x.wrapping_sub(1), s(test_case.x));
            assert_eq!(test_case.want, s(test_case.x));
        }
    }

    #[test]
    fn test_and() {
        struct TestCase {
            s: Vec<u64>,
            d: Vec<u64>,
            w: Vec<u64>,
        }
        let test_cases = vec![
            TestCase {
                s: vec![],
                d: vec![],
                w: vec![],
            },
            TestCase {
                s: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                d: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                s: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                d: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                s: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                d: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                s: vec![0b01001000_01110000_01111100_00100000_00111000_01001100_00110011_10000101],
                d: vec![0b01001010_00101000_01001010_00100010_01001100_11010100_00110100_00100100],
                w: vec![0b01001000_00100000_01001000_00100000_00001000_01000100_00110000_00000100],
            },
            TestCase {
                s: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                d: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
            },
            TestCase {
                s: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                d: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                s: vec![0b10100000_01110010_00100000_11100010_01110000_01000100_00100100_01111100,
                        0b00011100_10000001_01001100_00011000_00000110_00100110_00100000_00000111],
                d: vec![0b01110010_01000100_00100000_00001000_00101010_00101100_00100101_00000000,
                        0b00000000_00000000_00010010_00110000_00010100_00000010_01000000_01111100],
                w: vec![0b00100000_01000000_00100000_00000000_00100000_00000100_00100100_00000000,
                        0b00000000_00000000_00000000_00010000_00000100_00000010_00000000_00000100],
            },
            TestCase {
                s: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                d: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
            },
        ];
        for t in test_cases {
            let mut d = t.d.clone();
            and(&t.s, &mut d);
            assert_eq!(t.w, d);
        }
    }

    #[test]
    fn test_shift_right_by_one() {
        struct TestCase {
            v: Vec<u64>,
            w: Vec<u64>
        }
        let test_cases = vec![
            TestCase {
                v: vec![],
                w: vec![],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
            },
            TestCase {
                v: vec![0b00000000_00000000_01000000_10000000_00000000_00010000_00000100_00010000],
                w: vec![0b00000000_00000000_00100000_01000000_00000000_00001000_00000010_00001000],
            },
            TestCase {
                v: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b01111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
                w: vec![0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011,
                        0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011,
                        0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011],
                w: vec![0b11100000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b11100000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b01100000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
            },
            TestCase {
                v: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b01111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
            },
        ];
        for t in test_cases {
            let mut v = t.v.clone();
            shift_right_by_one(&mut v);
            assert_eq!(t.w, v);
        }
    }

    #[test]
    fn test_shift_left_by_one() {
        struct TestCase {
            v: Vec<u64>,
            w: Vec<u64>
        }
        let test_cases = vec![
            TestCase {
                v: vec![],
                w: vec![],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_01000000_10000000_00000000_00010000_00000100_00010000],
                w: vec![0b00000000_00000000_10000001_00000000_00000000_00100000_00001000_00100000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010],
            },
            TestCase {
                v: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111110],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000],
                w: vec![0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000,
                        0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000],
            },
            TestCase {
                v: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001],
                w: vec![0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011,
                        0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011],
            },
            TestCase {
                v: vec![0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011,
                        0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011,
                        0b11000000_00000000_00000000_00000000_00000000_00000000_00000000_00000011],
                w: vec![0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000110,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000111,
                        0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000111],
            },
            TestCase {
                v: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
                w: vec![0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111110,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111,
                        0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111],
            },
        ];
        for t in test_cases {
            let mut v = t.v.clone();
            shift_left_by_one(&mut v);
            assert_eq!(t.w, v);
        }
    }
}
