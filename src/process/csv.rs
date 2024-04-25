use std::collections::HashMap;

use anyhow::anyhow;
use csv::Reader;

use crate::cli::OutputFormat;

// #[allow(dead_code)]
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: String,
// }

pub fn process_csv(input: &str, format: &OutputFormat) -> Result<String, anyhow::Error> {
    let mut reader = Reader::from_path(input)?;
    let mut records: Vec<HashMap<String, String>> = Vec::with_capacity(128);
    let header = reader.headers()?.clone();

    // print!("{:?}", format);
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);

        // let mut map = std::collections::HashMap::new();
        // for i in 0..header.len() {
        //     map.insert(header[i].to_string(), serde_json::json!(record[i]));
        // }
        // let json_value = serde_json::json!(map);
        // records.push(json_value);

        let value = header
            .iter()
            .zip(record.iter())
            .map(|(h, r)| (h.to_owned(), r.to_owned())) // 创建键值对，拥有字符串
            .collect::<HashMap<String, String>>();
        records.push(value);
    }
    let content = match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&records)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&records)?),
        _ => Err(anyhow!("Unsupported format")),
    }?;

    Ok(content)
}
