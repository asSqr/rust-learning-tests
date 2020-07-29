use std::io;

fn main() {
    let mut n = String::new();
    
    loop {
        io::stdin().read_line(&mut n)
            .expect("Failed to read line.");
        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let mut f: Vec<i32> = vec![-1; n+1];

        println!( "{}", fib( n, &mut f ) );

        break;
    }
}

fn fib( n: usize, f: &mut Vec<i32> ) -> i32 {
    if f[n] != -1 {
        return f[n];
    }
    if n <= 1 {
        f[n] = n as i32;
        return n as i32;
    }

    let prv_f = fib(n-1, f);
    let prv_prv_f = fib(n-2, f);

    f[n] = prv_f+prv_prv_f;
    return f[n];
}