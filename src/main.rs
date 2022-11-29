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

fn sum(xs: Vec<Int>) {
    let out: Vec<Int> = vec![xs.iter().sum()];
    showvec(&out);
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

fn trydiv(p: Int, n: &mut Int, out: &mut Vec<Int>) {
    while n % p == 0 {
        out.push(p);
        n /= p;
    }
}

fn factor(mut n: Int) {
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
    showvec(&out);
}

fn parse(line: String) {
    let mut words: Vec<&str> = line.trim().split(' ').collect();
    let mut ins: Vec<Int> = vec![];
    let cmd = words[0];
    words.remove(0);

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
            sum(ins);
        }
        "f" => {
            factor(ins[0]);
        }
        "p" => {
            partition(ins[0], ins[1], ins[2]);
        }
        _ => {
            println!(
                "Help:
   + a b c...          - sum of arguments
   f n                 - factorize n
   p count max n       - partitions of n"
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
        let v: Vec<u32> = vec![4, 1, 1];
        let i = vec.len() - 1;
        while i > 0 && v[i] == 1 {
            i -= 1;
        }

        //let v: Vec<_> = xs.into_iter().filter_map(|x| x > 1).collect();

        assert_eq!(v.len(), 1);
    }
}
