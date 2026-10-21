//! CADCraft's icon set, drawn in code (original work, MIT OR Apache-2.0 with the codebase).
//!
//! Icons are line drawings on a 24×24 design grid: light strokes for existing geometry, blue for
//! the geometry a command creates and orange dots for picked points.

use craft_ui::buttons::{ButtonVisuals, IconButton, IconButtonStyle};
use egui::{Color32, Painter, Pos2, Rect, Stroke, pos2};

use crate::theme::Tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    New,
    Open,
    Save,
    SaveAs,
    Undo,
    Redo,
    Plot,
    Publish,
    PageSetup,
    Preview,
    Import,
    Export,
    Attach,
    Share,
    ZoomWindow,
    Pan,
    Orbit,
    ZoomExtents,
    Properties,
    Blocks,
    Layers,
    Help,
    Line,
    Polyline,
    Circle,
    Arc,
    Rectangle,
    Polygon,
    Ellipse,
    Spline,
    Point,
    XLine,
    Ray,
    Donut,
    RevCloud,
    Hatch,
    Gradient,
    Boundary,
    Region,
    Wipeout,
    Table,
    Text,
    MText,
    Move,
    Copy,
    Rotate,
    Scale,
    Mirror,
    Offset,
    Trim,
    Extend,
    Fillet,
    Chamfer,
    Explode,
    Erase,
    Stretch,
    ArrayRect,
    ArrayPolar,
    Break,
    Join,
    Lengthen,
    MatchProp,
    DimLinear,
    DimAligned,
    DimRadius,
    DimDiameter,
    DimAngular,
    DimArc,
    DimOrdinate,
    DimBaseline,
    DimContinue,
    QDim,
    MLeader,
    AddLeader,
    RemoveLeader,
    AlignLeaders,
    BlockCreate,
    Insert,
    BlockEdit,
    AttDef,
    LayerProps,
    LayerOff,
    LayerIso,
    LayerFreeze,
    LayerLock,
    LayerMatch,
    MakeCurrent,
    LayerPrev,
    Bulb,
    BulbOff,
    Lock,
    Unlock,
    Sun,
    Snowflake,
    Grid,
    Snap,
    Ortho,
    Polar,
    Osnap,
    OTrack,
    Lineweight,
    Transparency,
    Isodraft,
    DynInput,
    QuickProps,
    Gear,
    Annotation,
    Workspace,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
    Plus,
    Switcher,
    Close,
    Search,
    Menu,
    Constraint,
    Coincident,
    Parallel,
    Perpendicular,
    Horizontal,
    Vertical,
    Tangent,
    Concentric,
    Equal,
    Fix,
    Collinear,
    Smooth,
    Symmetric,
    DelConstraint,
    Parameters,
    Measure,
    Distance,
    Area,
    List,
    PickAdd,
    SelectObjects,
    QuickSelect,
    Paste,
}

struct Pen<'a> {
    p: &'a Painter,
    r: Rect,
    w: f32,
    base: Color32,
    acc: Color32,
    pt: Color32,
}

impl Pen<'_> {
    fn at(&self, x: f32, y: f32) -> Pos2 {
        let s = self.r.width() / 24.0;
        pos2(self.r.min.x + x * s, self.r.min.y + y * s)
    }
    fn sc(&self) -> f32 {
        self.r.width() / 24.0
    }
    fn st(&self, c: Color32) -> Stroke {
        Stroke::new(self.w, c)
    }
    fn l(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.p.line_segment([self.at(x1, y1), self.at(x2, y2)], self.st(self.base));
    }
    fn la(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.p.line_segment([self.at(x1, y1), self.at(x2, y2)], self.st(self.acc));
    }
    fn poly(&self, pts: &[(f32, f32)], closed: bool, c: Color32) {
        let mut v: Vec<Pos2> = pts.iter().map(|(x, y)| self.at(*x, *y)).collect();
        if closed && let Some(f) = v.first().copied() {
            v.push(f);
        }
        self.p.line(v, self.st(c));
    }
    fn c(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.p.circle_stroke(self.at(x, y), r * self.sc(), self.st(c));
    }
    fn fill_c(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.p.circle_filled(self.at(x, y), r * self.sc(), c);
    }
    /// Arc in degrees, counter-clockwise on screen (y up visually).
    fn arc(&self, x: f32, y: f32, r: f32, a0: f32, a1: f32, c: Color32) {
        let n = (((a1 - a0).abs() / 10.0).ceil() as usize).max(2);
        let pts: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = (a0 + (a1 - a0) * i as f32 / n as f32).to_radians();
                self.at(x + r * a.cos(), y - r * a.sin())
            })
            .collect();
        self.p.line(pts, self.st(c));
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.poly(&[(x, y), (x + w, y), (x + w, y + h), (x, y + h)], true, c);
    }
    fn fill_rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.p.rect_filled(Rect::from_min_max(self.at(x, y), self.at(x + w, y + h)), 0.0, c);
    }
    fn dot(&self, x: f32, y: f32) {
        self.fill_c(x, y, 1.6, self.pt);
    }
    fn tri(&self, pts: [(f32, f32); 3], c: Color32) {
        self.p.add(egui::Shape::convex_polygon(pts.iter().map(|(x, y)| self.at(*x, *y)).collect(), c, Stroke::NONE));
    }
    /// A filled convex polygon with a thin outline.
    fn fill_poly(&self, pts: &[(f32, f32)], fill: Color32, outline: Color32) {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| self.at(*x, *y)).collect();
        self.p.add(egui::Shape::convex_polygon(v, fill, Stroke::new((self.w * 0.7).max(0.8), outline)));
    }
    /// A small square grip, as the large drawing icons mark their vertices.
    fn grip(&self, x: f32, y: f32, c: Color32) {
        let h = 1.5 * self.sc();
        let at = self.at(x, y);
        self.p.rect_filled(Rect::from_center_size(at, egui::vec2(2.0 * h, 2.0 * h)), 0.0, c);
    }
    fn arrow_tip(&self, x: f32, y: f32, dx: f32, dy: f32, c: Color32) {
        let l = (dx * dx + dy * dy).sqrt().max(1e-3);
        let (ux, uy) = (dx / l, dy / l);
        let (nx, ny) = (-uy, ux);
        self.tri([(x, y), (x - ux * 3.0 + nx * 1.4, y - uy * 3.0 + ny * 1.4), (x - ux * 3.0 - nx * 1.4, y - uy * 3.0 - ny * 1.4)], c);
    }
    fn text(&self, x: f32, y: f32, s: &str, size: f32, c: Color32) {
        self.p.text(self.at(x, y), egui::Align2::CENTER_CENTER, s, egui::FontId::proportional(size * self.sc()), c);
    }
}

