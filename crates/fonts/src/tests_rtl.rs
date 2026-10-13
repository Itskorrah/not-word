//! RTL cluster ordering: preserve the shaper's mark order, with a reproducible timing probe.

use super::*;

/// The previous implementation, retained only as an equivalence and performance reference.
fn legacy_logical_order(glyphs: Vec<ShapedGlyph>) -> Vec<ShapedGlyph> {
    let mut groups: Vec<Vec<ShapedGlyph>> = Vec::new();
    for glyph in glyphs {
        match groups.last_mut() {
            Some(last) if last.first().is_some_and(|first| first.cluster == glyph.cluster) => last.push(glyph),
            _ => groups.push(vec![glyph]),
        }
    }
    groups.sort_by_key(|group| group.first().map_or(0, |first| first.cluster));
    groups.into_iter().flatten().collect()
}

#[test]
fn rtl_cluster_order_preserves_marks_and_mixed_script_runs() {
    // Two descending script runs, including several glyphs at each cluster. Glyph ids identify
    // the original visual order, so a plain reversal or an unstable sort cannot pass this test.
    let clusters = [9, 9, 7, 4, 4, 3, 18, 18, 15, 12, 12];
    let mut glyphs: Vec<_> = clusters
        .into_iter()
        .enumerate()
        .map(|(gid, cluster)| ShapedGlyph { gid: gid as u32, cluster, x_advance: 10, x_offset: gid as i32, y_offset: 3 })
        .collect();
    let expected = legacy_logical_order(glyphs.clone());
    logical_cluster_order(&mut glyphs);
    assert_eq!(glyphs, expected);
    assert_eq!(glyphs.iter().map(|g| g.gid).collect::<Vec<_>>(), [5, 3, 4, 2, 0, 1, 9, 10, 8, 6, 7]);
    logical_cluster_order(&mut []);
}

#[test]
fn rtl_shaping_matches_previous_glyphs_clusters_and_positions() {
    let db = FontDb::global();
    let base = db.face(DEFAULT_FAMILY, "Regular");
    let face = db.fallback_for('ع', base.id()).unwrap_or(base);
    for text in ["العَرَبِيَّةُ لغةٌ جميلةٌ", "שָׁלוֹם עוֹלָם", "العربية עברית 123 Latin (علامات)", "", "\u{200d}\u{200f}\u{2067}ع\u{2069}"]
    {
        for features in [Vec::new(), vec![feature("-liga").unwrap(), feature("-kern").unwrap()]] {
            let expected = legacy_logical_order(shape_dir(&face, text, &features, &|c| c, true));
            assert_eq!(shape_run(&face, text, &features, |c| c, true), expected, "{text:?}");
        }
    }
}

/// Run with `cargo test -p wordcraft-fonts --lib rtl_shaping_benchmark -- --ignored --nocapture`.
/// Compares the exact previous algorithm and the current path in one binary, using identical font
/// data, text and build options. Medians alternate the execution order to reduce warm-up bias.
#[test]
#[ignore = "manual RTL shaping performance measurement"]
fn rtl_shaping_benchmark() {
    use std::hint::black_box;
    use std::time::Instant;

    let db = FontDb::global();
    let base = db.face(DEFAULT_FAMILY, "Regular");
    let face = db.fallback_for('ع', base.id()).unwrap_or(base);
    eprintln!("RTL benchmark face={} {} Arabic={}", face.family, face.style, face.covers('ع'));
    for (name, text) in [
        ("Arabic with marks", "العَرَبِيَّةُ لغةٌ جميلةٌ تُكتَبُ من اليمين إلى اليسار. ".repeat(128)),
        ("Mixed Arabic Hebrew Latin", "العَرَبِيَّةُ שָׁלוֹם text 123 (علامات) ".repeat(128)),
    ] {
        let visual = shape_dir(&face, &text, &[], &|c| c, true);
        let expected = legacy_logical_order(visual.clone());
        assert_eq!(shape_run(&face, &text, &[], |c| c, true), expected);
        for (stage, rounds) in [("order", 200), ("shape", 40)] {
            let mut samples = [Vec::new(), Vec::new()];
            for sample in 0..7 {
                for algorithm in if sample % 2 == 0 { [0, 1] } else { [1, 0] } {
                    let started = Instant::now();
                    for _ in 0..rounds {
                        let glyphs = if stage == "order" { visual.clone() } else { shape_dir(&face, &text, &[], &|c| c, true) };
                        let result = if algorithm == 0 {
                            legacy_logical_order(glyphs)
                        } else {
                            let mut glyphs = glyphs;
                            logical_cluster_order(&mut glyphs);
                            glyphs
                        };
                        black_box(result);
                    }
                    samples[algorithm].push(started.elapsed().as_secs_f64() * 1e6 / f64::from(rounds));
                }
            }
            for sample in &mut samples {
                sample.sort_by(f64::total_cmp);
            }
            let (before, after) = (samples[0][3], samples[1][3]);
            eprintln!(
                "{name}: stage={stage} bytes={} glyphs={} before_us={before:.1} after_us={after:.1} speedup={:.2}x",
                text.len(),
                visual.len(),
                before / after
            );
        }
    }
}
