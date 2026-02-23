pub fn validate_threshold_range(warn: i64, critical: i64) -> Result<(), String> {
    if !(0..=100).contains(&warn) || !(0..=100).contains(&critical) {
        return Err("threshold must be between 0 and 100".to_string());
    }
    Ok(())
}
