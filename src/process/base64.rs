use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;

use crate::cli::Base64Format;

use super::read_content;

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let data = read_content(input)?;

    match format {
        Base64Format::Standard => Ok(BASE64_STANDARD.encode(data)),
        Base64Format::UrlSafe => Ok(URL_SAFE.encode(data)),
    }
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let data = read_content(input)?;

    match format {
        Base64Format::Standard => Ok(String::from_utf8(BASE64_STANDARD.decode(data)?)?),
        Base64Format::UrlSafe => Ok(String::from_utf8(URL_SAFE.decode(data)?)?),
    }
}

// window: make sure your powershell's $PSVersionTable.PSVersion > 7
// cargo run base64 encode --input fixtures/b64_plain.txt | Out-File -FilePath "fixtures/b64.txt" -Encoding UTF8 -NoNewline
// cargo run base64 decode --input fixtures/b64.txt
#[cfg(test)]
mod tests {
    use crate::cli::Base64Format;
    use crate::{process_base64_decode, process_base64_encode};

    const PLAIN_FILE: &str = "fixtures/b64_plain.txt";
    const B64_FILE: &str = "fixtures/b64.txt";

    #[test]
    fn test_process_base64_encode() -> anyhow::Result<()> {
        let b64 = std::fs::read_to_string(B64_FILE)?;
        assert_eq!(
            process_base64_encode(PLAIN_FILE, Base64Format::Standard)?,
            b64.trim()
        );
        Ok(())
    }
    #[test]
    fn test_process_base64_decode() -> anyhow::Result<()> {
        let plain = std::fs::read_to_string(PLAIN_FILE)?;
        assert_eq!(
            process_base64_decode(B64_FILE, Base64Format::Standard)?,
            plain.trim()
        );
        Ok(())
    }
}
