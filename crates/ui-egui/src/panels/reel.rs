//! @Film's Reel panel: the few big actions a short vertical video needs, each an existing command
//! (the Reel workspace puts it beside the Program monitor and the Timeline).

use egui::{Align2, Color32, Rect, Sense, pos2, vec2};
use serde_json::{Value, json};

use crate::FilmcraftApp;
use crate::theme::Tokens;

/// (automation id, label, command, params) for every button, top to bottom.
pub fn actions() -> Vec<(&'static str, &'static str, &'static str, Value)> {
    vec![
        ("new", "New Reel", "file.newSequence", json!({"name": "Reel", "width": 1080, "height": 1920, "fps": 30})),
        ("import", "Import Media…", "file.import", json!({})),
        ("split", "Split", "sequence.addEdit", json!({})),
        ("delete", "Delete", "edit.rippleDelete", json!({})),
        ("speed", "Speed", "clip.speedDuration", json!({})),
        ("text", "Text", "tool.type", json!({})),
        ("captions", "Captions", "sequence.transcribe", json!({})),
        ("export", "Export", "mode.export", json!({})),
    ]
}

pub fn show(app: &mut FilmcraftApp, ui: &mut egui::Ui, rect: Rect) {
    let t = app.tokens;
    let ctx = ui.ctx().clone();
    let gap = 8.0;
    let h = 40.0;
    let mut y = rect.min.y + 12.0;
    for (i, (id, label, cmd, params)) in actions().into_iter().enumerate() {
        if y + h > rect.max.y {
            break;
        }
        let r = Rect::from_min_size(pos2(rect.min.x + 12.0, y), vec2((rect.width() - 24.0).max(40.0), h));
        let resp = ui.interact(r, egui::Id::unique(("reel", id)), Sense::click());
        let label = crate::i18n::t(label);
        app.auto.add(&format!("reel.{id}"), r, label);
        let primary = i == 0;
        let (bg, fg) = if primary {
            (if resp.hovered() { t.accent_hover } else { t.accent }, Color32::WHITE)
        } else {
            (if resp.hovered() { t.hover } else { t.field_bg }, t.text)
        };
        ui.painter().rect_filled(r, 10.0, bg);
        ui.painter().text(pos2(r.min.x + 14.0, r.center().y), Align2::LEFT_CENTER, label, Tokens::semibold(14.0), fg);
        if resp.clicked()
            && let Err(e) = crate::menus::invoke(app, &ctx, cmd, params)
        {
            app.ui.status = e;
        }
        y += h + gap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dock::{self, LayoutExt, PanelKind};

    #[test]
    fn every_reel_button_runs_a_known_command() {
        let app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
        let mut menu: Vec<String> = crate::menus::menu_items(&app).into_iter().map(|m| m.id).collect();
        menu.extend(crate::menus::UI_COMMANDS.iter().map(|c| c.id.to_string()));
        let engine: Vec<String> = filmcraft_engine::command_specs().iter().map(|c| c.id.to_string()).collect();
        for (_, label, cmd, _) in actions() {
            assert!(menu.iter().chain(&engine).any(|id| id == cmd), "{label}: `{cmd}` is not a command");
        }
    }

    #[test]
    fn new_reel_is_a_vertical_1080_by_1920_sequence_and_the_workspace_shows_it() {
        let mut app = crate::FilmcraftApp::new(filmcraft_engine::Session::default());
        let ctx = egui::Context::default();
        let (_, _, cmd, params) = actions().remove(0);
        let r = crate::menus::invoke(&mut app, &ctx, cmd, params).unwrap();
        let seq = app.session.state.active_sequence.expect("a sequence");
        let s = app.session.project.sequence(seq).expect("sequence").settings.clone();
        assert_eq!((s.width, s.height), (1080, 1920), "{r}");
        let layout = dock::workspace("Reel");
        layout.validate().unwrap();
        assert!(layout.is_visible(PanelKind::Reel) && layout.is_visible(PanelKind::Program) && layout.is_visible(PanelKind::Timeline));
    }
}
