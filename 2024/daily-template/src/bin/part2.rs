use {{crate_name}}::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt").trim();
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}