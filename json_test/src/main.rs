use std::result::Result;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    id: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let jstr : &str = r#"{"id":123, "value":{"param":456}}"#;
    let j: Sample = serde_json::from_str(jstr).unwrap();
    dbg!(j);

    let jv : serde_json::Value = serde_json::from_str(jstr).unwrap();
    dbg!(&jv);
    let jpara = jv.get("value")
        .and_then(|v| v.get("param"));
    println!("{}", jpara.unwrap());

    Ok(())
}
