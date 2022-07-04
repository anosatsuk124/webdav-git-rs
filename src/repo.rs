use crate::error::RepoError;
use git2::Repository;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{ffi::OsStr, path::Path, sync::mpsc::channel, time::Duration};

struct UserName(String);

impl UserName {
    fn new(username: &str) -> Self {
        Self(username.to_string())
    }
}

pub struct Repo {
    repo: Repository,
    user: UserName,
}

impl Repo {
    pub fn new(dir: &str, username: &str) -> Result<Self, RepoError> {
        let dir = Path::new(dir);
        let repo = Repository::open(dir)?;
        Ok(Repo {
            repo: if repo.is_bare() {
                Err(RepoError::GitError(git2::Error::new(
                    git2::ErrorCode::BareRepo,
                    git2::ErrorClass::Repository,
                    "This operation is not permitted in a bare repository.",
                )))?
            } else {
                repo
            },
            user: UserName::new(username),
        })
    }
    pub fn init(dir: &str, username: &str) -> Result<Self, RepoError> {
        let dir = Path::new(dir);
        if dir.join(".git").exists() {
            Err(RepoError::GitError(git2::Error::new(
                git2::ErrorCode::Exists,
                git2::ErrorClass::Repository,
                "There is an existing repository.",
            )))?
        } else {
            if !dir.exists() {
                std::fs::create_dir(dir)?;
            }
            Ok(Repo {
                repo: git2::Repository::init(dir)?,
                user: UserName::new(username),
            })
        }
    }
    pub fn watch(&self) -> Result<(), RepoError> {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

        match self.repo.workdir() {
            Some(dir) => Ok(watcher.watch(dir, RecursiveMode::Recursive)?),
            None => Err(RepoError::GitError(git2::Error::new(
                git2::ErrorCode::NotFound,
                git2::ErrorClass::Repository,
                "not found the working directory in your repository.",
            ))),
        }
    }
    pub fn user(&self) -> &UserName {
        &self.user
    }
}
