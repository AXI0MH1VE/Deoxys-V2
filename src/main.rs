use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import core modules
mod core;
mod security;
mod deployable;

use core::toon_rs::ToonParser;
use deployable::risk_calculator::RiskCalculator;

#[derive(Clone, serde::Serialize)]
struct AppState {
    toon_parser: Arc<Mutex<ToonParser>>,
    risk_calculator: Arc<Mutex<RiskCalculator>>,
}

#[tauri::command]
async fn parse_toon_data(state: tauri::State<'_, AppState>, data: String) -> Result<String, String> {
    let parser = state.toon_parser.lock().await;
    match parser.parse(&data) {
        Ok(result) => Ok(format!("{:?}", result)),
        Err(e) => Err(format!("TOON parsing error: {}", e)),
    }
}

#[tauri::command]
async fn calculate_risk(state: tauri::State<'_, AppState>, input: String) -> Result<String, String> {
    let calculator = state.risk_calculator.lock().await;
    let result = calculator.calculate_risk(&input);
    Ok(result.to_boot_log())
}

fn main() {
    // Initialize core components
    let toon_parser = Arc::new(Mutex::new(ToonParser::new().expect("Failed to initialize TOON parser")));
    let risk_calculator = Arc::new(Mutex::new(RiskCalculator::new()));

    let app_state = AppState {
        toon_parser,
        risk_calculator,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            parse_toon_data,
            calculate_risk
        ])
        .setup(|app| {
            // Initialize window
            let window = app.get_window("main").unwrap();

            // Set window title
            window.set_title("AxiomHive Sovereign Manifold v2.1.0")?;

            // Apply visual theme
            window.eval(&format!(
                "document.body.style.backgroundColor = '{}';",
                "#000000"
            ))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
