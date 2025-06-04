use rgb2spec::{ RGB2Spec, eval_precise };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = RGB2Spec::load("srgb_64.spec")?;
    let rgb = [0.8, 0.3, 0.1];
    let coeffs = model.fetch(rgb);
    let λ = 550.0;
    let refl = eval_precise(coeffs, λ);

    println!("Reflectance at {} nm: {}", λ, refl);

    Ok(())
}
