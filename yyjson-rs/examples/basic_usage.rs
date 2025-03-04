use yyjson_rs::*;

fn main() -> anyhow::Result<()> {
    let pa = PoolAllocProvider::new(10, None).unwrap();
    let dc = DocContext::new(Box::new(pa), ReadOptions::default());
    let b1 = b"{\"foo\": {\"bar\": null}}";
    let b2 = b"{\"foo\": 30, \"bar\": 42}";
    let b3 = b"[1,2,3,4,5,6,7,8,9]";
    let bufs: &[&[u8]] = &[b1, b2, b3];
    for buf in bufs {
        let d = dc.parse(buf)?;
        println!("doc: {}", d);
    }
    Ok(())
}
