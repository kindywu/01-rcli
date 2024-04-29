use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    sub: String,
    exp: u64,
    input: String,
}

pub fn process_jwt_sign(
    algorithm: Algorithm,
    key: String,
    aud: String,
    sub: String,
    exp: u64,
    input: String,
) -> anyhow::Result<String> {
    let now = chrono::Utc::now();
    let exp = (now.timestamp() + exp as i64) as u64;
    let claims = Claims {
        aud,
        sub,
        exp,
        input,
    };
    let header = Header::new(algorithm);
    let signed = encode(&header, &claims, &EncodingKey::from_secret(key.as_bytes()))?;
    Ok(signed)
}

pub fn process_jwt_verify(
    algorithm: Algorithm,
    key: String,
    signed: String,
) -> anyhow::Result<Claims> {
    let mut validation = Validation::new(algorithm);
    validation.validate_aud = false;
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        &signed,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}

// cargo run jwt sign -i fixtures\b64_plain.txt --aud all -s kindy --exp 15s
// cargo run jwt verify -i fixtures\jwt_signed_HS256.txt

#[cfg(test)]
mod tests {
    use jsonwebtoken::Algorithm;

    use crate::{process_jwt_sign, process_jwt_verify};

    #[test]
    fn test_process_jwt_sign_verify_hs512() -> anyhow::Result<()> {
        let algorithm = Algorithm::HS512;
        let key = String::from("abc");
        let aud = String::from("abc");
        let sub = String::from("abc");
        let exp = 3;
        let input = String::from("hello world");
        let signed = process_jwt_sign(algorithm, key.clone(), aud, sub, exp, input.clone())?;
        let claims = process_jwt_verify(algorithm, key, signed)?;
        assert_eq!(claims.input, input);
        Ok(())
    }
}
