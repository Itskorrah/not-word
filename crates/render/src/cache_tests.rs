//! Decoded-media cache: hits need the same allocation, and the byte budget bounds what is kept.

use super::*;

#[test]
fn media_cache_only_hits_the_same_allocation() {
    let mut c: MediaCache<u32> = MediaCache { map: HashMap::new(), bytes: 0 };
    let a = Arc::new(vec![1u8, 2, 3]);
    let b = Arc::new(vec![1u8, 2, 3]);
    c.insert("pic", &a, 7, 3);
    assert_eq!(c.get("pic", &a), Some(7));
    assert_eq!(c.get("pic", &b), None, "equal bytes in another allocation are another picture");
}

#[test]
fn media_cache_is_bounded_by_bytes() {
    let mut c: MediaCache<u32> = MediaCache { map: HashMap::new(), bytes: 0 };
    let src = Arc::new(vec![0u8]);
    let big = CACHE_BYTES / 2 + 1;
    c.insert("a", &src, 1, big);
    c.insert("b", &src, 2, big);
    assert_eq!(c.get("a", &src), None, "the first entry is evicted to make room");
    assert_eq!(c.get("b", &src), Some(2));
    assert!(c.bytes <= CACHE_BYTES);
    c.insert("huge", &src, 3, CACHE_BYTES + 1);
    assert_eq!(c.get("huge", &src), None, "a value over the whole budget is not kept");
}

#[test]
fn media_cache_counts_the_source_bytes_it_holds() {
    let mut c: MediaCache<Option<u32>> = MediaCache { map: HashMap::new(), bytes: 0 };
    let src = Arc::new(vec![0u8; 1000]);
    c.insert("broken", &src, None, 0);
    assert_eq!(c.bytes, 1000, "an entry that decoded to nothing still holds its source");
    c.insert("pic", &src, Some(1), 24);
    assert_eq!(c.bytes, 2024);
    // A source over the whole budget is not kept, whatever it decoded to (calloc'd, so cheap).
    let huge = Arc::new(vec![0u8; CACHE_BYTES + 1]);
    c.insert("huge", &huge, None, 0);
    assert!(!c.map.contains_key("huge"));
    assert_eq!(CACHE_BYTES * 2, 256 * 1024 * 1024, "the image and vector caches share about 256 MB");
}

/// A wave stroke starting around 1e20 never advanced (`x + step == x`), pushing curves until memory ran out.
#[test]
fn wave_stroke_at_a_huge_coordinate_ends() {
    assert!(wave_path(1e20, 2e20, 0.0, 1.0).elements().len() < 3);
    assert!(wave_path(0.0, 1e20, 0.0, 1.0).elements().len() < 20_100);
    assert!(wave_path(0.0, 100.0, 0.0, 1.0).elements().len() > 10);
}

#[test]
fn malformed_rendered_buffers_fail_without_padding_or_panicking() {
    for image in [
        Rendered::default(),
        Rendered { width: u32::MAX, height: u32::MAX, pixels: Vec::new() },
        Rendered { width: 16_000, height: 16_000, pixels: vec![255; 4] },
        Rendered { width: 1, height: 1, pixels: vec![255; 3] },
        Rendered { width: 1, height: 1, pixels: vec![255; 8] },
    ] {
        assert!(image.to_png().is_empty());
        assert!(image.to_jpeg(90).is_empty());
    }
    let hostile = Rendered { width: u32::MAX, height: u32::MAX, pixels: Vec::new() };
    assert_eq!(hostile.pixel(u32::MAX - 1, u32::MAX - 1), [0; 4]);
    assert_eq!(hostile.pixel(0, 0), [0; 4]);
}

#[test]
fn valid_translucent_pixels_survive_png_and_jpeg_export() {
    let rendered = Rendered { width: 2, height: 1, pixels: vec![64, 32, 0, 128, 0, 0, 0, 0] };
    assert_eq!(rendered.pixel(0, 0), [128, 64, 0, 128]);
    assert_eq!(rendered.pixel(1, 0), [0; 4]);
    let decoded = image::load_from_memory(&rendered.to_png()).unwrap().into_rgba8();
    assert_eq!(decoded.dimensions(), (2, 1));
    assert_eq!(decoded.get_pixel(0, 0).0, [128, 64, 0, 128]);
    let jpeg = image::load_from_memory(&rendered.to_jpeg(90)).unwrap();
    assert_eq!((jpeg.width(), jpeg.height()), (2, 1));
    let pixmap = decode_pixmap(&rendered.to_png()).unwrap();
    assert_eq!((pixmap.width(), pixmap.height()), (2, 1));
}

#[test]
fn pixmap_rejects_dimensions_it_cannot_represent() {
    // A valid, highly compressible PNG with a narrow row: old decoding converted and copied
    // its pixels before discovering the width did not fit the pixmap's u16 dimensions.
    let mut png = Vec::new();
    image::GrayImage::new(u16::MAX as u32 + 1, 1).write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png).unwrap();
    assert_eq!(image_size(&png), Some((u16::MAX as u32 + 1, 1)));
    assert!(decode_pixmap(&png).is_none());
    assert!(premul_pixmap(u32::MAX, u32::MAX, &[]).is_none());
}

/// `cargo test -p wordcraft-render --lib image_decode_benchmark -- --ignored --nocapture`.
/// The old path copied an already-RGBA decoded image before premultiplying it. Compare both
/// complete decode paths on valid media in one binary, so trust limits and format setup count too.
#[test]
#[ignore = "manual image decoding performance measurement"]
fn image_decode_benchmark() {
    use std::hint::black_box;
    use std::time::Instant;

    let image = image::RgbaImage::from_fn(2048, 1024, |x, y| image::Rgba([x as u8, y as u8, (x ^ y) as u8, 180]));
    for format in [image::ImageFormat::Png, image::ImageFormat::Bmp] {
        let mut encoded = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut encoded), format).unwrap();
        let mut samples = [Vec::new(), Vec::new()];
        for sample in 0..7 {
            for algorithm in if sample % 2 == 0 { [0, 1] } else { [1, 0] } {
                let started = Instant::now();
                for _ in 0..10 {
                    let pixmap = if algorithm == 0 {
                        let rgba = image::load_from_memory(black_box(&encoded)).unwrap().to_rgba8();
                        premul_pixmap(rgba.width(), rgba.height(), rgba.as_raw()).unwrap()
                    } else {
                        decode_pixmap(black_box(&encoded)).unwrap()
                    };
                    assert_eq!((pixmap.width(), pixmap.height()), (2048, 1024));
                    black_box(pixmap);
                }
                samples[algorithm].push(started.elapsed().as_secs_f64() * 1e3 / 10.0);
            }
        }
        for sample in &mut samples {
            sample.sort_by(f64::total_cmp);
        }
        let (before, after) = (samples[0][3], samples[1][3]);
        eprintln!("decode format={format:?} pixels=2097152 before_ms={before:.2} after_ms={after:.2} speedup={:.2}x", before / after);
    }
}
