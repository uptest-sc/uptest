use glob::glob;
use regex::Regex;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn file_find(file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    //let file_path = "/tmp/search_crates/pallet-balances-21.0.0/src/migration.rs";

    // Open the file asynchronously
    let file = File::open(&file_name).await?;
    let reader = BufReader::new(file);

    let re = Regex::new(r".*(fn\smigrate.[a-z-0-9-A-Z-_]{0,})(<.*>|)\(").unwrap();

    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        // println!("line is = {}", line);

        let caps = re.find(&line);
        //   println!("The name is: {:?}", &caps);
        //  println!("line is: {line:?}");
        match caps {
            Some(result) => {
                println!("found match: {:?} in file: {file_name:}", result.as_str());
            }
            _ => continue, //println!("no match")
        }
    }

    Ok(())
}

pub async fn test_glob(folder_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let folder_name = folder_name.replace(".gzip", "");
    println!("Glob search with {folder_name:}");
    let findo = format!("{}/*/*.rs", folder_name);
    println!("findo: {findo:}");
    for entry in glob(&findo).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let pathen = path.display();
                //    println!("checking path: {pathen:}");
                let _loots = file_find(pathen.to_string()).await?;
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
