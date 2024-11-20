use git2::{Repository, Error};
use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use tokio;
use tracing::{info, Level};

#[derive(Serialize)]
struct GitHubRepo {
    name: String,
    private: bool,  // You can adjust this as needed (e.g., make the repository private)
    auto_init: bool,
    gitignore_template: Option<String>,
}

pub async fn create_repository(
    org: &str,
    project_name: &str,
    framework: &str,
    github_token: &str,
) -> Result<String, String> {
    let temp_dir = format!("/tmp/{}_{}", project_name, Uuid::new_v4());
    let repo_path = Path::new(&temp_dir);

    // Step 1: Initialize a local Git repository
    let repo = match Repository::init(repo_path) {
        Ok(repo) => repo,
        Err(e) => return Err(format!("Git repo initialization failed: {}", e)),
    };

    info!("Initialized an empty Git repository at {}", repo_path.display());

    // Step 2: Add a framework-specific README file
    let readme_path = repo_path.join("README.md");
    if let Err(e) = fs::write(
        &readme_path,
        format!("# {}\n\nFramework: {}", project_name, framework),
    ) {
        return Err(format!("Failed to write README: {}", e));
    }
    info!("Created README.md for project: {}", project_name);

    // Step 3: Add files to repository and make the initial commit
    if let Err(e) = add_all_and_commit(&repo, "Initial commit") {
        return Err(format!("Failed to commit files: {}", e));
    }
    info!("Committed files to the repository");

    // Step 4: Create a GitHub repository using the GitHub API
    let client = Client::new();
    let url = format!("https://api.github.com/orgs/{}/repos", org);
    let repo_data = GitHubRepo {
        name: project_name.to_string(),
        private: false,  // Change this if you want private repositories
        auto_init: true,  // Automatically initialize the repository with a README
        gitignore_template: None, // Optional: specify a gitignore template if needed
    };

    let res = create_github_repo(&client, &url, &repo_data, github_token).await;

    match res {
        Ok(repo_url) => {
            // Step 5: Add the GitHub remote and push to GitHub
            if let Err(e) = push_to_github(&repo, &repo_url) {
                return Err(format!("Failed to push to GitHub: {}", e));
            }
            Ok(repo_url)
        }
        Err(e) => Err(format!("GitHub repository creation failed: {}", e)),
    }
}

// Helper function to commit all files in the repository
fn add_all_and_commit(repo: &Repository, commit_message: &str) -> Result<(), Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_oid = index.write_tree()?;
    let tree =  repo.find_tree(tree_oid)?;

    let sig = repo.signature()?;
    let commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        commit_message,
        &tree,
        &[] as &[&git2::Commit],
    )?;

    Ok(())
}

// Function to create a GitHub repository via GitHub's API
async fn create_github_repo(
    client: &Client,
    url: &str,
    repo_data: &GitHubRepo,
    github_token: &str,
) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    info!("{}", format!("{}", github_token));
    headers.insert("Authorization", HeaderValue::from_str(&format!("token {}", github_token)).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github.v3+json"));

    let res = client
        .post(url)
        .headers(headers)
        .json(&repo_data)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().await.unwrap();
                let repo_url = json["html_url"]
                    .as_str()
                    .ok_or_else(|| "Failed to extract URL from GitHub response".to_string())?
                    .to_string();
                Ok(repo_url)
            } else {
                Err(format!("Failed to create GitHub repo: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Error creating GitHub repo: {}", e)),
    }
}

fn push_to_github(repo: &Repository, repo_url: &str) -> Result<(), git2::Error> {
    // Check if remote "origin" exists; otherwise, create it
    let mut remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(_) => repo.remote("origin", repo_url)?,
    };

    // Push the local repository to the GitHub remote
    let mut push_options = git2::PushOptions::new();
    let refspecs = vec!["refs/heads/master:refs/heads/master"];
    remote.push(&refspecs, Some(&mut push_options))?;

    Ok(())
}
