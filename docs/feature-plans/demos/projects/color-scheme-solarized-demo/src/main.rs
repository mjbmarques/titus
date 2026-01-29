use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_solarized};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: None,
        footer_text: Some("Palette: #002B36 #839496 #586E75 #268BD2"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_solarized(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../color-scheme/solarized.svg");
}