/// Paint `icon` into `rect` (square), with `dim` for disabled.
pub fn paint(p: &Painter, rect: Rect, icon: Icon, dim: bool) {
    let t = Tokens::get();
    let alpha = |c: Color32| if dim { c.gamma_multiply(0.4) } else { c };
    // The colours of the large ribbon icons (original drawings): pale lines, blue grips and
    // faces, yellow measuring tools, a white sheet.
    let pale = alpha(Color32::from_rgb(0xdc, 0xe1, 0xe8));
    let blue = alpha(Color32::from_rgb(0x3d, 0x9b, 0xf0));
    let blue_dark = alpha(Color32::from_rgb(0x23, 0x5f, 0xa8));
    let blue_light = alpha(Color32::from_rgb(0x9c, 0xcd, 0xf7));
    let yellow = alpha(Color32::from_rgb(0xec, 0xb8, 0x32));
    let yellow_dark = alpha(Color32::from_rgb(0x9a, 0x70, 0x12));
    let sheet = alpha(Color32::from_rgb(0xf2, 0xf4, 0xf7));
    let outline = alpha(Color32::from_rgb(0x1c, 0x22, 0x2b));
    let pen = Pen { p, r: rect, w: (rect.width() / 16.0).clamp(1.0, 1.6), base: alpha(t.icon), acc: alpha(t.icon_accent), pt: alpha(t.icon_point) };
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        New => {
            pen.poly(&[(6.0, 3.0), (15.0, 3.0), (19.0, 7.0), (19.0, 21.0), (6.0, 21.0)], true, b);
            pen.poly(&[(15.0, 3.0), (15.0, 7.0), (19.0, 7.0)], false, b);
        }
        Open => {
            pen.poly(&[(3.0, 6.0), (9.0, 6.0), (11.0, 8.0), (20.0, 8.0), (20.0, 19.0), (3.0, 19.0)], true, b);
            pen.poly(&[(3.0, 19.0), (6.0, 11.0), (22.0, 11.0), (20.0, 19.0)], false, b);
        }
        Save | SaveAs => {
            pen.poly(&[(4.0, 4.0), (17.0, 4.0), (20.0, 7.0), (20.0, 20.0), (4.0, 20.0)], true, b);
            pen.rect(8.0, 4.0, 8.0, 5.0, b);
            pen.rect(7.0, 13.0, 10.0, 7.0, b);
            if icon == SaveAs {
                pen.la(15.0, 22.0, 22.0, 15.0);
                pen.fill_c(15.0, 22.0, 1.2, a);
            }
        }
        Undo => {
            pen.arc(12.0, 13.0, 7.0, 150.0, -40.0, b);
            pen.arrow_tip(5.0, 9.0, -1.0, 2.5, b);
        }
        Redo => {
            pen.arc(12.0, 13.0, 7.0, 30.0, 220.0, b);
            pen.arrow_tip(19.0, 9.0, 1.0, 2.5, b);
        }
        Plot | Preview => {
            pen.rect(4.0, 9.0, 16.0, 8.0, b);
            pen.rect(7.0, 4.0, 10.0, 5.0, b);
            pen.rect(7.0, 14.0, 10.0, 7.0, b);
            if icon == Preview {
                pen.c(17.0, 17.0, 3.0, a);
                pen.la(19.0, 19.0, 22.0, 22.0);
            }
        }
        Publish => {
            pen.rect(3.0, 10.0, 13.0, 7.0, b);
            pen.rect(6.0, 6.0, 7.0, 4.0, b);
            pen.rect(9.0, 3.0, 12.0, 9.0, a);
        }
        PageSetup => {
            pen.rect(5.0, 3.0, 12.0, 17.0, b);
            pen.c(17.0, 17.0, 3.0, a);
            pen.fill_c(17.0, 17.0, 1.0, a);
        }
        Import | Export => {
            pen.poly(&[(5.0, 3.0), (14.0, 3.0), (18.0, 7.0), (18.0, 21.0), (5.0, 21.0)], true, b);
            if icon == Import {
                pen.la(1.0, 13.0, 11.0, 13.0);
                pen.arrow_tip(12.0, 13.0, 1.0, 0.0, a);
            } else {
                pen.la(11.0, 13.0, 21.0, 13.0);
                pen.arrow_tip(23.0, 13.0, 1.0, 0.0, a);
            }
        }
        Attach => {
            pen.rect(4.0, 5.0, 16.0, 14.0, b);
            pen.poly(&[(6.0, 17.0), (10.0, 11.0), (13.0, 15.0), (15.0, 13.0), (18.0, 17.0)], false, a);
            pen.c(15.0, 9.0, 1.5, a);
        }
        Share => {
            pen.poly(&[(4.0, 12.0), (20.0, 4.0), (14.0, 20.0), (11.0, 13.0)], true, a);
            pen.l(11.0, 13.0, 20.0, 4.0);
        }
        ZoomWindow => {
            pen.rect(3.0, 3.0, 12.0, 12.0, b);
            pen.c(14.0, 14.0, 4.5, a);
            pen.la(17.5, 17.5, 21.0, 21.0);
        }
        ZoomExtents => {
            pen.c(10.0, 10.0, 6.0, b);
            pen.l(14.5, 14.5, 20.0, 20.0);
            pen.la(7.0, 10.0, 13.0, 10.0);
            pen.la(10.0, 7.0, 10.0, 13.0);
        }
        Pan => {
            pen.poly(
                &[
                    (8.0, 20.0),
                    (5.0, 13.0),
                    (6.0, 11.0),
                    (8.0, 13.0),
                    (8.0, 5.0),
                    (10.0, 4.0),
                    (11.0, 11.0),
                    (11.0, 3.0),
                    (13.0, 3.0),
                    (14.0, 11.0),
                    (14.0, 4.0),
                    (16.0, 4.0),
                    (17.0, 12.0),
                    (17.0, 7.0),
                    (19.0, 7.0),
                    (19.0, 15.0),
                    (16.0, 20.0),
                ],
                true,
                b,
            );
        }
        Orbit => {
            pen.c(12.0, 12.0, 4.0, b);
            pen.arc(12.0, 12.0, 9.0, 20.0, 160.0, a);
            pen.arc(12.0, 12.0, 9.0, 200.0, 340.0, a);
            pen.arrow_tip(3.5, 9.5, -0.4, 1.0, a);
        }
        Properties => {
            pen.rect(4.0, 3.0, 16.0, 18.0, b);
            for y in [8.0, 12.0, 16.0] {
                pen.l(7.0, y, 10.0, y);
                pen.la(12.0, y, 17.0, y);
            }
        }
        Blocks => {
            pen.poly(&[(12.0, 3.0), (20.0, 7.0), (12.0, 11.0), (4.0, 7.0)], true, b);
            pen.poly(&[(4.0, 7.0), (4.0, 16.0), (12.0, 20.0), (20.0, 16.0), (20.0, 7.0)], false, b);
            pen.l(12.0, 11.0, 12.0, 20.0);
        }
        Layers => {
            for (k, y) in [5.0f32, 10.0, 15.0].iter().enumerate() {
                let c = if k == 0 { a } else { b };
                pen.poly(&[(3.0, y + 2.0), (12.0, *y - 2.0), (21.0, y + 2.0), (12.0, y + 6.0)], true, c);
            }
        }
        LayerProps => {
            // Three filled sheets, bottom to top: yellow, blue, white.
            for (y, fill) in [(15.0f32, yellow), (10.0, blue_light), (5.0, sheet)] {
                pen.fill_poly(&[(2.0, y + 2.5), (12.0, y - 2.0), (22.0, y + 2.5), (12.0, y + 7.0)], fill, outline);
            }
        }
        Help => {
            pen.c(12.0, 12.0, 9.0, b);
            pen.text(12.0, 12.5, "?", 12.0, b);
        }
        Line => {
            pen.poly(&[(4.0, 20.0), (20.0, 4.0)], false, pale);
            pen.grip(4.0, 20.0, blue);
            pen.grip(20.0, 4.0, blue);
        }
        Polyline => {
            pen.poly(&[(3.0, 19.0), (3.0, 9.0), (13.0, 9.0)], false, pale);
            pen.arc(13.0, 14.0, 5.0, 90.0, -90.0, pale);
            pen.poly(&[(13.0, 19.0), (8.0, 19.0)], false, pale);
            pen.grip(3.0, 19.0, blue);
            pen.grip(3.0, 9.0, blue);
            pen.grip(13.0, 9.0, blue);
            pen.grip(8.0, 19.0, blue);
        }
        Circle => {
            pen.c(12.0, 12.0, 8.5, pale);
            pen.poly(&[(12.0, 12.0), (18.0, 6.0)], false, blue);
            pen.grip(12.0, 12.0, blue);
        }
        Arc => {
            pen.arc(12.0, 17.0, 9.0, 10.0, 170.0, pale);
            pen.grip(3.1, 15.4, blue);
            pen.grip(12.0, 8.0, blue);
            pen.grip(20.9, 15.4, blue);
        }
        Rectangle => {
            pen.rect(3.0, 6.0, 18.0, 12.0, a);
            pen.dot(3.0, 18.0);
            pen.dot(21.0, 6.0);
        }
        Polygon => {
            let pts: Vec<(f32, f32)> = (0..6)
                .map(|i| ((12.0 + 8.5 * (i as f32 * 60.0f32).to_radians().cos()), 12.0 + 8.5 * (i as f32 * 60.0f32).to_radians().sin()))
                .collect();
            pen.poly(&pts, true, a);
            pen.dot(12.0, 12.0);
        }
        Ellipse => {
            let pts: Vec<(f32, f32)> = (0..36)
                .map(|i| (12.0 + 9.0 * (i as f32 * 10.0f32).to_radians().cos(), 12.0 + 5.0 * (i as f32 * 10.0f32).to_radians().sin()))
                .collect();
            pen.poly(&pts, true, a);
            pen.dot(3.0, 12.0);
            pen.dot(21.0, 12.0);
        }
        Spline => {
            let pts: Vec<(f32, f32)> =
                (0..=20).map(|i| (3.0 + 18.0 * i as f32 / 20.0, 12.0 - 6.0 * (i as f32 / 20.0 * std::f32::consts::TAU).sin())).collect();
            pen.poly(&pts, false, a);
            pen.dot(3.0, 12.0);
            pen.dot(7.5, 6.0);
            pen.dot(16.5, 18.0);
            pen.dot(21.0, 12.0);
        }
        Point => {
            pen.dot(12.0, 12.0);
            pen.l(12.0, 5.0, 12.0, 9.0);
            pen.l(12.0, 15.0, 12.0, 19.0);
            pen.l(5.0, 12.0, 9.0, 12.0);
            pen.l(15.0, 12.0, 19.0, 12.0);
        }
        XLine => {
            pen.la(1.0, 22.0, 23.0, 2.0);
            pen.dot(12.0, 12.0);
        }
        Ray => {
            pen.la(5.0, 19.0, 23.0, 3.0);
            pen.dot(5.0, 19.0);
        }
        Donut => {
            pen.c(12.0, 12.0, 8.0, a);
            pen.c(12.0, 12.0, 4.5, a);
            pen.fill_c(12.0, 12.0, 1.0, b);
        }
        RevCloud => {
            for i in 0..6 {
                let ang = i as f32 * 60.0;
                let x = 12.0 + 6.0 * ang.to_radians().cos();
                let y = 12.0 + 6.0 * ang.to_radians().sin();
                pen.arc(x, y, 3.2, -ang - 70.0, -ang + 110.0, a);
            }
        }
        Hatch => {
            pen.rect(3.0, 3.0, 18.0, 18.0, b);
            for k in 0..5 {
                let o = k as f32 * 4.5;
                pen.la((3.0 + o).min(21.0), 21.0, 21.0, (21.0 - (18.0 - o)).max(3.0));
                pen.la(3.0, 21.0 - o, (3.0 + o).min(21.0), 21.0 - o - o.min(18.0) + o);
            }
        }
        Gradient => {
            for i in 0..9 {
                let c = Color32::from_rgb(40 + i * 22, 80 + i * 16, 160 + i * 9);
                pen.fill_rect(3.0 + i as f32 * 2.0, 3.0, 2.0, 18.0, if dim { c.gamma_multiply(0.4) } else { c });
            }
            pen.rect(3.0, 3.0, 18.0, 18.0, b);
        }
        Boundary => {
            pen.rect(4.0, 4.0, 16.0, 16.0, b);
            pen.c(12.0, 12.0, 6.0, b);
            pen.poly(&[(4.0, 4.0), (20.0, 4.0), (20.0, 20.0), (4.0, 20.0)], true, a);
            pen.dot(6.0, 6.0);
        }
        Region => {
            pen.poly(&[(4.0, 18.0), (6.0, 6.0), (16.0, 4.0), (20.0, 14.0), (12.0, 20.0)], true, a);
            pen.fill_c(12.0, 12.0, 3.0, a.gamma_multiply(0.5));
        }
        Wipeout => {
            pen.rect(3.0, 3.0, 13.0, 13.0, b);
            pen.fill_rect(8.0, 8.0, 13.0, 13.0, Tokens::get().chrome);
            pen.rect(8.0, 8.0, 13.0, 13.0, a);
        }
        Table => {
            pen.rect(3.0, 4.0, 18.0, 16.0, b);
            pen.fill_rect(3.0, 4.0, 18.0, 4.0, a.gamma_multiply(0.6));
            for y in [8.0, 12.0, 16.0] {
                pen.l(3.0, y, 21.0, y);
            }
            pen.l(9.0, 8.0, 9.0, 20.0);
            pen.l(15.0, 8.0, 15.0, 20.0);
        }
        Text => pen.text(12.0, 12.5, "A", 19.0, b),
        MText => {
            // A bold white capital (drawn twice, a hair apart) and blue lines of text.
            pen.text(9.0, 11.0, "A", 17.0, sheet);
            pen.text(9.5, 11.0, "A", 17.0, sheet);
            for (y, x1) in [(15.0, 22.0), (18.5, 22.0), (22.0, 19.0)] {
                pen.poly(&[(15.0, y), (x1, y)], false, blue);
            }
        }
        Move => {
            pen.la(12.0, 3.0, 12.0, 21.0);
            pen.la(3.0, 12.0, 21.0, 12.0);
            pen.arrow_tip(12.0, 2.0, 0.0, -1.0, a);
            pen.arrow_tip(12.0, 22.0, 0.0, 1.0, a);
            pen.arrow_tip(2.0, 12.0, -1.0, 0.0, a);
            pen.arrow_tip(22.0, 12.0, 1.0, 0.0, a);
        }
        Copy => {
            pen.rect(3.0, 3.0, 11.0, 13.0, b);
            pen.rect(10.0, 8.0, 11.0, 13.0, a);
        }
        Rotate => {
            pen.arc(12.0, 12.0, 8.0, 100.0, 380.0, a);
            pen.arrow_tip(10.6, 4.1, -1.0, 0.1, a);
            pen.dot(12.0, 12.0);
        }
        Scale => {
            pen.rect(3.0, 11.0, 10.0, 10.0, b);
            pen.rect(3.0, 3.0, 18.0, 18.0, a);
            pen.la(9.0, 15.0, 18.0, 6.0);
            pen.arrow_tip(19.0, 5.0, 1.0, -1.0, a);
        }
        Mirror => {
            pen.poly(&[(3.0, 18.0), (9.0, 6.0), (9.0, 18.0)], true, b);
            pen.poly(&[(21.0, 18.0), (15.0, 6.0), (15.0, 18.0)], true, a);
            for y in [3.0, 7.0, 11.0, 15.0, 19.0] {
                pen.l(12.0, y, 12.0, y + 2.0);
            }
        }
        Offset => {
            pen.arc(14.0, 18.0, 6.0, 90.0, 180.0, b);
            pen.l(14.0, 12.0, 21.0, 12.0);
            pen.l(8.0, 18.0, 8.0, 22.0);
            pen.arc(14.0, 18.0, 10.0, 90.0, 180.0, a);
            pen.la(14.0, 8.0, 21.0, 8.0);
            pen.la(4.0, 18.0, 4.0, 22.0);
        }
        Trim => {
            pen.l(8.0, 2.0, 8.0, 22.0);
            pen.l(16.0, 2.0, 16.0, 22.0);
            pen.l(2.0, 12.0, 8.0, 12.0);
            pen.l(16.0, 12.0, 22.0, 12.0);
            for x in [9.5, 12.0, 14.5] {
                pen.la(x, 12.0, x + 1.0, 12.0);
            }
            pen.poly(&[(10.0, 17.0), (12.0, 15.0), (14.0, 17.0)], false, a);
        }
        Extend => {
            pen.l(20.0, 2.0, 20.0, 22.0);
            pen.l(3.0, 12.0, 11.0, 12.0);
            pen.la(11.0, 12.0, 19.0, 12.0);
            pen.arrow_tip(19.5, 12.0, 1.0, 0.0, a);
        }
        Fillet => {
            pen.l(4.0, 20.0, 4.0, 12.0);
            pen.l(12.0, 4.0, 20.0, 4.0);
            pen.arc(12.0, 12.0, 8.0, 90.0, 180.0, a);
        }
        Chamfer => {
            pen.l(4.0, 20.0, 4.0, 10.0);
            pen.l(10.0, 4.0, 20.0, 4.0);
            pen.la(4.0, 10.0, 10.0, 4.0);
        }
        Explode => {
            pen.poly(&[(4.0, 9.0), (9.0, 4.0)], false, a);
            pen.poly(&[(15.0, 4.0), (20.0, 9.0)], false, a);
            pen.poly(&[(20.0, 15.0), (15.0, 20.0)], false, a);
            pen.poly(&[(9.0, 20.0), (4.0, 15.0)], false, a);
            pen.fill_c(12.0, 12.0, 2.0, pen.pt);
        }
        Erase => {
            pen.poly(&[(4.0, 16.0), (13.0, 7.0), (19.0, 13.0), (12.0, 20.0), (8.0, 20.0)], true, b);
            pen.la(9.0, 11.0, 15.0, 17.0);
            pen.l(8.0, 20.0, 20.0, 20.0);
        }
        Stretch => {
            pen.poly(&[(3.0, 18.0), (3.0, 8.0), (10.0, 8.0)], false, b);
            pen.poly(&[(10.0, 8.0), (20.0, 8.0), (20.0, 18.0), (3.0, 18.0)], false, a);
            for x in [12.0, 15.0, 18.0, 21.0] {
                pen.l(x, 5.0, x + 1.5, 5.0);
            }
        }
        ArrayRect => {
            for i in 0..3 {
                for j in 0..3 {
                    pen.rect(3.0 + i as f32 * 7.0, 3.0 + j as f32 * 7.0, 4.0, 4.0, if i + j == 0 { b } else { a });
                }
            }
        }
        ArrayPolar => {
            pen.dot(12.0, 12.0);
            for i in 0..6 {
                let ang = (i as f32 * 60.0).to_radians();
                pen.c(12.0 + 7.5 * ang.cos(), 12.0 + 7.5 * ang.sin(), 2.0, if i == 0 { b } else { a });
            }
        }
        Break => {
            pen.l(3.0, 12.0, 9.0, 12.0);
            pen.l(15.0, 12.0, 21.0, 12.0);
            pen.dot(9.0, 12.0);
            pen.dot(15.0, 12.0);
        }
        Join => {
            pen.l(3.0, 16.0, 11.0, 8.0);
            pen.la(11.0, 8.0, 21.0, 16.0);
            pen.dot(11.0, 8.0);
        }
        Lengthen => {
            pen.l(3.0, 12.0, 13.0, 12.0);
            pen.la(13.0, 12.0, 20.0, 12.0);
            pen.arrow_tip(21.0, 12.0, 1.0, 0.0, a);
        }
        MatchProp => {
            // A sheet with property lines, and a yellow brush laying them on.
            pen.fill_poly(&[(3.0, 2.0), (15.0, 2.0), (15.0, 17.0), (3.0, 17.0)], sheet, outline);
            for y in [6.0, 9.5, 13.0] {
                pen.poly(&[(5.5, y), (12.5, y)], false, blue_dark);
            }
            pen.fill_poly(&[(17.0, 9.0), (21.5, 9.0), (21.5, 15.0), (17.0, 15.0)], yellow, yellow_dark);
            pen.fill_poly(&[(18.3, 15.0), (20.2, 15.0), (20.2, 22.5), (18.3, 22.5)], yellow_dark, yellow_dark);
        }
        DimLinear | DimAligned | DimBaseline | DimContinue | QDim => {
            let (y0, y1) = (8.0, 18.0);
            pen.poly(&[(4.0, y1), (4.0, 5.0)], false, yellow);
            pen.poly(&[(20.0, y1), (20.0, 5.0)], false, yellow);
            pen.poly(&[(4.0, y0), (20.0, y0)], false, yellow);
            pen.arrow_tip(4.0, y0, -1.0, 0.0, yellow);
            pen.arrow_tip(20.0, y0, 1.0, 0.0, yellow);
            match icon {
                DimBaseline => pen.la(4.0, 4.0, 13.0, 4.0),
                DimContinue => {
                    pen.l(12.0, y1, 12.0, 5.0);
                }
                QDim => pen.text(12.0, 15.0, "Q", 7.0, b),
                DimAligned => {}
                _ => pen.text(12.0, 14.5, "1.5", 6.0, b),
            }
        }
        DimRadius => {
            pen.arc(12.0, 14.0, 8.0, 0.0, 180.0, b);
            pen.la(12.0, 14.0, 17.6, 8.4);
            pen.arrow_tip(17.6, 8.4, 1.0, -1.0, a);
            pen.dot(12.0, 14.0);
        }
        DimDiameter => {
            pen.c(12.0, 12.0, 8.0, b);
            pen.la(6.4, 17.6, 17.6, 6.4);
            pen.arrow_tip(6.4, 17.6, -1.0, 1.0, a);
            pen.arrow_tip(17.6, 6.4, 1.0, -1.0, a);
        }
        DimAngular => {
            pen.l(4.0, 20.0, 20.0, 20.0);
            pen.l(4.0, 20.0, 16.0, 6.0);
            pen.arc(4.0, 20.0, 11.0, 0.0, 49.0, a);
        }
        DimArc => {
            pen.arc(12.0, 18.0, 8.0, 20.0, 160.0, b);
            pen.arc(12.0, 18.0, 12.0, 25.0, 155.0, a);
        }
        DimOrdinate => {
            pen.l(4.0, 20.0, 4.0, 4.0);
            pen.l(4.0, 20.0, 20.0, 20.0);
            pen.la(10.0, 20.0, 10.0, 8.0);
            pen.la(16.0, 20.0, 16.0, 11.0);
        }
        MLeader | AddLeader | RemoveLeader | AlignLeaders => {
            pen.la(3.0, 20.0, 10.0, 9.0);
            pen.la(10.0, 9.0, 14.0, 9.0);
            pen.arrow_tip(3.0, 20.0, -0.55, 1.0, a);
            pen.l(15.0, 6.0, 22.0, 6.0);
            pen.l(15.0, 10.0, 21.0, 10.0);
            match icon {
                AddLeader => pen.text(19.0, 18.0, "+", 9.0, pen.pt),
                RemoveLeader => pen.text(19.0, 18.0, "−", 9.0, pen.pt),
                _ => {}
            }
        }
        BlockCreate | Insert | BlockEdit | AttDef => {
            // A blue block, lit from the top left.
            pen.fill_poly(&[(9.0, 4.0), (17.0, 8.0), (9.0, 12.0), (1.0, 8.0)], blue_light, outline);
            pen.fill_poly(&[(1.0, 8.0), (9.0, 12.0), (9.0, 19.0), (1.0, 15.0)], blue, outline);
            pen.fill_poly(&[(9.0, 12.0), (17.0, 8.0), (17.0, 15.0), (9.0, 19.0)], blue_dark, outline);
            match icon {
                BlockCreate => pen.text(19.0, 5.0, "✳", 9.0, pen.pt),
                Insert => {
                    pen.la(14.0, 20.0, 22.0, 20.0);
                    pen.arrow_tip(13.0, 20.0, -1.0, 0.0, a);
                }
                BlockEdit => pen.la(14.0, 22.0, 22.0, 14.0),
                _ => pen.text(19.0, 19.0, "A", 9.0, a),
            }
        }
        LayerOff | LayerIso | LayerFreeze | LayerLock | LayerMatch | MakeCurrent | LayerPrev => {
            pen.poly(&[(2.0, 9.0), (10.0, 5.0), (18.0, 9.0), (10.0, 13.0)], true, b);
            pen.poly(&[(2.0, 13.0), (10.0, 17.0), (18.0, 13.0)], false, b);
            let (x, y) = (18.0, 17.0);
            match icon {
                LayerOff => pen.fill_c(x, y, 3.5, Color32::from_rgb(0x8a, 0x8a, 0x5a)),
                LayerIso => pen.fill_c(x, y, 3.5, a),
                LayerFreeze => pen.text(x, y, "❄", 9.0, Color32::from_rgb(0x9f, 0xd4, 0xff)),
                LayerLock => pen.rect(x - 3.0, y - 1.0, 6.0, 5.0, Color32::from_rgb(0xe6, 0xb4, 0x3c)),
                LayerMatch => pen.la(x - 4.0, y, x + 4.0, y),
                MakeCurrent => pen.text(x, y, "✓", 9.0, Color32::from_rgb(0x6f, 0xe0, 0x6f)),
                _ => {
                    pen.arc(x, y, 3.5, 90.0, 360.0, a);
                }
            }
        }
        Bulb | BulbOff => {
            let c = if icon == Bulb { Color32::from_rgb(0xf5, 0xd0, 0x3b) } else { Color32::from_rgb(0x70, 0x78, 0x86) };
            pen.fill_c(12.0, 10.0, 6.0, alpha(c));
            pen.fill_rect(9.5, 15.0, 5.0, 4.0, alpha(Color32::from_rgb(0xb0, 0xb4, 0xba)));
        }
        Lock | Unlock => {
            let c = alpha(Color32::from_rgb(0xe6, 0xb4, 0x3c));
            pen.fill_rect(6.0, 11.0, 12.0, 9.0, c);
            if icon == Lock {
                pen.arc(12.0, 11.0, 4.0, 0.0, 180.0, c);
            } else {
                pen.arc(15.0, 9.0, 4.0, 0.0, 180.0, c);
            }
        }
        Sun => {
            let c = alpha(Color32::from_rgb(0xf5, 0xd0, 0x3b));
            pen.fill_c(12.0, 12.0, 4.0, c);
            for i in 0..8 {
                let ang = (i as f32 * 45.0).to_radians();
                pen.poly(&[(12.0 + 6.0 * ang.cos(), 12.0 + 6.0 * ang.sin()), (12.0 + 9.0 * ang.cos(), 12.0 + 9.0 * ang.sin())], false, c);
            }
        }
        Snowflake => {
            let c = alpha(Color32::from_rgb(0x9f, 0xd4, 0xff));
            for i in 0..3 {
                let ang = (i as f32 * 60.0).to_radians();
                pen.poly(&[(12.0 - 8.0 * ang.cos(), 12.0 - 8.0 * ang.sin()), (12.0 + 8.0 * ang.cos(), 12.0 + 8.0 * ang.sin())], false, c);
            }
        }
        Grid => {
            for k in 0..4 {
                let v = 4.0 + k as f32 * 5.33;
                pen.l(v, 3.0, v, 21.0);
                pen.l(3.0, v, 21.0, v);
            }
        }
        Snap => {
            for i in 0..4 {
                for j in 0..4 {
                    pen.fill_c(4.0 + i as f32 * 5.33, 4.0 + j as f32 * 5.33, 1.0, b);
                }
            }
        }
        Ortho => {
            pen.poly(&[(4.0, 3.0), (4.0, 20.0), (21.0, 20.0)], false, b);
        }
        Polar => {
            pen.l(4.0, 20.0, 21.0, 20.0);
            pen.l(4.0, 20.0, 18.0, 6.0);
            pen.arc(4.0, 20.0, 9.0, 0.0, 45.0, b);
            pen.c(4.0, 20.0, 1.2, b);
        }
        Osnap => {
            pen.rect(7.0, 7.0, 10.0, 10.0, b);
            pen.l(12.0, 2.0, 12.0, 7.0);
            pen.l(12.0, 17.0, 12.0, 22.0);
            pen.l(2.0, 12.0, 7.0, 12.0);
            pen.l(17.0, 12.0, 22.0, 12.0);
        }
        OTrack => {
            pen.rect(9.0, 9.0, 6.0, 6.0, b);
            for x in [2.0, 5.0, 17.0, 20.0] {
                pen.l(x, 12.0, x + 1.5, 12.0);
            }
            for y in [2.0, 5.0, 17.0, 20.0] {
                pen.l(12.0, y, 12.0, y + 1.5);
            }
        }
        Lineweight => {
            for (k, w) in [1.0f32, 2.0, 3.5].iter().enumerate() {
                let y = 6.0 + k as f32 * 6.0;
                p.line_segment([pen.at(3.0, y), pen.at(21.0, y)], Stroke::new(w * pen.sc(), b));
            }
        }
        Transparency => {
            pen.fill_rect(3.0, 3.0, 12.0, 12.0, b.gamma_multiply(0.7));
            pen.fill_rect(9.0, 9.0, 12.0, 12.0, b.gamma_multiply(0.35));
        }
        Isodraft => {
            pen.poly(&[(12.0, 3.0), (20.0, 8.0), (20.0, 17.0), (12.0, 21.0), (4.0, 17.0), (4.0, 8.0)], true, b);
            pen.poly(&[(4.0, 8.0), (12.0, 12.0), (20.0, 8.0)], false, b);
            pen.l(12.0, 12.0, 12.0, 21.0);
        }
        DynInput => {
            pen.rect(3.0, 6.0, 12.0, 7.0, b);
            pen.l(6.0, 9.5, 12.0, 9.5);
            pen.l(17.0, 13.0, 21.0, 21.0);
            pen.l(17.0, 13.0, 17.0, 21.0);
        }
        Gear => {
            pen.c(12.0, 12.0, 4.0, b);
            for i in 0..8 {
                let ang = (i as f32 * 45.0).to_radians();
                p.line_segment(
                    [pen.at(12.0 + 6.0 * ang.cos(), 12.0 + 6.0 * ang.sin()), pen.at(12.0 + 9.0 * ang.cos(), 12.0 + 9.0 * ang.sin())],
                    Stroke::new(2.2 * pen.sc(), b),
                );
            }
            pen.c(12.0, 12.0, 6.5, b);
        }
        Annotation => {
            pen.poly(&[(12.0, 3.0), (21.0, 20.0), (3.0, 20.0)], true, b);
            pen.l(12.0, 9.0, 12.0, 15.0);
            pen.fill_c(12.0, 17.5, 1.0, b);
        }
        Workspace => {
            pen.rect(3.0, 4.0, 18.0, 16.0, b);
            pen.l(9.0, 4.0, 9.0, 20.0);
            pen.l(9.0, 9.0, 21.0, 9.0);
        }
        ChevronDown => pen.poly(&[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)], false, b),
        ChevronUp => pen.poly(&[(6.0, 15.0), (12.0, 9.0), (18.0, 15.0)], false, b),
        ChevronLeft => pen.poly(&[(15.0, 6.0), (9.0, 12.0), (15.0, 18.0)], false, b),
        ChevronRight => pen.poly(&[(9.0, 6.0), (15.0, 12.0), (9.0, 18.0)], false, b),
        Plus => {
            pen.l(12.0, 4.0, 12.0, 20.0);
            pen.l(4.0, 12.0, 20.0, 12.0);
        }
        Switcher => {
            for (x, y) in [(3.0, 3.0), (13.0, 3.0), (3.0, 13.0), (13.0, 13.0)] {
                pen.fill_rect(x, y, 8.0, 8.0, if x == 3.0 && y == 3.0 { a } else { b });
            }
        }
        Close => {
            pen.l(6.0, 6.0, 18.0, 18.0);
            pen.l(18.0, 6.0, 6.0, 18.0);
        }
        Search => {
            pen.c(10.0, 10.0, 6.0, b);
            pen.l(14.5, 14.5, 20.0, 20.0);
        }
        Menu => {
            for y in [7.0, 12.0, 17.0] {
                pen.l(4.0, y, 20.0, y);
            }
        }
        Constraint | Fix => {
            pen.fill_rect(7.0, 11.0, 10.0, 9.0, Color32::from_rgb(0xe6, 0xb4, 0x3c));
            pen.arc(12.0, 11.0, 4.0, 0.0, 180.0, Color32::from_rgb(0xe6, 0xb4, 0x3c));
        }
        Collinear => {
            pen.l(3.0, 19.0, 9.0, 15.0);
            pen.la(12.0, 13.0, 21.0, 7.0);
        }
        Smooth => {
            // A line running into an arc without a corner.
            pen.l(3.0, 18.0, 11.0, 18.0);
            pen.arc(11.0, 10.0, 8.0, -90.0, 0.0, a);
            pen.dot(11.0, 18.0);
        }
        Symmetric => {
            pen.poly(&[(9.0, 5.0), (4.0, 12.0), (9.0, 19.0)], false, b);
            pen.poly(&[(15.0, 5.0), (20.0, 12.0), (15.0, 19.0)], false, b);
            for y in [3.0, 9.0, 15.0] {
                pen.la(12.0, y, 12.0, y + 4.0);
            }
        }
        DelConstraint => {
            let lock = alpha(Color32::from_rgb(0xe6, 0xb4, 0x3c));
            pen.fill_rect(3.0, 12.0, 9.0, 8.0, lock);
            pen.arc(7.5, 12.0, 3.5, 0.0, 180.0, lock);
            pen.l(15.0, 13.0, 21.0, 19.0);
            pen.l(21.0, 13.0, 15.0, 19.0);
        }
        Parameters => {
            pen.rect(3.0, 4.0, 18.0, 16.0, b);
            pen.text(12.0, 12.0, "fx", 10.0, a);
        }
        Coincident => {
            pen.l(3.0, 20.0, 12.0, 12.0);
            pen.l(12.0, 12.0, 21.0, 18.0);
            pen.fill_c(12.0, 12.0, 2.5, a);
        }
        Parallel => {
            pen.l(4.0, 18.0, 16.0, 4.0);
            pen.la(9.0, 20.0, 21.0, 6.0);
        }
        Perpendicular => {
            pen.l(4.0, 20.0, 20.0, 20.0);
            pen.la(12.0, 20.0, 12.0, 4.0);
            pen.rect(12.0, 16.0, 4.0, 4.0, b);
        }
        Horizontal => {
            pen.la(3.0, 12.0, 21.0, 12.0);
            pen.dot(3.0, 12.0);
            pen.dot(21.0, 12.0);
        }
        Vertical => {
            pen.la(12.0, 3.0, 12.0, 21.0);
            pen.dot(12.0, 3.0);
            pen.dot(12.0, 21.0);
        }
        Tangent => {
            pen.c(10.0, 13.0, 6.0, b);
            pen.la(3.0, 7.0, 21.0, 7.0);
        }
        Concentric => {
            pen.c(12.0, 12.0, 8.0, b);
            pen.c(12.0, 12.0, 4.0, a);
        }
        QuickProps => {
            // A small palette window with property rows and a pointer.
            pen.rect(3.0, 4.0, 15.0, 13.0, b);
            pen.l(3.0, 7.0, 18.0, 7.0);
            for y in [10.0, 13.0] {
                pen.l(5.0, y, 9.0, y);
                pen.la(11.0, y, 16.0, y);
            }
            pen.poly(&[(15.0, 14.0), (15.0, 22.0), (17.0, 20.0), (19.5, 22.5), (20.5, 21.5), (18.0, 19.0), (21.0, 18.5)], true, b);
        }
        Equal => {
            pen.l(5.0, 9.0, 19.0, 9.0);
            pen.la(5.0, 15.0, 19.0, 15.0);
        }
        Measure => {
            // A yellow ruler with its marks.
            pen.fill_poly(&[(2.0, 9.0), (22.0, 9.0), (22.0, 16.0), (2.0, 16.0)], yellow, yellow_dark);
            for (i, x) in [5.0f32, 8.0, 11.0, 14.0, 17.0, 20.0].iter().enumerate() {
                let len = if i % 2 == 0 { 3.5 } else { 2.0 };
                pen.poly(&[(*x, 9.0), (*x, 9.0 + len)], false, yellow_dark);
            }
        }
        Paste => {
            // A clipboard with a sheet on it.
            pen.fill_poly(&[(3.0, 4.0), (17.0, 4.0), (17.0, 22.0), (3.0, 22.0)], alpha(Color32::from_rgb(0xb8, 0x7d, 0x3e)), outline);
            pen.fill_poly(&[(7.0, 2.0), (13.0, 2.0), (13.0, 6.0), (7.0, 6.0)], pale, outline);
            pen.fill_poly(&[(10.0, 9.0), (21.0, 9.0), (21.0, 23.0), (10.0, 23.0)], sheet, outline);
            for y in [13.0, 16.0, 19.0] {
                pen.poly(&[(12.5, y), (18.5, y)], false, blue_dark);
            }
        }
        Distance => {
            pen.poly(&[(3.0, 15.0), (21.0, 15.0), (21.0, 20.0), (3.0, 20.0)], true, b);
            for x in [6.0, 9.0, 12.0, 15.0, 18.0] {
                pen.l(x, 15.0, x, 17.5);
            }
            pen.la(3.0, 8.0, 21.0, 8.0);
            pen.arrow_tip(3.0, 8.0, -1.0, 0.0, a);
            pen.arrow_tip(21.0, 8.0, 1.0, 0.0, a);
        }
        Area => {
            pen.poly(&[(4.0, 18.0), (6.0, 5.0), (19.0, 7.0), (20.0, 19.0)], true, a);
            pen.text(12.0, 12.5, "A", 9.0, b);
        }
        List => {
            for y in [6.0, 11.0, 16.0] {
                pen.fill_c(5.0, y, 1.2, b);
                pen.l(8.0, y, 20.0, y);
            }
        }
        PickAdd => {
            // An object box with a plus: picks add to the selection.
            pen.rect(3.0, 3.0, 11.0, 11.0, b);
            pen.la(17.0, 11.0, 17.0, 21.0);
            pen.la(12.0, 16.0, 22.0, 16.0);
        }
        SelectObjects => {
            // A pointer picking an object box.
            pen.rect(12.0, 3.0, 9.0, 9.0, a);
            pen.poly(&[(4.0, 6.0), (4.0, 20.0), (7.5, 16.5), (10.5, 21.5), (12.5, 20.5), (9.5, 15.5), (14.0, 15.0)], true, b);
        }
        QuickSelect => {
            // A filter funnel with a spark.
            pen.poly(&[(3.0, 4.0), (19.0, 4.0), (13.0, 11.0), (13.0, 19.0), (9.0, 17.0), (9.0, 11.0)], true, b);
            pen.poly(&[(20.0, 10.0), (16.5, 15.5), (20.0, 15.5), (17.0, 21.0)], false, a);
        }
    }
}

