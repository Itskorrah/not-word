//! Headless UI screenshots: renders the whole Not Word window offscreen (wgpu), so it works
//! with a locked screen or a hidden window.
//!
//! `cargo run -p wordcraft-ui-egui --example ui_shot -- script.jsonl`
//!
//! The script is JSON lines: control-channel requests (`{"method": …, "params": …}`, see
//! `docs/control-protocol.md`), `{"shot": "/abs/out.png"}` to save the window as PNG, or
//! `{"steps": n}` to run extra frames, or `{"wait": ms}` to keep running frames for a while (for
//! background work such as a Zotero session). The window is 1440×900 pt at 2× and opens the sample
//! document unless the first line is `{"empty": true}`; `{"empty": true, "startScreen": true}`
//! starts the way the desktop app does with nothing to open (File › Home, #766).

use wordcraft_engine::Session;
use wordcraft_ui_egui::{ControlRequest, Services, WordApp};

static READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ui_shot: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let path = std::env::args().nth(1).ok_or("usage: ui_shot script.jsonl")?;
    let script = std::fs::read_to_string(&path).map_err(|error| format!("reading {path}: {error}"))?;
    let (tx, rx) = std::sync::mpsc::channel::<ControlRequest>();
    let lines: Vec<serde_json::Value> = script
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| serde_json::from_str(line).map_err(|error| format!("line {}: {error}", index + 1)))
        .collect::<Result<_, _>>()?;
    let doc = if lines.first().is_some_and(|l| l.get("empty").is_some()) {
        wordcraft_doc::Document::new()
    } else {
        wordcraft_engine::sample::sample_document()
    };
    let mut app = WordApp::new(Session::new(doc), Services::default()).with_control(rx);
    app.integrated_titlebar = true;
    if lines.first().is_some_and(|l| l.get("startScreen").is_some()) {
        app.start_screen();
    }
    let w = dimension(&lines, "width", 1440.0)?;
    let h = dimension(&lines, "height", 900.0)?;
    let mut harness =
        egui_kittest::Harness::builder().with_size(egui::vec2(w, h)).with_pixels_per_point(2.0).with_max_steps(1_000_000).wgpu().build_ui_state(
            |ui, app: &mut WordApp| {
                if !READY.load(std::sync::atomic::Ordering::Relaxed) {
                    return;
                }
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
    harness.input_mut().max_texture_side = Some(8192);
    READY.store(true, std::sync::atomic::Ordering::Relaxed);
    step_n(&mut harness, 6);
    for l in lines {
        if let Some(p) = l.get("shot").and_then(|v| v.as_str()) {
            step_n(&mut harness, 4);
            match harness.render() {
                Ok(img) => {
                    img.save(p).map_err(|error| format!("saving {p}: {error}"))?;
                    println!("{p}");
                }
                Err(error) => return Err(format!("rendering {p}: {error}")),
            }
        } else if let Some(n) = l.get("steps").and_then(|v| v.as_u64()) {
            let n = usize::try_from(n).ok().filter(|n| *n <= 1_000_000).ok_or("steps must be at most 1000000")?;
            step_n(&mut harness, n);
        } else if let Some(ms) = l.get("wait").and_then(|v| v.as_u64()) {
            if ms > 300_000 {
                return Err("wait must be at most 300000 milliseconds".into());
            }
            let end = std::time::Instant::now() + std::time::Duration::from_millis(ms);
            while std::time::Instant::now() < end {
                step(&mut harness);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        } else if let Some(m) = l.get("method").and_then(|v| v.as_str()) {
            let (req, reply) = ControlRequest::new(m, l.get("params").cloned().unwrap_or_default());
            tx.send(req).map_err(|error| format!("sending {m}: {error}"))?;
            let mut answered = false;
            for _ in 0..60 {
                step(&mut harness);
                if let Ok(r) = reply.try_recv() {
                    if r.get("ok").and_then(serde_json::Value::as_bool) == Some(false) {
                        return Err(format!("{m}: {r}"));
                    }
                    let s = r.to_string();
                    // `UI_SHOT_FULL=1` prints whole replies (e.g. `document.inspect` for scripted checks).
                    let cut = if std::env::var_os("UI_SHOT_FULL").is_some() { s.len() } else { s.len().min(300) };
                    println!("{m}: {}", s.get(..cut).unwrap_or(&s));
                    answered = true;
                    break;
                }
            }
            if !answered {
                return Err(format!("{m}: no reply after 60 frames"));
            }
            step_n(&mut harness, 2);
        }
    }
    Ok(())
}

/// Bound the offscreen texture allocation, including the harness's 2× pixel scale.
fn dimension(lines: &[serde_json::Value], key: &str, default: f32) -> Result<f32, String> {
    let Some(value) = lines.iter().find_map(|line| line.get(key)) else { return Ok(default) };
    let value =
        value.as_f64().filter(|v| v.is_finite() && (320.0..=4096.0).contains(v)).ok_or_else(|| format!("{key} must be a number from 320 to 4096"))?;
    Ok(value as f32)
}

fn step(harness: &mut egui_kittest::Harness<'_, WordApp>) {
    let mut raw = std::mem::take(harness.input_mut());
    harness.state_mut().raw_input_hook(&mut raw);
    *harness.input_mut() = raw;
    harness.step();
}

fn step_n(harness: &mut egui_kittest::Harness<'_, WordApp>, n: usize) {
    for _ in 0..n {
        step(harness);
    }
}

#[cfg(test)]
mod tests {
    use super::dimension;
    use serde_json::json;

    #[test]
    fn screenshot_dimensions_reject_invalid_or_unbounded_allocations() {
        assert_eq!(dimension(&[], "width", 1440.0).unwrap(), 1440.0);
        assert_eq!(dimension(&[json!({"width": 800})], "width", 1440.0).unwrap(), 800.0);
        for value in [json!(-1), json!(1e100), json!("wide"), json!(null), json!(4097)] {
            assert!(dimension(&[json!({"width": value})], "width", 1440.0).is_err());
        }
    }
}
