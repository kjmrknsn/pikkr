use x86intrin::{m256i, mm256_setr_epi8};

#[inline]
pub fn mm256i(i: i8) -> m256i {
    mm256_setr_epi8(i, i, i, i, i, i, i, i,
                    i, i, i, i, i, i, i, i,
                    i, i, i, i, i, i, i, i,
                    i, i, i, i, i, i, i, i)
}

#[inline]
pub fn u8_to_m256i(s: &[u8], d: &mut Vec<m256i>) {
    let n = s.len();
    let mut i: usize = 0;
    while i + 31 < n {
        d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                               s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                               s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                               s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, s[i+27] as i8, s[i+28] as i8, s[i+29] as i8, s[i+30] as i8, s[i+31] as i8));
        i += 32;
    }
    u8_to_m256i_rest(s, d, i);
}

#[inline]
fn u8_to_m256i_rest(s: &[u8], d: &mut Vec<m256i>, i: usize) {
    match s.len() - i {
        31 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, s[i+27] as i8, s[i+28] as i8, s[i+29] as i8, s[i+30] as i8, 0)); }
        30 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, s[i+27] as i8, s[i+28] as i8, s[i+29] as i8, 0, 0)); }
        29 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, s[i+27] as i8, s[i+28] as i8, 0, 0, 0)); }
        28 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, s[i+27] as i8, 0, 0, 0, 0)); }
        27 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, s[i+26] as i8, 0, 0, 0, 0, 0)); }
        26 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, s[i+25] as i8, 0, 0, 0, 0, 0, 0)); }
        25 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       s[i+24] as i8, 0, 0, 0, 0, 0, 0, 0)); }
        24 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, s[i+23] as i8,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        23 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, s[i+22] as i8, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        22 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, s[i+21] as i8, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        21 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, s[i+20] as i8, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        20 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, s[i+19] as i8, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        19 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, s[i+18] as i8, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        18 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, s[i+17] as i8, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        17 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       s[i+16] as i8, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        16 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, s[i+15] as i8,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        15 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, s[i+14] as i8, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        14 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, s[i+13] as i8, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        13 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, s[i+12] as i8, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        12 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, s[i+11] as i8, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        11 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, s[i+10] as i8, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        10 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                       s[i+8] as i8, s[i+9] as i8, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0)); }
        9 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                      s[i+8] as i8, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        8 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, s[i+7] as i8,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        7 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, s[i+6] as i8, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        6 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, s[i+5] as i8, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        5 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, s[i+4] as i8, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        4 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, s[i+3] as i8, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        3 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, s[i+2] as i8, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        2 => { d.push(mm256_setr_epi8(s[i] as i8, s[i+1] as i8, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        1 => { d.push(mm256_setr_epi8(s[i] as i8, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0,
                                      0, 0, 0, 0, 0, 0, 0, 0)); }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm256i() {
        let test_cases = vec![0, 1, 2, 3];
        for i in test_cases {
            let want = mm256_setr_epi8(i,i,i,i,i,i,i,i,
                                       i,i,i,i,i,i,i,i,
                                       i,i,i,i,i,i,i,i,
                                       i,i,i,i,i,i,i,i);
            let got = mm256i(i);
            assert_eq!(want.as_u8x32().as_array(), got.as_u8x32().as_array());
        }
    }

    #[test]
    fn test_u8_to_m256i() {
        struct TestCase {
            s: Vec<u8>,
            d: Vec<m256i>,
        }
        let test_cases = vec![
            TestCase {
                s: vec![],
                d: vec![],
            },
            TestCase {
                s: vec![1],
                d: vec![mm256_setr_epi8(1, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2],
                d: vec![mm256_setr_epi8(1, 2, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3],
                d: vec![mm256_setr_epi8(1, 2, 3, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 0, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 0, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 0, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 0, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 29],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 29, 0, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 29, 30],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 29, 30, 0, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 29, 30, 31],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 29, 30, 31, 0)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 29, 30, 31, 32],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 29, 30, 31, 32)],
            },
            TestCase {
                s: vec![1, 2, 3, 4, 5, 6, 7, 8,
                        9, 10, 11, 12, 13, 14, 15, 16,
                        17, 18, 19, 20, 21, 22, 23, 24,
                        25, 26, 27, 28, 29, 30, 31, 32,
                        33],
                d: vec![mm256_setr_epi8(1, 2, 3, 4, 5, 6, 7, 8,
                                        9, 10, 11, 12, 13, 14, 15, 16,
                                        17, 18, 19, 20, 21, 22, 23, 24,
                                        25, 26, 27, 28, 29, 30, 31, 32),
                        mm256_setr_epi8(33, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0,
                                        0, 0, 0, 0, 0, 0, 0, 0)],
            },

        ];
        for t in test_cases {
            let mut d = Vec::with_capacity((t.s.len() + 31) / 32);
            u8_to_m256i(t.s.as_slice(), &mut d);
            assert_eq!(t.d.len(), d.len());
            for i in 0..t.d.len() {
                assert_eq!(t.d[i].as_u8x32().as_array(), d[i].as_u8x32().as_array());
            }
        }
    }

    #[test]
    fn test_u8_to_m256i_rest() {
        let test_cases: Vec<Vec<u8>> = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26, 27, 28, 29, 30, 31],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26, 27, 28, 29, 30],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26, 27, 28, 29],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26, 27, 28],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26, 27],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25, 26],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24,
                 25],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23, 24],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22, 23],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21, 22],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20, 21],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19, 20],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18, 19],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17, 18],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16,
                 17],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15, 16],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14, 15],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13, 14],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12, 13],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11, 12],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10, 11],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9, 10],
            vec![1, 2, 3, 4, 5, 6, 7, 8,
                 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3],
            vec![1, 2],
            vec![1],
            vec![],
        ];
        for t in test_cases {
            let mut w = [0i8; 32];
            for i in 0..32 {
                if i < t.len() {
                    w[i] = t[i] as i8;
                }
            }
            let mut want = Vec::new();
            if t.len() > 0 {
                want.push(mm256_setr_epi8(w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7],
                                          w[8], w[9], w[10], w[11], w[12], w[13], w[14], w[15],
                                          w[16], w[17], w[18], w[19], w[20], w[21], w[22], w[23],
                                          w[24], w[25], w[26], w[27], w[28], w[29], w[30], w[31]));
            }
            let mut got = Vec::new();
            u8_to_m256i_rest(t.as_slice(), &mut got, 0);
            assert_eq!(want.len(), got.len());
            for i in 0..want.len() {
                assert_eq!(want[i].as_u8x32().as_array(), got[i].as_u8x32().as_array());
            }
        }
    }
}
