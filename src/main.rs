
use std::{error::Error};

async fn foo(stuff: &str) -> Result<SomeStruct, Box<dyn Error>> {

}

#[tokio::main]
async fn main() {
    let (w, y) = tokio::join!(
        foo(u),
        foo(v));

    return;

    let x = bar(x, &[11u8; 20]);
}
