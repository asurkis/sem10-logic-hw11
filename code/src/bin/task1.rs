macro_rules! source {
    () => {"macro_rules! source {{\n    () => {{{:?}}};\n}}\n\nfn main() {{\n    let s = format!(source!(), source!());\n    let h = md5::compute(s);\n    println!(\"{{:x}}  task1.rs\", h);\n}}\n"};
}

fn main() {
    let s = format!(source!(), source!());
    let h = md5::compute(s);
    println!("{:x}  task1.rs", h);
}
