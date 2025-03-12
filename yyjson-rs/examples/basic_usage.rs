use yyjson_rs::DocContext;

fn main() -> anyhow::Result<()> {
    let json = r#"
    {
        "name": "Alice",
        "age": 30,
        "scores": [95.5, 89.2, 92.8],
        "metadata": {
            "active": true,
            "tags": ["rust", "c", "json"]
        }
    }"#;

    let ctx = DocContext::default();
    let doc = ctx.parse(json.as_bytes())?;
    let root = doc.root();

    // Access primitive values
    let name: Option<&str> = root.at_key("name").and_then(|v| v.str());
    let age: Option<u64> = root.at_key("age").and_then(|v| v.u64());

    // Navigate nested structures
    let scores = root.at_key("scores").and_then(|v| v.list()).unwrap();
    let first_score = scores.get(0).and_then(|v| v.f64()).unwrap();

    let active: Option<bool> = root
        .at_key("metadata")
        .and_then(|m| m.at_key("active"))
        .unwrap()
        .bool();

    // Array iteration
    let scores = root.at_key("scores").and_then(|v| v.list()).unwrap();
    for score in scores.iter() {
        println!("Score: {}", score.f64().unwrap());
    }

    // Object iteration
    let metadata = root.at_key("metadata").and_then(|v| v.obj()).unwrap();
    for (key, value) in metadata.iter() {
        println!("{}: {}", key, value);
    }
    Ok(())
}
