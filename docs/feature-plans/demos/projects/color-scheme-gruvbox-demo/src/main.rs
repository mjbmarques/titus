use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_gruvbox};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: None,
        footer_text: Some("Palette: #282828 #EBDBB2 #928374 #FE8019"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_gruvbox(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../color-scheme/gruvbox.svg");
}
