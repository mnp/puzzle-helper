use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

type Int = u32;

fn showvec(xs: &Vec<Int>) {
    print!("  ");
    for e in xs {
        print!("{} ", e);
    }
    println!();
}

fn sum(xs: Vec<Int>, out: &mut Vec<Int>) {
    out.push(0);
    for x in &xs {
        out[0] += x;
    }
}

fn partition(xs: Vec<Int>, out: &mut Vec<Int>) {
    out.push(0);
    print!("niy");
}

fn trydiv(p: Int, x: Int, out: &mut Vec<Int>) -> Int {
    let mut n = x;
    while n % p == 0 {
        out.push(p);
        n = n / p;
    }
    return n;
}

fn factor(mut n: Int, out: &mut Vec<Int>) {
    let mut p = 3;
    n = trydiv(2, n, out);
    while p <= f32::sqrt(n as f32) as Int {
        n = trydiv(p, n, out);
        p += 2;
    }
    if n > 1 {
        out.push(n);
    }
}

fn parse(line: String) {
    let mut words: Vec<&str> = line.trim().split(' ').collect();
    let mut outs: Vec<Int> = vec![];
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
            sum(ins, &mut outs);
            showvec(&outs);
        }
        "f" => {
            factor(ins[0], &mut outs);
            showvec(&outs);
        }
        "p" => {
            partition(ins, &mut outs);
            showvec(&outs);
        }
        _ => {
            println!("Help:
   + n n n     - sum
   f n         - factorize
   p n         - partitions");
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
