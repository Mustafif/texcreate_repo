use std::path::PathBuf;
use crate::Repo;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, write};
use tokio::io::Result;
use toml::from_str;

/// A type to generate Release notes
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release{
    /// Minimum TexCreate version 
    min_texc: String, 
    /// TexCore version used 
    texc_vers: String, 
    /// A description of the release 
    description: String, 
    /// Path to the repo
    repo: PathBuf, 
}

impl Release{
    pub async fn new(path: PathBuf) -> Result<Self>{
        let s = read_to_string(path).await?;
        Ok(from_str(&s).unwrap())
    }
    pub async fn get_repo(&self) -> Result<Repo>{
        let s = read_to_string(&self.repo).await?;
        Ok(from_str(&s).unwrap())
    }
    fn build_specification(&self, release: u64) -> String{
        let spec = format!("# Specifications of Release {release}  ");
        let min_texc = format!("- Minimum TexCreate Version: v{}", &self.min_texc);
        let texcore_v = format!("- TexCore Version: v{}", &self.texc_vers);
        vec![spec, min_texc, texcore_v].join("\n")
    }
    fn build_description(&self) -> String{
        let header = "# Description:  ";
        vec![header, &self.description].join("\n")
    }
    fn build_repo_info(&self, repo: &Repo) -> String{
        let num = repo.num;
        let templates = &repo.info;

        let header = "# Repo Information  ".to_string();
        let num_temp = format!("Number of Templates: {num}");
        let mut vec = vec![header, num_temp, "Templates: ".to_string()];
        for (name, desc) in templates{
            let s = format!("- {name}: {desc}");
            vec.push(s)
        }
        vec.join("\n")
    }
    pub async fn build_release(&self, path: PathBuf) -> Result<()>{
        let repo = self.get_repo().await?;
        let vers = repo.current_vers;
        let file_name = format!("v{vers}-release_note.md");
        let path = path.join(&file_name);
        let spec = self.build_specification(vers);
        let desc = self.build_description();
        let repo_info = self.build_repo_info(&repo);

        let content = vec![spec, desc, repo_info].join("\n");
        write(path, content).await?;
        Ok(())
    }
}