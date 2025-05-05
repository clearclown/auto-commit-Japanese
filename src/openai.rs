// src/openai.rs

use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionFunctionCall, ChatCompletionFunctions, ChatCompletionRequestMessage,
        CreateChatCompletionRequestArgs, FunctionCall, Role,
    },
};
use crate::commit::Commit;

pub async fn generate_commit(diff: &str) -> Result<Commit> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");
    let client = async_openai::Client::with_config(
        OpenAIConfig::new().with_api_key(api_key),
    );

    let req = CreateChatCompletionRequestArgs::default()
        .messages(vec![
            ChatCompletionRequestMessage {
                role: Role::System,
                content: Some("あなたは経験豊富なソフトウェアエンジニアであり、日本語で簡潔かつ具体的なコミットメッセージを書く。タイトルは 50 文字以内で要点を示し、本文では 72 文字幅を目安に詳細を記述せよ。".into()),
                ..Default::default()
            },
            ChatCompletionRequestMessage {
                role: Role::Assistant,
                content: None,
                function_call: Some(FunctionCall{
                    name: "get_diff".into(),
                    arguments: "{}".into(),
                }),
                ..Default::default()
            },
            ChatCompletionRequestMessage {
                role: Role::Function,
                name: Some("get_diff".into()),
                content: Some(diff.into()),
                ..Default::default()
            },
        ])
        .functions(vec![
            ChatCompletionFunctions{
                name: "get_diff".into(),
                description: Some("git diff の出力を文字列で返す".into()),
                parameters: Some(serde_json::json!({"type":"object","properties":{}})),
            },
            ChatCompletionFunctions{
                name: "commit".into(),
                description: Some("指定されたタイトルと説明でコミットを作成する".into()),
                parameters: Some(crate::schema::commit_schema()),
            },
        ])
        .function_call(ChatCompletionFunctionCall::Object(
            serde_json::json!({"name":"commit"})
        ))
        .model("gpt-4o-mini-2024-07-18")
        .temperature(0.0)
        .max_tokens(1024u16)
        .build()?;

    let completion = client.chat().create(req).await?;
    let fc = completion.choices[0].message.function_call
        .as_ref().expect("function_call");
    let commit: Commit = serde_json::from_str(&fc.arguments)?;

    Ok(commit)
}
