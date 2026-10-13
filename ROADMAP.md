# Not Word roadmap

**Stage: alpha.** Not Word is an independent fork maintained by Itskorrah. The current goal is
reliable daily writing and document exchange, with a configurable native interface and useful
headless automation. Complete Microsoft Word parity remains an aspiration, not a release claim.

## Priorities

| Priority | Outcome | Evidence needed |
|---|---|---|
| P0 | Preserve users' documents and recovery data | Hostile-input tests, safe-write failures, save/reopen and recovery regressions |
| P1 | Independent Not Word identity | Original attributed assets, correct app/packaging/docs names and retained legal provenance |
| P2 | Correctness and measured performance | Focused tests, representative benchmarks and maintained crate boundaries |
| P3 | Modern, configurable interface | Real UI screenshots, keyboard behavior and preferences migration tests |
| P4 | Better interoperability and document workflows | Synthetic and redistributable DOCX fixtures, round-trip structure checks and useful automation |

Completed fork work and its verification are recorded in
[docs/engineering-log.md](docs/engineering-log.md). Detailed next steps are in
[docs/roadmap.md](docs/roadmap.md). Keep source claims separate from completed validation.

## Release readiness

A registered ribbon command is a measure of catalog breadth. It does not establish fidelity,
error handling, discoverability or performance. The generated
[command checklist](docs/parity-checklist.md) and inherited
[parity assessment](docs/target-app-parity.md) remain useful, but their historical percentages
are not current fork readiness measurements.

Before a beta claim, establish a legally redistributable document corpus; validate open →
edit → save → reopen; test unsupported content preservation; exercise autosave and recovery;
measure large-document layout/rendering; and verify native/browser integrations on their
actual platforms. A Linux headless test cannot certify macOS, Windows, printing or accessibility.

| Area | Detailed evidence / remaining work |
|---|---|
| File exchange | [File-format parity](docs/file-format-parity.md), especially DOCX retained parts and unknown structures |
| Layout and typing | [Layout](docs/layout-parity.md) · [Typing](docs/typing-parity.md) |
| Interface and accessibility | [UI parity](docs/ui-parity.md) · [Customisation](docs/customization.md) |
| Languages and platforms | [Localisation](docs/localization-parity.md) · [Hardware](docs/hardware-parity.md) |
| Automation and architecture | [MCP](docs/mcp.md) · [Architecture](docs/architecture.md) |
| Ranked inherited gaps | [Gap list](docs/gaps.md); re-check each item against current sources |

Builds and releases of upstream WordCraft do not establish published or signed Not Word
artifacts. Source builds are documented in the [README](README.md); the
[release guide](docs/releasing.md) describes the remaining packaging and signing checks.

## Inherited WordCraft progress log

The following dated log is retained from the upstream project. WordCraft names and upstream
issue/PR numbers here describe that history, rather than ownership or delivery by Not Word.
Measurements were made in the environments recorded by the upstream authors and have not
all been re-run for this fork. Current changes belong in the engineering log above.

