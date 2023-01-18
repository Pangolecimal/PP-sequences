#![allow(unused_imports, dead_code, non_snake_case, non_camel_case_types)]

fn get_input_range(max: i128, msg: &str) -> i128 {
    print!("{}", msg);
    let n_result: Result<i128, _> = get_input_line().parse();

    let mut n = match n_result {
        Ok(m) => m,
        Err(_) => 0,
    };

    if n < 1 || n >= max {
        n = 0;
    }

    if n == 0 {
        println!("Error: expected integer in range: [1, {}]", max);
        println!();
        return get_input_range(max, msg);
    }

    return n;
}

fn get_input_line() -> String {
    let mut input_line = String::new();
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let line = input_line.trim();
    return line.to_string();
}

pub mod miller_rabin {
    pub fn primality_test() -> Option<(Vec<i128>, String)> {
        let num: i128 = crate::get_input_range(i128::MAX, "Input a number to test: ");

        let prime = is_prime(num);
        println!(
            "Your number is {}",
            if prime { "prime" } else { "composite" }
        );

        None
    }

    pub fn is_prime(n: i128) -> bool {
        if n < 2 || n % 2 == 0 {
            return false;
        }

        let (s, d) = prepare(n - 1, 0); // n = (2 ^ s) * d + 1
        let a_vec = get_a(n); // get the values for "a"

        for a in a_vec {
            let rem = power_mod(a, d, s, n); // calculate  a ^ (2^r * d) mod n, for 0 <= r < s
            if rem != 1 && rem != n - 1 {
                return false; // +1 or -1 mod n
            }
        }
        true
    }

    fn prepare(n: i128, p: i128) -> (i128, i128) {
        if n % 2 == 1 {
            return (p, n);
        }
        prepare(n / 2, p + 1)
    }

    fn get_a(n: i128) -> Vec<i128> {
        if n < 2_047 {
            return vec![2];
        }
        if n < 1_373_653 {
            return vec![2, 3];
        }
        if n < 9_080_191 {
            return vec![31, 73];
        }
        if n < 25_326_001 {
            return vec![2, 3, 5];
        }
        if n < 3_215_031_751 {
            return vec![2, 3, 5, 7];
        }
        if n < 4_759_123_141 {
            return vec![2, 7, 61];
        }
        if n < 1_122_004_669_633 {
            return vec![2, 13, 23, 1_662_803];
        }
        if n < 2_152_302_898_747 {
            return vec![2, 3, 5, 7, 11];
        }
        if n < 3_474_749_660_383 {
            return vec![2, 3, 5, 7, 11, 13];
        }
        if n < 341_550_071_728_321 {
            return vec![2, 3, 5, 7, 11, 13, 17];
        }
        if n < 3_825_123_056_546_413_051 {
            return vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
        }
        if n < 318_665_857_834_031_151_167_461 {
            return vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        }
        if n < 3_317_044_064_679_887_385_961_981 {
            return vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
        }

        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ]
    }

    fn power_mod(a: i128, d: i128, s: i128, n: i128) -> i128 {
        let mut ans: i128 = 1;
        for i in 0..s {
            ans = fme(a, (2 as i128).pow(i as u32) * d, n);
            if ans == 1 || ans == n - 1 {
                return ans;
            }
        }
        ans
    }
    // Fast modular exponentiation
    fn fme(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
        let mut result: i128 = 1;
        base = base % modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp = exp >> 1;
            base = base * base % modulus
        }
        result
    }
}

pub mod inventory {
    pub fn A342585() -> Option<(Vec<i128>, String)> {
        let num: i128 = crate::get_input_range(
            i128::MAX as i128,
            "Input number of rows of terms to generate: ",
        ) as i128;

        let seq = get_inv(num)
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<i128>>();

        Some((seq, "A342585 Sequence: ".into()))
    }

    fn get_inv(n: i128) -> Vec<Vec<i128>> {
        let mut ans: Vec<Vec<i128>> = vec![Vec::new(); n as usize];
        ans[0] = vec![0];
        for i in 1..n {
            let mut j = 0;
            let mut c = count(&ans, j);
            while c != 0 {
                ans[i as usize].push(c);
                j += 1;
                c = count(&ans, j);
            }
            ans[i as usize].push(0);
        }

        ans
    }

