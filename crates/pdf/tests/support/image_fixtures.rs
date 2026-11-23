//! Original procedural CMYK TIFFs shared by conversion and public-export contract tests.

pub const INKS: [u8; 8] = [0, 0, 0, 255, 0, 255, 255, 0];
pub const ALPHA: [u8; 2] = [255, 128];

pub fn cmyk_tiff(alpha: Option<u16>, other_inks: bool) -> Vec<u8> {
    use tiff::encoder::colortype::{CMYK8, CMYKA8};
    use tiff::tags::Tag;
    let mut bytes = Vec::new();
    let mut encoder = tiff::encoder::TiffEncoder::new(std::io::Cursor::new(&mut bytes)).unwrap();
    if let Some(kind) = alpha {
        let mut image = encoder.new_image::<CMYKA8>(2, 1).unwrap();
        image.encoder().write_tag(Tag::ExtraSamples, &[kind][..]).unwrap();
        // Associated alpha (1) stores each ink multiplied by alpha: 255 at alpha 128 is 128.
        let ink = if kind == 1 { 128 } else { 255 };
        image.write_data(&[0, 0, 0, 255, ALPHA[0], 0, ink, ink, 0, ALPHA[1]]).unwrap();
    } else {
        let mut image = encoder.new_image::<CMYK8>(2, 1).unwrap();
        if other_inks {
            image.encoder().write_tag(Tag::Unknown(332), 2u16).unwrap();
        }
        image.write_data(&INKS).unwrap();
    }
    bytes
}

/// A CMYK PSD (2×1): pixel 0 is 100% K, pixel 1 full CMY; no alpha. Straight inks in (0 = no
/// ink); the builder stores them inverted as Photoshop does, so a decode gives `INKS` back.
#[allow(dead_code)] // the in-test module compiles into both the lib tests and the `it` suite
pub fn cmyk_psd() -> Vec<u8> {
    use craft_psd::{Compression, PixelData, PsdBuilder, header::ColorMode};
    let pixels = PixelData::Cmyka8(vec![0, 0, 0, 255, 255, 0, 255, 255, 0, 255]);
    PsdBuilder::new(2, 1).color_mode(ColorMode::Cmyk).compression(Compression::Raw).composite(pixels).build().unwrap().to_bytes().unwrap()
}
