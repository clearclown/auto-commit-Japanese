// src/main.rs
use anyhow::Result;

mod cli;
mod commit;
mod git;
mod openai;
mod schema;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::init_logger(&cli);

    // ① Git まわり
    git::ensure_repo()?;
    let staged = git::staged_diff()?;
    if staged.is_empty() {
        anyhow::bail!("There are no staged files to commit");
    }

    // ② OpenAI へ投げる（HEAD 差分も添付）
    let head_diff = git::head_diff()?;
    let spinner = util::start_spinner(&cli, "Analyzing codebase…");
    let commit = openai::generate_commit(&head_diff).await?; // → commit::Commit
    util::stop_spinner(spinner, "Finished Analyzing!");

    // ③ gitmoji＋スラッグをタイトルへ付与
    let formatted = commit::apply_gitmoji(commit)?;

    // ④ Dry-run or 実際にコミット
    git::maybe_commit(&cli, &formatted)?;

    Ok(())
}
