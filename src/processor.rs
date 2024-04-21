use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<(), anyhow::Error> {
    let mut reader = Reader::from_path(input)?;
    let mut players = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let player: Player = result?;
        // println!("{:?}", player);
        players.push(player);
    }
    let json = serde_json::to_string_pretty(&players)?;
    std::fs::write(output, json)?;
    Ok(())
}
