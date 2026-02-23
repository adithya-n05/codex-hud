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