| Date | What landed |
|---|---|
| 2026-10-12 | RTF import now uses the RTF spec's defaults where a file names none (#902): no space before or after paragraphs, single line spacing, and the `\deff` default font, instead of WordCraft's Normal style; RTF files that relied on the old defaults open tighter, like in Word |
| 2026-10-11 | Right-clicking a table cell offers the table's commands, and as in Word they follow the selection: with the caret in a cell Insert, Delete Cells, Split Cells, Text Direction and Table Properties…; with several cells Merge Cells instead of Split; with whole columns or rows Delete Columns or Rows, Merge Cells and Distribute Rows and Columns; with the whole table Delete Table, Distribute, AutoFit and Insert Caption…. A right-click beside selected cells moves the caret there instead of keeping them |
| 2026-10-11 | Drags, Shift+arrows and Shift+click over table cells select whole cells, as in Word: from one cell into another the rectangle between them (bold, alignment and Delete leave the other cells alone), into or out of a table whole rows; extending again goes on from where the selection started |
| 2026-10-11 | More of the table mouse: arrows above a column, left of a row and along a cell's left edge select it (a drag selects more; a selected column is edited cell by cell, numbering it makes one list), "+" buttons beside the borders insert a row or column, the move handle past the top-left corner selects the table or drags it to float where it's dropped, and a double-click on a column border fits the column; Select Column in the ribbon; `table.selectAt`, `table.insertAt`, `table.moveDrag`, `table.fitColumn` and `table.selectColumn` for agents |
| 2026-10-12 | AutoCorrect keys its built-in English replacement list by language (#835): Polish, Croatian, Bosnian and Serbian `i` ("and") is no longer turned into `I`; English and untagged text unchanged |
| 2026-10-12 | Ribbon groups fit above their labels: View › Show splits its four check boxes into two columns (Hide White Space, #645, had pushed the fourth onto the "Show" label), Developer › Controls loses the gaps that ran into its label, and Layout › Paragraph (#654) no longer trips a debug assertion and sits 2.5 pt higher; a test draws every tab in every language and fails on any group taller than the ribbon |
| 2026-10-12 | Window caption: a non-Latin file name is drawn with a document fallback face (or the CJK UI font), instead of `?` |
| 2026-10-12 | 32-bit Windows starts on OpenGL. DirectX 12 graphics pipeline creation returns "The parameter is incorrect" and the app exited before the window appeared |
| 2026-10-12 | New documents follow the system region (#838): A4 and centimetres outside the US (Letter in the CLDR Letter territories, inches in the US), lengths written with the region's decimal separator (`2,54 cm`) and typed with either; an explicit Measurement units choice wins; `file.options {"region"}` for agents |
| 2026-10-12 | Paste from other programs keeps their formatting (#687): Ctrl+V and Home › Paste read the system clipboard's HTML (and RTF on Windows and Wayland); Merge Formatting takes the destination's font and paragraph and keeps bold, italic, underline and strikethrough; Paste Special lists what the system clipboard holds (HTML, RTF, picture, text). Linux: pictures copied in a browser or Spectacle paste on KDE and other Wayland desktops (data-control clipboard), and a paste that finds nothing usable names what the clipboard holds instead of "empty" (#833) |
| 2026-10-12 | Undo history list (#701): a ▾ beside the Quick Access Toolbar's Undo lists recent steps newest first; hovering highlights what a click undoes ("Undo 3 Actions"), and a click undoes them in one go. `edit.undoTo {count}` and `edit.undoHistory` (newest first, up to 100) for CLI and MCP |
| 2026-10-12 | Start screen (#766): launched with nothing to open, WordCraft starts on File › Home (Blank document, templates, Recent) like Word, desktop and web; Esc leaves a blank document; File › Options › General › Start-up turns it off |
| 2026-10-12 | Word 365 cover pages (#743): a freeform inside a group keeps its shape (it was drawn as a rectangle); text boxes keep their own internal margins and Resize shape to fit text, so a tight title box no longer loses its second line; both survive saving. The missing fills in the report were theme-coloured shapes, fixed by #611 |
| 2026-10-12 | Alt+Q moves to "Tell me" command search, as in Word for Windows (#819); not on macOS, where Option+Q types a character |
| 2026-10-12 | PDF export of long documents with many internal links (a linked table of contents) is linear: NIH's 67-page protocol template 10.6 s → 0.9 s |
| 2026-10-12 | Keytips: Word for Windows' tab letters (File F, Design G, References S, Review R, View W, Help Y) |
| 2026-10-12 | Table-of-contents links look like Word's: plain text, not blue and underlined (Word ignores the Hyperlink style on TOC entry links; measured in 9 cases) |
| 2026-10-12 | Line heights like Word's: spaces and tabs never make a line taller (13 cases measured in Word for Windows); tables of contents with big tab runs were a third taller |
| 2026-10-12 | `.doc` import: tab stops (centred titles, right-aligned dates, leaders) and sections ending at their section marks (continuous breaks, footers); EPA .doc templates now paginate like Word |
| 2026-10-12 | DOCX headings numbered like Word when a heading style gives only a list level (6.4.5, not 6.15) |
| 2026-10-12 | A list label makes its line taller above the baseline only, as in Word (6 cases measured in Word for Windows): Symbol bullets no longer add their deeper descent, so bulleted lists keep Word's height and page breaks |
| 2026-10-12 | Hidden end-of-cell marks (`w:hideMark`) read, written and laid out like Word (9 cases measured in Word for Windows): a cell's empty last paragraph takes no room, so rows of such empty cells shrink to their borders |
| 2026-10-12 | Text frames (`w:framePr`) in headers, footers, cells and text boxes: placed apart with text wrapping around them, and kept on save (a framed footer page number took a line of its own and was lost on save) |
| 2026-10-12 | List indents like Word's when a numbered style has indents of its own: the style giving the list (and styles based on it) beat the level's indents, and a paragraph with its list switched off drops the styles' indents (27 cases measured in Word for Windows); NIH's Heading 3s now sit at their 0.9" |
| 2026-10-12 | Keep with next like Word (20 cases measured in Word for Windows): a kept paragraph moves to the next page only when it would fit on this one by itself, otherwise it breaks as usual; chains of kept paragraphs move together |
| 2026-10-12 | An empty paragraph that only carries a continuous section break takes no room, as in Word (10 cases measured in Word for Windows): text after it follows right on and balanced columns don't count it |
| 2026-10-12 | Floating objects positioned relative to their paragraph start above its space before, as in Word (10 cases measured in Word for Windows) |
| 2026-10-12 | Unequal columns: text after a column break is broken for the column it lands in (NIH's SARP form had a paragraph a word or two wide) |
| 2026-10-12 | Space before at the top of a page like Word after page breaks (32 cases measured): a paragraph ending with a page break keeps its mark with it, the next paragraph gets no space before; page break before drops it in Word 2013 mode |
| 2026-10-12 | Automatic paragraph spacing drops between list items, as in Word (16 cases measured in Word for Windows): bulleted items that ask for automatic space before and after now touch, as in NIH's monitoring plan |
| 2026-10-12 | Rendering like Word for Windows, found by comparing real .gov documents page by page: text is kerned only where Kerning for fonts asks for it (line breaks now match Word's), and automatic paragraph spacing (`w:beforeAutospacing`/`w:afterAutospacing`, common in pasted web text) is 14 pt, dropped at the document's and cells' edges |
| 2026-10-12 | Keyboard context menu (#821): Shift+F10 opens the page's right-click menu below the caret; `ui.contextMenu` |
| 2026-10-12 | The Font list no longer keeps every family it previews in memory (#848): the Font box drew each family's name in its own face by loading the whole family for good, so every trip through the list kept tens of megabytes (1.47 GB after 40 changes on macOS); entries are now drawn from a face read for the preview alone and freed (120 families previewed: +17 MB RSS instead of +411 MB); only fonts actually used load |
| 2026-10-12 | Scrolling a document with one very large table stays smooth (#811): pages are fingerprinted for the texture cache once per layout and only when shown (not every page every frame), the markup-area check is cached per revision, and textures far from the view are dropped. The issue's 1,096-page table: ~600 ms → ~2.4 ms idle frames, ~590 ms → ~4.5 ms scroll frames |
| 2026-10-12 | Search (#819): Alt+Q, or Alt then Q, opens the Search box from anywhere with nothing typed into it; queries match every word; `tools.search` for agents |
| 2026-10-12 | Section breaks (#852): Layout › Breaks with text selected puts the break before the selection and keeps the text, as Word does; it used to delete it, leaving the first section empty |
| 2026-10-12 | Find, Replace and Go To like Word (#712): Ctrl+F types into the Navigation pane's search, one tabbed Find and Replace dialog, Go To by page (absolute or relative) and bookmark; `edit.findReplace`, `edit.goto` `pageBy` |
| 2026-10-12 | Symbol fonts (#776): text in Wingdings, Wingdings 2, Wingdings 3, Webdings and Symbol (proprietary, never bundled) shows its intended symbols — full code tables from Unicode's published correspondence, both storage forms (`w:sym` private-use codes and plain codes); PDF text, copy and TXT export use the Unicode characters; the saved text is unchanged |
| 2026-10-11 | Hide White Space (#645): `view.hideWhiteSpace` (View › Show checkbox, or double-click the gap between pages) collapses margins, headers and footers between pages to a thin line in Print Layout; screen only, page count unchanged |
| 2026-10-11 | Fonts: Palatino Linotype and Book Antiqua fall back to Palatino (macOS) or TeX Gyre Pagella / P052 / URW Palladio (Linux) before generic serif (#670) |
|---|---|
| 2026-10-12 | Track Changes: Accept All after a tracked deletion from the end of one paragraph into a later one no longer joins the paragraph that follows the range, and Reject All after typing into an empty paragraph keeps that paragraph (#864) |
| 2026-10-11 | Table border drags like Word's (#711): Ctrl shares a column border's change among the columns right of it (the table keeps its width), the table's left edge moves a left-aligned table's indent (a centred or right-aligned table keeps its alignment, as in Word), a border of cells selected across moves only in their rows, and Alt shows the column widths on the ruler |
| 2026-10-11 | Tables on the rulers (#700): with the caret in a table the horizontal ruler marks its row's column borders and the vertical ruler its rows' bottoms; dragging a marker resizes the column (the columns right of it move along; with Shift the neighbour makes up the difference) or the row, one undo step, Escape cancels; in a cell the indents and tabs are measured from the cell |
| 2026-10-12 | Save Changes prompt answers from the keyboard: Save has focus when it opens; arrow keys and Tab move between Save, Don't Save and Cancel; Enter answers (#861) |
| 2026-10-12 | Builds with craft-fonts draw simplified-Chinese characters in documents from the bundled Noto Sans CJK SC (it was only used by the interface), so a CJK run gets one face covering it; fixes the FreeBSD job's `east_asian_run_uses_one_face_covering_the_whole_run` failure after #567 |
| 2026-10-12 | Insert › Picture, drag-and-drop and paste take Windows Metafiles (`.wmf`) and Enhanced Metafiles (`.emf`) at their own frame size (#802) |
| 2026-10-12 | AutoCorrect capitals (#544): Correct TWo INitial CApitals (with exceptions), accidental cAPS LOCK, first letter of table cells; each undoable and switchable |
| 2026-10-11 | Turkish (`tr`) interface completed (#774): 2,325 entries, every key of the Russian catalog (equation gallery, built-in style names, watermarks, paper sizes, preview tiles); the interface audit checks Turkish too |
| 2026-10-12 | Navigation headings truncate within the resizable pane, with an ellipsis for long titles and full text on hover (#759) |
| 2026-10-11 | Reading Mode: existing View ribbon button now opens a read-only, viewport-column layout with animated horizontal paging, transient display controls, heading navigation (Navigation pane) and resize anchoring; document layout and export unchanged |
| 2026-10-11 | DOCX open/save keeps inherited complex-script size, bold and italic independent of Latin formatting; explicit values are retained, and font commands still format both scripts. Covered by synthetic round trips and a local GUI save/reopen check. |
| 2026-10-11 | Czech (`cs`) interface catalog: 2,021 entries (the full zh-hans key set), Czech locale detection and persisted language choice |
| 2026-10-12 | Portuguese proofing: pt-BR spelling against the installed Hunspell `pt_BR` dictionary (VERO; read at runtime, not bundled) — the affix engine grew `FLAG UTF-8`, 128 flag bits, continuation classes and `FORBIDDENWORD` to read it — suggestions tuned for Portuguese (missing accents first), conservative grammar rules with Portuguese messages, TeX Portuguese hyphenation patterns (BSD 3-clause, attributed); Review › Language offers Português (Brasil); a pt-br interface writes new documents in pt-BR |
| 2026-10-11 | Polish: interface (`pl`, the thirteenth language, every catalog key) and proofing — spelling against the installed Hunspell `pl_PL` dictionary (read at runtime, not bundled) through a new Hunspell-style affix engine, suggestions tuned for Polish, Polish grammar rules with Polish messages, TeX Polish hyphenation patterns (MIT option); proofing language per run (`w:lang`) for squiggles, F7, suggestions and hyphenation, languages without tools left unmarked; Review › Language menu and the status bar's language; a Polish interface writes new documents in Polish |
| 2026-10-11 | German (`de`) interface (#574): 2,021 translated entries covering the full zh-hans key set, locale and saved-preference tests, bundled-font checks; no German proofing resources |
| 2026-10-12 | Sorting and table numbers follow the text's language (#836, #837): Table › Sort, Home › Sort, the index (with its letter headings), the bibliography and the Table of Authorities use the language's alphabet (Polish `Łódź` between `Kraków` and `Zakopane`, Swedish `å ä ö` after `z`, German `ä` with `a`); table sort and formulas read `1,5` and `1 250,00` as Polish decimals and `=SUM` writes `5,75` |
| 2026-10-11 | `.doc` import keeps pictures and drawings (#551): inline and floating pictures (PNG, JPEG, DIB, EMF, WMF), shapes, lines, groups and text boxes with position and wrap, from [MS-ODRAW] |
| 2026-10-11 | Tables resize with the mouse (#217): drag a column border (the neighbour makes up the difference; Shift moves one column), a row's bottom border, or the handle past the bottom-right corner to scale the table; a dashed guide while dragging, one undo step per drag; `table.resizeDrag` for agents |
| 2026-10-11 | Animation effects setting: File › Options › Accessibility › Animation effects (Use system setting, On, Off). With the system's animations off (Windows' Animation effects, macOS' Reduce motion, the Linux/BSD desktop's setting, a browser's reduced motion) the page no longer smooth-scrolls: wheel notches and Ctrl+wheel zoom land at once, in the document and in every list and pane, and menus and buttons don't fade; `ui.animations` for agents |
| 2026-10-11 | Linux/BSD: Open, Save As, Export and Insert Picture work without a file chooser portal or zenity (labwc, sway…): WordCraft shows its own file browser and says how to get the system dialog; File › Open › Browse in WordCraft… is always there (#576) |
| 2026-10-11 | Every measurement follows File › Options › Advanced › Measurement units (#755): the Line Numbers "From text" distance and the Hyphenation zone, still fixed in inches after #485, now convert like every other dialog |
| 2026-10-11 | Review › Track Changes Options › Show revisions in balloons: tracked deletions in the body leave the text for "Deleted: …" balloons in the margin, joined to where they were (the option was saved but drew nothing new before) |
| 2026-10-11 | Security review fixes: `ui.render`/`ui.screenshot` without a `path` (and the MCP bridge's `render_page`/`screenshot`) return the PNG in the reply instead of writing a fixed name into the shared temporary folder; exported HTML/Markdown write `href` only for web, mail, bookmark and relative links, never `javascript:` and other schemes (one allow-list in `wordcraft_doc::link`) |
| 2026-10-11 | Design tab: the style set gallery gets a box and gaps of its own, so it no longer runs into Colors and Fonts (#638) |
| 2026-10-11 | File › Print: page preview, page range (all, current, selected, custom 1-3,5), odd/even, copies and collate go into the printed PDF (`file.printPdf`, `ui.print` params); printer, duplex and pages per sheet are still the system dialog's (#641, stage 1) |
| 2026-10-11 | A vertically merged table cell (eraser on a horizontal border) lays out, shows and clicks as one cell spanning its rows (#640) |
| 2026-10-11 | Word XML files (#548): Word XML Document (Flat OPC `.xml`) opens and saves through the DOCX reader and writer; Word 2003 XML opens (styles, lists, tables, sections, notes, comments, pictures, hyperlinks) |
| 2026-10-12 | File › Print selects a system printer, copies, duplex and grayscale, submits PDF jobs through craft-print on a worker and shows spooler errors; browser printing and PDF export remain available |
| 2026-10-11 | Font fallback indexes which characters each installed font maps, reading each font file once (#605): text in a script no installed font covers no longer re-reads every system font per distinct character (200 such characters: 13 s → 0.2 s) |
| 2026-10-11 | Find and Replace special characters (`^p`, `^t`, `^#`, `^&` …) and Use wildcards (ranges, counts, word anchors, groups with `\1` in Replace); `^p` finds and splits paragraphs; Special ▾ menu and search options in the dialog (#543) |
| 2026-10-11 | Typing parity (#546): `+---+` Enter makes a table, `> ` `-> ` `=> ` arrow bullets, smileys inside words (`ok:)`), Undo right after a border line brings the characters back |
| 2026-10-11 | Windows: DirectX 12 shaders compile with FXC, built into Windows, so the app no longer quits at launch with "Parent device is lost" when no matching `dxcompiler.dll` + `dxil.dll` pair is around (#603) |
| 2026-10-11 | Insert › SmartArt (#497): list, process, cycle, hierarchy, pyramid, radial, matrix and Venn graphics laid out by our own code, Text Pane, SmartArt Design tab (Add Shape, Promote, Demote, layouts, colours, Reset); saved as DOCX diagram parts with the drawing. Catalog 410/441 (93.0%) |
| 2026-10-11 | Outline view (#493): Outlining tab, outline symbols and indents, collapse/expand, Show Level, first line only, plain text; catalog 428/450 → 441/463 (95.2%) |
| 2026-10-11 | Vertical sections (#491): Layout › Text Direction turns the section (or the cell or text box the caret is in); lines run top to bottom, right to left, ideographs upright; `w:textDirection` in `w:sectPr` read and written |
| 2026-10-11 | File › Options in Word's panes (#485): General, Display, Proofing, Save, Language, Accessibility, Advanced, Agents; initials, individual formatting marks, spelling ignore options, custom dictionary, overtype, typing replaces selection, measurement units, recent-documents count, print options; `file.options` reads and sets them headlessly |
| 2026-10-11 | Right-click Wrap Text submenu on pictures, shapes and text boxes: wrapping styles, Move with Text / Fix Position on Page (#513) |
| 2026-10-11 | Caret animation: keys and typing glide the caret to its new place in 100 ms along the curve measured from Word for Windows, and the blink fades; clicks and scripted moves don't glide. File › Options › Advanced › Animation turns it off, kept between runs; `ui.caretAnimation` for agents |
| 2026-10-11 | Paragraph dialog options (#512): mirror indents (`w:mirrorIndents`, laid out on even pages), same-style spacing, suppress line numbers, don't hyphenate |
| 2026-10-11 | Screen readers can read the document canvas (#488): paragraphs, headings, lists, tables, links, pictures and the caret exposed through AccessKit |
| 2026-10-11 | Add fonts without rebuilding (#582): Home › Font › Add Font… (end of the font list), dropping a TTF/OTF/TTC/WOFF/WOFF2 file, or `font.add`/`font.list`/`font.remove` for agents. Files are checked (50 MB cap, every face must parse and map characters), never replace a bundled or loaded face, and are kept between runs (desktop: `fonts` in the settings folder; web: IndexedDB). Linking a font by URL is still to come |
| 2026-10-11 | Print Markup (#589): File › Print/Export checkbox, `file.printMarkup`, `markup` on save/export commands and `wordcraft-cli convert --markup` print comments (with replies) in balloons beside each page, scaled to the paper like Word's "Document showing markup"; HTML, Markdown and text copies end with the comments |
| 2026-10-11 | Custom colours (#606): every colour drop-down gains Recent Colors and More Colors…, which opens a Colors dialog with a Standard honeycomb and a Custom hue/saturation field, lightness slider, RGB/HSL numbers and a hex box |
| 2026-10-11 | Complex scripts (#604): Myanmar, Khmer and Indic text shaped one script at a time in OpenType fallback fonts (regular weight, not AAT-only fonts), the caret and line breaks never split a syllable, Myanmar wraps between syllables, Backspace removes one sign at a time |
| 2026-10-11 | Any text file opens (#569): `.m3u`/`.m3u8` playlists and other unknown extensions open as plain text with encoding detection (UTF-8, BOMs, UTF-16, Windows-1252), misnamed Word/RTF files by their signature; binary files give a clear error |
| 2026-10-11 | Select by dragging whole units (#549): after a double-click the drag grows by words, after a triple-click by paragraphs, after Ctrl/⌘+click by sentences, as in Word |
| 2026-10-12 | Selection bar (#856): in the left margin beside body text a click selects the line, a double-click the paragraph, a triple-click or Ctrl/⌘+click the document; dragging there selects whole lines and Shift+click extends by lines; the pointer turns to an arrow. Right margin for right-to-left paragraphs not yet |
| 2026-10-11 | Compare / Combine dialog with comparison settings, Restrict Editing pane (exceptions, SHA-512 password, locked styles; DOCX round trip), AutoCorrect Options dialog saved per user (#412) |
| 2026-10-11 | DOCX diagrams (#611): lines, connectors and boxes styled by the theme now show (they were invisible); callouts and flowchart boxes drawn as the nearest shape; VML lines, rectangles and ovals |
| 2026-10-11 | WordArt (#416): text fill/outline/shadow/glow/reflection, Transform warps, our own styles; straight/elbow/curved connectors glued to shapes that re-route; DOCX round trip |
| 2026-10-11 | Content controls (`w:sdt`) kept, shown and edited; Developer tab, Design Mode, Properties dialog (#381) |
| 2026-10-11 | Insert › Chart (#409): column, bar, line, area, pie, doughnut and scatter charts with Chart Design (type, Edit Data, title, legend, data labels), saved as real chart parts; catalog 408/439 (92.9%) |
| 2026-10-11 | Line Numbers, Hyphenation (zone, CAPS, consecutive limit, Manual), Language, Envelopes and Labels dialogs (#407); hyphenation options kept in DOCX; 30 modal dialogs |
| 2026-10-11 | Accessibility (#386): Alt Text pane for every object and table, Mark as decorative, alt text round-trips in DOCX, deeper checker with fix actions |
| 2026-10-11 | References dialogs (#402): Caption (labels, chapter numbering), Index and Mark Entry, Custom Table of Contents, Source Manager and Create/Edit Source, with TOC/INDEX/XE/SEQ switches honoured on update |
| 2026-10-11 | Font dialog: separate Asian text font and Latin text font (#505); `format.font {"eastAsia"}`; East Asian fonts found by the font scan (OS/2 code pages) |
| 2026-10-11 | Document grid (#391): Page Setup › Document Grid sets lines per page and characters per line (`w:docGrid` round trip); lines snap to the line pitch, East Asian characters to the character pitch; View › Gridlines shows it |
| 2026-10-11 | Move tracking (#400): `w:moveFrom`/`w:moveTo` and their named ranges kept in DOCX, a tracked cut and paste of a paragraph or sentence records a move, accept/reject at either end resolves both, green double strikethrough/underline and "Moved from/to" balloons |
| 2026-10-11 | Footnote and Endnote dialog from the References › Footnotes launcher: options per section or document (below text, end of section, number format, start at, restart each section or page), custom marks, convert footnotes ↔ endnotes; kept in DOCX (#385) |
| 2026-10-11 | Shape Format › Text: text direction, Align Text and linked text boxes (Create Link / Break Link), read and written in DOCX (#369) |
| 2026-10-11 | Quick Access Toolbar KeyTips in the customized button order (Alt/F10 then a number), disabled-command checks and a saved File › Options switch (`ui.keytips`), #620 |
| 2026-10-11 | Mail-merge recipients from Excel (.xlsx/.xlsm) and OpenDocument (.ods) spreadsheets, with Select Table to pick the sheet (#334) |
| 2026-10-11 | Insert › Icons (#309): a library of 61 original icons drawn in code, a searchable picker and Icon Color |
| 2026-10-11 | Home › Font › Enclose Characters (#297): circle, square, triangle or diamond around a character, saved as Word's `EQ \o\ac` field |
| 2026-10-11 | Phonetic Guide (#288): ruby text over base text, `w:ruby` round trip |
| 2026-10-12 | Custom XML backing stores preserved as opaque data and related properties through edited DOCX saves; content-control markup is handled by the existing reader/writer and #532 bibliography I/O remains separate; binding evaluation and readiness unchanged |
| 2026-10-12 | Imported TOCs keep actual result boundaries, nested page fields, formatting and locks through DOCX saves without relying on TOC styles; existing engine-managed styled TOCs remain refreshable |
| 2026-10-11 | Customize Ribbon and the Quick Access Toolbar: custom tabs and groups, hide and reorder tabs, add or remove toolbar commands (right-click any button, or the toolbar's … menu); kept between runs (#379, #376) |
| 2026-10-11 | Customize Keyboard: assign, remove and reset keyboard shortcuts; custom keys override built-ins and are kept between runs (#368) |
| 2026-10-12 | Native saves use exclusively created sibling temporary files, synchronize before replacement and clean up on failure; existing temporary-name files/links and original files on failed saves are protected. Synthetic CLI and focused tests on macOS; no new Word parity or readiness claim. |
| 2026-10-11 | Field codes like Word's field keys (#339): show codes (Alt+F9, Shift+F9), edit, unlink, lock and step through fields |
| 2026-10-11 | Word's common keyboard shortcuts (#330): 101 → 125 of ~250 |
| 2026-10-12 | Bulgarian (`bg`) interface: 2,419 catalog entries (every key of the reference catalog, the equation gallery and everything the interface audit finds), `bg-BG` system locales and a saved language choice; locale, catalog and bundled-font tests; no Bulgarian proofing resources |
| 2026-10-11 | Hungarian (`hu`) interface: all 2,419 catalog entries, built-in style names translated, `hu-HU` system locales and a saved language choice; locale, catalog and bundled-font tests; no Hungarian proofing resources |
| 2026-10-11 | Turkish (`tr`) interface: all 1,307 catalog entries, `tr-TR`/`tr-CY` system locales and a saved language choice; command search keeps Turkish dotted and dotless I apart (`italik` finds İtalik); locale, catalog, search and bundled-font tests; no Turkish proofing resources |
| 2026-10-11 | Italian (`it`) interface (#594): 2,399 entries, feature names as Italian Word users know them, locale and saved-preference tests, bundled-font checks; no Italian proofing resources |
| 2026-10-12 | Native drawing-level Word ink preserved through DOCX edits and repeated saves, including its InkML part and optional VML picture fallback; native ink rendering and editing remain missing |
| 2026-10-11 | Citations and bibliography in all 12 of Word's styles: Harvard, ISO 690 (author-date and numerical), Turabian, GB/T 7714, GOST (name and title sort) and SIST02 join APA, MLA, Chicago and IEEE; numbered styles count in order of first citation (#383) |
| 2026-10-11 | Interface size setting (#475): File › Options › General scales the interface 80–200 % on top of the system display scale, kept between runs; `ui.interfaceSize` for agents |
| 2026-10-11 | DOCX paragraph defaults: preserve omission separately from explicit empty defaults, with repeated-save and default-edit regressions |
| 2026-10-11 | Formatting revisions (#41): `w:rPrChange`/`w:pPrChange`, table/row/cell, section and numbering changes kept in DOCX; formatting recorded as revisions while tracking; accept/reject, Reviewing Pane descriptions, "Formatted: …" balloons and change bars |
| 2026-10-11 | Draw: Lasso Select (move, recolour, delete the strokes inside a loop), Add Pen gallery kept between runs, Ink to Shape (own recogniser: line, rectangle/square, ellipse/circle, triangle, polygon), Ink Replay in drawing order with play/pause/rewind/forward; drawing order kept in DOCX (#395); catalog 406/434 (93.5%) |
| 2026-10-10 | Estonian (`et`) interface (#294): 1,161 translated entries, locale and saved-preference tests, bundled-font checks; no Estonian proofing resources |
| 2026-10-10 | Draw tab: pen, pencil and highlighter ink, stroke eraser, Select, Review › Hide Ink; ink saved to DOCX as DrawingML freeforms (#307); catalog 399/431 (92.6%) |
| 2026-10-10 | Draw Table and Eraser (#303): a pen draws one-cell tables and splits cells along drawn lines; the eraser merges the cells beside a border. Catalog 393/431 (91.2%) |
| 2026-10-10 | DOCX charts, SmartArt and OLE objects survive open and save with their parts (#319) |
| 2026-10-10 | Full re-measure against Word for Mac 16.113.4 and progress docs to the craftrules standard (this page, `docs/target-app-parity.md`, `gaps.md`, `roadmap.md`, `architecture.md`, format/layout/UI/hardware/localization parity; `docs/parity.md` → `docs/parity-checklist.md`). Landed the same day (~80 PRs): equations with OMML and an Equation tab (#191); Word 97-2003 `.doc` import (#145); right-to-left and Persian text (#207); Zotero integration (#189); Read Aloud player (#190); text boxes and floating objects editable (#46) and placed like Word (#136); floating tables (#137); track-changes fixes (#125, #244); typing parity with Word (#204); hidden text (#173); save prompts and AutoSave rules (#151); LaTeX import/export (#11); keytips and mini-toolbar (#43); Spanish, Ukrainian and Brazilian Portuguese interfaces; system theme (#249); Linux file dialogs no longer freeze (#246); Chinese UI font on Windows (#248); every installed font weight (#239); cell text direction (#245); Paste Special (#235), Advanced Find (#234), Column Selection (#237), Style Inspector (#236), custom table styles (#256), View gridlines (#243), zoom buttons (#232), inertial touchpad scrolling (#252), File Info fields kept (#268), ¶ in mixed-direction paragraphs (#278), Serbian interface (#250), table Height/Width boxes, AutoFit Contents and Table Properties (#44), AutoSave switch greyed out with the reason where it can't save (.md/.txt/.html, unsaved, browser) (#196, #176), OpenGL fallback on Windows PCs without a DirectX 12 driver (#316), Columns, Symbol and Field dialogs (#321), Tabs and Borders and Shading dialogs (#320); free rotation and flips for pictures, shapes and text boxes with a rotation handle (#332); v0.4.0 released; file associations on Windows and Linux with a Make-default button in Options (#295) |
| 2026-10-09 | Interface languages follow the system or Options (#12; zh-hans, zh-hant, ja); TOC page numbers as an updatable field (#52); rotating log file (#17); window geometry remembered (#38) |
| 2026-10-08 | v0.2.0 and v0.3.0 released; Flatpak and AppImage auto-update (#22) |
| 2026-10-07 | v0.1.0 released; contributor credits in About (#6) |
| 2026-10-06 | Layout arc: wrap around floats, text boxes, page borders, line numbers, row splitting, drop caps, hyphenation, comment balloons; release pipeline for every platform; first estimate (~62% real feature parity, 120–150 h to 100%) |
| 2026-10-05 | M0–M12 first versions: model, layout, render, engine with 200+ commands, Word-style UI, DOCX I/O, PDF and other formats, references, review, mailings, CLI, MCP, web |
