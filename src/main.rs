use clap::Parser;
use rcli::{CmdExector, Opts};
use tracing::Level;

// cargo run -- csv -i assets/juventus.csv
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}

// cargo run -- csv -i assets/juventus.csv
// select * from read_csv('assets/juventus.csv', auto_detect=true);
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // tracing_subscriber::fmt::init();
//     tracing_subscriber::fmt().with_max_level(Level::INFO).init();

//     let opts = Opts::parse();
//     // println!("{:?}", opts);
//     match opts.cmd {
//         SubCommand::Csv(opts) => {
//             let json = process_csv(&opts.input, &opts.format)?;
//             let output = if let Some(output) = opts.output {
//                 output
//             } else {
//                 format!("output.{}", &opts.format)
//             };
//             // println!("{}", output);
//             fs::write(output, json)?;
//         }
//         SubCommand::GenPass(opts) => {
//             let password = process_gen_pass(
//                 opts.length,
//                 opts.no_upper_case,
//                 opts.no_lower_case,
//                 opts.no_number,
//                 opts.no_symbol,
//             )?;
//             println!("{}", password);
//             let estimate = zxcvbn(&password, &[]).unwrap();
//             eprintln!("password strength score is {}", estimate.score());
//         }
//         SubCommand::Base64(subcmd) => match subcmd {
//             Base64SubCommand::Encode(opts) => {
//                 eprintln!("encode {:?}", opts);
//                 let data = read_content(&opts.input)?;
//                 let encode = process_base64_encode(data.as_str(), opts.format)?;
//                 println!("{}", encode);
//             }
//             Base64SubCommand::Decode(opts) => {
//                 eprintln!("decode {:?}", opts);
//                 let data = read_content(&opts.input)?;
//                 let decode = process_base64_decode(&data, opts.format)?;
//                 println!("{}", decode)
//             }
//         },
//         SubCommand::Text(subcmd) => match subcmd {
//             TextSubCommand::Encrypt(opts) => {
//                 eprintln!("encrypt {:?}", opts);
//                 let plain_text = read_content(&opts.input)?;
//                 let encrypt_result = process_text_encrypt(plain_text.trim())?;
//                 println!("{}", encrypt_result.ciphertext_base64);

//                 eprintln!("encrypt text is {}", encrypt_result.ciphertext_base64);
//                 eprintln!(
//                     "Make sure to save the values of the key and nonce for decryption later!"
//                 );
//                 eprintln!(
//                     "key is {} nonce is {}",
//                     encrypt_result.key_base64, encrypt_result.nonce_base64
//                 )
//             }
//             TextSubCommand::Decrypt(opts) => {
//                 eprintln!("decrypt {:?}", opts);
//                 let cipher_text = read_content(&opts.input)?;
//                 let plain_text =
//                     process_text_decrypt(cipher_text.trim(), &opts.key_base64, &opts.nonce_base64)?;
//                 println!("plain_text is {}", plain_text)
//             }
//             TextSubCommand::Sign(opts) => {
//                 eprintln!("sign {:?}", opts);
//                 let data = read_content(&opts.input)?;
//                 let key = read_content(&opts.key)?;

//                 // println!("plain_text is {}", plain_text);
//                 let signed = process_text_sign(data.as_str(), key.as_str(), opts.format)?;
//                 println!("{}", signed);
//             }
//             TextSubCommand::Verify(opts) => {
//                 eprintln!("verify {:?}", opts);
//                 let data = read_content(&opts.input)?;
//                 let key = read_content(&opts.key)?;
//                 let signed = read_content(&opts.signed)?;
//                 eprintln!("verify {:?}", opts);
//                 let verify =
//                     process_text_verify(data.as_str(), key.as_str(), signed.as_str(), opts.format)?;
//                 println!("{}", verify);
//             }
//             TextSubCommand::GenerateKey(opts) => {
//                 // eprintln!("generate key {:?}", opts);
//                 let keys = process_text_generate_key(opts.format)?;
//                 match opts.format {
//                     TextSignFormat::Blake3 => {
//                         let content = keys
//                             .first()
//                             .ok_or(anyhow::anyhow!("key is empty"))?
//                             .as_bytes();
//                         println!("{:?}", content);
//                         std::fs::write(opts.path.join("blake.txt"), content)?
//                     }
//                     TextSignFormat::Ed25519 => {
//                         let content = keys
//                             .first()
//                             .ok_or(anyhow::anyhow!("secret key is empty"))?
//                             .as_bytes();
//                         println!("{:?}", content);

//                         std::fs::write(opts.path.join("ed25519.sk"), content)?;
//                         let content = keys
//                             .get(1)
//                             .ok_or(anyhow::anyhow!("public key is empty"))?
//                             .as_bytes();
//                         println!("{:?}", content);

//                         std::fs::write(opts.path.join("ed25519.pk"), content)?
//                     }
//                 }
//             }
//         },
//         SubCommand::Jwt(subcmd) => match subcmd {
//             JwtSubCommand::Sign(opts) => {
//                 eprintln!("sign {:?}", opts);
//                 let data = read_content(&opts.input)?;
//                 // println!("plain_text is {}", data);
//                 let signed =
//                     process_jwt_sign(opts.algorithm, opts.key, opts.aud, opts.sub, opts.exp, data)?;
//                 println!("{}", signed);
//             }
//             JwtSubCommand::Verify(opts) => {
//                 eprintln!("verify {:?}", opts);
//                 let signed = read_content(&opts.input)?;
//                 let claims = process_jwt_verify(opts.algorithm, opts.key, signed);
//                 println!("{:?}", claims);
//             }
//         },
//         SubCommand::Http(subcmd) => match subcmd {
//             HttpSubCommand::Serve(opts) => {
//                 // eprintln!("serve {:?}", opts);
//                 process_http_serve(opts.path, opts.port).await?
//             }
//         },
//     }
//     Ok(())
// }
