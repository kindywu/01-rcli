use anyhow::{anyhow, Result};
use rand::prelude::*;

const UPPER_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CHARS: &[u8] = b"abcdefghjkmnpqrstuvwxyz";
const SYMBOL_CHARS: &[u8] = b"*&^%?$#@!";
const NUMBER_CHARS: &[u8] = b"23456789";

// cargo run gen-pass --length 32 --no-number --no-symbol --no-upper-case
pub fn process_gen_pass(
    length: u8,
    no_upper_case: bool,
    no_lower_case: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<String, anyhow::Error> {
    let mut password = Vec::new();
    let mut chars = Vec::new();
    let mut rng = rand::thread_rng();

    if !no_upper_case {
        password.push(
            *UPPER_CHARS
                .choose(&mut rng)
                .ok_or(anyhow!("UPPER_CHARS won't be empty"))?,
        );
        chars.extend_from_slice(UPPER_CHARS);
    }
    if !no_lower_case {
        password.push(
            *LOWER_CHARS
                .choose(&mut rng)
                .ok_or(anyhow!("LOWER_CHARS won't be empty"))?,
        );
        chars.extend_from_slice(LOWER_CHARS);
    }
    if !no_symbol {
        password.push(
            *SYMBOL_CHARS
                .choose(&mut rng)
                .ok_or(anyhow!("SYMBOL_CHARS won't be empty"))?,
        );
        chars.extend_from_slice(SYMBOL_CHARS);
    }
    if !no_number {
        password.push(
            *NUMBER_CHARS
                .choose(&mut rng)
                .ok_or(anyhow!("NUMBER_CHARS won't be empty"))?,
        );
        chars.extend_from_slice(NUMBER_CHARS);
    }

    for _ in 0..(length - password.len() as u8) {
        // let idx = rng.gen_range(0..chars.len());
        // password.push(chars[idx] as char);

        let c = chars
            .choose(&mut rng)
            .ok_or(anyhow!("CHARS won't be empty"))?;
        password.push(*c);
    }

    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
