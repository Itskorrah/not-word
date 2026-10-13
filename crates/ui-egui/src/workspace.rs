//! Small, persistent workflow arrangements. These change interface visibility, never document
//! formatting, track-change settings, custom ribbon tabs, or keyboard shortcuts.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::WordApp;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum Workspace {
    Writing,
    Academic,
    #[default]
    Professional,
    Review,
    Focus,
    Custom,
}

impl Workspace {
    pub const ALL: [Self; 6] = [Self::Writing, Self::Academic, Self::Professional, Self::Review, Self::Focus, Self::Custom];

    pub fn code(self) -> &'static str {
        match self {
            Self::Writing => "writing",
            Self::Academic => "academic",
            Self::Professional => "professional",
            Self::Review => "review",
            Self::Focus => "focus",
            Self::Custom => "custom",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Writing => "Minimal writing",
            Self::Academic => "Academic",
            Self::Professional => "Professional",
            Self::Review => "Editing and review",
            Self::Focus => "Distraction-free",
            Self::Custom => "Custom",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|w| w.code().eq_ignore_ascii_case(code.trim()))
    }
}

impl From<String> for Workspace {
    fn from(s: String) -> Self {
        Self::from_code(&s).unwrap_or_default()
    }
}

impl From<Workspace> for String {
    fn from(w: Workspace) -> Self {
        w.code().into()
    }
}

/// Saved visibility, also for layouts the user edits after selecting a preset. Keeping the actual
/// settings avoids overwriting their choices with a preset at the next launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Layout {
    pub focus: bool,
    pub ruler: bool,
    pub navigation: bool,
    pub styles: bool,
    pub comments: bool,
}

impl Default for Layout {
    fn default() -> Self {
        Self { focus: false, ruler: true, navigation: false, styles: false, comments: false }
    }
}

impl Layout {
    pub fn capture(app: &WordApp) -> Self {
        let v = &app.session.view;
        Self { focus: v.focus_mode, ruler: v.ruler, navigation: v.nav_pane, styles: v.styles_pane, comments: v.comments_pane }
    }

    pub fn restore(self, app: &mut WordApp) {
        let v = &mut app.session.view;
        v.focus_mode = self.focus;
        v.ruler = self.ruler;
        v.nav_pane = self.navigation;
        v.styles_pane = self.styles;
        v.comments_pane = self.comments;
    }
}

/// `ui.workspace`: no value reads the arrangement; selecting Custom keeps every current choice.
/// All presets leave user-defined toolbars and document content alone.
pub fn command(app: &mut WordApp, params: &Value) -> Result<Value, String> {
    if let Some(value) = params.get("value") {
        let selected =
            value.as_str().and_then(Workspace::from_code).ok_or("`value` must be writing, academic, professional, review, focus or custom")?;
        if selected != Workspace::Custom {
            if selected != Workspace::Focus {
                // Editing arrangements need their ribbon: Read Mode and Outline replace it.
                app.session.run("view.printLayout", &json!({})).map_err(|error| error.to_string())?;
            }
            let layout = match selected {
                Workspace::Writing => Layout { ruler: false, ..Layout::default() },
                Workspace::Academic => Layout { navigation: true, styles: true, ..Layout::default() },
                Workspace::Review => Layout { navigation: true, comments: true, ..Layout::default() },
                Workspace::Focus => Layout { focus: true, ..Layout::capture(app) },
                Workspace::Professional | Workspace::Custom => Layout::default(),
            };
            layout.restore(app);
            if selected != Workspace::Focus {
                app.ui.ribbon_collapsed = selected == Workspace::Writing;
                app.ui.tab = match selected {
                    Workspace::Academic => "References",
                    Workspace::Review => "Review",
                    _ => "Home",
                }
                .into();
            }
        }
        app.ui.workspace = selected;
        app.canvas.want_focus = true;
    }
    Ok(json!({"workspace": app.ui.workspace.code(), "layout": Layout::capture(app), "ribbonCollapsed": app.ui.ribbon_collapsed}))
}

