macro_rules! source {
    () => {"macro_rules! source {{\n    () => {{{:?}}}\n}}\n\nfn main() {{\n    println!(source!(), source!());\n}}"}
}

fn main() {
    println!(source!(), source!());
}
