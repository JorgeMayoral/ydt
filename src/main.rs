mod commands;
mod project;

fn main() {
    let project = commands::init();

    println!("{project:#?}")
}
