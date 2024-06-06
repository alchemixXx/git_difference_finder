use std::path::Path;

mod config;
mod workers;
mod cli;
mod logger;
use cli::CLi;
use workers::branch::BranchSwitcher;
use workers::loginer::login;
use workers::history::HistoryProvider;
use clap::Parser;

use workers::writer::save_to_file;
fn main() {
    println!("Reading cli args...");
    let cli_args = CLi::parse();
    println!("CLI args: {:#?}", cli_args);

    let config = config::read_config(&cli_args.path);

    // let config = config::read_config("config.toml");

    crate::logger::Logger::init(config.logger.log_level);
    let logger = crate::logger::Logger::new();
    logger.info("Version updater started!");
    let repos = config.repos.get_repos_list();
    logger.info(format!("Repos to update: {:#?}", repos).as_str());

    logger.debug("Logging in to AWS...");
    login(&config.git.target_branch, &config.aws.role_script_path, &config.aws.role);
    logger.debug("Logged in to AWS");

    let mut results_string: String = String::new();

    for repo in repos.iter() {
        logger.debug(format!("Getting repo type for repo: {}", repo).as_str());
        let repo_type = config.repos.get_repo_type(repo);
        logger.debug(format!("Got repo type for repo: {}.Type={:?}", repo, repo_type).as_str());

        let repo_path = Path::new(&config.root)
            .join(repo)
            .to_str()
            .expect("Cant't build path")
            .to_string();
        let switcher = BranchSwitcher { target_branch: &config.git.target_branch };

        logger.debug(format!("Checking out to target branch for repo: {}", repo_path).as_str());
        switcher.checkout_target_branch(&repo_path);
        logger.debug(format!("Checked out to target branch for repo: {}", repo_path).as_str());

        let history_provider = HistoryProvider {
            path: &repo_path,
            target_branch: &config.git.target_branch,
            source_branch: &config.git.source_branch,
        };
        logger.debug(format!("Collecting repo history: {}", repo_path).as_str());
        let history = history_provider.provide();
        logger.debug(
            format!("Collected repo history: {}. Results: {:?}", repo_path, history).as_str()
        );

        results_string.push_str(repo);
        results_string.push('\n');
        results_string.push_str(&history);
        results_string.push_str("\n\n");
        logger.debug(format!("Updated version in repo: {}", repo_path).as_str());
    }

    logger.warn("Repos history logs:");
    let _ = save_to_file(&results_string, &"results.txt".to_string());
    logger.warn(format!("Results:\n{}", results_string).as_str());

    logger.info("Git difference finder finished!");
}
