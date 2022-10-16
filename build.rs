// Build mandoc 
use std::fs;

include!("./src/cli.rs");

fn main() -> std::io::Result<()> {

    
    let out_dir = std::env::current_dir()?;

    let cmd = gen_cli();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    fs::create_dir_all("./target/man")?;
    std::fs::write(out_dir.join
        ("./target/man/uptest.man"), 
        buffer)?;

    Ok(())
}
