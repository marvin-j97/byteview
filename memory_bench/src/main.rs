const COUNT: usize = 50_000_000;
const STR: &'static [u8] = b"helloworldhelloworldhelloworld";

pub fn main() {
    let mut v = Vec::with_capacity(COUNT);

    //let root = byteview::ByteView::new(STR);
    // let root = bytes::Bytes::from(STR.to_vec());

    for _ in 0..COUNT {
        v.push(byteview::ByteView::new(&STR));
        //v.push(root.slice(0..1));
        // v.push(bytes::Bytes::from(STR.to_vec()));
        // v.push(std::sync::Arc::<[u8]>::from(STR.to_vec()));
    }

    println!("done");

    // drop(v);

    // println!("dropped");

    loop {}
}
