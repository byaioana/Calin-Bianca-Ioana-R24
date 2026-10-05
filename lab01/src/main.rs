fn prime(n: u32) -> bool {
    let mut d = 2;
    if n < 2 {
        return false;
    }
    while d <= n / 2 {
        if n % d == 0 {
            return false;
        }
        d = d + 1;
    }
    return true;
}
fn coprime(mut a: u32, mut b: u32) -> bool {
    while b != 0 {
        let r: u32 = a % b;
        a = b;
        b = r;
    }
    if a == 1 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    //ex 1
    let mut x: u32 = 0;
    while x <= 100 {
        if prime(x) {
            println!("{}", x);
        }
        x = x + 1;
    }

    //ex 2
    let mut i = 0;
    while i <= 100 {
        let mut j = i + 1;
        while j <= 100 {
            if coprime(i, j) {
                println!("{} {}", i, j);
            }
            j = j + 1;
        }
        i = i + 1;
    }

    //ex 3
    let mut n = 99;
    while n >= 0 {
        if n > 1 {
            println!("{} bottles of beer on the wall,", n);
            println!("{} bottles of beer.", n);
            println!("Take one down, pass it around,");
        }
        n = n - 1;
        if n == 1 {
            println!("{} bottle of beer on the wall.", n);
            println!(" ");
            println!("{} bottle of beer on the wall.", n);
            println!("{} bottle of beer.", n);
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
            println!(" ");
        } else if n == 0 {
            println!("No bottle of beer on the wall.");
            println!("No bottle of beer.");
            println!("Take one down, pass it around,");
            println!("99 bottles of beer on the wall.");
        } else if n < 99 && n > 0 {
            println!("{} bottles of beer on the wall.", n);
            println!(" ");
        }
    }
}