/// Discoverable metadata for UI settings through the same control endpoint as engine commands.
pub fn commands() -> Vec<Value> {
    [
        (
            "ui.accent",
            "Accent colour",
            "File › Options › General",
            r#"{"value"?: "teal"|"blue"|"violet"|"rose"|"amber"}; omitted value reads the setting"#,
        ),
        ("ui.highContrast", "High contrast interface", "File › Options › Accessibility", r#"{"value"?: boolean}; omitted value reads the setting"#),
        (
            "ui.workspace",
            "Workspace",
            "File › Options › General",
            r#"{"value"?: "writing"|"academic"|"professional"|"review"|"focus"|"custom"}; omitted value reads the layout"#,
        ),
        ("ui.theme", "Interface theme", "File › Options › General", r#"{"value"?: "light"|"dark"|"system"}; omitted value reads the setting"#),
    ]
    .into_iter()
    .map(|(id, label, location, params)| json!({"id": id, "label": label, "location": location, "params": params, "shortcut": null, "enabled": true}))
    .collect()
}

pub(crate) struct Action {
    pub label: String,
    pub command: &'static str,
    pub params: Value,
}

/// Ready-to-run palette entries rather than settings commands with unspecified parameters.
pub(crate) fn actions(app: &WordApp, query: &str) -> Vec<Action> {
    if query.trim().is_empty() {
        return Vec::new();
    }
    let mut actions = Vec::new();
    for workspace in Workspace::ALL {
        actions.push(Action {
            label: format!("{} — {}", tl!("Workspace"), tl!(workspace.label())),
            command: "ui.workspace",
            params: json!({"value": workspace.code()}),
        });
    }
    for appearance in crate::theme::Appearance::ALL {
        actions.push(Action {
            label: format!("{} — {}", tl!("Interface theme"), tl!(appearance.label())),
            command: "ui.theme",
            params: json!({"value": appearance.code()}),
        });
    }
    for accent in crate::theme::Accent::ALL {
        actions.push(Action {
            label: format!("{} — {}", tl!("Accent colour"), tl!(accent.label())),
            command: "ui.accent",
            params: json!({"value": accent.code()}),
        });
    }
    actions.push(Action {
        label: format!("{} — {}", tl!("High contrast interface"), tl!(if app.ui.high_contrast { "Off" } else { "On" })),
        command: "ui.highContrast",
        params: json!({"value": !app.ui.high_contrast}),
    });
    let query = crate::i18n::lowercase(query);
    actions.retain(|action| {
        let hay = format!("{} {}", crate::i18n::lowercase(&action.label), action.command);
        query.split_whitespace().all(|word| hay.contains(word))
    });
    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Services, UiState};

    fn app() -> WordApp {
        WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Services::default())
    }

    #[test]
    fn workspace_presets_change_real_visibility_without_editing_the_document() {
        let mut a = app();
        a.run("text.insert", json!({"text": "Keep my words"})).unwrap();
        let rev = a.session.rev();
        for w in Workspace::ALL {
            a.run("ui.workspace", json!({"value": w.code()})).unwrap();
            assert_eq!(a.ui.workspace, w);
            assert_eq!(a.session.rev(), rev, "workspace must not edit content");
            match w {
                Workspace::Writing => assert!(a.ui.ribbon_collapsed && !a.session.view.ruler),
                Workspace::Academic => assert!(a.session.view.nav_pane && a.session.view.styles_pane && a.ui.tab == "References"),
                Workspace::Review => assert!(a.session.view.nav_pane && a.session.view.comments_pane && a.ui.tab == "Review"),
                Workspace::Focus => assert!(a.session.view.focus_mode),
                _ => {}
            }
        }
        let before = Layout::capture(&a);
        assert!(a.run("ui.workspace", json!({"value": ["academic"]})).is_err());
        assert_eq!(Layout::capture(&a), before);
        a.run("view.readMode", json!({})).unwrap();
        a.run("ui.workspace", json!({"value": "academic"})).unwrap();
        assert!(!a.session.view.read_mode, "academic workspace exposes its references ribbon");
        a.run("view.outline", json!({})).unwrap();
        a.run("ui.workspace", json!({"value": "review"})).unwrap();
        assert_eq!(a.session.view.mode, wordcraft_layout::ViewMode::Print);
        assert_eq!(a.ui.tab, "Review");
    }

    #[test]
    fn customised_workspace_visibility_survives_restart_and_bad_saved_fields() {
        let mut a = app();
        a.run("ui.workspace", json!({"value": "academic"})).unwrap();
        a.run("view.stylesPane", json!({"value": false})).unwrap();
        a.run("view.commentsPane", json!({"value": true})).unwrap();
        let saved = serde_json::to_vec(&a.prefs()).unwrap();
        let mut b = app();
        b.apply_prefs(UiState::from_json(&saved).unwrap());
        assert_eq!(Layout::capture(&a), Layout::capture(&b));
        assert_eq!(b.ui.workspace, Workspace::Academic);
        let prefs = UiState::from_json(br#"{"workspace":"future","workspaceLayout":{"ruler":"broken","comments":true},"theme":"dark"}"#).unwrap();
        assert_eq!(prefs.workspace, Workspace::Professional);
        assert!(prefs.workspace_layout.ruler && prefs.workspace_layout.comments);
        assert_eq!(prefs.theme, crate::theme::Appearance::Dark);
    }

    #[test]
    fn focus_hides_and_restores_chrome_without_changing_pane_preferences() {
        use egui_kittest::kittest::Queryable;
        let mut a = app();
        a.run("ui.workspace", json!({"value": "review"})).unwrap();
        let before = Layout::capture(&a);
        let collapsed = a.ui.ribbon_collapsed;
        a.run("view.focus", json!({"value": true})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        assert!(h.query_by_label("Exit Focus").is_some());
        assert!(h.query_by_label("Search document").is_none(), "navigation is hidden");
        assert!(h.state().session.view.nav_pane && h.state().session.view.comments_pane, "visibility preferences remain intact");
        h.get_by_label("Exit Focus").click();
        h.run_steps(4);
        assert_eq!(Layout::capture(h.state()), before);
        assert_eq!(h.state().ui.ribbon_collapsed, collapsed);
        assert!(h.query_by_label("Exit Focus").is_none());
        h.state_mut().run("view.focus", json!({"value": true})).unwrap();
        h.run_steps(4);
        h.event(egui::Event::Key { key: egui::Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() });
        h.run_steps(4);
        assert_eq!(Layout::capture(h.state()), before, "Escape exits Focus");
    }

    #[test]
    fn changing_interface_theme_keeps_document_texture_visible_and_untinted() {
        let a = WordApp::new(wordcraft_engine::Session::new(wordcraft_engine::sample::sample_document()), Services::default());
        let before = serde_json::to_value(&a.session.doc).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(6);
        let texture = h.state().canvas.page_texture(0).unwrap().id();
        for theme in ["dark", "light", "system", "dark"] {
            h.state_mut().run("ui.theme", json!({"value": theme})).unwrap();
            h.run_steps(4);
            let current = h.state().canvas.page_texture(0).unwrap();
            assert_eq!(current.id(), texture, "interface changes reuse the unchanged page raster");
            assert!(current.size().into_iter().all(|size| size > 0), "page texture is allocated");
            let page_is_painted = h.output().shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Mesh(mesh) if mesh.texture_id == texture && mesh.vertices.iter().all(|v| v.color == egui::Color32::WHITE))
            });
            assert!(page_is_painted, "{theme}: page remains in the paint list with unchanged document colours");
            assert!(!h.state().session.view.dark_mode, "interface appearance never switches document colours");
            assert_eq!(serde_json::to_value(&h.state().session.doc).unwrap(), before);
        }
    }

    #[test]
    fn command_palette_applies_customisation_and_metadata_describes_it() {
        use egui_kittest::kittest::Queryable;
        let mut a = app();
        a.dialog = Some(crate::dialogs::Dialog::Commands { query: "accent violet".into(), swallow: false });
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(6);
        h.get_by_label("Accent colour — Violet").click();
        h.run_steps(4);
        assert_eq!(h.state().ui.accent, crate::theme::Accent::Violet);
        assert!(h.state().dialog.is_none());
        let metadata = commands();
        for command in ["ui.accent", "ui.highContrast", "ui.workspace", "ui.theme"] {
            assert!(metadata.iter().any(|entry| entry["id"] == command && entry["enabled"] == true));
        }
    }
}