    fn count(arr: &Vec<Vec<i128>>, n: i128) -> i128 {
        let mut c: i128 = 0; // count of n in arr
        let flattened = arr.clone().into_iter().flatten().collect::<Vec<i128>>();
        for i in flattened {
            if n == i {
                c += 1;
            }
        }
        c
    }
}

pub mod stern_brocot {
    pub fn A002487() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let mut seq = vec![0; num as usize];
        for i in 0..num {
            seq[i as usize] = term(i);
        }

        Some((seq, "A002487 Sequence: ".into()))
    }

    fn term(n: i128) -> i128 {
        if n < 2 {
            return n;
        }
        if n % 2 == 0 {
            return term(n / 2);
        }
        if n % 2 == 1 {
            return term((n - 1) / 2) + term((n + 1) / 2);
        }

        0
    }
}

pub mod kolakoski {
    pub fn A000002() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A000002 Sequence: ".into()))
    }

    fn gen(n: i128) -> Vec<i128> {
        let mut ans = vec![1, 2, 2];

        for i in 2..n {
            let term = ans[ans.len() - 1];
            for _ in 0..ans[i as usize] {
                ans.push(3 - term);
            }

            if ans.len() >= n as usize {
                return ans;
            }
        }

        ans
    }
}

pub mod dammit {
    pub fn A133058() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A133058 Sequence: ".into()))
    }

    fn gen(n: i128) -> Vec<i128> {
        let mut ans = vec![1, 1];

        for i in 2..n {
            let g = gcd(i, ans[(i - 1) as usize]);
            if g == 1 {
                ans.push(ans[(i - 1) as usize] + i + 1);
            } else {
                ans.push(ans[(i - 1) as usize] / g);
            }
        }

        ans
    }

    fn gcd(a: i128, b: i128) -> i128 {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

pub mod prime_bin_rev {
    use crate::miller_rabin::is_prime;

    pub fn A265326() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A265326 Sequence: ".into()))
    }

    fn gen(n: i128) -> Vec<i128> {
        let mut ans = vec![1];
        let mut p: i128 = 2;

        for _ in 0..n {
            while !is_prime(p as i128) {
                p += 1;
            }
            ans.push(p as i128 - bin_reverse(p) as i128);
            p += 1;
        }

        ans
    }

    fn bin_reverse(p: i128) -> i128 {
        let mut ans: i128 = 0;
        let mut n: i128 = p;

        while n > 0 {
            ans <<= 1;
            if n & 1 == 1 {
                ans ^= 1;
            }
            n >>= 1;
        }

        ans
    }
}

pub mod remy_sigrist {
    pub fn A279125() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A279125 Sequence: ".into()))
    }

    fn gen(n: i128) -> Vec<i128> {
        let m = n;
        let mut ans = vec![];
        let mut g = vec![0; m as usize];

        for i in 1..=n {
            let mut a = 0;
            while g[a] & i != 0 {
                a += 1;
            }
            g[a] += i;
            ans.push(a as i128);
        }

        ans
    }
}

pub mod wisteria {
    pub fn A063543() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A063543 Sequence: ".into()))
    }

    fn gen(n: i128) -> Vec<i128> {
        let mut ans = vec![];

        for i in 1..=n {
            ans.push(term(i));
        }

        ans
    }

    fn term(n: i128) -> i128 {
        let mut product = 1;
        let mut m = n;
        while m > 0 {
            if m % 10 != 0 {
                product *= m % 10;
            }
            m = m / 10;
        }

        (n - product) as i128
    }
}

pub mod forest_fire {
    pub fn A229037() -> Option<(Vec<i128>, String)> {
        let num: i128 =
            crate::get_input_range(i128::MAX as i128, "Input number of terms to generate: ")
                as i128;

        let seq = gen(num);

        Some((seq, "A229037 Sequence: ".into()))
    }

    fn gen(m: i128) -> Vec<i128> {
        let mut ans = vec![];

        for n in 0..m {
            let (mut i, mut j, mut b) = (1, 1, vec![0]);
            while (n as i128 - 2 * i as i128) >= 0 {
                let v = 2 * ans[(n - i) as usize] - ans[(n - 2 * i) as usize];
                if !b.contains(&v) {
                    b.push(v);
                }
                i += 1;
                while b.contains(&j) {
                    let index = b.iter().position(|x| *x == j).unwrap();
                    b.remove(index);
                    j += 1;
                }
            }
            ans.push(j);
        }

        ans
    }
}
