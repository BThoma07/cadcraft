//! Italian command names, as typed in the Italian edition of the reference CAD program
//! (LINEA, CERCHIO, TAGLIA, SPOSTA…), and the matching default aliases.
//!
//! A name here is typed instead of the English command id; the English id still works, and a
//! leading underscore (`_LINE`) always means the English name, as in the Italian edition.
//! Command ids that this build doesn't have are left out by [`command`], so an entry for a
//! missing command is harmless.

/// Italian name → command id (lower case).
const NOMI: &[(&str, &str)] = &[
    // Disegna
    ("linea", "line"),
    ("plinea", "pline"),
    ("polilinea", "pline"),
    ("3dpoli", "3dpoly"),
    ("cerchio", "circle"),
    ("arco", "arc"),
    ("rettangolo", "rectang"),
    ("rettang", "rectang"),
    ("poligono", "polygon"),
    ("ellisse", "ellipse"),
    ("xlinea", "xline"),
    ("raggio", "ray"),
    ("punto", "point"),
    ("anello", "donut"),
    ("elica", "helix"),
    ("tratteggio", "hatch"),
    ("sfumatura", "gradient"),
    ("contorni", "boundary"),
    ("regione", "region"),
    ("nuvolarev", "revcloud"),
    ("copri", "wipeout"),
    ("mlinea", "mline"),
    ("dividi", "divide"),
    ("misura", "measure"),
    // Edita
    ("sposta", "move"),
    ("copia", "copy"),
    ("ruota", "rotate"),
    ("scala", "scale"),
    ("specchio", "mirror"),
    ("taglia", "trim"),
    ("estendi", "extend"),
    ("raccordo", "fillet"),
    ("cima", "chamfer"),
    ("raccordacurve", "blend"),
    ("serie", "array"),
    ("serierett", "arrayrect"),
    ("seriepolare", "arraypolar"),
    ("seriepercorso", "arraypath"),
    ("stira", "stretch"),
    ("cancella", "erase"),
    ("esplodi", "explode"),
    ("unisci", "join"),
    ("spezza", "break"),
    ("spezzainpunto", "breakatpoint"),
    ("allunga", "lengthen"),
    ("allinea", "align"),
    ("editpl", "pedit"),
    ("editspline", "splinedit"),
    ("inverti", "reverse"),
    ("corrisprop", "matchprop"),
    ("ordinevis", "draworder"),
    ("eliminadup", "overkill"),
    // Annotazione
    ("testo", "text"),
    ("testom", "mtext"),
    ("edittesto", "textedit"),
    ("tabella", "table"),
    ("stiletabella", "tablestyle"),
    ("stile", "style"),
    ("direttrice", "leader"),
    ("multidir", "mleader"),
    ("stilemultidir", "mleaderstyle"),
    ("quotalinea", "dimlinear"),
    ("quotallinea", "dimaligned"),
    ("quotango", "dimangular"),
    ("quotarco", "dimarc"),
    ("quotaraggio", "dimradius"),
    ("quotadiam", "dimdiameter"),
    ("quotaord", "dimordinate"),
    ("quotacont", "dimcontinue"),
    ("quotalinb", "dimbaseline"),
    ("quotarapida", "qdim"),
    ("stilequota", "dimstyle"),
    ("tolleranza", "tolerance"),
    ("centro", "centermark"),
    ("lineaasse", "centerline"),
    // Layer
    ("isolay", "layiso"),
    ("rimisolay", "layuniso"),
    ("disattlay", "layoff"),
    ("attlay", "layon"),
    ("conglay", "layfrz"),
    ("sconglay", "laythw"),
    ("bloccalay", "laylck"),
    ("sbloccalay", "layulk"),
    ("corrlay", "laymch"),
    ("rendicorrlay", "laymcur"),
    ("statolay", "layerstate.list"),
    ("cancellay", "laydel"),
    ("unisclay", "laymrg"),
    // Blocchi e gruppi
    ("blocco", "block"),
    ("inser", "insert"),
    ("mblocco", "wblock"),
    ("editb", "bedit"),
    ("chiudib", "bclose"),
    ("defatt", "attdef"),
    ("editatt", "attedit"),
    ("sincratt", "attsync"),
    ("gruppo", "group"),
    ("separagruppo", "ungroup"),
    ("xrif", "xref"),
    ("allega", "attach"),
    // Proprietà e utilità
    ("proprieta", "properties"),
    ("proprietà", "properties"),
    ("colore", "color"),
    ("tipolinea", "linetype"),
    ("scalatl", "ltscale"),
    ("spessorelinea", "lweight"),
    ("lista", "list"),
    ("misurageom", "measuregeom"),
    ("propmass", "massprop"),
    ("conteggio", "count"),
    ("calcrapida", "cal"),
    ("trova", "find"),
    ("selezrapida", "qselect"),
    ("selezsimile", "selectsimilar"),
    ("isolaoggetti", "isolateobjects"),
    ("nascondioggetti", "hideobjects"),
    ("fineisolaoggetti", "unisolateobjects"),
    ("unita", "units"),
    ("unità", "units"),
    ("limiti", "limits"),
    ("rinomina", "rename"),
    ("elimina", "purge"),
    ("controllo", "audit"),
    ("recupera", "recover"),
    // Vista
    ("rigen", "regen"),
    ("rigent", "regenall"),
    ("ridis", "redraw"),
    ("vista", "view"),
    ("finestre", "vports"),
    ("mvista", "mview"),
    ("mspazio", "mspace"),
    ("pspazio", "pspace"),
    ("3dorbita", "3dorbit"),
    // Impostazioni
    ("opzioni", "options"),
    ("impostadis", "dsettings"),
    ("orto", "ortho"),
    ("griglia", "grid"),
    ("polare", "polar"),
    ("modtrasc", "pickstyle"),
    // File
    ("nuovo", "new"),
    ("apri", "open"),
    ("salva", "qsave"),
    ("salvarapido", "qsave"),
    ("salvacome", "saveas"),
    ("chiudi", "close"),
    ("chiuditutto", "closeall"),
    ("stampa", "plot"),
    ("anteprima", "preview"),
    ("impostapag", "pagesetup"),
    ("pubblica", "publish"),
    ("esporta", "export"),
    ("esportapdf", "exportpdf"),
    ("importa", "import"),
    ("annulla", "undo"),
    ("ripristina", "redo"),
    ("copiaapp", "copyclip"),
    ("tagliaapp", "cutclip"),
    ("incolla", "pasteclip"),
    ("incollaorig", "pasteorig"),
    ("incollablocco", "pasteblock"),
    ("copiabase", "copybase"),
    // Solidi 2D in pianta
    ("unione", "union"),
    ("sottrai", "subtract"),
    ("interseca", "intersect"),
];

