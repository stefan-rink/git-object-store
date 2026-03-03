use std::path::Path;

use anyhow::Context;
use git2::Oid;

#[derive(Debug)]
pub struct GitStorage {
    /// The path to the git repository
    repo_path: String,
}

/**
 * Handler which receives and stores json objects in git
 */
impl GitStorage {
    pub fn new(path: &String) -> Self {
        Self {
            repo_path: path.to_string(),
        }
    }

    /**
     * Check if the repo already exists and instatiate if not
     */
    pub fn get_repo_storage(&self) -> Result<git2::Repository, anyhow::Error> {
        match git2::Repository::open(Path::new(&self.repo_path)) {
            Ok(repo) => Ok(repo),
            Err(e) => {
                print!("Storage Repo did not exist, creating a new one {e}");
                self.create_repo()
            }
        }
    }

    /**
     * Create a new git repository
     */
    fn create_repo(&self) -> Result<git2::Repository, anyhow::Error> {
        let repo = git2::Repository::init(Path::new(&self.repo_path))
            .with_context(|| "Creating a new repo failed")?;

        // Set HEAD to point to main branch (creates the branch reference without a commit)
        repo.set_head("refs/heads/main")
            .context("Failed to set HEAD to main")?;

        Ok(repo)
    }

    /**
     * Read a file from the git repository
     */
    fn read_file(&self, file_name: &str, revision: &str) -> Result<String, anyhow::Error> {
        let repo = self.get_repo_storage()?;

        let git_object = repo.revparse_single(revision)?;
        let commit = git_object.peel_to_commit()?;
        let commit_tree = commit.tree()?;
        let file_entry = commit_tree
            .get_name(file_name)
            .context("file not found in git tree")?;
        let blob = repo.find_blob(file_entry.id())?;
        let content = std::str::from_utf8(blob.content())?.to_owned();

        Ok(content)
    }

    /**
     * Commit a file to the git repository
     */
    fn commit_file(&self, file_name: &str, content: &str) -> Result<Oid, anyhow::Error> {
        let repo = self.get_repo_storage()?;

        let blob_oid = repo.blob(content.as_bytes())?;

        let mut treebuilder = repo.treebuilder(None)?;
        treebuilder.insert(file_name, blob_oid, git2::FileMode::Blob.into())?;
        let tree_oid = treebuilder.write()?;
        let tree = repo.find_tree(tree_oid)?;

        let parent = repo.head()?.peel_to_commit()?;
        let parents = &[&parent];

        let signature = repo.signature()?;

        let message = file_name;
        let commit_oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            parents,
        )?;

        Ok(commit_oid)
    }
}
