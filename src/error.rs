use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Git Error: {:?}", 0)]
    GitError(git2::Error),
    #[error("Notify Error: {:?}", 0)]
    NotifyError(notify::Error),
    #[error("io error: {:?}", 0)]
    IoError(std::io::Error),
}

impl From<notify::Error> for RepoError {
    fn from(err: notify::Error) -> Self {
        Self::NotifyError(err)
    }
}

impl From<git2::Error> for RepoError {
    fn from(err: git2::Error) -> Self {
        Self::GitError(err)
    }
}

impl From<std::io::Error> for RepoError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
