use super::docs_config::DocsConfig;
use super::document::generate;
use super::markdown::overrides;
use compositor_api::types::{
    DeckLink, Image, ImageSpec, InputStream, Mp4Input, Mp4Output, Rescaler, RtpInput, RtpOutput,
    Shader, ShaderSpec, Text, Tiles, View, WebRendererSpec, WebView,
};
use std::{fs, path::PathBuf};

pub fn generate_docs() {
    let docs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/pages/api/generated");
    if docs_path.exists() {
        fs::remove_dir_all(&docs_path).unwrap();
    }
    fs::create_dir(&docs_path).unwrap();

    let config = DocsConfig::default();

    let mut img_component_config = config.clone();
    img_component_config
        .overrides
        .insert("Image", overrides::force_multiline);

    let renderer_pages = [
        generate::<ShaderSpec>("Shader", &config),
        generate::<ImageSpec>("Image", &config),
        generate::<WebRendererSpec>("WebRenderer", &config),
        generate::<RtpInput>("RtpInputStream", &config),
        generate::<Mp4Input>("Mp4Input", &config),
        generate::<DeckLink>("DeckLink", &config),
    ];

    let component_pages = [
        generate::<Shader>("Shader", &config),
        generate::<InputStream>("InputStream", &config),
        generate::<View>("View", &config),
        generate::<WebView>("WebView", &config),
        generate::<Image>("Image", &img_component_config),
        generate::<Text>("Text", &config),
        generate::<Tiles>("Tiles", &config),
        generate::<Rescaler>("Rescaler", &config),
    ];

    let output_pages = [
        generate::<RtpOutput>("RtpOutputStream", &config),
        generate::<Mp4Output>("Mp4Output", &config),
    ];

    for page in renderer_pages {
        fs::write(
            docs_path.join(format!("renderer-{}.md", page.title)),
            page.markdown,
        )
        .unwrap();
    }
    for page in component_pages {
        fs::write(
            docs_path.join(format!("component-{}.md", page.title)),
            page.markdown,
        )
        .unwrap();
    }
    for page in output_pages {
        fs::write(
            docs_path.join(format!("output-{}.md", page.title)),
            page.markdown,
        )
        .unwrap();
    }
}
