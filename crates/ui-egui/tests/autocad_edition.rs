//! The Italian AutoCAD-style edition: the window laid out as AutoCAD 2025 for Windows (menu bar,
//! ribbon, Properties on the right, black model space), the Italian command names and aliases.
//! With CRAFT_UI_VISUAL_DIR set, the window is also saved as PNGs to look at.

use cadcraft_engine::Session;
use cadcraft_engine::cmd::resolve_alias;
use cadcraft_ui_egui::{CadApp, Services};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

fn harness(tab: &str) -> Harness<'static, CadApp> {
    harness_sized(tab, vec2(1600.0, 900.0))
}

/// The window at `size` points (1 pixel per point).
fn harness_sized(tab: &str, size: egui::Vec2) -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", serde_json::json!({})).unwrap();
    app.ui.start_tab = false;
    app.apply_autocad_edition();
    app.ui.in_window_menu = true;
    app.windows_titlebar = true;
    app.ui.ribbon_tab = tab.into();
    for c in ["rectang 0,0 400,250", "circle 200,125 80", "line 0,0 400,250 ", "pline 50,50 350,50 350,200 "] {
        let _ = app.session.cmdline(c);
    }
    let _ = app.run("zoom.extents", serde_json::json!({}));
    let mut h = Harness::builder().with_size(size).with_pixels_per_point(1.0).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(6);
    h
}

#[test]
fn la_finestra_e_come_autocad() {
    for tab in ["home", "insert", "annotate", "parametric", "view", "manage", "output"] {
        let mut h = harness(tab);
        if let Ok(dir) = std::env::var("CRAFT_UI_VISUAL_DIR") {
            std::fs::create_dir_all(&dir).unwrap();
            h.render().unwrap().save(format!("{dir}/autocad-it-{tab}.png")).unwrap();
        }
        // A label can be on a tab and a button both (Inserisci): any match will do.
        for label in ["Inizio", "Inserisci", "Annota", "Output"] {
            assert!(h.query_all_by_label(label).next().is_some(), "{tab}: no ribbon tab {label}");
        }
    }
    // The Properties palette with a line, then a circle, selected.
    for (kind, name) in [("LINE", "linea"), ("CIRCLE", "cerchio")] {
        let mut h = harness("home");
        let handle = {
            let app = h.state();
            let d = app.session.doc().unwrap();
            d.model.iter().find(|e| e.kind.type_name().eq_ignore_ascii_case(kind)).map(|e| e.handle.hex())
        };
        if let Some(handle) = handle {
            h.state_mut().run("select", serde_json::json!({ "handles": [handle] })).unwrap();
            h.run_steps(4);
            if let Ok(dir) = std::env::var("CRAFT_UI_VISUAL_DIR") {
                h.render().unwrap().save(format!("{dir}/autocad-it-proprieta-{name}.png")).unwrap();
            }
        }
    }
    // The whole window at the size of the reference screen (2560 x 1440), to set beside it.
    if let Ok(dir) = std::env::var("CRAFT_UI_VISUAL_DIR") {
        let mut h = harness_sized("home", vec2(2560.0, 1440.0));
        h.render().unwrap().save(format!("{dir}/autocad-it-schermo-2560.png")).unwrap();
    }
    let h = harness("home");
    for label in ["Linea", "Polilinea", "Cerchio", "Sposta", "Taglia", "Testo", "Quota", "Inserisci"] {
        assert!(h.query_all_by_label(label).next().is_some(), "no ribbon button {label}");
    }
}

#[test]
fn comandi_italiani_e_alias() {
    for (typed, id) in [("linea", "line"), ("SPOSTA", "move"), ("taglia", "trim"), ("cerchio", "circle"), ("tt", "trim"), ("2", "line"), ("rr", "hatch"), ("u", "join"), ("c", "copy"), ("e", "extend")] {
        assert_eq!(resolve_alias(typed), id, "{typed}");
    }
    let mut s = Session::new();
    s.cmdline("cerchio 0,0 5").unwrap();
    s.cmdline("_circle 10,0 5").unwrap();
    assert_eq!(s.doc().unwrap().model.iter().count(), 2);
}

/// The Properties palette of the remaining object types (point, ray, xline, spline, dimension
/// with Alternate Units and Tolerances, multileader, table, region): each one shows without
/// panicking, and with CRAFT_UI_VISUAL_DIR set is saved to look at.
#[test]
fn proprieta_di_tutti_i_tipi() {
    use serde_json::json;
    let make: [(&str, &str, serde_json::Value); 8] = [
        ("punto", "point", json!({ "at": [10, 10] })),
        ("raggio", "ray", json!({ "base": [0, 0], "through": [10, 5] })),
        ("xlinea", "xline", json!({ "base": [0, 0], "angle": 30 })),
        ("spline", "spline", json!({ "fit": [[0, 0], [50, 40], [100, 0], [150, 30]] })),
        ("quota", "dimlinear", json!({ "p1": [0, 0], "p2": [100, 0], "at": [50, -20] })),
        ("multidirettrice", "mleader", json!({ "points": [[0, 0], [40, 30]], "text": "Nota" })),
        ("tabella", "table", json!({ "at": [0, 0], "rows": 3, "cols": 3 })),
        ("regione", "rectang", json!({ "p1": [0, 0], "p2": [40, 20] })),
    ];
    for (name, cmd, params) in make {
        let mut h = harness("home");
        let Ok(r) = h.state_mut().run(cmd, params) else { continue };
        let mut handle = r["handle"].as_str().or_else(|| r["handles"][0].as_str()).map(str::to_string);
        if name == "regione" {
            if let Some(hd) = &handle {
                let _ = h.state_mut().run("region", json!({ "handles": [hd] }));
            }
        }
        if handle.is_none() {
            handle = h.state().session.doc().ok().and_then(|d| d.model.last().map(|e| e.handle.hex()));
        }
        let Some(handle) = handle else { continue };
        h.state_mut().run("select", json!({ "handles": [handle] })).unwrap();
        h.run_steps(4);
        if let Ok(dir) = std::env::var("CRAFT_UI_VISUAL_DIR") {
            std::fs::create_dir_all(&dir).unwrap();
            h.render().unwrap().save(format!("{dir}/autocad-it-proprieta-{name}.png")).unwrap();
        }
    }
}
