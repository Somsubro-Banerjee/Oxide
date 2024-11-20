use crate::{grpc_client, storage, utils};
use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::process::{exit, Command, Stdio};

pub async fn start_onboarding() {
    println!("{}", "🚀 Welcome to Oxide Onboarding! 🚀".bright_blue());
    println!(
        "{}",
        "Let's set up your project step by step.\n".bright_yellow()
    );

    // Gather user input
    let project_name = utils::ask("What is the name of your project?");
    let company_name = utils::ask("What is your company name?");
    let framework_stack = utils::ask_with_options(
        "Select your framework stack:",
        &[
            "Rust",
            "C++",
            "Java",
            ".Net",
            "Python",
            "React",
            "Bash",
            "Powershell",
        ],
    );
    let repository_host =
        utils::ask_with_options("Where do you want to create the repository?", &["GitHub"]);

    // Construct onboarding data
    let data = HashMap::from([
        ("project_name", project_name.clone()),
        (
            "org",
            company_name.clone(),
        ),
        ("framework_stack", framework_stack.clone()),
        ("repository_host", repository_host.clone()),
    ]);

    println!(
        "\n{}\n",
        format!(
            "🎉 Great! Setting up {} on {} 🚀",
            project_name, repository_host
        )
        .bright_green()
    );

    
    println!(
        "\n{}\n",
        format!("🚀 Hot starting Cargo CI...").bright_black()
    );

  

    println!("⚙️ Getting it ready for incoming requests...");

  
    // Call cargo_ci to create the repository
    match grpc_client::create_repository(&data).await {
        Ok(repo_url) => {
            println!(
                "{}",
                format!("✅ Repository created: {}", repo_url).bright_blue()
            );
            storage::persist_onboarding(&data, &repo_url);
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to create repository: {}", e).red());
        }
    }

    println!("{}", "🎯 Onboarding completed!".bright_magenta());
}