/// Aliases built into the Italian edition: the user's own acad.pgp, loaded with ALIASEDIT or
/// found at startup, comes first. These are the ones of the alias file this edition was made
/// from (`acad.pgp` of the person it was built for).
const ALIAS: &[(&str, &str)] = &[
    ("s", "move"),
    ("c", "copy"),
    ("co", "copy"),
    ("1", "copy"),
    ("tr", "find"),
    ("p", "pline"),
    ("3", "pline"),
    ("w", "matchprop"),
    ("r", "rotate"),
    ("is", "layiso"),
    ("ri", "layuniso"),
    ("sp", "mirror"),
    ("rgt", "regenall"),
    ("q", "dim"),
    ("ce", "circle"),
    ("o", "offset"),
    ("tt", "trim"),
    ("2", "line"),
    ("u", "join"),
    ("e", "extend"),
    ("es", "explode"),
    ("rr", "hatch"),
    ("sc", "scale"),
];

fn exists(id: &str) -> bool {
    super::find_command(id).is_some()
}

/// The command an Italian name stands for (`name` in lower case).
pub fn command(name: &str) -> Option<&'static str> {
    NOMI.iter().find(|(n, _)| *n == name).map(|(_, id)| *id).filter(|id| exists(id))
}

/// The command an alias of the Italian edition stands for (`name` in lower case).
pub fn alias(name: &str) -> Option<&'static str> {
    ALIAS.iter().find(|(n, _)| *n == name).map(|(_, id)| *id).filter(|id| exists(id))
}

/// The Italian name shown for a command id (the first one listed), if it has one.
pub fn name_of(id: &str) -> Option<&'static str> {
    NOMI.iter().find(|(_, i)| *i == id).map(|(n, _)| *n)
}

/// Every Italian name with its command, for AutoComplete.
pub fn names() -> impl Iterator<Item = (&'static str, &'static str)> {
    NOMI.iter().copied().filter(|(_, id)| exists(id))
}

/// Every alias of the Italian edition with its command, for AutoComplete.
pub fn aliases() -> impl Iterator<Item = (&'static str, &'static str)> {
    ALIAS.iter().copied().filter(|(_, id)| exists(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_comandi_piu_usati_esistono() {
        for n in [
            "sposta", "plinea", "copia", "taglia", "corrisprop", "ruota", "xlinea", "estendi", "cerchio", "unisci", "rigent",
            "rimisolay", "specchio", "isolay", "esplodi", "scala", "linea", "blocco", "allinea", "tratteggio",
        ] {
            assert!(command(n).is_some(), "{n} non trovato");
        }
        for (a, _) in ALIAS {
            assert!(alias(a).is_some(), "alias {a} senza comando");
        }
    }

    #[test]
    fn offset_ha_lo_stesso_nome_in_italiano() {
        // OFFSET is OFFSET in the Italian edition too: the English id, no entry needed.
        assert_eq!(super::super::resolve_alias("offset"), "offset");
    }

    #[test]
    fn nessun_nome_duplicato() {
        let mut seen = std::collections::HashSet::new();
        for (n, _) in NOMI {
            assert!(seen.insert(*n), "{n} due volte");
        }
    }
}
