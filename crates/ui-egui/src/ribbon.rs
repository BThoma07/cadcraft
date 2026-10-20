//! The ribbon of the Windows edition of the reference program, in Italian: tabs (Inizio,
//! Inserisci, Annota…) over panels of large and small buttons (Disegna, Edita, Annotazione,
//! Layer, Blocco, Proprietà, Gruppi, Utilità, Appunti, Vista), with the panel names underneath.
//!
//! Every button starts a command that exists, as a menu item would; the layer and property
//! lists set the current layer, colour, linetype and lineweight.

use cadcraft_color::Color;
use egui::{Align2, Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// A button: icon, command id, label ("\n" splits a large button's label in two lines).
type Btn = (Icon, &'static str, &'static str);

/// One column of a panel.
enum Col {
    /// A large button: icon over its label.
    Big(Btn),
    /// Up to three small buttons stacked, icon and label.
    Small(&'static [Btn]),
    /// Up to three small buttons stacked, icon only.
    Icons(&'static [Btn]),
    /// The current layer list and the layer tools under it.
    Layer,
    /// The current colour, linetype and lineweight lists.
    Props,
}

struct Panel {
    title: &'static str,
    cols: &'static [Col],
}

struct Tab {
    id: &'static str,
    title: &'static str,
    panels: &'static [Panel],
}

const HOME: &[Panel] = &[
    Panel {
        title: "Disegna",
        cols: &[
            Col::Big((Icon::Line, "line", "Linea")),
            Col::Big((Icon::Polyline, "pline", "Polilinea")),
            Col::Big((Icon::Circle, "circle", "Cerchio")),
            Col::Big((Icon::Arc, "arc", "Arco")),
            Col::Icons(&[(Icon::Rectangle, "rectang", "Rettangolo"), (Icon::Ellipse, "ellipse", "Ellisse"), (Icon::Hatch, "hatch", "Tratteggio")]),
        ],
    },
    Panel {
        title: "Edita",
        cols: &[
            Col::Small(&[(Icon::Move, "move", "Sposta"), (Icon::Copy, "copy", "Copia"), (Icon::Stretch, "stretch", "Stira")]),
            Col::Small(&[(Icon::Rotate, "rotate", "Ruota"), (Icon::Mirror, "mirror", "Specchio"), (Icon::Scale, "scale", "Scala")]),
            Col::Small(&[(Icon::Trim, "trim", "Taglia"), (Icon::Fillet, "fillet", "Raccordo"), (Icon::ArrayRect, "arrayrect", "Serie")]),
            Col::Icons(&[(Icon::Erase, "erase", "Cancella"), (Icon::Explode, "explode", "Esplodi"), (Icon::Offset, "offset", "Offset")]),
        ],
    },
    Panel {
        title: "Annotazione",
        cols: &[
            Col::Big((Icon::MText, "mtext", "Testo")),
            Col::Big((Icon::DimLinear, "dim", "Quota")),
            Col::Small(&[(Icon::DimLinear, "dimlinear", "Lineare"), (Icon::MLeader, "mleader", "Direttrice"), (Icon::Table, "table", "Tabella")]),
        ],
    },
    Panel { title: "Layer", cols: &[Col::Big((Icon::LayerProps, "ui.dialog.layers", "Proprietà\nlayer")), Col::Layer] },
    Panel {
        title: "Blocco",
        cols: &[
            Col::Big((Icon::Insert, "insert", "Inserisci")),
            Col::Small(&[(Icon::BlockCreate, "block", "Crea"), (Icon::BlockEdit, "bedit", "Modifica"), (Icon::AttDef, "attedit", "Edita attributi")]),
        ],
    },
    Panel { title: "Proprietà", cols: &[Col::Big((Icon::MatchProp, "matchprop", "Corrispondenza\ncon proprietà")), Col::Props] },
    Panel {
        title: "Gruppi",
        cols: &[
            Col::Big((Icon::SelectObjects, "group", "Gruppo")),
            Col::Icons(&[(Icon::Close, "ungroup", "Separa gruppo"), (Icon::PickAdd, "groupedit", "Modifica gruppo"), (Icon::QuickSelect, "qselect", "Selezione rapida")]),
        ],
    },
    Panel {
        title: "Utilità",
        cols: &[
            Col::Big((Icon::Measure, "measuregeom", "Misura")),
            Col::Icons(&[(Icon::QuickSelect, "qselect", "Selezione rapida"), (Icon::SelectObjects, "selectall", "Seleziona tutto"), (Icon::List, "cal", "Calcolatrice")]),
        ],
    },
    Panel {
        title: "Appunti",
        cols: &[
            Col::Big((Icon::Insert, "pasteclip", "Incolla")),
            Col::Icons(&[(Icon::Copy, "copyclip", "Copia negli Appunti"), (Icon::Erase, "cutclip", "Taglia negli Appunti")]),
        ],
    },
    Panel {
        title: "Vista",
        cols: &[
            Col::Big((Icon::ZoomExtents, "zoom.extents", "Estensioni")),
            Col::Icons(&[(Icon::ZoomWindow, "zoom.window", "Zoom finestra"), (Icon::Pan, "pan", "Pan"), (Icon::Orbit, "3dorbit", "Orbita")]),
        ],
    },
];

const INSERT: &[Panel] = &[
    Panel {
        title: "Blocco",
        cols: &[
            Col::Big((Icon::Insert, "insert", "Inserisci")),
            Col::Big((Icon::BlockCreate, "block", "Crea\nblocco")),
            Col::Small(&[(Icon::BlockEdit, "bedit", "Editor blocchi"), (Icon::Blocks, "ui.dialog.blocks", "Blocchi"), (Icon::Save, "wblock", "Scrivi blocco")]),
        ],
    },
    Panel {
        title: "Definizione blocco",
        cols: &[
            Col::Big((Icon::AttDef, "attdef", "Definisci\nattributi")),
            Col::Small(&[(Icon::AttDef, "attedit", "Edita attributi"), (Icon::List, "battman", "Gestione attributi"), (Icon::Redo, "attsync", "Sincronizza")]),
        ],
    },
    Panel {
        title: "Riferimento",
        cols: &[
            Col::Big((Icon::Attach, "attach", "Allega")),
            Col::Small(&[(Icon::Attach, "xref", "Riferimenti esterni"), (Icon::Wipeout, "vpclip", "Ritaglia"), (Icon::BlockEdit, "refedit", "Modifica riferimento")]),
        ],
    },
    Panel { title: "Importa", cols: &[Col::Big((Icon::Import, "import", "Importa")), Col::Big((Icon::Table, "datalink", "Collegamento\ndati"))] },
];

const ANNOTATE: &[Panel] = &[
    Panel {
        title: "Testo",
        cols: &[
            Col::Big((Icon::MText, "mtext", "Testo\nmultilinea")),
            Col::Small(&[(Icon::Text, "text", "Testo riga singola"), (Icon::Text, "textedit", "Modifica testo"), (Icon::Text, "style", "Stile di testo")]),
        ],
    },
    Panel {
        title: "Quote",
        cols: &[
            Col::Big((Icon::DimLinear, "dim", "Quota")),
            Col::Small(&[(Icon::DimLinear, "dimlinear", "Lineare"), (Icon::DimAligned, "dimaligned", "Allineata"), (Icon::DimAngular, "dimangular", "Angolare")]),
            Col::Small(&[(Icon::DimRadius, "dimradius", "Raggio"), (Icon::DimDiameter, "dimdiameter", "Diametro"), (Icon::DimArc, "dimarc", "Lunghezza arco")]),
            Col::Small(&[(Icon::DimContinue, "dimcontinue", "Continua"), (Icon::DimBaseline, "dimbaseline", "Linea di base"), (Icon::QDim, "qdim", "Quotatura rapida")]),
            Col::Icons(&[(Icon::DimOrdinate, "dimordinate", "Coordinata"), (Icon::Gear, "dimstyle", "Stile di quota"), (Icon::Break, "dimbreak", "Interruzione quota")]),
        ],
    },
    Panel {
        title: "Direttrici",
        cols: &[
            Col::Big((Icon::MLeader, "mleader", "Multidirettrice")),
            Col::Small(&[(Icon::AddLeader, "mleaderedit.add", "Aggiungi direttrice"), (Icon::RemoveLeader, "mleaderedit.remove", "Rimuovi direttrice"), (Icon::AlignLeaders, "mleaderalign", "Allinea")]),
        ],
    },
    Panel { title: "Tabelle", cols: &[Col::Big((Icon::Table, "table", "Tabella")), Col::Small(&[(Icon::Gear, "tablestyle", "Stile tabella"), (Icon::Export, "tableexport", "Esporta tabella")])] },
    Panel { title: "Contrassegni", cols: &[Col::Big((Icon::RevCloud, "revcloud", "Nuvola\nrevisione")), Col::Big((Icon::Wipeout, "wipeout", "Copertura"))] },
];

const PARAMETRIC: &[Panel] = &[
    Panel {
        title: "Geometrici",
        cols: &[
            Col::Big((Icon::Constraint, "autoconstrain", "Vincolo\nautomatico")),
            Col::Icons(&[(Icon::Coincident, "geomconstraint", "Coincidente"), (Icon::Parallel, "geomconstraint", "Parallelo"), (Icon::Perpendicular, "geomconstraint", "Perpendicolare")]),
            Col::Small(&[
                (Icon::Constraint, "constraintbar.showall", "Mostra tutto"),
                (Icon::Close, "constraintbar.hideall", "Nascondi tutto"),
                (Icon::Gear, "constraintsettings", "Impostazioni"),
            ]),
        ],
    },
    Panel {
        title: "Dimensionali",
        cols: &[
            Col::Big((Icon::DimLinear, "dimconstraint", "Vincolo\ndimensionale")),
            Col::Small(&[(Icon::Constraint, "dcdisplay.showall", "Mostra tutto"), (Icon::Close, "dcdisplay.hideall", "Nascondi tutto")]),
        ],
    },
    Panel { title: "Gestisci", cols: &[Col::Big((Icon::DelConstraint, "delconstraint", "Elimina\nvincoli")), Col::Big((Icon::Parameters, "parameters", "Gestione\nparametri"))] },
];

const VIEW: &[Panel] = &[
    Panel {
        title: "Navigazione",
        cols: &[
            Col::Big((Icon::ZoomExtents, "zoom.extents", "Estensioni")),
            Col::Small(&[(Icon::ZoomWindow, "zoom.window", "Finestra"), (Icon::Pan, "pan", "Pan"), (Icon::LayerPrev, "zoom.previous", "Precedente")]),
            Col::Small(&[(Icon::Plus, "zoom.in", "Zoom avanti"), (Icon::Close, "zoom.out", "Zoom indietro"), (Icon::ZoomExtents, "zoom.all", "Tutto")]),
        ],
    },
    Panel {
        title: "Finestre modello",
        cols: &[Col::Big((Icon::Workspace, "vports", "Configurazione\nfinestre")), Col::Small(&[(Icon::Workspace, "vports.1", "Una"), (Icon::Workspace, "vports.2", "Due"), (Icon::Workspace, "vports.4", "Quattro")])],
    },
    Panel {
        title: "Palette",
        cols: &[
            Col::Big((Icon::Properties, "ui.toggle.palettes", "Proprietà")),
            Col::Small(&[(Icon::Layers, "ui.dialog.layers", "Proprietà layer"), (Icon::Blocks, "ui.dialog.blocks", "Blocchi"), (Icon::Measure, "quickcalc.pick", "CalcRapida")]),
        ],
    },
];

const MANAGE: &[Panel] = &[
    Panel {
        title: "Personalizzazione",
        cols: &[Col::Big((Icon::Gear, "aliasedit", "Modifica\nalias")), Col::Small(&[(Icon::Gear, "options", "Opzioni"), (Icon::Snap, "dsettings", "Impostazioni disegno")])],
    },
    Panel {
        title: "Applicazioni",
        cols: &[Col::Big((Icon::Import, "appload", "Carica\napplicazione")), Col::Small(&[(Icon::List, "script", "Esegui script"), (Icon::Text, "lisp", "LISP")])],
    },
    Panel {
        title: "Pulizia",
        cols: &[Col::Big((Icon::Erase, "purge", "Elimina")), Col::Small(&[(Icon::Erase, "overkill", "Elimina duplicati"), (Icon::Search, "audit", "Controllo"), (Icon::Open, "recover", "Recupera")])],
    },
    Panel { title: "Confronto", cols: &[Col::Big((Icon::Switcher, "compare", "Confronto\nDWG"))] },
];

const OUTPUT: &[Panel] = &[
    Panel {
        title: "Stampa",
        cols: &[
            Col::Big((Icon::Plot, "plot", "Stampa")),
            Col::Big((Icon::Publish, "publish", "Pubblica")),
            Col::Small(&[(Icon::PageSetup, "pagesetup", "Gestione impostazioni pagina"), (Icon::Preview, "preview", "Anteprima"), (Icon::Gear, "plottermanager", "Gestione plotter")]),
        ],
    },
    Panel { title: "Esporta in DWF/PDF", cols: &[Col::Big((Icon::Export, "exportpdf", "Esporta\nPDF")), Col::Small(&[(Icon::Export, "export", "Esporta"), (Icon::Publish, "pdfbook", "Fascicolo PDF")])] },
];

const TABS: &[Tab] = &[
    Tab { id: "home", title: "Inizio", panels: HOME },
    Tab { id: "insert", title: "Inserisci", panels: INSERT },
    Tab { id: "annotate", title: "Annota", panels: ANNOTATE },
    Tab { id: "parametric", title: "Parametrico", panels: PARAMETRIC },
    Tab { id: "view", title: "Vista", panels: VIEW },
    Tab { id: "manage", title: "Gestisci", panels: MANAGE },
    Tab { id: "output", title: "Output", panels: OUTPUT },
];

/// The quick access tools at the start of the tab row.
const QUICK: &[Btn] = &[
    (Icon::New, "new", "Nuovo"),
    (Icon::Open, "open", "Apri"),
    (Icon::Save, "qsave", "Salva"),
    (Icon::SaveAs, "saveas", "Salva con nome"),
    (Icon::Plot, "plot", "Stampa"),
    (Icon::Undo, "u", "Annulla"),
    (Icon::Redo, "redo", "Ripristina"),
];

/// What opens under a button's small arrow, as on the reference program's split buttons.
const VARIANTS: &[(&str, &[Btn])] = &[
    (
        "circle",
        &[
            (Icon::Circle, "circle", "Centro, Raggio"),
            (Icon::Circle, "circle.cd", "Centro, Diametro"),
            (Icon::Circle, "circle.2p", "2 punti"),
            (Icon::Circle, "circle.3p", "3 punti"),
            (Icon::Circle, "circle.ttr", "Tan, Tan, Raggio"),
            (Icon::Circle, "circle.ttt", "Tan, Tan, Tan"),
        ],
    ),
    (
        "arc",
        &[
            (Icon::Arc, "arc", "3 punti"),
            (Icon::Arc, "arc.sce", "Inizio, Centro, Fine"),
            (Icon::Arc, "arc.sca", "Inizio, Centro, Angolo"),
            (Icon::Arc, "arc.scl", "Inizio, Centro, Lunghezza"),
            (Icon::Arc, "arc.sea", "Inizio, Fine, Angolo"),
            (Icon::Arc, "arc.sed", "Inizio, Fine, Direzione"),
            (Icon::Arc, "arc.ser", "Inizio, Fine, Raggio"),
            (Icon::Arc, "arc.cse", "Centro, Inizio, Fine"),
            (Icon::Arc, "arc.csa", "Centro, Inizio, Angolo"),
            (Icon::Arc, "arc.csl", "Centro, Inizio, Lunghezza"),
            (Icon::Arc, "arc.continue", "Continua"),
        ],
    ),
    ("rectang", &[(Icon::Rectangle, "rectang", "Rettangolo"), (Icon::Polygon, "polygon", "Poligono")]),
    ("ellipse", &[(Icon::Ellipse, "ellipse", "Centro"), (Icon::Ellipse, "ellipse.axis", "Asse, Fine"), (Icon::Ellipse, "ellipse.arc", "Arco ellittico")]),
    ("hatch", &[(Icon::Hatch, "hatch", "Tratteggio"), (Icon::Gradient, "gradient", "Sfumatura"), (Icon::Boundary, "boundary", "Contorno")]),
    ("trim", &[(Icon::Trim, "trim", "Taglia"), (Icon::Extend, "extend", "Estendi")]),
    ("fillet", &[(Icon::Fillet, "fillet", "Raccordo"), (Icon::Chamfer, "chamfer", "Cima"), (Icon::Spline, "blend", "Raccorda curve")]),
    ("arrayrect", &[(Icon::ArrayRect, "arrayrect", "Serie rettangolare"), (Icon::Spline, "arraypath", "Serie lungo percorso"), (Icon::ArrayPolar, "arraypolar", "Serie polare")]),
    ("mtext", &[(Icon::MText, "mtext", "Testo multilinea"), (Icon::Text, "text", "Testo su riga singola")]),
    (
        "dimlinear",
        &[
            (Icon::DimLinear, "dimlinear", "Lineare"),
            (Icon::DimAligned, "dimaligned", "Allineata"),
            (Icon::DimAngular, "dimangular", "Angolare"),
            (Icon::DimArc, "dimarc", "Lunghezza arco"),
            (Icon::DimRadius, "dimradius", "Raggio"),
            (Icon::DimDiameter, "dimdiameter", "Diametro"),
            (Icon::DimRadius, "dimjogged", "Con raccordo"),
            (Icon::DimOrdinate, "dimordinate", "Coordinata"),
        ],
    ),
    (
        "mleader",
        &[
            (Icon::MLeader, "mleader", "Multidirettrice"),
            (Icon::AddLeader, "mleaderedit.add", "Aggiungi direttrice"),
            (Icon::RemoveLeader, "mleaderedit.remove", "Rimuovi direttrice"),
            (Icon::AlignLeaders, "mleaderalign", "Allinea"),
            (Icon::MLeader, "mleadercollect", "Raccogli"),
        ],
    ),
    ("insert", &[(Icon::Insert, "insert", "Inserisci blocco"), (Icon::Blocks, "ui.dialog.blocks", "Blocchi recenti e preferiti")]),
    ("attedit", &[(Icon::AttDef, "attedit", "Singolo"), (Icon::AttDef, "-attedit", "Multiplo")]),
    (
        "measuregeom",
        &[
            (Icon::Measure, "measuregeom", "Rapida"),
            (Icon::Distance, "dist", "Distanza"),
            (Icon::DimRadius, "radius.measure", "Raggio"),
            (Icon::DimAngular, "angle.measure", "Angolo"),
            (Icon::Area, "area", "Area"),
        ],
    ),
    ("pasteclip", &[(Icon::Insert, "pasteclip", "Incolla"), (Icon::Blocks, "pasteblock", "Incolla come blocco"), (Icon::Insert, "pasteorig", "Incolla alle coordinate originali")]),
    ("copyclip", &[(Icon::Copy, "copyclip", "Copia negli Appunti"), (Icon::Move, "copybase", "Copia con punto base")]),
];

/// The tools a panel shows when its title is clicked (the panel's slide-out).
const EXTRA: &[(&str, &[Btn])] = &[
    (
        "Disegna",
        &[
            (Icon::Spline, "spline", "Spline adattata"),
            (Icon::Spline, "spline.cv", "Spline con vertici di controllo"),
            (Icon::XLine, "xline", "Linea di costruzione"),
            (Icon::Ray, "ray", "Raggio"),
            (Icon::Point, "point.multiple", "Punti multipli"),
            (Icon::Point, "divide", "Dividi"),
            (Icon::Point, "measure", "Misura"),
            (Icon::Region, "region", "Regione"),
            (Icon::Wipeout, "wipeout", "Copertura"),
            (Icon::RevCloud, "revcloud", "Nuvola di revisione"),
            (Icon::Spline, "helix", "Elica"),
            (Icon::Donut, "donut", "Anello"),
            (Icon::Polyline, "3dpoly", "Polilinea 3D"),
            (Icon::Polygon, "polygon", "Poligono"),
        ],
    ),
    (
        "Edita",
        &[
            (Icon::Break, "breakatpoint", "Spezza nel punto"),
            (Icon::Break, "break", "Spezza"),
            (Icon::Join, "join", "Unisci"),
            (Icon::Lengthen, "lengthen", "Allunga"),
            (Icon::Move, "align", "Allinea"),
            (Icon::Rotate, "reverse", "Inverti"),
            (Icon::Polyline, "pedit", "Modifica polilinea"),
            (Icon::Spline, "splinedit", "Modifica spline"),
            (Icon::Layers, "draworder", "Ordine di visualizzazione"),
            (Icon::Erase, "overkill", "Elimina oggetti duplicati"),
            (Icon::Hatch, "hatchedit", "Modifica tratteggio"),
        ],
    ),
    (
        "Annotazione",
        &[(Icon::Text, "style", "Stile di testo"), (Icon::Gear, "dimstyle", "Stile di quota"), (Icon::MLeader, "mleaderstyle", "Stile multidirettrice"), (Icon::Table, "tablestyle", "Stile tabella")],
    ),
    (
        "Layer",
        &[
            (Icon::Layers, "layerstate.list", "Stati layer"),
            (Icon::Bulb, "layon", "Attiva tutti i layer"),
            (Icon::Sun, "laythw", "Scongela tutti i layer"),
            (Icon::LayerPrev, "layerp", "Layer precedente"),
            (Icon::Join, "laymrg", "Unisci layer"),
            (Icon::Erase, "laydel", "Elimina layer"),
            (Icon::LayerLock, "laylck", "Blocca layer"),
            (Icon::Unlock, "layulk", "Sblocca layer"),
        ],
    ),
    ("Blocco", &[(Icon::AttDef, "attdef", "Definisci attributi"), (Icon::List, "battman", "Gestione attributi"), (Icon::Redo, "attsync", "Sincronizza attributi"), (Icon::Save, "wblock", "Scrivi blocco")]),
    ("Proprietà", &[(Icon::Transparency, "transparencydisplay", "Trasparenza"), (Icon::List, "list", "Lista"), (Icon::Lineweight, "lwdisplay", "Visualizza spessori di linea")]),
    ("Gruppi", &[(Icon::SelectObjects, "groupedit", "Modifica gruppo"), (Icon::PickAdd, "pickstyle", "Selezione gruppi")]),
    ("Utilità", &[(Icon::Point, "id", "Coordinate punto"), (Icon::Distance, "dist", "Distanza"), (Icon::Area, "area", "Area"), (Icon::List, "list", "Lista")]),
    ("Vista", &[(Icon::ZoomWindow, "zoom.window", "Zoom finestra"), (Icon::ZoomExtents, "zoom.all", "Zoom tutto"), (Icon::LayerPrev, "zoom.previous", "Zoom precedente"), (Icon::Workspace, "vports", "Finestre")]),
];

fn variants(cmd: &str) -> Option<&'static [Btn]> {
    VARIANTS.iter().find(|(c, _)| *c == cmd).map(|(_, v)| *v)
}

fn extra(title: &str) -> Option<&'static [Btn]> {
    EXTRA.iter().find(|(t, _)| *t == title).map(|(_, v)| *v)
}

const TAB_H: f32 = 24.0;
const BODY_H: f32 = 66.0;
const TITLE_H: f32 = 17.0;
const ROW_H: f32 = 21.0;
const PAD: f32 = 6.0;

/// Every command a ribbon button starts, for the test that they all exist.
pub fn commands() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = QUICK.iter().map(|b| b.1).collect();
    for tab in TABS {
        for panel in tab.panels {
            for col in panel.cols {
                match col {
                    Col::Big(b) => v.push(b.1),
                    Col::Small(bs) | Col::Icons(bs) => v.extend(bs.iter().map(|b| b.1)),
                    Col::Layer | Col::Props => {}
                }
            }
        }
    }
    for (_, list) in VARIANTS.iter().chain(EXTRA) {
        v.extend(list.iter().map(|b| b.1));
    }
    v
}

