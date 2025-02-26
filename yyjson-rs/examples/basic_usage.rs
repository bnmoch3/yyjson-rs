use yyjson_rs::*;

fn main() -> anyhow::Result<()> {
    let json = "[1,2,3,4]";
    let buf = json.as_bytes();
    let doc = Doc::read_from(buf, None)?;
    println!("doc: {}", doc);
    Ok(())
}
