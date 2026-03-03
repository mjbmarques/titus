use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus",
        file_line: Some("File: /var/log/system.log (Loaded: 2000..2400 of 46832)"),
        footer_text: Some("Window: 2000..2400 [PgUp/PgDn loads] Cache: 3 windows"),
        log_title: "log view",
        log_lines: sample_log_lines(),
        left_panel: None,
        top_panel: None,
        overlay: None,
        overlay_footer: None,
        theme: theme_dracula(),
    };

    let mut renderer = DemoRenderer::new(96, 28);
    renderer.render(&config);
    renderer.write_svg("../../partial-loading/windowed.svg");
}
