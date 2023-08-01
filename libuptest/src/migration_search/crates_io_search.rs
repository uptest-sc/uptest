use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()

/// crates.io crates.crate.links field
#[derive(Deserialize, Serialize, Debug)]
pub struct Lankar {
    pub owner_team: String,
    pub owner_user: String,
    pub owners: String,
    pub reverse_dependencies: String,
    pub version_downloads: String,
    pub versions: String,
}

/// crates.io crates.crate json field
#[derive(Deserialize, Serialize, Debug)]
pub struct Crate {
    pub badges: Vec<String>,
    pub categories: Option<Vec<String>>,
    pub created_at: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub downloads: u32,
    pub exact_match: bool,
    pub homepage: Option<String>,
    pub id: String,
    pub keywords: Option<String>,
    pub links: Lankar,
    pub max_stable_version: String,
    pub max_version: String,
    pub name: String,
    #[serde(rename = "newest_version")]
    pub version: String,
    pub recent_downloads: u32,
    #[serde(rename = "repository")]
    pub repository_url: Option<String>,
    pub updated_at: Option<String>,
    pub versions: Option<String>,
}

/// crates.io meta key field
#[derive(Deserialize, Serialize, Debug)]
pub struct Metastruct {
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub total: u32,
}

/// Wrapper used by crates.io API.
/// crates.io search returns a json blob with crates and meta field
#[derive(Deserialize, Serialize, Debug)]
pub struct Crates {
    #[serde(rename = "crates")]
    pub crates: Vec<Crate>,
    pub meta: Metastruct,
}

/// download tarball/gzip of crate from crates.io, unstable version atm, does not handle errors properly
/// return file name if all is good
pub async fn download_crate_from_crates_io(
    crate_version: String,
    crate_name: String,
) -> Result<String, reqwest::Error> {
    let file_name = format!("{crate_name:}-{crate_version:}.gzip");
    //https://crates.io/api/v1/crates/serde/1.0.0/download
    let url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        crate_name, crate_version
    );
    let client = reqwest::Client::new();
    println!("Downloading crate: {crate_name:?} version: {crate_version:?}");
    let tanka = client
        .get(url)
        .header(USER_AGENT, "Uptest cli client")
        .send()
        .await?;
    println!("crate downloaded ok");
    let mut filen = File::create(file_name.clone()).await.unwrap();
    filen
        .write_all(&tanka.bytes().await.unwrap())
        .await
        .unwrap();
    println!("crate saved as {file_name:}");

    Ok(file_name)
}

/// search crates.io
pub async fn search_crates_io(query: &str) -> Result<Crates, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://crates.io/api/v1/crates?page=1&per_page=100&q={}",
        query
    ); // https://crates.io/api/v1/crates?page=1&per_page=100&q=pallet-balan
    let resp = client
        .get(url)
        .header(USER_AGENT, "Uptest cli client")
        .send()
        .await?
        .json::<Crates>()
        .await;
    resp
}