/// Height the ribbon takes.
pub fn height(app: &CadApp) -> f32 {
    if app.ui.ribbon_minimized { TAB_H } else { TAB_H + BODY_H + TITLE_H }
}

/// What a click asked for, run once the ribbon is drawn.
enum Action {
    Start(&'static str),
    Run(&'static str, Value),
}

pub fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let h = height(app);
    let mut action: Option<Action> = None;
    egui::Panel::top("cc_ribbon").exact_size(h).frame(egui::Frame::NONE.fill(t.chrome_dark)).show(ui, |ui| {
        let r = ui.max_rect();
        let tabs = Rect::from_min_size(r.min, vec2(r.width(), TAB_H));
        tab_row(app, ui, tabs, &mut action);
        if app.ui.ribbon_minimized {
            return;
        }
        let body = Rect::from_min_max(pos2(r.left(), tabs.bottom()), r.max);
        ui.painter().rect_filled(body, 0.0, t.chrome);
        ui.painter().hline(body.x_range(), body.bottom() - 0.5, Stroke::new(1.0, t.border));
        let tab = TABS.iter().find(|tb| tb.id == app.ui.ribbon_tab).unwrap_or(&TABS[0]);
        let mut x = body.left() + 2.0;
        for panel in tab.panels {
            let w = panel_width(ui, panel);
            if x + w > body.right() {
                break;
            }
            let pr = Rect::from_min_size(pos2(x, body.top()), vec2(w, body.height()));
            draw_panel(app, ui, panel, pr, &mut action);
            x += w;
            ui.painter().vline(x - 0.5, pr.top() + 4.0..=pr.bottom() - 4.0, Stroke::new(1.0, t.border));
        }
    });
    match action {
        Some(Action::Start(c)) => crate::menus::activate(app, c),
        Some(Action::Run(c, p)) => {
            if let Err(e) = app.run(c, p) {
                app.set_status(e.to_string());
            }
        }
        None => {}
    }
}

fn tab_row(app: &mut CadApp, ui: &mut egui::Ui, row: Rect, action: &mut Option<Action>) {
    let t = Tokens::get();
    let p = ui.painter().clone();
    let mut x = row.left() + 6.0;
    // Quick access tools.
    for (icon, cmd, label) in QUICK {
        let br = Rect::from_min_size(pos2(x, row.top() + 2.0), vec2(16.0, 16.0));
        let resp = ui.interact(br, ui.id().with(("rb_quick", *cmd)), Sense::click());
        if resp.hovered() {
            p.rect_filled(br.expand(1.0), 2.0, t.control_hover.gamma_multiply(0.6));
        }
        icons::paint(&p, br.shrink(1.0), *icon, false);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.on_hover_text(*label).clicked() {
            *action = Some(Action::Start(cmd));
        }
        x += 20.0;
    }
    x += 14.0;
    for tab in TABS {
        let galley = p.layout_no_wrap(tab.title.to_string(), ribbon_font(), t.text);
        let tr = Rect::from_min_size(pos2(x, row.top() + 2.0), vec2(galley.size().x + 18.0, row.height() - 2.0));
        let resp = ui.interact(tr, ui.id().with(("rb_tab", tab.id)), Sense::click());
        let active = app.ui.ribbon_tab == tab.id;
        if active && !app.ui.ribbon_minimized {
            p.rect_filled(tr, egui::CornerRadius { nw: 3, ne: 3, sw: 0, se: 0 }, t.chrome);
        } else if resp.hovered() {
            p.rect_filled(tr, 3.0, t.control_hover.gamma_multiply(0.4));
        }
        let color = if active { Color32::WHITE } else { t.text_dim };
        p.galley(pos2(tr.left() + 9.0, tr.center().y - galley.size().y / 2.0), galley, color);
        icons::describe_control(ui, &resp, tab.title, Some(active), true);
        if resp.double_clicked() || (resp.clicked() && active) {
            app.ui.ribbon_minimized = !app.ui.ribbon_minimized;
        } else if resp.clicked() {
            app.ui.ribbon_tab = tab.id.to_string();
            app.ui.ribbon_minimized = false;
        }
        x += tr.width() + 2.0;
    }
    let _ = action;
}

/// The ribbon's labels: smaller than the palettes' text, as in the reference program.
fn ribbon_font() -> egui::FontId {
    egui::FontId::proportional(12.0)
}

fn text_w(ui: &egui::Ui, s: &str) -> f32 {
    s.lines().map(|l| ui.painter().layout_no_wrap(l.to_string(), ribbon_font(), Color32::WHITE).size().x).fold(0.0, f32::max)
}

fn col_width(ui: &egui::Ui, col: &Col) -> f32 {
    match col {
        Col::Big((_, _, label)) => (text_w(ui, label) + 8.0).max(38.0),
        Col::Small(bs) => bs.iter().map(|b| text_w(ui, b.2) + if variants(b.1).is_some() { 14.0 } else { 0.0 }).fold(0.0, f32::max) + 26.0,
        Col::Icons(bs) => {
            if bs.iter().any(|b| variants(b.1).is_some()) {
                34.0
            } else {
                24.0
            }
        }
        Col::Layer => 214.0,
        Col::Props => 170.0,
    }
}

fn panel_width(ui: &egui::Ui, panel: &Panel) -> f32 {
    let cols: f32 = panel.cols.iter().map(|c| col_width(ui, c)).sum();
    (cols + PAD * 2.0).max(text_w(ui, panel.title) + 28.0)
}

fn draw_panel(app: &mut CadApp, ui: &mut egui::Ui, panel: &Panel, pr: Rect, action: &mut Option<Action>) {
    let t = Tokens::get();
    let p = ui.painter().clone();
    let content = Rect::from_min_max(pos2(pr.left() + PAD, pr.top() + 3.0), pos2(pr.right() - PAD, pr.bottom() - TITLE_H));
    // Panel title bar, as in the reference program: name and a small arrow.
    let title = Rect::from_min_max(pos2(pr.left(), pr.bottom() - TITLE_H), pr.max);
    p.rect_filled(title.shrink2(vec2(1.0, 1.0)), 0.0, t.chrome_dark.gamma_multiply(0.55));
    p.text(title.center() - vec2(5.0, 0.0), Align2::CENTER_CENTER, panel.title, ribbon_font(), t.text_dim);
    let tw = text_w(ui, panel.title);
    icons::paint(&p, Rect::from_center_size(pos2(title.center().x + tw / 2.0 + 3.0, title.center().y), vec2(8.0, 8.0)), Icon::ChevronDown, false);
    if let Some(list) = extra(panel.title) {
        let tresp = ui.interact(title, ui.id().with(("rb_panel_more", panel.title)), Sense::click());
        if tresp.hovered() {
            p.rect_filled(title.shrink2(vec2(1.0, 1.0)), 0.0, t.control_hover.gamma_multiply(0.35));
        }
        slide_out(ui, &tresp, list, action);
    }

    let mut x = content.left();
    for col in panel.cols {
        let w = col_width(ui, col);
        let cr = Rect::from_min_size(pos2(x, content.top()), vec2(w, content.height()));
        match col {
            Col::Big((icon, cmd, label)) => big_button(ui, cr, *icon, cmd, label, action),
            Col::Small(bs) => {
                for (i, (icon, cmd, label)) in bs.iter().enumerate() {
                    let rr = Rect::from_min_size(pos2(cr.left(), cr.top() + i as f32 * ROW_H + 1.0), vec2(w - 4.0, ROW_H - 2.0));
                    small_button(ui, rr, *icon, cmd, label, true, action);
                }
            }
            Col::Icons(bs) => {
                for (i, (icon, cmd, label)) in bs.iter().enumerate() {
                    let rr = Rect::from_min_size(pos2(cr.left(), cr.top() + i as f32 * ROW_H + 1.0), vec2(w - 4.0, ROW_H - 2.0));
                    small_button(ui, rr, *icon, cmd, label, false, action);
                }
            }
            Col::Layer => layer_column(app, ui, cr, action),
            Col::Props => props_column(app, ui, cr, action),
        }
        x += w;
    }
}

fn big_button(ui: &mut egui::Ui, r: Rect, icon: Icon, cmd: &'static str, label: &'static str, action: &mut Option<Action>) {
    let t = Tokens::get();
    let list = variants(cmd);
    let full = r.shrink2(vec2(2.0, 0.0));
    // A split button: the arrow strip at the foot opens the variants.
    let (main, arrow) = match list {
        Some(_) => {
            let cut = full.bottom() - 12.0;
            (Rect::from_min_max(full.min, pos2(full.right(), cut)), Some(Rect::from_min_max(pos2(full.left(), cut), full.max)))
        }
        None => (full, None),
    };
    let resp = ui.interact(main, ui.id().with(("rb_big", cmd, label)), Sense::click());
    let p = ui.painter().clone();
    if resp.hovered() {
        p.rect_filled(full, 3.0, t.control_hover.gamma_multiply(0.45));
        p.rect_filled(main, 3.0, t.control_hover.gamma_multiply(0.3));
    }
    if let (Some(ar), Some(list)) = (arrow, list) {
        let aresp = ui.interact(ar, ui.id().with(("rb_big_arrow", cmd, label)), Sense::click());
        if aresp.hovered() {
            p.rect_filled(full, 3.0, t.control_hover.gamma_multiply(0.3));
            p.rect_filled(ar, 3.0, t.control_hover.gamma_multiply(0.45));
        }
        icons::paint(&p, Rect::from_center_size(pos2(ar.center().x, ar.center().y + 1.0), vec2(8.0, 8.0)), Icon::ChevronDown, false);
        let _ = aresp.clone().on_hover_text(label.replace('\n', " "));
        variants_popup(ui, &aresp, list, action);
    }
    let p = ui.painter();
    icons::paint(p, Rect::from_center_size(pos2(r.center().x, r.top() + 16.0), vec2(28.0, 28.0)), icon, false);
    for (i, line) in label.lines().enumerate() {
        p.text(pos2(r.center().x, r.top() + 33.0 + i as f32 * 13.0), Align2::CENTER_TOP, line, ribbon_font(), t.text);
    }
    let tip = label.replace('\n', " ");
    icons::describe_control(ui, &resp, &tip, None, false);
    if resp.on_hover_text(tip).clicked() {
        *action = Some(Action::Start(cmd));
    }
}

fn small_button(ui: &mut egui::Ui, r: Rect, icon: Icon, cmd: &'static str, label: &'static str, with_label: bool, action: &mut Option<Action>) {
    let t = Tokens::get();
    let list = variants(cmd);
    // A split button: the small arrow at the right opens the variants.
    let (r, arrow) = match list {
        Some(_) => {
            let text_end = if with_label { r.left() + 22.0 + text_w(ui, label) + 2.0 } else { r.left() + 20.0 };
            let ar = Rect::from_min_max(pos2(text_end, r.top()), pos2((text_end + 12.0).min(r.right()), r.bottom()));
            (Rect::from_min_max(r.min, pos2(text_end, r.bottom())), Some(ar))
        }
        None => (r, None),
    };
    let resp = ui.interact(r, ui.id().with(("rb_small", cmd, label)), Sense::click());
    let p = ui.painter().clone();
    if resp.hovered() {
        p.rect_filled(r, 2.0, t.control_hover.gamma_multiply(0.6));
    }
    if let (Some(ar), Some(list)) = (arrow, list) {
        let aresp = ui.interact(ar, ui.id().with(("rb_small_arrow", cmd, label)), Sense::click());
        if aresp.hovered() {
            p.rect_filled(ar, 2.0, t.control_hover.gamma_multiply(0.6));
        }
        icons::paint(&p, Rect::from_center_size(ar.center(), vec2(7.0, 7.0)), Icon::ChevronDown, false);
        variants_popup(ui, &aresp, list, action);
    }
    let p = ui.painter();
    icons::paint(p, Rect::from_min_size(pos2(r.left() + 2.0, r.center().y - 7.0), vec2(16.0, 16.0)), icon, false);
    if with_label {
        p.text(pos2(r.left() + 22.0, r.center().y), Align2::LEFT_CENTER, label, ribbon_font(), t.text);
    }
    icons::describe_control(ui, &resp, label, None, false);
    if resp.on_hover_text(label).clicked() {
        *action = Some(Action::Start(cmd));
    }
}

/// The list under a split button's arrow: icon and name per variant, as in the reference program.
fn variants_popup(_ui: &mut egui::Ui, anchor: &egui::Response, list: &'static [Btn], action: &mut Option<Action>) {
    egui::Popup::from_toggle_button_response(anchor).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        let t = Tokens::get();
        let w = list.iter().map(|b| text_w(ui, b.2)).fold(120.0, f32::max) + 44.0;
        for (i, (icon, cmd, label)) in list.iter().enumerate() {
            let (r, row) = ui.allocate_exact_size(vec2(w, 28.0), Sense::click());
            if row.hovered() {
                ui.painter().rect_filled(r, 2.0, t.control_hover.gamma_multiply(0.6));
            }
            icons::paint(ui.painter(), Rect::from_min_size(pos2(r.left() + 6.0, r.center().y - 10.0), vec2(20.0, 20.0)), *icon, false);
            ui.painter().text(pos2(r.left() + 34.0, r.center().y), Align2::LEFT_CENTER, *label, ribbon_font(), t.text);
            if i + 1 < list.len() {
                ui.painter().hline(r.left() + 4.0..=r.right() - 4.0, r.bottom(), Stroke::new(1.0, t.border.gamma_multiply(0.5)));
            }
            icons::describe_control(ui, &row, label, None, false);
            if row.clicked() {
                *action = Some(Action::Start(*cmd));
                ui.close();
            }
        }
    });
}

