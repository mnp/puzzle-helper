use rug::integer::IsPrime;
use rug::Integer; // Float
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

// #![feature(repr128)]

type Int = u64;

struct Settings {
    max_val: u8,
}

struct Power {
    base: Int,
    exponent: Int,
}

fn showvec(xs: &Vec<Int>) {
    print!("  ");
    for e in xs {
        print!("{e} ");
    }
    println!();
}

// https://math.stackexchange.com/questions/298044/given-an-integer-how-can-i-detect-the-nearest-integer-perfect-power-efficiently
// fn next_power(n: Int) {
//     // -> Power
//     let nf = Float::with_val(53, n);
//     let max_k = nf.log2().floor().to_f32() as u32;
//     for k in 2..max_k {
//         let rootk = nf.clone().root(k);
//         let lo = Float::i_pow_u(rootk.clone().floor().to_i32_saturating().unwrap(), k);
//         let hi = Float::i_pow_u(rootk.ceil().to_i32_saturating().unwrap(), k);
//         println!("floor {}", Float::with_val(53, lo));
//         println!("ceil {}", Float::with_val(53, hi));
//     }
//     println!("max_k {}", max_k);
// }

fn next_power(n: Int) {
    let max_k = n.ilog2();
    // for k: Int in 2..max_k {
    //     let rootk = k.sqrt().floor();
    //     let lo = Float::i_pow_u(rootk.floor().to_i32_saturating().unwrap(), k);
    //     let hi = Float::i_pow_u(rootk.ceil().to_i32_saturating().unwrap(), k);
    //     println!("floor {}", Float::with_val(53, lo));
    //     println!("ceil {}", Float::with_val(53, hi));
    // }
    println!("max_k {}", max_k);
}

fn xfoo(n: Int) {
    next_power(n);

    let factors = factor(n); // they're sorted
    let mut powers: Vec<Power> = vec![];
    let mut exp: Int = 0;
    let mut base = factors[0];

    for factor in factors {
        if base != factor {
            powers.push(Power {
                base: base,
                exponent: exp,
            });
            print!("[{}] ", factor);
            base = factor;
            exp = 1;
        } else {
            print!("{} ", factor);
            exp += 1;
        }
    }
    powers.push(Power {
        base: base,
        exponent: exp,
    });

    println!("");
    for power in powers {
        println!("power: {}^{}", power.base, power.exponent);
    }

    let power = Integer::from(n).is_perfect_power();
    let nprime = Integer::from(n).next_prime();
    println!("  perfect power: {}", power);
    let isprime = match Integer::from(n).is_probably_prime(30) {
        IsPrime::Probably => "probably",
        IsPrime::Yes => "yes",
        IsPrime::No => "no",
    };
    println!("  probably prime: {}", isprime);
    println!("  next prime: {}", nprime);
}

fn add(xs: Vec<Int>) {
    let out: Vec<Int> = vec![xs.iter().sum()];
    showvec(&out);
}

fn mul(xs: Vec<Int>) {
    let out: Vec<Int> = vec![xs.iter().product()];
    showvec(&out);
}

fn subtract(mut xs: Vec<Int>) {
    xs.sort();

    let mut z: Int = xs.pop().unwrap();
    for n in xs {
        z = z - n;
    }

    println!("  {}", z);
}

// vec is sorted
fn is_unique(xs: &Vec<Int>) -> bool {
    let mut i: Int = 0;
    for x in xs {
        if i == *x {
            return false;
        }
        i = *x;
    }
    true
}

fn check_showvec(count: Int, max: Int, xs: &Vec<Int>, uniquify: bool) {
    if xs.len() as Int == count && xs.iter().max().unwrap() <= &max && (!uniquify || is_unique(&xs))
    {
        showvec(xs);
    }
}

fn partition(count: Int, max: Int, n: Int, uniquify: bool) {
    // generate all partitions of n but only print ones fitting the spec

    // Knuth 7.2.1.4: Generating All Partitions
    // https://web.archive.org/web/20170330174929/http://cs.utsa.edu/~wagner/knuth/fasc3b.pdf
    let mut v: Vec<Int> = vec![n];
    while v[0] != 1 {
        check_showvec(count, max, &v, uniquify);

        let mut x = v.pop().unwrap();
        while x == 1 {
            x = v.pop().unwrap();
        }
        x -= 1;

        let mut tot: Int = v.iter().sum();
        while tot + x < n {
            v.push(x);
            tot += x;
        }
        if tot < n {
            v.push(n - tot);
        }
    }
    check_showvec(count, max, &v, uniquify);
}

fn trydiv(p: Int, x: &mut Int, out: &mut Vec<Int>) {
    while *x % p == 0 {
        out.push(p);
        *x /= p;
    }
}

// this is no pollard-rho but i'll do for now
fn factor(mut n: Int) -> Vec<Int> {
    let mut out: Vec<Int> = vec![];
    let mut p = 3;
    trydiv(2, &mut n, &mut out);
    while p <= f32::sqrt(n as f32) as Int {
        trydiv(p, &mut n, &mut out);
        p += 2;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

fn check_max(n: u8) -> bool {
    if n > 0 {
        return true;
    }
    println!("Must set max first");
    false
}

fn check_args(xs: &Vec<Int>, n: usize) -> bool {
    if xs.len() == n {
        return true;
    }
    println!("Expected {} args", n);
    false
}

fn help() {
    println!(
        "Help:
   + a b c...          - sum of arguments
   - a b c...          - difference of arguments (left largest)
   * a b c...          - product of arguments
   f n                 - factorize n
   max n               - set max cell value for pu and pd
   pu count n          - partitions of n, unique only 
   pd count n          - partitions of n, duplicates allowed
   x n                 - experiments"
    );
}

fn parse_args(line: String, settings: &mut Settings) {
    let mut words: Vec<&str> = line.trim().split(' ').collect();
    let mut ins: Vec<Int> = vec![];
    let cmd = words.remove(0);

    for w in words.iter() {
        match w.parse::<Int>() {
            Ok(x) => {
                ins.push(x);
            }
            _ => {
                println!("Bad number {}", w);
                return;
            }
        }
    }

    match cmd {
        "m" | "-" => {
            subtract(ins);
        }
        "f" => {
            if check_args(&ins, 1) {
                let outs = factor(ins[0]);
                showvec(&outs);
            }
        }
        "x" => {
            xfoo(ins[0]);
        }
        "pu" => {
            if check_max(settings.max_val) && check_args(&ins, 2) {
                partition(ins[0], settings.max_val as Int, ins[1], true);
            }
        }
        "pd" => {
            if check_max(settings.max_val) && check_args(&ins, 2) {
                partition(ins[0], settings.max_val as Int, ins[1], false);
            }
        }
        "*" => {
            mul(ins);
        }
        "s" | "+" => {
            add(ins);
        }
        "max" => {
            if check_args(&ins, 1) {
                settings.max_val = ins[0] as u8;
            }
        }
        _ => {
            help();
        }
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let mut settings = Settings { max_val: 0 };
    loop {
        let readline = rl.readline(format!("{}>> ", settings.max_val).as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                parse_args(line, &mut settings);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("bye");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use rug::{Complete, Integer};
        assert_eq!(Integer::factorial(10).complete(), 3628800);
    }
}
