// src/git.rs

use anyhow::{ensure, Result};
use question::{Answer, Question};
use std::{
    io::Write,
    process::{Command, Stdio},
    str,
};

use crate::cli::Cli;

/// リポジトリ内かどうかを確認する
pub fn ensure_repo() -> Result<()> {
    let out = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()?;
    ensure!(
        str::from_utf8(&out.stdout)?.trim() == "true",
        "Not inside a git repository"
    );
    Ok(())
}

/// ステージ済み差分を取得する
pub fn staged_diff() -> Result<String> {
    let out = Command::new("git").args(["diff", "--staged"]).output()?;
    Ok(String::from_utf8(out.stdout)?)
}

/// HEAD との差分を取得する
pub fn head_diff() -> Result<String> {
    let out = Command::new("git").args(["diff", "HEAD"]).output()?;
    Ok(String::from_utf8(out.stdout)?)
}

/// --dry-run 判定・確認ダイアログ・コミット実行までを担当
pub fn maybe_commit(cli: &Cli, msg: &str) -> Result<()> {
    use log::{error, info};

    // dry-run モードなら出力して終了
    if cli.dry_run {
        info!("{msg}");
        return Ok(());
    }

    info!(
        "Proposed Commit:\n----------------------\n{}\n----------------------",
        msg
    );

    // 確認ダイアログ（--force でスキップ）
    if !cli.force {
        let ans = Question::new("Continue? (Y/n)")
            .yes_no()
            .default(Answer::YES)
            .ask()                      // Option<Answer>
            .unwrap_or(Answer::YES);    // 入力不能時は YES とみなす

        if ans == Answer::NO {
            error!("Commit aborted");
            std::process::exit(1);
        }
    }

    info!("Committing…");
    let mut child = Command::new("git")
        .arg("commit")
        .args(if cli.review { vec!["-e"] } else { vec![] })
        .arg("-F")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()?;

    // コミットメッセージをパイプで送信
    if let Some(mut stdin) = child.stdin.take() {
        let message = msg.to_owned();            // ← 所有権を取得して 'static 化
        std::thread::spawn(move || {
            let _ = stdin.write_all(message.as_bytes());
        });
    }

    let output = child.wait_with_output()?;
    info!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
