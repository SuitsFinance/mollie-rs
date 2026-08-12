use std::{env, fs::File, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or("missing OpenAPI input path")?;
    let output = args.next().ok_or("missing generated output path")?;

    let spec: openapiv3::OpenAPI = serde_yaml::from_reader(File::open(PathBuf::from(input))?)?;

    let mut settings = progenitor::GenerationSettings::default();
    settings.with_interface(progenitor::InterfaceStyle::Positional);
    settings.with_tag(progenitor::TagStyle::Merged);

    let mut generator = progenitor::Generator::new(&settings);
    let tokens = generator.generate_tokens(&spec)?;
    let syntax = syn::parse2(tokens)?;
    std::fs::write(PathBuf::from(output), prettyplease::unparse(&syntax))?;

    Ok(())
}
