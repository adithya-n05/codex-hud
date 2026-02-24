# Segment 13: Native TUI HUD Activation for Installed Codex (Strict Atomic TDD)

This segment defines the execution plan to make the custom Codex HUD actually visible in real Codex CLI sessions by patching the native TUI behavior (not just launcher env injection), then wiring codex-hud install to place the patched native binary into the installed npm Codex package.

## Scope
- Deliver visible two-line HUD in real Codex TUI.
- Deliver `/statusline` configuration UI with native + derived HUD toggles.
- Keep fallback to stock Codex when unsupported.
- Keep no tmux/no extra pane/no third HUD line constraints.

## Non-goals
- Brew package integration (handled in follow-up segment).
- Countdown timers to reset timestamps (explicitly deferred).
- Automation bot for release compatibility roll-forward (explicitly deferred).

## Hard execution rules (segment-local)
- One task = one failing test = one minimal implementation = one green run = one commit.
- No multi-test bundles in one task.
- Run exact test selector for red/green.
- After each task, run local guard for affected repo.
- Conventional commit format only.

## Repos used in this segment
- `codex-hud` (this repo): installer/runtime/orchestration.
- `codex-upstream` (local sibling checkout at `rust-v0.104.0`): native TUI behavior.

## Branch strategy
- Planning branch (this doc): `segment-13-native-tui-hud-execution-plan`.
- Execution branch A (native TUI): `segment-13a-native-tui-hud`.
- Execution branch B (installer/runtime): `segment-13b-native-install-runtime`.
- Final integration merge branch: `segment-13c-native-e2e`.

## Master To-Do (atomic slices)

### Slice J1: Native `/statusline` Item Model (codex-upstream)
- [ ] J1-001 Add HUD item enum variants for permission/auth/tools/context-bars.
- [ ] J1-002 Add descriptions for all new HUD items.
- [ ] J1-003 Add preview rendering strings for all new HUD items.
- [ ] J1-004 Ensure `/hud` remains rejected and `/statusline` remains accepted.

### Slice J2: HUD Runtime Metric Derivation (codex-upstream)
- [ ] J2-001 Derive permission chip from active approval+sandbox config.
- [ ] J2-002 Derive auth chip from auth mode/provider metadata.
- [ ] J2-003 Track tool-call counters in chatwidget state.
- [ ] J2-004 Render tool-call chip from tracked counter.

### Slice J3: Colored Progress Bar Rendering (codex-upstream)
- [ ] J3-001 Add bar helper for percentage clamping and block width.
- [ ] J3-002 Add green/yellow/red color threshold mapping.
- [ ] J3-003 Add context bar rendering (`CTX`) with percentage label.
- [ ] J3-004 Add 5h and weekly bar rendering with same rules.

### Slice J4: Two-Line Footer HUD Rendering (codex-upstream)
- [ ] J4-001 Add secondary status-line storage path in bottom pane/composer.
- [ ] J4-002 Render second line when secondary value exists.
- [ ] J4-003 Keep single-line behavior when secondary line absent.
- [ ] J4-004 Add wrap behavior test for narrow width while staying two-line max.

### Slice J5: `/statusline` UI Content Expansion (codex-upstream)
- [ ] J5-001 Add new selectable rows to setup view item list.
- [ ] J5-002 Preserve deterministic ordering for presets/full mode.
- [ ] J5-003 Ensure keyboard-only workflow remains unchanged.
- [ ] J5-004 Ensure cancel has no confirmation prompt.

### Slice J6: Build and Package Patched Native Binary (codex-hud)
- [ ] J6-001 Add deterministic local build command wrapper for codex-upstream `codex` binary.
- [ ] J6-002 Add compatibility-keyed cache path for patched binary artifact.
- [ ] J6-003 Add failure path when codex-upstream source/tree is unavailable.

### Slice J7: Install-time Native Binary Replacement for npm Codex (codex-hud)
- [ ] J7-001 Detect installed npm vendor binary path from stock codex launcher.
- [ ] J7-002 Replace vendored binary with patched cached binary when compatible.
- [ ] J7-003 Verify replacement idempotence.
- [ ] J7-004 Keep unsupported one-time notice policy unchanged.

