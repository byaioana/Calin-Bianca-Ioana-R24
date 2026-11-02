#[derive(PartialEq)]
enum Eroare {
    Adun,
    Inm,
}
enum Eroarechar {
    Notascii,
    Notdigit,
    Notbase16,
    Notletter,
    Notprint,
}
fn next_prime(mut x: u16) -> Option<u16> {
    let max = u16::MAX;
    while x < max {
        let mut ok = 0;
        x += 1;
        if x < 2 {
            ok = 1;
        }
        let mut d = 2;
        while d * d < x as u32 {
            if (x as u32) % d == 0 {
                ok = 1;
            }
            d += 1;
        }
        if ok == 0 {
            return Some(x);
        }
    }
    None
}
fn adunare(a: u32, b: u32) -> u32 {
    let max = u32::MAX;
    if a + b > max {
        panic!("a+b nu incape in u32");
    }
    return a + b;
}
fn inmultire(a: u32, b: u32) -> u32 {
    let max = u32::MAX;
    if a * b > max {
        panic!("a*b nu incape in u32");
    }
    return a * b;
}
fn erradunare(a: u32, b: u32) -> Result<u32, Eroare> {
    let max = u32::MAX;
    if a + b > max {
        Err(Eroare::Adun)
    } else {
        Ok(a + b)
    }
}
fn errinmultire(a: u32, b: u32) -> Result<u32, Eroare> {
    let max = u32::MAX;
    if a * b > max {
        Err(Eroare::Inm)
    } else {
        Ok(a * b)
    }
}
fn calcul(a: u32, b: u32, c: u32) -> Result<u32, Eroare> {
    let produs = errinmultire(a, b)?;
    let suma = erradunare(produs, c)?;
    Ok(suma)
}
fn to_uppercase(s: char) -> Result<char, Eroarechar> {
    if s >= 'a' && s <= 'z' {
        Ok(((s as u8) + 32) as char)
    } else if s >= 'A' && s <= 'Z' {
        Ok(s)
    } else {
        Err(Eroarechar::Notletter)
    }
}
fn to_lowercase(s: char) -> Result<char, Eroarechar> {
    if s >= 'A' && s <= 'Z' {
        Ok(((s as u8) + 32) as char)
    } else if s >= 'a' && s <= 'z' {
        Ok(s)
    } else {
        Err(Eroarechar::Notletter)
    }
}
fn print_char(s: char) -> Result<(), Eroarechar> {
    if (s as u32) < 32 || (s as u32) > 126 {
        Err(Eroarechar::Notprint)
    } else {
        println!("{s}");
        Ok(())
    }
}
fn char_to_number(s: char) -> Result<u8, Eroarechar> {
    if (s as u32) > 127 {
        return Err(Eroarechar::Notascii);
    }
    if s >= '0' && s <= '9' {
        Ok((s as u8) - b'0')
    } else {
        Err(Eroarechar::Notdigit)
    }
}
fn char_to_number_hex(s: char) -> Result<u8, Eroarechar> {
    if (s as u32) > 127 {
        return Err(Eroarechar::Notascii);
    }
    if s >= '0' && s <= '9' {
        Ok((s as u8) - b'0')
    } else if s >= 'a' && s <= 'f' {
        Ok((s as u8) - b'a' + 10)
    } else if s >= 'A' && s <= 'F' {
        Ok((s as u8) - b'A' + 10)
    } else {
        Err(Eroarechar::Notbase16)
    }
}
fn print_error(err: Eroarechar) {
    match err {
        Eroarechar::Notascii => println!("Caracterul nu este ASCII"),
        Eroarechar::Notdigit => println!("Caracterul nu este o cifra"),
        Eroarechar::Notbase16 => println!("Caracterul nu este o cifra in baza 16"),
        Eroarechar::Notletter => println!("Caracterul nu este o litera"),
        Eroarechar::Notprint => println!("Caracterul nu este printabil"),
    }
}
fn main() {
    let mut n = 65432;
    let mut ok = true;
    while ok {
        let nr = next_prime(n);
        match nr {
            Some(prim) => {
                println!("{prim}");
                n = prim;
            }
            None => {
                ok = false;
            }
        }
    }

    let max = u32::MAX;
    let suma = adunare(10, 20);
    println!("{suma}");
    let produs = inmultire(10, 20);
    println!("{produs}");
    //adunare(max, 2);
    //inmultire(max, 2);

    match calcul(2, 6, 8) {
        Ok(rez) => println!("{rez}"),
        Err(Eroare::Adun) => println!("Eroare adunare"),
        Err(Eroare::Inm) => println!("Eroare inmultire"),
    }
    match calcul(max, 6, 8) {
        Ok(rez) => println!("{rez}"),
        Err(Eroare::Adun) => println!("Eroare adunare"),
        Err(Eroare::Inm) => println!("Eroare inmultire"),
    }

    match to_uppercase('b') {
        Ok(res) => println!("{res}"),
        Err(e) => print_error(e),
    }
    match to_uppercase('5') {
        Ok(res) => println!("{res}"),
        Err(e) => print_error(e),
    }

    match to_lowercase('D') {
        Ok(res) => println!("{res}"),
        Err(e) => print_error(e),
    }
    match to_lowercase('5') {
        Ok(res) => println!("{res}"),
        Err(e) => print_error(e),
    }

    let _ = print_char('Z');
    match print_char('\n') {
        Ok(()) => (),
        Err(e) => print_error(e),
    }

    match char_to_number('8') {
        Ok(val) => println!("{val}"),
        Err(e) => print_error(e),
    }
    match char_to_number('e') {
        Ok(val) => println!("{val}"),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('f') {
        Ok(val) => println!("{val}"),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('z') {
        Ok(val) => println!("{val}"),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('ș') {
        Ok(val) => println!("{val}"),
        Err(e) => print_error(e),
    }
}
