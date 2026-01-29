use demo_support::{DemoConfig, DemoRenderer, sample_log_lines, theme_dracula};

fn main() {
    let config = DemoConfig {
        header_text: "titus [system.log] [access.log] [error.log]",
        file_line: None,
        footer_text: Some("<Search: ^f> <Nav: ↑/↓> <Page: ^↑/^↓> <Quit: q> <Files: alt-tab>"),
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
    renderer.write_svg("../../multiple-files/tabs.svg");
}
