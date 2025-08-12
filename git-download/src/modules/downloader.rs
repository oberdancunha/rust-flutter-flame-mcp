use crate::modules::logger::Logger;
use crate::structs::{copy_request::CopyRequest, repo::Repo};
use anyhow::Result;
use cmd_lib::run_cmd;
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
    string::String,
};
use tempfile::tempdir;

#[derive(Debug, Clone)]
pub struct Downloader {
    repo_path: String,
    branch_name: String,
    current_dir: PathBuf,
    temp_dir: PathBuf,
}

impl Downloader {
    pub fn new(repo: &Repo) -> Self {
        let current_dir = current_dir().unwrap();
        let temp_dir = tempdir().unwrap();
        let temp_dir = temp_dir.keep();

        Self {
            repo_path: repo.path.to_owned(),
            branch_name: "main".to_owned(),
            current_dir,
            temp_dir,
        }
    }

    pub fn add_git(self) -> Result<Self> {
        Logger::log(format!("Add {} repository", self.repo_path).as_str());
        set_current_dir(&self.temp_dir)?;
        let repo_path = &self.repo_path;
        run_cmd! {
            git init .;
            git config core.sparsecheckout true;
            git remote add origin $repo_path;
        }?;

        Ok(self)
    }

    pub fn get_files_from_repository(self, copy_request: &Vec<CopyRequest>) -> Result<Self> {
        Logger::log("Get files from repository");
        for req in copy_request {
            let from = &req.from;
            run_cmd! {
                echo $from >> .git/info/sparse-checkout;
            }?;
        }
        let branch_name = &self.branch_name;
        run_cmd! {
            git pull origin $branch_name;
        }?;

        Ok(self)
    }

    pub fn copy_files_to_local(self, copy_request: &Vec<CopyRequest>) -> Result<Self> {
        Logger::log("Copy files to local");
        for req in copy_request {
            let from = &req.from;
            let to = &req.to;
            let to = &self.current_dir.join(to);
            let to_dir = to.parent().unwrap();
            if !to_dir.exists() {
                create_dir_all(to_dir)?
            }
            run_cmd! {
                mv $from $to;
            }?;
        }

        Ok(self)
    }

    pub fn remove_git(self) -> Result<()> {
        Logger::log(format!("Remove {} repository", self.repo_path).as_str());
        set_current_dir(&self.current_dir)?;
        let git_dir = &self.temp_dir;
        run_cmd! {
            rm -rf $git_dir;
        }?;

        Ok(())
    }
}
