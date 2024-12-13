use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpassword(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<(), anyhow::Error> {
    let mut rng = rand::thread_rng();
    let mut password: Vec<u8> = vec![];
    let mut chars: Vec<u8> = vec![];

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _i in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("chars won't empty");
        password.push(*c);
    }
    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[]);
    eprintln!("{}", estimate.score()); // 3

    Ok(())
}
