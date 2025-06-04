use std::fs::File;
use std::io::Write;
use std::path::Path;

use image::ImageReader;
use image::Pixel;
use rgb2spec::{ eval_precise, RGB2Spec };

/// Convert 0‒1 sRGB component to linear‑light.
#[inline]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// 31 wavelength samples: 380 nm → 720 nm exclusive, 10 nm step.
const BANDS: [f32; 31] = {
    let mut arr = [0.0_f32; 31];
    let mut i = 0;
    while i < 31 {
        arr[i] = 380.0 + (i as f32) * 10.0;
        i += 1;
    }
    arr
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Hard‑coded assets for this prototype
    let img_path = Path::new("input.jpg");
    let spec_path = Path::new("srgb_64.spec");
    let out_path = Path::new("spectra.bin");

    // 1. Load image and model ------------------------------------------------
    let img = ImageReader::open(img_path)?.decode()?.to_rgb32f();
    let (w, h) = img.dimensions();
    let model = RGB2Spec::load(spec_path)?;

    // 2. Prepare output buffer: (w*h) * 31 bands -----------------------------
    let mut spectra: Vec<f32> = Vec::with_capacity((w * h * 31) as usize);

    // 3. Per‑pixel conversion ------------------------------------------------
    for pixel in img.pixels() {
        let channels = pixel.channels();
        let (r, g, b) = (channels[0], channels[1], channels[2]);

        let rgb_lin = [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b)];

        // 3‑coeff representation
        let coeffs = model.fetch(rgb_lin);

        // Reconstruct 31‑band spectrum
        for &λ in &BANDS {
            spectra.push(eval_precise(coeffs, λ));
        }
    }

    // 4. Write raw little‑endian f32
    let mut file = File::create(out_path)?;
    let bytes = bytemuck::cast_slice(&spectra);
    file.write_all(bytes)?;

    println!("Wrote spectra for {}×{} pixels ({} floats) → {:?}", w, h, spectra.len(), out_path);

    Ok(())
}