### Slice J8: End-to-End Local Validation (codex-hud + local codex)
- [ ] J8-001 Install command patches native binary and reports patched mode.
- [ ] J8-002 Interactive Codex session shows two-line HUD.
- [ ] J8-003 `/statusline` shows expanded item list and applies selection.
- [ ] J8-004 Uninstall returns to stock behavior.

---

## Detailed Task Cards

## J1-001
1. Task ID: `J1-001`
2. Behavior under test: statusline item enum includes new HUD variants.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/status_line_setup.rs`
4. Failing test to write (exact):
```rust
#[test]
fn hud_item_set_includes_native_and_derived_variants() {
    use strum::IntoEnumIterator;
    let names = StatusLineItem::iter().map(|i| i.to_string()).collect::<Vec<_>>();
    assert!(names.contains(&"permission-mode".to_string()));
    assert!(names.contains(&"auth-chip".to_string()));
    assert!(names.contains(&"tool-calls".to_string()));
    assert!(names.contains(&"ctx-bar".to_string()));
    assert!(names.contains(&"five-hour-bar".to_string()));
    assert!(names.contains(&"weekly-bar".to_string()));
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_item_set_includes_native_and_derived_variants
```
6. Minimal implementation:
- Add enum variants only, no rendering logic.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_item_set_includes_native_and_derived_variants
```
8. Commit command:
```bash
git add codex-rs/tui/src/bottom_pane/status_line_setup.rs
git commit -m "feat(tui): add native and derived hud statusline item variants"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_item_set_includes_native_and_derived_variants
cargo test -p codex-tui
```

## J1-002
1. Task ID: `J1-002`
2. Behavior under test: new HUD items expose non-empty descriptions.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/status_line_setup.rs`
4. Failing test to write (exact):
```rust
#[test]
fn hud_items_have_descriptions() {
    let items = [
        StatusLineItem::PermissionMode,
        StatusLineItem::AuthChip,
        StatusLineItem::ToolCalls,
        StatusLineItem::CtxBar,
        StatusLineItem::FiveHourBar,
        StatusLineItem::WeeklyBar,
    ];
    for item in items {
        assert!(!item.description().trim().is_empty());
    }
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_descriptions
```
6. Minimal implementation:
- Add match arms in `description()` for new variants.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_descriptions
```
8. Commit:
```bash
git add codex-rs/tui/src/bottom_pane/status_line_setup.rs
git commit -m "feat(tui): define descriptions for new hud statusline items"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_descriptions
cargo test -p codex-tui
```

## J1-003
1. Task ID: `J1-003`
2. Behavior under test: new HUD items expose preview render strings.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/status_line_setup.rs`
4. Failing test to write (exact):
```rust
#[test]
fn hud_items_have_preview_samples() {
    assert!(StatusLineItem::PermissionMode.render().contains("perm"));
    assert!(StatusLineItem::AuthChip.render().contains("auth"));
    assert!(StatusLineItem::ToolCalls.render().contains("tools"));
    assert!(StatusLineItem::CtxBar.render().contains("CTX"));
    assert!(StatusLineItem::FiveHourBar.render().contains("5H"));
    assert!(StatusLineItem::WeeklyBar.render().contains("7D"));
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_preview_samples
```
6. Minimal implementation:
- Add `render()` match arms for new variants.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_preview_samples
```
8. Commit:
```bash
git add codex-rs/tui/src/bottom_pane/status_line_setup.rs
git commit -m "feat(tui): add preview samples for hud statusline items"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_items_have_preview_samples
cargo test -p codex-tui
```

## J2-001
1. Task ID: `J2-001`
2. Behavior under test: permission chip derives `approval+sandbox` label.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/chatwidget.rs`
4. Failing test to write (exact):
```rust
#[test]
fn permission_chip_displays_approval_and_sandbox() {
    let label = format_permission_chip(AskForApproval::Never, &SandboxPolicy::DangerFullAccess);
    assert_eq!(label, "never+dang-full");
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui permission_chip_displays_approval_and_sandbox
```
6. Minimal implementation:
- Add pure helper `format_permission_chip(...) -> String` only.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui permission_chip_displays_approval_and_sandbox
```
8. Commit:
```bash
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): add permission chip formatter for hud"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui permission_chip_displays_approval_and_sandbox
cargo test -p codex-tui
```

## J2-002
1. Task ID: `J2-002`
2. Behavior under test: auth chip supports chatgpt/api-key/foundry/bedrock/gcp/azure labels.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/chatwidget.rs`
4. Failing test to write (exact):
```rust
#[test]
fn auth_chip_detects_provider_patterns() {
    assert_eq!(detect_auth_chip("chatgpt", None, None), "ChatGPT");
    assert_eq!(detect_auth_chip("api_key", None, None), "API key");
    assert_eq!(detect_auth_chip("azure", Some("https://foo.openai.azure.com"), None), "Azure");
    assert_eq!(detect_auth_chip("bedrock", Some("https://bedrock-runtime.us-east-1.amazonaws.com"), None), "Bedrock");
    assert_eq!(detect_auth_chip("gcp", Some("https://us-central1-aiplatform.googleapis.com"), None), "GCP");
    assert_eq!(detect_auth_chip("foundry", Some("https://api.foundry.ai"), None), "Foundry");
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui auth_chip_detects_provider_patterns
```
6. Minimal implementation:
- Add pure helper with explicit matching only.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui auth_chip_detects_provider_patterns
```
8. Commit:
```bash
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): add auth chip provider detection for hud"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui auth_chip_detects_provider_patterns
cargo test -p codex-tui
```

## J2-003
1. Task ID: `J2-003`
2. Behavior under test: tool counters increment on exec/mcp/web/patch begin-end families.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/chatwidget.rs`
4. Failing test to write (exact):
```rust
#[test]
fn hud_tool_counter_tracks_core_event_families() {
    let mut c = HudToolCounter::default();
    c.on_exec_call();
    c.on_mcp_call();
    c.on_web_call();
    c.on_patch_apply();
    assert_eq!(c.total(), 4);
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_tool_counter_tracks_core_event_families
```
6. Minimal implementation:
- Add tiny `HudToolCounter` struct and increment methods.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_tool_counter_tracks_core_event_families
```
8. Commit:
```bash
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): add hud tool call counter state"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_tool_counter_tracks_core_event_families
cargo test -p codex-tui
```

## J3-001
1. Task ID: `J3-001`
2. Behavior under test: bar helper clamps percent and produces fixed-width fill.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/chatwidget.rs`
4. Failing test to write (exact):
```rust
#[test]
fn progress_bar_clamps_percent_and_width() {
    let bar = render_hud_bar(135, 10);
    assert_eq!(bar, "██████████");
    let bar2 = render_hud_bar(-10, 10);
    assert_eq!(bar2, "░░░░░░░░░░");
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui progress_bar_clamps_percent_and_width
```
6. Minimal implementation:
- Add pure helper for unicode bar fill only.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui progress_bar_clamps_percent_and_width
```
8. Commit:
```bash
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): add clamped fixed-width hud bar helper"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui progress_bar_clamps_percent_and_width
cargo test -p codex-tui
```

## J3-002
1. Task ID: `J3-002`
2. Behavior under test: bar color map uses threshold buckets green/yellow/red.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/chatwidget.rs`
4. Failing test to write (exact):
```rust
#[test]
fn hud_color_bucket_maps_thresholds() {
    assert_eq!(hud_color_bucket(40), "green");
    assert_eq!(hud_color_bucket(80), "yellow");
    assert_eq!(hud_color_bucket(95), "red");
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_color_bucket_maps_thresholds
```
6. Minimal implementation:
- Add pure threshold helper only.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_color_bucket_maps_thresholds
```
8. Commit:
```bash
git add codex-rs/tui/src/chatwidget.rs
git commit -m "feat(tui): add hud color threshold bucket helper"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui hud_color_bucket_maps_thresholds
cargo test -p codex-tui
```

## J4-001
1. Task ID: `J4-001`
2. Behavior under test: bottom pane supports a secondary status-line value.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/chat_composer.rs`
- `codex-upstream/codex-rs/tui/src/bottom_pane/mod.rs`
4. Failing test to write (exact):
```rust
#[test]
fn composer_stores_secondary_status_line() {
    let mut composer = test_composer();
    assert!(composer.set_secondary_status_line(Some(Line::from("L2"))));
    assert!(!composer.set_secondary_status_line(Some(Line::from("L2"))));
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui composer_stores_secondary_status_line
```
6. Minimal implementation:
- Add field + setter, no render path yet.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui composer_stores_secondary_status_line
```
8. Commit:
```bash
git add codex-rs/tui/src/bottom_pane/chat_composer.rs codex-rs/tui/src/bottom_pane/mod.rs
git commit -m "feat(tui): add secondary status line state path"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui composer_stores_secondary_status_line
cargo test -p codex-tui
```

## J4-002
1. Task ID: `J4-002`
2. Behavior under test: footer renders two lines when secondary status line exists.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/chat_composer.rs`
- `codex-upstream/codex-rs/tui/src/bottom_pane/footer.rs`
4. Failing test to write (exact):
```rust
#[test]
fn footer_renders_two_lines_when_secondary_status_line_present() {
    let props = test_footer_props_with_two_lines();
    let lines = footer_from_props_lines_for_test(&props);
    assert_eq!(lines.len(), 2);
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui footer_renders_two_lines_when_secondary_status_line_present
```
6. Minimal implementation:
- Append second line only in base footer modes when secondary exists.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui footer_renders_two_lines_when_secondary_status_line_present
```
8. Commit:
```bash
git add codex-rs/tui/src/bottom_pane/chat_composer.rs codex-rs/tui/src/bottom_pane/footer.rs
git commit -m "feat(tui): render two-line footer hud when secondary line is set"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui footer_renders_two_lines_when_secondary_status_line_present
cargo test -p codex-tui
```

## J5-001
1. Task ID: `J5-001`
2. Behavior under test: `/statusline` setup shows new HUD items in picker list.
3. Files touched:
- `codex-upstream/codex-rs/tui/src/bottom_pane/status_line_setup.rs`
4. Failing test to write (exact):
```rust
#[test]
fn statusline_setup_view_includes_hud_items() {
    let view = StatusLineSetupView::new(None, test_app_event_sender());
    let ids = view.test_item_ids();
    assert!(ids.contains(&"permission-mode".to_string()));
    assert!(ids.contains(&"ctx-bar".to_string()));
}
```
5. Red command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui statusline_setup_view_includes_hud_items
```
6. Minimal implementation:
- Make setup builder consume expanded enum only.
7. Green command:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui statusline_setup_view_includes_hud_items
```
8. Commit:
```bash
git add codex-rs/tui/src/bottom_pane/status_line_setup.rs
git commit -m "feat(tui): expose hud items in statusline setup picker"
```
9. Post-commit guard:
```bash
cd ../codex-upstream/codex-rs
cargo test -p codex-tui statusline_setup_view_includes_hud_items
cargo test -p codex-tui
```

## J6-001
1. Task ID: `J6-001`
2. Behavior under test: codex-hud can derive npm vendor codex binary path from stock launcher path.
3. Files touched:
- `rust/crates/codex_hud_ops/src/codex_probe.rs`
- `rust/crates/codex_hud_ops/tests/codex_probe.rs`
4. Failing test to write (exact):
```rust
#[test]
fn resolves_npm_vendor_binary_path_from_stock_launcher() {
    let p = resolve_npm_vendor_binary_path(Path::new("/opt/node/bin/codex"), "darwin-arm64").unwrap();
    assert!(p.ends_with("@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/codex/codex"));
}
```
5. Red command:
```bash
cd rust
cargo test -p codex_hud_ops resolves_npm_vendor_binary_path_from_stock_launcher
```
6. Minimal implementation:
- Add pure path resolver helper.
7. Green command:
```bash
cd rust
cargo test -p codex_hud_ops resolves_npm_vendor_binary_path_from_stock_launcher
```
8. Commit:
```bash
git add rust/crates/codex_hud_ops/src/codex_probe.rs rust/crates/codex_hud_ops/tests/codex_probe.rs
git commit -m "feat(ops): resolve npm codex vendor binary path for native replacement"
```
9. Post-commit guard:
```bash
cd rust
cargo test -p codex_hud_ops resolves_npm_vendor_binary_path_from_stock_launcher
cargo test --workspace
```

## J7-001
1. Task ID: `J7-001`
2. Behavior under test: install replaces vendor binary with cached patched binary when compatibility matches.
3. Files touched:
- `rust/crates/codex_hud_ops/src/native_install.rs`
- `rust/crates/codex_hud_ops/tests/native_install.rs`
4. Failing test to write (exact):
```rust
#[test]
fn install_replaces_vendor_binary_with_cached_patched_binary() {
    let fx = setup_npm_vendor_fixture();
    let out = install_native_patch_using_cached_binary(&fx.home, &fx.stock_codex, &fx.compat_key).unwrap();
    assert_eq!(out, InstallOutcome::Patched);
    assert_eq!(std::fs::read(&fx.vendor_binary).unwrap(), std::fs::read(&fx.cached_patched_binary).unwrap());
}
```
5. Red command:
```bash
cd rust
cargo test -p codex_hud_ops install_replaces_vendor_binary_with_cached_patched_binary
```
6. Minimal implementation:
- Add replacement write path only for supported npm vendor layout.
7. Green command:
```bash
cd rust
cargo test -p codex_hud_ops install_replaces_vendor_binary_with_cached_patched_binary
```
8. Commit:
```bash
git add rust/crates/codex_hud_ops/src/native_install.rs rust/crates/codex_hud_ops/tests/native_install.rs
git commit -m "feat(ops): replace npm codex vendor binary with cached patched native build"
```
9. Post-commit guard:
```bash
cd rust
cargo test -p codex_hud_ops install_replaces_vendor_binary_with_cached_patched_binary
cargo test --workspace
```

## J8-001
1. Task ID: `J8-001`
2. Behavior under test: local real codex flow shows patched state after install and stock after uninstall.
3. Files touched:
- `rust/crates/codex_hud_cli/tests/main_install.rs`
- `rust/crates/codex_hud_cli/tests/main_uninstall.rs`
4. Failing test to write (exact):
```rust
#[test]
fn install_then_uninstall_transitions_patch_mode() {
    let fx = setup_realistic_npm_codex_fixture();
    assert_eq!(run_cli_install(&fx).unwrap(), "install: patched");
    assert!(run_cli_status(&fx).unwrap().contains("patch_mode: patched"));
    assert_eq!(run_cli_uninstall(&fx).unwrap(), "uninstall: ok");
    assert!(run_cli_status(&fx).unwrap().contains("patch_mode: stock"));
}
```
5. Red command:
```bash
cd rust
cargo test -p codex_hud_cli install_then_uninstall_transitions_patch_mode
```
6. Minimal implementation:
- Wire install/uninstall orchestration through vendor binary replacement path and policy state updates.
7. Green command:
```bash
cd rust
cargo test -p codex_hud_cli install_then_uninstall_transitions_patch_mode
```
8. Commit:
```bash
git add rust/crates/codex_hud_cli/tests/main_install.rs rust/crates/codex_hud_cli/tests/main_uninstall.rs rust/crates/codex_hud_cli/src/runtime.rs
git commit -m "feat(cli): wire install and uninstall patch mode transitions for native npm binary"
```
9. Post-commit guard:
```bash
cd rust
cargo test -p codex_hud_cli install_then_uninstall_transitions_patch_mode
cargo test --workspace
npm test
```

---

## Segment completion checklist
- [ ] All tasks in J1-J8 completed with atomic commits.
- [ ] `codex-upstream` TUI shows two-line HUD with color bars.
- [ ] `/statusline` shows expanded toggles and applies changes.
- [ ] `codex-hud install` patches npm Codex native binary path.
- [ ] `codex-hud uninstall` restores stock behavior.
- [ ] Local manual validation captured with exact commands and outputs.

## Final merge procedure (for execution branches)
```bash
# from codex-hud repo

git checkout main
git pull --ff-only

git checkout segment-13a-native-tui-hud
# complete J1-J5 work + commits
git checkout main
git merge --ff-only segment-13a-native-tui-hud

git checkout segment-13b-native-install-runtime
# complete J6-J7 work + commits
git checkout main
git merge --ff-only segment-13b-native-install-runtime

git checkout segment-13c-native-e2e
# complete J8 work + commits
git checkout main
git merge --ff-only segment-13c-native-e2e

git push origin main
```