/// A panel's slide-out: its further tools as icons, in rows of seven.
fn slide_out(_ui: &mut egui::Ui, anchor: &egui::Response, list: &'static [Btn], action: &mut Option<Action>) {
    egui::Popup::from_toggle_button_response(anchor).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        for row in list.chunks(7) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
                for (icon, cmd, label) in row {
                    if icons::button(ui, *icon, 26.0, label, false).clicked() {
                        *action = Some(Action::Start(*cmd));
                        ui.close();
                    }
                }
            });
        }
    });
}

/// The current-layer list (bulb, sun, lock, colour, name) and two rows of layer tools.
fn layer_column(app: &mut CadApp, ui: &mut egui::Ui, cr: Rect, action: &mut Option<Action>) {
    let t = Tokens::get();
    let Ok(d) = app.session.doc() else { return };
    let cur = d.header.str("CLAYER", "0");
    let layers: Vec<(String, bool, bool, bool, Color)> = d.layers.iter().map(|l| (l.name.clone(), l.on, l.frozen, l.locked, l.color)).collect();
    let combo = Rect::from_min_size(pos2(cr.left(), cr.top() + 1.0), vec2(cr.width() - 6.0, ROW_H - 2.0));
    let resp = ui.interact(combo, ui.id().with("rb_layer_combo"), Sense::click());
    let p = ui.painter().clone();
    p.rect_filled(combo, 2.0, t.field);
    p.rect_stroke(combo, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    if let Some((name, on, frozen, locked, color)) = layers.iter().find(|l| l.0.eq_ignore_ascii_case(&cur)) {
        let mut x = combo.left() + 4.0;
        for icon in [if *on { Icon::Bulb } else { Icon::BulbOff }, if *frozen { Icon::Snowflake } else { Icon::Sun }, if *locked { Icon::Lock } else { Icon::Unlock }] {
            icons::paint(&p, Rect::from_min_size(pos2(x, combo.center().y - 7.0), vec2(14.0, 14.0)), icon, false);
            x += 16.0;
        }
        crate::layers::swatch(&p, Rect::from_min_size(pos2(x + 2.0, combo.center().y - 5.0), vec2(10.0, 10.0)), *color);
        p.text(pos2(x + 18.0, combo.center().y), Align2::LEFT_CENTER, name, ribbon_font(), t.text);
    }
    icons::paint(&p, Rect::from_center_size(pos2(combo.right() - 9.0, combo.center().y), vec2(10.0, 10.0)), Icon::ChevronDown, false);
    icons::describe_control(ui, &resp, "Layer corrente", None, false);
    egui::Popup::from_toggle_button_response(&resp).width(combo.width()).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
            for (name, on, frozen, locked, color) in &layers {
                ui.horizontal(|ui| {
                    if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 18.0, "Attiva/disattiva", false).clicked() {
                        *action = Some(Action::Run("layer.set", json!({ "name": name, "on": !*on })));
                    }
                    if icons::button(ui, if *frozen { Icon::Snowflake } else { Icon::Sun }, 18.0, "Congela/scongela", false).clicked() {
                        *action = Some(Action::Run("layer.set", json!({ "name": name, "frozen": !*frozen })));
                    }
                    if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 18.0, "Blocca/sblocca", false).clicked() {
                        *action = Some(Action::Run("layer.set", json!({ "name": name, "locked": !*locked })));
                    }
                    let (sr, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                    crate::layers::swatch(ui.painter(), sr, *color);
                    if ui.selectable_label(name.eq_ignore_ascii_case(&cur), name).clicked() {
                        *action = Some(Action::Run("layer.current", json!({ "name": name })));
                        ui.close();
                    }
                });
            }
        });
    });
    // Two rows of layer tools, as on the reference program's Layer panel.
    let rows: [(&[Btn], Btn); 2] = [
        (
            &[(Icon::LayerOff, "layoff", "Disattiva"), (Icon::LayerIso, "layiso", "Isola"), (Icon::LayerFreeze, "layfrz", "Congela"), (Icon::LayerLock, "laylck", "Blocca")],
            (Icon::MakeCurrent, "laymcur", "Rendi corrente"),
        ),
        (
            &[(Icon::Bulb, "layon", "Attiva tutti"), (Icon::LayerPrev, "layuniso", "Rimuovi isolamento"), (Icon::Sun, "laythw", "Scongela tutti"), (Icon::Unlock, "layulk", "Sblocca")],
            (Icon::LayerMatch, "laymch", "Corrispondenza layer"),
        ),
    ];
    for (i, (icons_row, labelled)) in rows.iter().enumerate() {
        let y = cr.top() + (i as f32 + 1.0) * ROW_H + 1.0;
        let mut x = cr.left();
        for (icon, cmd, label) in icons_row.iter() {
            small_button(ui, Rect::from_min_size(pos2(x, y), vec2(19.0, ROW_H - 2.0)), *icon, cmd, label, false, action);
            x += 19.0;
        }
        let (icon, cmd, label) = labelled;
        small_button(ui, Rect::from_min_size(pos2(x + 4.0, y), vec2(cr.right() - x - 10.0, ROW_H - 2.0)), *icon, cmd, label, true, action);
    }
}

