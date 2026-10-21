//! Italian edition: how long a frame takes while drawing (LINE with its rubber band following the
//! mouse), with the AutoCAD-style window (ribbon, Properties, status bar, command line). Prints a
//! table of timings per part of the frame, so the slow part can be found; run it in release mode:
//! `cargo test --release -p cadcraft-ui-egui --test velocita_disegno -- --nocapture`.

use std::time::Instant;

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services};
use serde_json::json;

/// Physical size of the reference screen (2560 × 1440).
const SCREEN: egui::Vec2 = egui::vec2(2560.0, 1440.0);
const FRAMES: usize = 90;

fn app(lines: usize) -> CadApp {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", json!({})).unwrap();
    app.ui.start_tab = false;
    app.apply_autocad_edition();
    app.ui.in_window_menu = true;
    let side = (lines as f64).sqrt().ceil().max(1.0) as usize;
    for i in 0..lines {
        let (x, y) = ((i % side) as f64 * 10.0, (i / side) as f64 * 10.0);
        app.session.execute("line", &json!({ "points": [[x, y], [x + 7.0, y + 4.0]] })).unwrap();
    }
    let _ = app.run("zoom.extents", json!({}));
    app
}

#[derive(Default)]
struct Times {
    logic: Vec<f64>,
    ui: Vec<f64>,
    tess: Vec<f64>,
    verts: usize,
    shapes: usize,
}

fn stat(v: &[f64]) -> (f64, f64, f64) {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.total_cmp(b));
    if s.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let med = s[s.len() / 2];
    let p95 = s[(s.len() * 95 / 100).min(s.len() - 1)];
    (med, p95, s[s.len() - 1])
}

/// Run `FRAMES` frames moving the pointer across the drawing area; `setup` runs after the first
/// frame (start a command, click its first point).
fn measure(app: &mut CadApp, setup: impl Fn(&mut CadApp)) -> Times {
    let ctx = egui::Context::default();
    let screen = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN));
    let mut t = Times::default();
    let mut time = 0.0;
    let mut frame = |app: &mut CadApp, events: Vec<egui::Event>, t: Option<&mut Times>| {
        time += 1.0 / 120.0;
        let t0 = Instant::now();
        app.logic(&ctx);
        let t1 = Instant::now();
        let input = egui::RawInput { screen_rect: screen, events, time: Some(time), focused: true, ..Default::default() };
        let mut out = ctx.run_ui(input, |ui| app.ui(ui));
        let t2 = Instant::now();
        let shapes = out.shapes.len();
        let prims = ctx.tessellate(std::mem::take(&mut out.shapes), out.pixels_per_point);
        let t3 = Instant::now();
        out.textures_delta.clear();
        if let Some(t) = t {
            t.logic.push((t1 - t0).as_secs_f64() * 1000.0);
            t.ui.push((t2 - t1).as_secs_f64() * 1000.0);
            t.tess.push((t3 - t2).as_secs_f64() * 1000.0);
            t.shapes = shapes;
            t.verts = prims
                .iter()
                .map(|p| match &p.primitive {
                    egui::epaint::Primitive::Mesh(m) => m.vertices.len(),
                    _ => 0,
                })
                .sum();
        }
    };
    for _ in 0..3 {
        frame(app, Vec::new(), None);
    }
    setup(app);
    // The drawing area is roughly the middle of the window.
    let (x0, y0) = (SCREEN.x * 0.30, SCREEN.y * 0.35);
    for i in 0..FRAMES {
        let f = i as f32 / FRAMES as f32;
        let p = egui::pos2(x0 + f * SCREEN.x * 0.25, y0 + (f * 6.0).sin() * 60.0 + 100.0);
        frame(app, vec![egui::Event::PointerMoved(p)], Some(&mut t));
    }
    t
}

fn line_started(app: &mut CadApp) {
    app.session.cmdline("line").unwrap();
    app.session.cmdline("1,1").unwrap();
}

fn report(name: &str, t: &Times) -> f64 {
    let (lm, _, _) = stat(&t.logic);
    let (um, up, ux) = stat(&t.ui);
    let (tm, tp, _) = stat(&t.tess);
    println!(
        "{name:<42} logic {lm:6.2} | ui {um:6.2} (p95 {up:6.2}, max {ux:6.2}) | tessellate {tm:6.2} (p95 {tp:6.2}) | total {:6.2} ms | {} shapes, {} vertices",
        lm + um + tm,
        t.shapes,
        t.verts
    );
    lm + um + tm
}

#[test]
fn tempo_per_fotogramma_mentre_si_disegna() {
    println!("\n=== Tempo per fotogramma (mediana, ms), finestra {}x{} ===", SCREEN.x, SCREEN.y);
    let mut a = app(0);
    report("vuoto, nessun comando", &measure(&mut a, |_| {}));
    let mut a = app(0);
    let base = report("vuoto, LINEA attiva", &measure(&mut a, line_started));
    let mut a = app(2000);
    report("2000 linee, LINEA attiva", &measure(&mut a, line_started));
    let mut a = app(2000);
    a.session.settings.osmode = 0;
    report("2000 linee, LINEA, snap spenti", &measure(&mut a, line_started));
    // What each part of the window costs: the same frame without it.
    let parts: [(&str, fn(&mut CadApp)); 6] = [
        ("  senza Ribbon", |a| a.ui.show_ribbon = false),
        ("  senza palette (Proprieta')", |a| a.ui.show_palettes = false),
        ("  senza barra di stato", |a| a.ui.show_status_bar = false),
        ("  senza riga di comando", |a| a.ui.show_command_line = false),
        ("  senza ViewCube e UCS", |a| {
            a.ui.show_viewcube = false;
            a.ui.show_ucs_icon = false;
        }),
        ("  senza schede dei file", |a| a.ui.show_file_tabs = false),
    ];
    for (name, off) in parts {
        let mut a = app(0);
        off(&mut a);
        let t = report(name, &measure(&mut a, line_started));
        println!("{:<42} risparmio {:6.2} ms", "", base - t);
    }
}

#[test]
fn graphicsconfig_mostra_scheda_e_tempi() {
    let mut a = app(10);
    measure(&mut a, line_started);
    let r = a.run("graphicsconfig", json!({})).unwrap();
    let lines = r["lines"].as_array().map(|l| l.len()).unwrap_or(0);
    assert!(lines >= 3, "{r}");
    assert!(r["fps"].as_f64().unwrap_or(0.0) >= 0.0);
}
