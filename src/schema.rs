// src/schema.rs

use schemars::schema_for;
use serde_json::Value;
use crate::commit::Commit;

/// OpenAI function-call 用 JSON Schema を生成
pub fn commit_schema() -> Value {
    // Commit 構造体の RootSchema を取得
    let schema = schema_for!(Commit);
    // `schema.schema` 部分だけ JSON に変換して返す
    serde_json::to_value(&schema.schema).expect("schema serialization")
}
