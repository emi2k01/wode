mod project;
mod snapshot;

use project::Project;

fn main() {
    let mut project = Project::new("Hello, World");
    let mut rec = project.start_snapshot();

    // setup project
    rec.create_dir("src");
    rec.create_file("src/main.rs", "fn main() {\n    \n}".to_string());

    let cargo_contents = r#"[package]
name = "hello"
version = "0.1.0"
authors = ["Emilio González <emi2k01@gmail.com>"]
edition = "2018"
"#
    .to_string();
    rec.create_file("Cargo.toml", cargo_contents);

    // write println
    let project = rec.end();
    let mut rec = project.start_snapshot();
    rec.modify_file("src/main.rs", "    println!(\"hello, world\");", 2..1);
    let project = rec.end();

    // output player file
    project.output_rec("out/rec.wode").unwrap();
}
