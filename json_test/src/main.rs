use std::result::Result;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    id: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let jstr : &str = r#"{"id":123, "value":{"param":456}}"#;

    // let j: Sample = serde_json::from_str(jstr).unwrap();
    // dbg!(j);

    let jv : serde_json::Value = serde_json::from_str(jstr).unwrap();
    // dbg!(&jv);
    let jpara = jv.get("value")
        .and_then(|v| v.get("param"));
    println!("{}", jpara.unwrap());

    let _ = main2();

    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let f = File::open("./src/sample.json");
    let f = match f {
        Ok(o) => o,
        Err(e) => {
            println!("File open error");
            return Err(Box::new(e));
        }
    };
    let reader: BufReader<File> = BufReader::new(f);

    // let json: Result<serde_json::Value, serde_json::Error> = serde_json::from_reader::<BufReader<File>, serde_json::Value>(reader);
    // dbg!(&json);

    // let json = serde_json::from_reader::<BufReader<File>, serde_json::Value>(reader)?;
    let json = serde_json::from_reader::<BufReader<File>, serde_json::Value>(reader)?;
    // dbg!(&json);

    // let adata = json.get("adata");
    let adata = &json["adata"];
    dbg!(&adata);
    dbg!(&adata.as_array().unwrap().len());
    dbg!(&adata[0]);
    dbg!(&adata[1]);
    dbg!(&adata[2]);

    // if let Some(adata_ary) = adata {
    //     dbg!(adata_ary);

    //     for i in adata_ary {
    //         dbg!(i.get("name"));
    //     }
    // }

    // for i in json?.get("adata")?.iter() {
    //     dbg!(&i);
    // }
    Ok(())
}
