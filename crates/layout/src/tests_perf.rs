//! Reproducible large styled-table layout measurements.

use super::*;

#[test]
fn table_text_keys_reuse_equal_formatting_and_track_changes() {
    use wordcraft_doc::props::{CharProps, ParaProps, Rgb, TextColor};

    let mut keys = TableTextKeys::default();
    let style = para::CellText {
        para: ParaProps { space_after: Some(0.0), ..Default::default() },
        chr: CharProps { font: Some("Source Serif 4".into()), bold: Some(true), ..Default::default() },
    };
    let first = keys.key(&style);
    assert_eq!(first, hash_of(&format!("{style:?}")));
    for _ in 0..1000 {
        assert_eq!(keys.key(&style.clone()), first, "equal content in a new allocation shares the key");
    }
    assert_eq!(keys.entries.len(), 1);
    let mut changed = style.clone();
    changed.chr.color = Some(TextColor::Rgb(Rgb(20, 40, 80)));
    let color_key = keys.key(&changed);
    assert_ne!(color_key, first);
    assert_eq!(color_key, hash_of(&format!("{changed:?}")));
    changed.para.space_after = Some(12.0);
    assert_ne!(keys.key(&changed), color_key, "paragraph formatting also changes the key");
    assert_eq!(keys.key(&style), first, "another table's formatting cannot replace this key");
}

#[test]
fn table_text_keys_stay_bounded_with_many_distinct_styles() {
    let mut keys = TableTextKeys::default();
    for size in 0..2000 {
        let mut style = para::CellText::default();
        style.chr.size = Some(size as f32);
        assert_eq!(keys.key(&style), hash_of(&format!("{style:?}")));
        assert!(keys.entries.len() <= TableTextKeys::MAX_ENTRIES);
    }
}

#[test]
#[ignore = "manual large styled-table layout performance measurement"]
fn styled_table_layout_benchmark() {
    use std::hint::black_box;
    use std::time::Instant;
    use wordcraft_doc::table::{Cell, Table};

    let mut doc = Document::from_text("Before the table\nAfter the table");
    let mut table = Table::new(1000, 8, 468.0);
    table.props.fixed = true;
    for (r, row) in table.rows.iter_mut().enumerate() {
        for (c, cell) in row.cells.iter_mut().enumerate() {
            let width = cell.props.width;
            *cell = Cell::with_text(&format!("Row {r} column {c} value {}", r * 8 + c));
            cell.props.width = width;
        }
    }
    doc.insert_block(StoryRef::Body, &wordcraft_doc::Path::top(1), wordcraft_doc::Block::Table(table)).unwrap();
    let opts = LayoutOptions { proofing: false, ..Default::default() };
    let mut cache = LayoutCache::new();
    let reference = layout(&doc, &mut cache, &opts);
    let pages = reference.pages.len();
    let items: usize = reference.pages.iter().map(|page| page.items.len()).sum();
    let paragraphs = cache.len();
    drop(reference);
    let mut cold = Vec::new();
    let mut warm = Vec::new();
    for _ in 0..7 {
        let mut cold_cache = LayoutCache::new();
        let started = Instant::now();
        let result = layout(black_box(&doc), &mut cold_cache, &opts);
        cold.push(started.elapsed().as_secs_f64() * 1e3);
        assert_eq!(result.pages.len(), pages);
        drop(result);
        let started = Instant::now();
        let result = layout(black_box(&doc), &mut cache, &opts);
        warm.push(started.elapsed().as_secs_f64() * 1e3);
        assert_eq!(result.pages.len(), pages);
        assert_eq!(result.pages.iter().map(|page| page.items.len()).sum::<usize>(), items);
        black_box(result);
    }
    cold.sort_by(f64::total_cmp);
    warm.sort_by(f64::total_cmp);
    eprintln!(
        "styled_table rows=1000 cols=8 pages={pages} items={items} paragraphs={paragraphs} cold_median_ms={:.2} warm_median_ms={:.2}",
        cold[3], warm[3]
    );
}
