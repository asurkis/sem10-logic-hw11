macro_rules! source {
    () => {"macro_rules! source {{\n    () => {{{:?}}};\n}}\n\nfn main() {{\n    let args: Vec<String> = std::env::args().collect();\n    let n: usize = args[1].parse().unwrap();\n    let s = format!(source!(), source!());\n    println!(\"{{}}\", &s[..n]);\n}}"};
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let s = format!(source!(), source!());
    println!("{}", &s[..n]);
}
