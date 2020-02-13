#![feature(str_strip)]

mod test;
/// Extracts the name of the Lambda function being updated from the code zip's full path and filename.
fn bare_name(path: &str) -> Option<String> {
    // explicit type needed here because CLion can't handle Splits
    let x: Option<&str> = path.split("/").into_iter().last();
    x
        .and_then(|s| s.strip_suffix(".zip"))
        .map(|s| s.to_owned())
}

fn main() {
    println!("Hello, world!");
}
