use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

pub fn parse_os_release(path: &str) -> Result<HashMap<String, String>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    parse_os_release_from_reader(reader)
}

pub fn parse_os_release_from_reader<R: BufRead>(reader: R) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for line_result in reader.lines() {
        let raw = line_result?;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let v = v.trim().trim_matches('"').trim_matches('\'');
            map.insert(k.trim().to_string(), v.to_string());
        }
    }
    Ok(map)
}