/// A shared button interaction with CADCraft's original multicolor icon painting.
pub fn button(ui: &mut egui::Ui, icon: Icon, size: f32, tooltip: &str, selected: bool) -> egui::Response {
    let t = Tokens::get();
    let visuals = |fill| ButtonVisuals { fill, stroke: Stroke::NONE, icon: t.icon };
    let style = IconButtonStyle {
        normal: visuals(Color32::TRANSPARENT),
        hovered: visuals(t.control_hover.gamma_multiply(0.6)),
        pressed: None,
        selected: visuals(t.tab_active),
        corner_radius: egui::CornerRadius::same(3),
        focus_stroke: Stroke::new(1.0, t.accent),
    };
    let resp = IconButton::new(crate::i18n::t(tooltip), egui::vec2(size, size), &style).selected(selected).show(ui, |ui, rect, _| {
        // A single tint would erase the blue geometry and orange picked points. Keep the
        // original painter and its explicit dimming on the same parent-enabled UI.
        paint(ui.painter(), rect.shrink(size * 0.08), icon, !ui.is_enabled());
    });
    crate::control::record_widget(ui, &resp, tooltip, "button", Some(selected));
    resp.on_hover_text(crate::i18n::t(tooltip))
}

/// Labels and focus outlines for controls whose existing geometry is painted by the app.
/// Pointer focus is opt-in for navigation tabs; drafting buttons keep their command-line flow.
pub fn describe_control(ui: &egui::Ui, response: &egui::Response, label: &str, selected: Option<bool>, tab: bool) {
    let label = crate::i18n::t(label);
    response.widget_info(|| match selected {
        Some(selected) => egui::WidgetInfo::selected(egui::WidgetType::Button, response.enabled(), selected, label),
        None => egui::WidgetInfo::labeled(egui::WidgetType::Button, response.enabled(), label),
    });
    if tab {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Tab);
            node.clear_toggled();
            if let Some(selected) = selected {
                node.set_selected(selected);
            }
        });
        if response.clicked() {
            response.request_focus();
        }
    }
    if response.has_focus() && response.rect.width() > 4.0 && response.rect.height() > 4.0 {
        ui.painter().rect_stroke(response.rect.shrink(2.0), 2.0, Stroke::new(1.0, Tokens::get().accent), egui::StrokeKind::Inside);
    }
    crate::control::record_widget(ui, response, label, if tab { "tab" } else { "button" }, selected);
}
