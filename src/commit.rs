// src/commit.rs

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lazy_static::lazy_static;

/// gitmojis.json をコンパイル時にバイナリへ埋め込む
const GITMOJIS_RAW: &str = include_str!("./lib/gitmojis.json");

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Commit {
    pub title: String,
    pub description: String,
}

impl ToString for Commit {
    fn to_string(&self) -> String {
        format!("{}\n\n{}", self.title, self.description)
    }
}

/// gitmoji レコード
#[derive(Deserialize)]
struct Record {
    emoji: String,
    // code: String,   // 例 ":sparkles:"
    name: String,   // 例 "sparkles"
    description: String,
}

lazy_static! {
    static ref INDEX: Vec<Record> = {
        let v: Value = serde_json::from_str(GITMOJIS_RAW)
            .expect("gitmojis.json parse");
        serde_json::from_value::<Vec<Record>>(v["gitmojis"].clone())
            .expect("gitmojis list")
    };
}

/// AI が返したタイトルを「emoji slug: title」形式へ変換
pub fn apply_gitmoji(mut commit: Commit) -> Result<String> {
    // 1. slug 推定（単純に最初の単語を取る: "Introduce new features." -> "introduce"）
    let slug = commit.title
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_lowercase();

    // 2. gitmoji 検索（name or description に部分一致）
    if let Some(rec) = INDEX.iter()
        .find(|r| r.name.contains(&slug) || r.description.to_lowercase().contains(&slug))
    {
        commit.title = format!("{} {}: {}", rec.emoji, slug, commit.title);
    }

    Ok(commit.to_string())
}
