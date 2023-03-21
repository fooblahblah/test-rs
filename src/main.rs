use bson::{doc, Document};
use cacache;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Cache {
    docs: Vec<Document>,
}

#[tokio::main]
async fn main() -> Result<(), cacache::Error> {
    let dir = String::from("./my-cache");

    let docs = (0..100000)
        .into_iter()
        .map(|i| {
            doc! {
                "index": i
            }
        })
        .collect::<Vec<_>>();
    // println!("{:?}", docs);

    // Write some data!
    let start = Instant::now();
    let data = docs
        .iter()
        .map(|d| serde_json::to_string(d).unwrap())
        .collect::<Vec<_>>();
    let data = postcard::to_allocvec(&data).unwrap();
    cacache::write(&dir, "key", data).await.unwrap();
    println!("Write time {}ms", start.elapsed().as_millis());

    // Get the data back!
    let start = Instant::now();
    let data = cacache::read(&dir, "key").await.unwrap();
    let decoded: Vec<String> = postcard::from_bytes(&data).unwrap();
    let docs2 = decoded
        .iter()
        .map(|s| serde_json::from_str(s.as_str()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(docs, docs2);
    println!("Read time {}ms", start.elapsed().as_millis());

    // Clean up the data!
    //cacache::rm::all(&dir).await?;

    Ok(())
}
