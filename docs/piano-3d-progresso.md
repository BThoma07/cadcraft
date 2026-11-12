# Edizione italiana stile AutoCAD — avanzamento

Appunti di lavoro del ramo `autocad-it`. Il comportamento di riferimento si osserva solo
dall'esterno (clean-room): nessun file, testo della guida, icona o immagine del programma di
riferimento entra nel repository.

## Richieste 2D (prima del 3D)

| # | Richiesta | Stato |
|---|-----------|-------|
| 1 | Velocità mentre si disegna | In verifica. Prova automatica `velocita_disegno`: con 2000 linee e LINEA attiva il lavoro del processore per fotogramma è circa 1 ms (mediana) su 2560x1440, quindi il ritardo non nasce dal calcolo dell'anteprima. Presentazione senza vsync a bassa latenza, limite fotogrammi 1 ms su Windows (it-21), GRAPHICSCONFIG mostra scheda grafica e tempi. **Manca:** prova dell'utente sul suo PC con l'ultima versione installata, confronto affiancato. |
| 2 | Aspetto "come AutoCAD" | In corso: barra del titolo con accesso rapido e ricerca, riga di comando grigia mobile, icone grandi colorate del Ribbon (disegni nostri). **Manca:** confronto sullo stesso schermo di sfumature dei pannelli, schede dei file, barra di stato, ViewCube e barra di navigazione. |
| 3 | Dimensioni identiche | In corso: misure prese il 21 ottobre a 2560x1440 (Ribbon, schede, palette). **Manca:** verifica con zoom su tutte le zone. |
| 4 | Palette Proprietà | Fatto il 12 novembre: punto, raggio e xlinea (punto base modificabile), spline (metodo, grado, vertici, lunghezza, area), direttrice, multidirettrice (dimensione freccia modificabile), tabella, finestra (Attiva, Visualizzazione bloccata e Scala personalizzata modificabili), regione (area, perimetro); quote con **Unità alternative** e **Tolleranze** modificabili; ellisse (centro e raggi), inclinazione del testo ed elevazione della polilinea ora modificabili. **Manca:** confronto riga per riga con il programma di riferimento (nomi e ordine). |
| 5 | CORRISPROP e layer | Da fare. |
| 6 | Prova dei comandi uno per uno | Da fare. |

## Piano 3D

Dopo le richieste 2D: fondamenta 3D (Z, UCS, ORBITA 3D, viste, ViewCube), ESTRUDI e stili di
visualizzazione, solidi base e modifiche 3D, booleane (motore B-rep in Rust), RIVOLUZIONE /
LOFT / SWEEP, sezioni e viste, materiali e render.

## Registro

- 12/11/2026: Proprietà dei tipi mancanti e categorie Unità alternative / Tolleranze delle
  quote (`crates/ui-egui/src/palettes.rs`, `properties.set` in
  `crates/engine/src/cmd/props.rs`), prove `proprieta_*` e schermate
  `autocad-it-proprieta-*.png`. Il PC dell'utente era bloccato: nessun confronto dal vivo.