/// Colour, linetype and lineweight for new objects ("DaLayer" and so on).
fn props_column(app: &mut CadApp, ui: &mut egui::Ui, cr: Rect, action: &mut Option<Action>) {
    let Ok(d) = app.session.doc() else { return };
    let h = &d.header;
    let aci = h.i64("CECOLOR", 256) as i16;
    let ltype = h.str("CELTYPE", "ByLayer");
    let lw = cadcraft_doc::Lineweight::from_dxf(h.i64("CELWEIGHT", -1) as i16);
    let linetypes: Vec<String> = d.linetypes.iter().map(|l| l.name.clone()).filter(|n| !["ByLayer", "ByBlock"].contains(&n.as_str())).collect();
    let italian = |s: &str| match s.to_ascii_lowercase().as_str() {
        "bylayer" => "DaLayer".to_string(),
        "byblock" => "DaBlocco".to_string(),
        "default" => "Default".to_string(),
        _ => s.to_string(),
    };
    let row = |i: usize| Rect::from_min_size(pos2(cr.left(), cr.top() + i as f32 * ROW_H + 1.0), vec2(cr.width() - 6.0, ROW_H - 2.0));
    let mut set: Option<Action> = None;
    ui.scope_builder(egui::UiBuilder::new().max_rect(row(0)), |ui| {
        ui.spacing_mut().button_padding = vec2(4.0, 1.0);
        let shown = match aci {
            256 => "DaLayer".to_string(),
            0 => "DaBlocco".to_string(),
            n => crate::i18n::t(&Color::from_aci(n).name()).to_string(),
        };
        ui.menu_button(format!("■ {shown}"), |ui| {
            for (n, c) in [
                ("DaLayer", 256),
                ("DaBlocco", 0),
                ("Rosso", 1),
                ("Giallo", 2),
                ("Verde", 3),
                ("Ciano", 4),
                ("Blu", 5),
                ("Magenta", 6),
                ("Bianco", 7),
                ("Grigio", 8),
                ("Grigio chiaro", 9),
            ] {
                if ui.button(n).clicked() {
                    set = Some(Action::Run("color", json!({ "color": c })));
                    ui.close();
                }
            }
        });
    });
    ui.scope_builder(egui::UiBuilder::new().max_rect(row(1)), |ui| {
        ui.spacing_mut().button_padding = vec2(4.0, 1.0);
        ui.menu_button(format!("— {}", italian(&ltype)), |ui| {
            for l in ["ByLayer".to_string(), "ByBlock".to_string()].into_iter().chain(linetypes) {
                if ui.button(italian(&l)).clicked() {
                    set = Some(Action::Run("linetype", json!({ "current": l })));
                    ui.close();
                }
            }
        });
    });
    ui.scope_builder(egui::UiBuilder::new().max_rect(row(2)), |ui| {
        ui.spacing_mut().button_padding = vec2(4.0, 1.0);
        ui.menu_button(format!("≡ {}", italian(&lw.name())), |ui| {
            for n in ["ByLayer", "ByBlock", "Default"] {
                if ui.button(italian(n)).clicked() {
                    set = Some(Action::Run("lweight", json!({ "lineweight": n })));
                    ui.close();
                }
            }
            for v in cadcraft_doc::Lineweight::STANDARD {
                if ui.button(format!("{:.2} mm", f64::from(v) / 100.0)).clicked() {
                    set = Some(Action::Run("lweight", json!({ "lineweight": f64::from(v) / 100.0 })));
                    ui.close();
                }
            }
        });
    });
    if set.is_some() {
        *action = set;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ogni_pulsante_avvia_un_comando_che_esiste() {
        for c in super::commands() {
            let ui_cmd = c.starts_with("ui.");
            assert!(ui_cmd || cadcraft_engine::cmd::find_command(c).is_some(), "il comando {c} non esiste");
        }
    }
}
