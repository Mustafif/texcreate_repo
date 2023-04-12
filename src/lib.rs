#[cfg(feature="release")]
/// Provides a way to generate Release notes 
pub mod release;

use serde::{Serialize, Deserialize};
use toml::{from_str, to_string_pretty};
use std::collections::{HashMap, hash_map::IntoIter};
use std::iter::IntoIterator;
use std::path::PathBuf;
use reqwest::Client;
use texcore::template::Template;
use tokio::io::{Result, AsyncWriteExt};
use tokio::fs::File;

/// A Metadata file used to maintain the TexCreate Template Releases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo{
    /// Current version of the repo
    current_vers: u64,
    /// Number of templates
    num: u64,
    /// Contains a HashMap of the `<Name, Description>`
    info: HashMap<String, String>
}

impl Repo{
    /// Creates a new `Repo`
    pub fn new(current_vers: u64, templates: &Vec<Template>) -> Self{
        let num = templates.len() as u64;
        let mut map = HashMap::new();
        for t in templates{
            let name = t.name.to_string();
            let desc = t.description.to_string();
            map.insert(name, desc);
        }
        Self{current_vers, num, info: map}
    }
    /// Returns the version of the repo
    pub fn version(&self) -> u64{
        self.current_vers
    }
    /// Returns the number of templates
    pub fn num(&self) -> u64{
        self.num
    }
    /// Returns the info
    pub fn info(&self) -> HashMap<String, String>{
        self.info.clone()
    }
    /// Creates a new `Repo` using a TOML String
    pub fn from_string(s: &str) -> Self{
        from_str(s).unwrap()
    }
    /// Turns a `Repo` into a TOML String
    pub fn to_string(&self) -> String{
        to_string_pretty(&self).unwrap()
    }
    /// Given a url it will return a `Repo`
    pub async fn get_repo(url: &str) -> Self{
        let client = Client::new();
        let resp = client.get(url)
            .send()
            .await
            .unwrap();
        let text = resp.text().await.unwrap();
        Self::from_string(&text)
    }
    /// Prints out the Name and Description in the repo
    pub fn display(&self){
        println!("TexCreate Repo: v{}", self.current_vers);
        println!("Number of Templates: {}", self.num);
        println!("======TEMPLATES======");
        for (n, d) in self.clone().into_iter(){
            println!("=> {n}: {d}")
        }
        println!("=====================");
    }
    /// Checks if a name exists in `info`
    pub fn template_exist(&self, name: &str) -> bool{
        self.info.contains_key(name)
    }
}
/// Downloads a `repo.toml` file given a path
pub async fn download_repo(url: &str, out_path: PathBuf) -> Result<()>{
    let client = Client::new();
    let resp = client.get(url)
        .send().await.unwrap();
    let data = resp.bytes().await.unwrap();
    let path = out_path.join("repo.toml");
    let mut file = File::create(&path).await?;
    file.write_all(&data).await?;
    Ok(())
}

impl IntoIterator for Repo{
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.info.into_iter()
    }
}

