//use std::fmt::format;
use libuptest::migration_search::crates_io_search::{
    download_crate_from_crates_io, search_crates_io, Crates,
};
use libuptest::migration_search::decompress::tar_xvf;
use libuptest::migration_search::file_search::test_glob;

pub struct MigFind {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting search, looking for pallet-balances");

    let resp: Crates = search_crates_io("pallet-balan").await?;
    for results in resp.crates.iter() {
        println!("Found crate: {:?}", results.name);
        //   println!("Found repo: {:?}", results.repository_url);
        println!("Crates version is: {}", results.version);
        let _url = results.repository_url.clone().unwrap();
        println!("docs are at: {:?}", results.documentation);
        // download crate to parent working dir
        let filen =
            download_crate_from_crates_io(results.version.clone(), results.name.clone()).await.unwrap();
        println!("unziping file");
        let _unzip = tar_xvf(filen.clone());
        println!("file downloaded");
        let _test_o = test_glob(filen).await?;
        println!("All done")
    }
    //   let files_found = file_test().await?;

    //  println!("{:#?}", resp);
    Ok(())
}
