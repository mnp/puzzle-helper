use rug::{Integer};
use rug::integer::IsPrime;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

type Int = u64;

fn showvec(xs: &Vec<Int>) {
    print!("  ");
    for e in xs {
        print!("{e} ");
    }
    println!();
}

fn xfoo(n: Int) {
    let power = Integer::from(n).is_perfect_power();    
    let nprime = Integer::from(n).next_prime();
    println!("  perfect power: {}", power);
    let isprime = match Integer::from(n).is_probably_prime(30) {
        IsPrime::Probably => { "probably" }
        IsPrime::Yes => { "yes" }
        IsPrime::No => { "no" }
    };
    println!("  probably prime: {}", isprime);
    println!("  next prime: {}", nprime);
}

fn add(xs: Vec<Int>) {
    let out: Vec<Int> = vec![xs.iter().sum()];
    showvec(&out);
}

fn subtract(xs: Vec<Int>) {
    let mut zs = xs;
    zs.sort();

    let mut z: Int = zs.pop().unwrap();
    while let Some(n) = zs.pop() {
        z = z - n;
    }

    println!("  {}", z);
}

fn check_showvec(count: Int, max: Int, xs: &Vec<Int>) {
    if xs.len() as Int == count && xs.iter().max().unwrap() <= &max {
        showvec(xs);
    }
}

fn partition(count: Int, max: Int, n: Int) {
    // generate all partitions of n but only print ones fitting the spec

    // Knuth 7.2.1.4: Generating All Partitions
    // https://web.archive.org/web/20170330174929/http://cs.utsa.edu/~wagner/knuth/fasc3b.pdf
    let mut v: Vec<Int> = vec![n];
    while v[0] != 1 {
        check_showvec(count, max, &v);

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
    check_showvec(count, max, &v);
}

fn trydiv(p: Int, x: &mut Int, out: &mut Vec<Int>) {
    while *x % p == 0 {
        out.push(p);
        *x /= p;
    }
}

// this is no pollard-rho but i'll do for now
fn factor(mut n: Int, mut out: &mut Vec<Int>) {
    let mut p = 3;
    trydiv(2, &mut n, &mut out);
    while p <= f32::sqrt(n as f32) as Int {
        trydiv(p, &mut n, &mut out);
        p += 2;
    }
    if n > 1 {
        out.push(n);
    }
}

fn parse(line: String) {
    let mut words: Vec<&str> = line.trim().split(' ').collect();
    let mut ins: Vec<Int> = vec![];
    let cmd = words.remove(0);

    if words.len() < 1 {
        println!("Need at least one number");
        return;
    }

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
        "s" | "+" => {
            add(ins);
        }
        "m" | "-" => {
            subtract(ins);
        }
        "f" => {
	    let mut outs: Vec<Int> = vec![];
            factor(ins[0], &mut outs);
	    showvec(&outs);
        }
        "x" => {
            xfoo(ins[0]);
        }
        "p" => {
            partition(ins[0], ins[1], ins[2]);
        }
        _ => {
            println!(
                "Help:
   + a b c...          - sum of arguments
   f n                 - factorize n
   p count max n       - partitions of n
   x n                 - experiments"
            );
        }
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                parse(line);
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
