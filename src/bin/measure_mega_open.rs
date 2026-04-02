use std::process::Command;

const TARGET_FILTER: &str = "test_gen_then_calculate_lines_for_all_chunks_e2e";

fn main() {
    eprintln!(
        "measure_mega_open is a compatibility wrapper. Prefer: cargo run --bin docker_test_runner -- --filter {}",
        TARGET_FILTER
    );

    let status = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "docker_test_runner",
            "--",
            "--filter",
            TARGET_FILTER,
            "--out",
            "result-rg/mega-open-metrics",
        ])
        .status()
        .expect("failed to run docker_test_runner");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
