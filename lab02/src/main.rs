fn add_chars_n(mut s: String, c: char, mut n: u8) -> String {
    while n > 0 {
        s.push(c);
        n -= 1;
    }
    return s;
}
fn add_chars_m(s: &mut String, c: char, mut n: u8) {
    while n > 0 {
        s.push(c);
        n -= 1;
    }
}
fn add_space(s: &mut String, mut n: u8) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}
fn add_str(s: &mut String, c: &str) {
    s.push_str(c);
}
fn add_integer(s: &mut String, mut n: i64) {
    if n == 0 {
        s.push('0');
    }
    let mut cn = n;
    let mut cfn = 0;
    let mut p = 1;
    while cn > 0 {
        cfn += 1;
        cn /= 10;
        if cn > 0 {
            p *= 10;
        }
    }
    while p > 0 {
        let cf = (n / p) as u8;
        s.push((b'0' + cf) as char);
        n = n % p;
        p /= 10;
        cfn -= 1;
        if cfn > 0 && cfn % 3 == 0 {
            s.push('_');
        }
    }
}
fn add_float(s: &mut String, n: f64) {
    let pint = n as i64;
    add_integer(s, pint);
    s.push('.');
    let mut f = n - (pint as f64) + 0.0001;
    let mut ok = 0;
    while ok < 3 {
        f *= 10.0;
        let cf = f as u8;
        s.push((b'0' + cf) as char);
        f -= cf as f64;
        ok += 1;
    }
}
fn main() {
    let mut s = String::from("");
    let mut s2 = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_m(&mut s2, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
    print!("{}", s2);

    let mut a = String::new();
    a.push('\n');
    add_space(&mut a, 40);
    add_str(&mut a, "I 💚\n");
    add_space(&mut a, 40);
    add_str(&mut a, "RUST.\n\n");
    add_space(&mut a, 4);
    add_str(&mut a, "Most");
    add_space(&mut a, 12);
    add_str(&mut a, "crate");
    add_space(&mut a, 6);
    add_integer(&mut a, 306437968);
    add_space(&mut a, 11);
    add_str(&mut a, "and");
    add_space(&mut a, 5);
    add_str(&mut a, "lastest");
    add_space(&mut a, 9);
    add_str(&mut a, "is\n");
    add_space(&mut a, 9);
    add_str(&mut a, "downloaded");
    add_space(&mut a, 8);
    add_str(&mut a, "has");
    add_space(&mut a, 13);
    add_str(&mut a, "downloads");
    add_space(&mut a, 5);
    add_str(&mut a, "the");
    add_space(&mut a, 9);
    add_str(&mut a, "version");
    add_space(&mut a, 4);
    add_float(&mut a, 2.038);
    add_str(&mut a, ".\n");
    print!("{}", a);
}
