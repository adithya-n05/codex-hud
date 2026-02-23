use codex_hud_classifier::{classify, ClassifierInput};

#[test]
fn detect_openai_auth_label() {
    let input = ClassifierInput {
        provider_id: Some("openai".to_string()),
        explicit_auth_override: Some("openai-auth".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "OpenAI auth");
}

#[test]
fn detect_bedrock_api_key_label() {
    let input = ClassifierInput {
        base_url: Some("https://bedrock-runtime.us-east-1.amazonaws.com/openai/v1".to_string()),
        has_api_key: true,
        env_key_name: Some("BEDROCK_API_KEY".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "Bedrock API key");
}

#[test]
fn detect_env_key_label() {
    let input = ClassifierInput {
        env_key_name: Some("CUSTOM_PROVIDER_KEY".to_string()),
        has_api_key: false,
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "Env key");
}

#[test]
fn detect_bearer_token_label() {
    let input = ClassifierInput {
        has_bearer_header: true,
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "Bearer token");
}

#[test]
fn detect_no_auth_label() {
    let input = ClassifierInput {
        explicit_auth_override: Some(" no AUTH ".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "No auth");
}

#[test]
fn detect_chatgpt_auth() {
    let input = ClassifierInput {
        explicit_auth_override: Some("chatgpt".to_string()),
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "ChatGPT");
}

#[test]
fn detect_api_key_from_env_key_name() {
    let input = ClassifierInput {
        env_key_name: Some("OPENAI_API_KEY".to_string()),
        has_api_key: false,
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "API key");
}

#[test]
fn detect_entra_token_for_azure_openai() {
    let input = ClassifierInput {
        base_url: Some("https://my-resource.openai.azure.com/openai/v1".to_string()),
        has_bearer_header: true,
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "Entra token");
}

#[test]
fn detect_bedrock_auth_label_from_aws_signing_headers() {
    let input = ClassifierInput {
        base_url: Some("https://bedrock-runtime.us-east-1.amazonaws.com/openai/v1".to_string()),
        env_header_keys: vec!["X-Amz-Date".to_string()],
        ..ClassifierInput::default()
    };
    let out = classify(&input);
    assert_eq!(out.auth_label, "AWS creds");
}
