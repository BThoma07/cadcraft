//! Italian edition on Windows: the window's own title bar, as in the reference program — the app
//! button, the quick access tools, the drawing's name in the middle, a search box and the window
//! buttons, on the dark chrome colour. The system title bar is turned off (`with_decorations`),
//! so this bar moves, maximizes, minimizes and closes the window, and the window's edges resize it.

use egui::{Color32, CursorIcon, Rect, Sense, Stroke, ViewportCommand, pos2, vec2};

use crate::CadApp;
use crate::icons;
use crate::theme::Tokens;

/// Height of the bar (the reference program's is 27–28 px at 100 %).
pub const HEIGHT: f32 = 28.0;
/// Width of each window button, as Windows draws them.
const WIN_BUTTON_W: f32 = 46.0;
/// Width of the search box.
const SEARCH_W: f32 = 210.0;

pub fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    if app.title_icon_tex.is_none()
        && let Some(img) = app.title_icon.take()
    {
        app.title_icon_tex = Some(ctx.load_texture("cadcraft-title-icon", img, egui::TextureOptions::LINEAR));
    }
    let mut start: Option<&'static str> = None;
    egui::Panel::top("cc_win_title").exact_size(HEIGHT).frame(egui::Frame::NONE.fill(t.chrome_dark)).show(ui, |ui| {
        let r = ui.max_rect();
        let p = ui.painter().clone();
        // The whole bar moves the window; the controls on it are registered after, so they win.
        let drag = ui.interact(r, ui.id().with("win_title_drag"), Sense::CLICK | Sense::DRAG);
        let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if drag.drag_started() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }
        if drag.double_clicked() {
            ctx.send_viewport_cmd(ViewportCommand::Maximized(!maximized));
        }
        // App button: CADCraft's own icon.
        let mut x = r.left() + 6.0;
        let ib = Rect::from_min_size(pos2(x, r.center().y - 10.0), vec2(20.0, 20.0));
        if let Some(tex) = &app.title_icon_tex {
            p.image(tex.id(), ib, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        }
        x += 30.0;
        // Quick access tools.
        for (icon, cmd, label) in crate::ribbon::QUICK {
            let br = Rect::from_min_size(pos2(x, r.center().y - 9.0), vec2(18.0, 18.0));
            let resp = ui.interact(br, ui.id().with(("win_quick", *cmd)), Sense::click());
            if resp.hovered() {
                p.rect_filled(br.expand(1.0), 2.0, t.control_hover.gamma_multiply(0.6));
            }
            icons::paint(&p, br.shrink(1.0), *icon, false);
            icons::describe_control(ui, &resp, label, None, false);
            if resp.on_hover_text(*label).clicked() {
                start = Some(*cmd);
            }
            x += 22.0;
        }
        let qat_end = x + 8.0;
        // Window buttons, right to left: close, maximize/restore, minimize.
        let mut right = r.right();
        for kind in [WinButton::Close, WinButton::Maximize, WinButton::Minimize] {
            let br = Rect::from_min_max(pos2(right - WIN_BUTTON_W, r.top()), pos2(right, r.bottom()));
            let resp = ui.interact(br, ui.id().with(("win_btn", kind as u8)), Sense::click());
            if resp.hovered() {
                let fill = if kind == WinButton::Close { Color32::from_rgb(0xc4, 0x2b, 0x1c) } else { t.control_hover.gamma_multiply(0.7) };
                p.rect_filled(br, 0.0, fill);
            }
            paint_win_button(&p, br.center(), kind, maximized, if resp.hovered() { Color32::WHITE } else { t.text });
            let tip = match kind {
                WinButton::Close => "Chiudi",
                WinButton::Maximize if maximized => "Ripristina",
                WinButton::Maximize => "Ingrandisci",
                WinButton::Minimize => "Riduci a icona",
            };
            icons::describe_control(ui, &resp, tip, None, false);
            if resp.on_hover_text(tip).clicked() {
                match kind {
                    WinButton::Close => ctx.send_viewport_cmd(ViewportCommand::Close),
                    WinButton::Maximize => ctx.send_viewport_cmd(ViewportCommand::Maximized(!maximized)),
                    WinButton::Minimize => ctx.send_viewport_cmd(ViewportCommand::Minimized(true)),
                }
            }
            right -= WIN_BUTTON_W;
        }
        // Search box: a command name starts it (the reference program searches its help there).
        let sr = Rect::from_min_size(pos2(right - 16.0 - SEARCH_W, r.center().y - 10.0), vec2(SEARCH_W, 20.0));
        let search_left = if sr.left() > qat_end + 120.0 {
            p.rect_filled(sr, 2.0, t.chrome);
            p.rect_stroke(sr, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
            icons::paint(&p, Rect::from_center_size(pos2(sr.right() - 11.0, sr.center().y), vec2(13.0, 13.0)), icons::Icon::Search, false);
            let field = Rect::from_min_max(sr.min, pos2(sr.right() - 22.0, sr.bottom()));
            let edit = egui::TextEdit::singleline(&mut app.title_search)
                .hint_text("Digitare parola chiave o frase")
                .font(crate::theme::small())
                .frame(egui::Frame::NONE)
                .margin(egui::Margin::symmetric(6, 2));
            let resp = ui.put(field, edit);
            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let typed = std::mem::take(&mut app.title_search);
                let id = cadcraft_engine::cmd::resolve_alias(typed.trim());
                if !id.is_empty() && (cadcraft_engine::find_command(&id).is_some() || crate::menus::is_ui_command(&id)) {
                    app.start(&id);
                } else if !typed.trim().is_empty() {
                    app.set_status(format!("Nessun comando \"{}\"", typed.trim()));
                }
            }
            sr.left()
        } else {
            right
        };
        // The title in the middle of the bar, between the tools and the search box.
        let title = match app.session.state() {
            Ok(st) if !app.ui.start_tab => format!("CADCraft    {}{}", st.title, if st.title.contains('.') { "" } else { ".dwg" }),
            _ => "CADCraft".to_string(),
        };
        let galley = p.layout_no_wrap(title, crate::theme::body(), t.text_dim);
        let room = Rect::from_min_max(pos2(qat_end, r.top()), pos2(search_left - 8.0, r.bottom()));
        if room.width() > 40.0 {
            let tx = (r.center().x - galley.size().x / 2.0).clamp(room.left(), (room.right() - galley.size().x).max(room.left()));
            p.with_clip_rect(room).galley(pos2(tx, r.center().y - galley.size().y / 2.0), galley, t.text_dim);
        }
    });
    if let Some(cmd) = start {
        app.start(cmd);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WinButton {
    Minimize,
    Maximize,
    Close,
}

fn paint_win_button(p: &egui::Painter, c: egui::Pos2, kind: WinButton, maximized: bool, color: Color32) {
    let s = Stroke::new(1.0, color);
    match kind {
        WinButton::Minimize => {
            p.hline(c.x - 5.0..=c.x + 5.0, c.y + 0.5, s);
        }
        WinButton::Maximize if maximized => {
            p.rect_stroke(Rect::from_min_size(pos2(c.x - 5.0, c.y - 3.0), vec2(8.0, 8.0)), 0.0, s, egui::StrokeKind::Middle);
            p.line(vec![pos2(c.x - 3.0, c.y - 3.0), pos2(c.x - 3.0, c.y - 5.0), pos2(c.x + 5.0, c.y - 5.0), pos2(c.x + 5.0, c.y + 3.0), pos2(c.x + 3.0, c.y + 3.0)], s);
        }
        WinButton::Maximize => {
            p.rect_stroke(Rect::from_center_size(c, vec2(10.0, 10.0)), 0.0, s, egui::StrokeKind::Middle);
        }
        WinButton::Close => {
            p.line_segment([pos2(c.x - 5.0, c.y - 5.0), pos2(c.x + 5.0, c.y + 5.0)], s);
            p.line_segment([pos2(c.x + 5.0, c.y - 5.0), pos2(c.x - 5.0, c.y + 5.0)], s);
        }
    }
}

/// Without the system frame the window's edges resize it: the pointer near an edge shows the
/// resize cursor, and pressing there hands the drag to the system.
pub fn resize_edges(ctx: &egui::Context) {
    use egui::viewport::ResizeDirection as D;
    if ctx.input(|i| i.viewport().maximized.unwrap_or(false) || i.viewport().fullscreen.unwrap_or(false)) {
        return;
    }
    let r = ctx.content_rect();
    let Some(p) = ctx.input(|i| i.pointer.hover_pos()) else { return };
    const M: f32 = 5.0;
    let (l, rt, tp, b) = (p.x - r.left() < M, r.right() - p.x < M, p.y - r.top() < M, r.bottom() - p.y < M);
    let dir = match (l, rt, tp, b) {
        (true, _, true, _) => D::NorthWest,
        (_, true, true, _) => D::NorthEast,
        (true, _, _, true) => D::SouthWest,
        (_, true, _, true) => D::SouthEast,
        (true, _, _, _) => D::West,
        (_, true, _, _) => D::East,
        (_, _, true, _) => D::North,
        (_, _, _, true) => D::South,
        _ => return,
    };
    ctx.set_cursor_icon(match dir {
        D::West | D::East => CursorIcon::ResizeHorizontal,
        D::North | D::South => CursorIcon::ResizeVertical,
        D::NorthWest | D::SouthEast => CursorIcon::ResizeNwSe,
        D::NorthEast | D::SouthWest => CursorIcon::ResizeNeSw,
    });
    if ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}
