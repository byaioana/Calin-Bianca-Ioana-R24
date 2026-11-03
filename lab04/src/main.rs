use std::{io, fs};
#[derive(PartialEq, Debug)]
enum Rot13error {
    Notascii(char),
}

fn p1(fisier: &str) -> Result<(), io::Error> {
    let f = fs::read_to_string(fisier)?;

    let mut linmaxb = "";
    let mut maxb = 0;

    let mut linmaxc = "";
    let mut maxc = 0;

    for lin in f.lines() {
        let bytes = lin.len();
        if bytes > maxb {
            maxb = bytes;
            linmaxb = lin;
        }

        let ch = lin.chars().count();
        if ch > maxc {
            maxc = ch;
            linmaxc = lin;
        }
    }

    println!("{linmaxb}");
    println!("{linmaxc}");

    Ok(())
}
fn p2(text: &str) -> Result<String, Rot13error> {
    let mut s = String::new();
    for c in text.chars() {
        if (c as u32) > 127 {
            return Err(Rot13error::Notascii(c));
        }
        let mut car = c;
        if (c>='a' && c<='m') || (c>='A' && c<='M') {
            car=((c as u8) +13) as char;
        }
        else if (c>='n' && c<='z') || (c>='N' && c<='Z') {
            car=((c as u8) -13) as char;
        }
        s.push(car);
    }

    Ok(s)
}
fn abrevieri(text: &str) -> String {
    let lista = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ];

    let mut rez = String::new();
    let mut cuv = String::new();

    for c in text.chars() {
        if c == ' ' {
            let mut ok = false;
            for pereche in lista {
                if cuv == pereche.0 {
                    rez.push_str(pereche.1);
                    ok = true;
                    break;
                }
            }

            if !ok {
                rez.push_str(&cuv);
            }
            rez.push(' ');
            cuv.clear(); 
        } 
        else {
            cuv.push(c);
        }
    }
    if cuv.len()!=0 {
        let mut ok = false;
        for pereche in lista {
            if cuv == pereche.0 {
                rez.push_str(pereche.1);
                ok = true;
                break;
            }
        }

        if !ok {
            rez.push_str(&cuv);
        }
    }

    rez
}
fn p3(intrare: &str, iesire: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(intrare)?;
    let textmodif = abrevieri(&text);
    fs::write(iesire, &textmodif)?;
    println!("{textmodif}");
    Ok(())
}
fn p4(hosts: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(hosts)?;
    for lin in text.lines() {
        let mut ip = String::new();
        let mut hostname = String::new();
        let mut col = 0;
        let mut spatiuant = true;
        let mut ok = false;

        for c in lin.chars() {
            if c == ' ' || c == '\t' {
                spatiuant = true;
                if col == 2 {
                    break;
                }
            } 
            else {
                if spatiuant {
                    col += 1;
                    spatiuant = false;
                }
                if col == 1 {
                    ip.push(c);
                } 
                else if col == 2 {
                    hostname.push(c);
                }
            }
        }

        if ip.len() == 0 {
            continue;
        }

        for c in ip.chars() {
            if c == '#' {
                ok = true;
            }
            break;
        }

        if ok {
            continue;
        }
        if hostname.len() > 0 {
            println!("{hostname} => {ip}");
        }
    }

    Ok(())
}
fn main() {
    let text = "strings are fun
🎁🎶🎉👀🎈🎃🍕☕🍉
rust
supercalifragilisticexpialidocious";
    if let Err(e) = fs::write("inputp1.txt", text) {
        println!("{e}");
    }
    match p1("inputp1.txt") {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }
    match p1("fisierinexistent.txt") {
        Ok(()) => println!("Fisierul trebuia sa lipseasca"),
        Err(e) => println!("{e}"),
    }

    match p2("Hello World! abc XYZ") {
        Ok(text2) => println!("{text2}"),
        Err(e) => println!("{e:?}"),
    }
    match p2("litera ă") {
        Ok(text2) => println!("Eroare {text2}"),
        Err(Rot13error::Notascii(c)) => {
            println!("caracterul '{c}' nu e ASCII");
        }
    }

    let text3 = "Am fost la dl Matei pt ca m-a invitat cu o zi inainte";
    if let Err(e) = fs::write("inputp3.txt", text3) {
        println!(" {e}");
    }
    match p3("inputp3.txt", "outputp3.txt") {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }
    match p3("lipsap3.txt", "out.txt") {
        Ok(()) => println!("Fisierul trebuia sa lipseasca"),
        Err(e) => println!("{e}"),
    }

    let hosts1 = "# Comentariu\n127.0.0.1 localhost\n::1 ip6-localhost\n   192.168.1.1   router\n\n# Alt comentariu";
    if let Err(e) = fs::write("hosts1.txt", hosts1) {
        println!("{e}");
    }
    match p4("hosts1.txt") {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    }
    match p4("hostslipsa.txt") {
        Ok(()) => println!("Fisierul trebuia sa lipseasca"),
        Err(e) => println!("{e}"),
    }
}