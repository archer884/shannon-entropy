// #![feature(test)]
use std::collections::BTreeMap;
use std::ascii::AsciiExt;

/// Calculates the shannon entropy of 's'.
/// https://en.wiktionary.org/wiki/Shannon_entropy
///
/// # Arguments
///
/// * `s` - The string slice to calculate entropy for
///
/// # Example
/// let entropy : f32 = shannon_entropy("Hi there!");
pub fn shannon_entropy(s: &str) -> f32 {
    if s.is_empty() {
        return 0.0
    }
    let mut char_map : BTreeMap<char, usize> = BTreeMap::new();

    let mut ascii_map: [usize; 128] = [0;128];

    let mut s_len = 0;

    for ch in s.chars() {
        s_len += 1;
        if ch.is_ascii() {
            ascii_map[ch as usize] += 1;
        } else {
            *char_map.entry(ch).or_insert(0) += 1;
        }
    }

    let s_len = (s_len as f32).round();
    let log_div = (2.0 as f32).ln();

    let result = char_map.values().fold(0.0, |acc, c| {
        if * c == 0 {
            acc
        } else {
            acc + (*c as f32 * (*c as f32 / s_len).ln())
        }
    });

    let result = ascii_map.into_iter().fold(result, |acc, c| {
        if * c == 0 {
            acc
        } else {
            acc + (*c as f32 * (*c as f32 / s_len).ln())
        }
    }).abs();
    let result = result / (s_len * log_div);
    result
}


#[cfg(test)]
mod tests {
    // extern crate test;
    extern crate rand;

    use super::*;
    // use self::test::Bencher;
    use self::rand::{thread_rng, Rng};

    #[test]
    fn test_shannon() {

        let test_strings = vec![
                            // Make sure we're consistent
                            ("horse staple battery", shannon_entropy("horse staple battery")),
                            // All-ASCII strings hit the fast path
                            ("hello world", 2.845351),
                            ("hello worldd", 2.8553884),
                            ("a", 0.0),
                            ("", 0.0),
                            // // Test non-ascii characters for slow path
                            ("i ❤ rust", 2.7499998),
                            ("ßℝ💣", 1.5849625),
                            ("abc", 1.5849625),
                            ("hello world💣", 3.0220554),
                            ];
        for (test, answer) in test_strings {
            let entropy = shannon_entropy(test);
            assert_eq!(entropy, answer);
        }
    }
}

    // #[bench]
    // fn bench_shannon_empty(b: &mut Bencher) {
    //     b.iter(|| shannon_entropy(""));
    // }
    //
    // #[bench]
    // fn bench_shannon_small(b: &mut Bencher) {
    //     let s: String = thread_rng().gen_ascii_chars().take(64).collect();
    //     b.iter(|| shannon_entropy(&s));
    // }
    //
    // #[bench]
    // fn bench_shannon_medium(b: &mut Bencher) {
    //     let s: String = thread_rng().gen_ascii_chars().take(1024).collect();
    //     b.iter(|| shannon_entropy(&s));
    // }
    //
    // #[bench]
    // fn bench_shannon_large(b: &mut Bencher) {
    //     let s: String = thread_rng().gen_ascii_chars().take(65536).collect();
    //     b.iter(|| shannon_entropy(&s));
    // }
