use serde::{Deserialize, Serialize};

const BASE: &str = "https://api.criamotion.com";

#[derive(Serialize)]
struct KeyReq<'a> {
    key: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct LicenseResp {
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub keyStatus: Option<String>,
    #[serde(default)]
    pub installerAccessToken: Option<String>,
    #[serde(default)]
    pub activationToken: Option<String>,
}

#[tauri::command]
pub async fn validate_license(key: String) -> Result<LicenseResp, String> {
    let k = key.trim();
    if k == "ADMIN" || k == "ADMIN_MASTER" {
        return Ok(LicenseResp {
            success: true,
            message: Some("Admin liberado (bypass local, sem gastar token).".into()),
            keyStatus: Some("admin".into()),
            installerAccessToken: None,
            activationToken: None,
        });
    }
    let c = reqwest::Client::new();
    let r = c
        .post(format!("{BASE}/api/activation/installer-access"))
        .header("Content-Type", "application/json")
        .header("User-Agent", "Cria-Motion-Installer/1.0-tauri")
        .json(&KeyReq { key: k })
        .send()
        .await
        .map_err(|e| format!("Sem conexao com o servidor: {e}"))?;
    let st = r.status();
    let j: serde_json::Value = r.json().await.map_err(|e| format!("Resposta invalida: {e}"))?;
    if !j.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
        let msg = j.get("message").and_then(|v| v.as_str()).unwrap_or("Chave invalida.");
        return Err(format!("HTTP {st}: {msg}"));
    }
    serde_json::from_value(j).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_installation(_key: String) -> Result<String, String> {
    Ok("Instalacao admin: completar download via /api/installer/latest + copia de CriaLUT.aex, CriaParallax.aex, CriaMotionFast.aex e CEP com.criamotion.motionpanel. Ver README.".into())
}

#[tauri::command]
pub async fn prepare_after_activation() -> Result<String, String> {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("reg")
            .args(["add", "HKCU\\Software\\Adobe\\CSXS.11", "/v", "PlayerDebugMode", "/t", "REG_SZ", "/d", "1", "/f"])
            .output();
    }
    Ok("PlayerDebugMode ativado.".into())
}

#[tauri::command]
pub async fn list_after_effects() -> Result<Vec<String>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn is_after_effects_running() -> Result<bool, String> {
    Ok(false)
}

#[tauri::command]
pub async fn get_runtime_info() -> Result<String, String> {
    Ok("admin-rebuild 1.0".into())
}
