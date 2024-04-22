use csv::Reader;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
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

pub fn process_csv(input: &str) -> Result<String, anyhow::Error> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);

        // let mut map = std::collections::HashMap::new();
        // for i in 0..header.len() {
        //     map.insert(header[i].to_string(), serde_json::json!(record[i]));
        // }
        // let json_value = serde_json::json!(map);
        // records.push(json_value);

        let json_value = header
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        records.push(json_value);
    }
    let json = serde_json::to_string_pretty(&records)?;
    Ok(json)
}
