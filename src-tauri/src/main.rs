use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::process::Command;
use serde::{Deserialize, Serialize};

use toon_rs::ToonParser;
use axiom_risk_calculator::RiskCalculator;

#[derive(Clone)]
struct AppState {
    risk_calculator: Arc<Mutex<RiskCalculator>>,
}

#[derive(Serialize, Deserialize)]
struct MambaModelResult {
    output: String,
    metrics: Option<serde_json::Value>,
    risk_score: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct FHEResult {
    ciphertext: String,
    keys: String,
}

#[tauri::command]
async fn parse_toon_data(data: String) -> Result<String, String> {
    // Create parser with the input data
    let parser = ToonParser::new(&data);
    match parser.parse() {
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

#[tauri::command]
async fn run_mamba_model(
    prompt: String,
    state_dim: u32,
    input_dim: u32,
    temperature: f64,
    frozen_seed: bool,
) -> Result<MambaModelResult, String> {
    // Call Python script to run Mamba-2 model
    let script_path = std::env::current_dir()
        .unwrap()
        .join("src")
        .join("core")
        .join("mamba_runner.py");
    
    let output = Command::new("python")
        .arg(script_path)
        .arg("--prompt")
        .arg(&prompt)
        .arg("--state-dim")
        .arg(state_dim.to_string())
        .arg("--input-dim")
        .arg(input_dim.to_string())
        .arg("--temperature")
        .arg(temperature.to_string())
        .arg("--frozen-seed")
        .arg(if frozen_seed { "true" } else { "false" })
        .output()
        .map_err(|e| format!("Failed to run Mamba model: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Mamba model error: {}", error));
    }

    let result_str = String::from_utf8_lossy(&output.stdout);
    let result: MambaModelResult = serde_json::from_str(&result_str)
        .map_err(|e| format!("Failed to parse model output: {}", e))?;

    Ok(result)
}

#[tauri::command]
async fn encrypt_fhe(message: i32) -> Result<FHEResult, String> {
    let script_path = std::env::current_dir()
        .unwrap()
        .join("src")
        .join("security")
        .join("fhe_runner.py");
    
    let output = Command::new("python")
        .arg(script_path)
        .arg("--action")
        .arg("encrypt")
        .arg("--message")
        .arg(message.to_string())
        .output()
        .map_err(|e| format!("Failed to run FHE encryption: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FHE encryption error: {}", error));
    }

    let result_str = String::from_utf8_lossy(&output.stdout);
    let result: FHEResult = serde_json::from_str(&result_str)
        .map_err(|e| format!("Failed to parse FHE result: {}", e))?;

    Ok(result)
}

#[tauri::command]
async fn decrypt_fhe(ciphertext: String, keys: String) -> Result<i32, String> {
    let script_path = std::env::current_dir()
        .unwrap()
        .join("src")
        .join("security")
        .join("fhe_runner.py");
    
    let output = Command::new("python")
        .arg(script_path)
        .arg("--action")
        .arg("decrypt")
        .arg("--ciphertext")
        .arg(&ciphertext)
        .arg("--keys")
        .arg(&keys)
        .output()
        .map_err(|e| format!("Failed to run FHE decryption: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FHE decryption error: {}", error));
    }

    let result_str = String::from_utf8_lossy(&output.stdout);
    let plaintext: i32 = result_str.trim().parse()
        .map_err(|e| format!("Failed to parse decrypted result: {}", e))?;

    Ok(plaintext)
}

#[tauri::command]
async fn get_system_status() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "toon_parser": "READY",
        "mamba_core": "READY",
        "deoxys_fhe": "READY",
        "risk_calculator": "READY",
        "entropy_count": 1,
        "risk_score": 0
    }))
}

fn main() {
    // Initialize core components
    let risk_calculator = Arc::new(Mutex::new(RiskCalculator::new()));

    let app_state = AppState {
        risk_calculator,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            parse_toon_data,
            calculate_risk,
            run_mamba_model,
            encrypt_fhe,
            decrypt_fhe,
            get_system_status
        ])
        .setup(|app| {
            // Initialize window
            let window = app.get_window("main").unwrap();

            // Set window title
            window.set_title("AxiomHive Sovereign Manifold v2.1.0")?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

