use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "stdlib/"]
pub struct StdLibAsset;
