//! `copythat config get|set|reset <key>`. Manipulates the persistent
//! `settings.toml` via `copythat_settings::Settings`.
//!
//! Keys are dotted paths into the settings tree (e.g.
//! `transfer.buffer_size`). Backing storage is TOML, so reading the
//! value is just `toml::from_str` → walk the table tree → emit JSON;
//! writing is the reverse. The CLI deliberately does not enforce a
//! schema beyond "must round-trip through Settings"; a malformed TOML
//! after the round-trip is rejected.

use std::path::PathBuf;
use std::sync::Arc;

use copythat_settings::Settings;

use crate::ExitCode;
use crate::cli::{ConfigArgs, ConfigOp, GlobalArgs};
use crate::output::{JsonEventKind, OutputWriter};

pub(crate) async fn run(
    global: &GlobalArgs,
    args: ConfigArgs,
    writer: Arc<OutputWriter>,
) -> ExitCode {
    let path = match resolve_config_path(global) {
        Ok(p) => p,
        Err(code) => return code,
    };

    let mut settings = match Settings::load_from(&path) {
        Ok(s) => s,
        Err(e) => {
            let _ = writer.emit(JsonEventKind::Error {
                message: format!("load settings: {e}"),
                code: ExitCode::ConfigInvalid.as_u8(),
            });
            return ExitCode::ConfigInvalid;
        }
    };

    match args.op {
        ConfigOp::Get { key } => match get_key(&settings, &key) {
            Some(value) => {
                let _ = writer.emit(JsonEventKind::ConfigValue {
                    key: key.clone(),
                    value: value.clone(),
                });
                let _ = writer.human(&format!("{key} = {value}"));
                ExitCode::Success
            }
            None => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: format!("unknown config key `{key}`"),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                ExitCode::ConfigInvalid
            }
        },
        ConfigOp::Set { key, value } => match set_key(&mut settings, &key, &value) {
            Ok(()) => {
                if let Err(e) = settings.save_to(&path) {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("save settings: {e}"),
                        code: ExitCode::GenericError.as_u8(),
                    });
                    return ExitCode::GenericError;
                }
                let _ = writer.human(&format!("set {key} = {value}"));
                ExitCode::Success
            }
            Err(msg) => {
                let _ = writer.emit(JsonEventKind::Error {
                    message: msg,
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                ExitCode::ConfigInvalid
            }
        },
        ConfigOp::Reset { all, key } => {
            if all {
                if let Err(e) = Settings::default().save_to(&path) {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("reset settings: {e}"),
                        code: ExitCode::GenericError.as_u8(),
                    });
                    return ExitCode::GenericError;
                }
                let _ = writer.human("settings reset to defaults");
                ExitCode::Success
            } else if let Some(key) = key {
                let defaults = Settings::default();
                let Some(default_value) = get_key(&defaults, &key) else {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("unknown config key `{key}`"),
                        code: ExitCode::ConfigInvalid.as_u8(),
                    });
                    return ExitCode::ConfigInvalid;
                };
                let value_str = match &default_value {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                if let Err(msg) = set_key(&mut settings, &key, &value_str) {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: msg,
                        code: ExitCode::ConfigInvalid.as_u8(),
                    });
                    return ExitCode::ConfigInvalid;
                }
                if let Err(e) = settings.save_to(&path) {
                    let _ = writer.emit(JsonEventKind::Error {
                        message: format!("save settings: {e}"),
                        code: ExitCode::GenericError.as_u8(),
                    });
                    return ExitCode::GenericError;
                }
                let _ = writer.human(&format!("reset {key} to default"));
                ExitCode::Success
            } else {
                let _ = writer.emit(JsonEventKind::Error {
                    message: "reset requires --all or a key".into(),
                    code: ExitCode::ConfigInvalid.as_u8(),
                });
                ExitCode::ConfigInvalid
            }
        }
    }
}

fn resolve_config_path(global: &GlobalArgs) -> Result<PathBuf, ExitCode> {
    if let Some(p) = &global.config {
        return Ok(p.clone());
    }
    Settings::default_path().map_err(|_| ExitCode::ConfigInvalid)
}

/// Walk the settings TOML tree and return the leaf as a JSON value.
fn get_key(settings: &Settings, key: &str) -> Option<serde_json::Value> {
    let toml_str = toml::to_string_pretty(settings).ok()?;
    let value: toml::Value = toml::from_str(&toml_str).ok()?;
    let mut cursor: &toml::Value = &value;
    for segment in key.split('.') {
        cursor = match cursor {
            toml::Value::Table(t) => t.get(segment)?,
            _ => return None,
        };
    }
    Some(toml_value_to_json(cursor))
}

fn set_key(settings: &mut Settings, key: &str, value: &str) -> Result<(), String> {
    let toml_str =
        toml::to_string_pretty(settings).map_err(|e| format!("serialize settings: {e}"))?;
    let mut value_root: toml::Value =
        toml::from_str(&toml_str).map_err(|e| format!("parse settings: {e}"))?;

    let parsed_value = parse_value(value)?;
    let segments: Vec<&str> = key.split('.').collect();
    if segments.is_empty() {
        return Err("empty config key".into());
    }
    insert_into(&mut value_root, &segments, parsed_value)?;

    let updated =
        toml::to_string_pretty(&value_root).map_err(|e| format!("re-serialize settings: {e}"))?;
    let parsed: Settings = toml::from_str(&updated)
        .map_err(|e| format!("config key `{key}` produced an invalid Settings tree: {e}"))?;
    *settings = parsed;
    Ok(())
}

fn insert_into(
    cursor: &mut toml::Value,
    segments: &[&str],
    value: toml::Value,
) -> Result<(), String> {
    let toml::Value::Table(table) = cursor else {
        return Err(format!(
            "config segment `{}` is not a table",
            segments.first().copied().unwrap_or_default()
        ));
    };
    let head = segments[0];
    if segments.len() == 1 {
        table.insert(head.to_string(), value);
        return Ok(());
    }
    let entry = table
        .entry(head.to_string())
        .or_insert_with(|| toml::Value::Table(toml::value::Table::new()));
    insert_into(entry, &segments[1..], value)
}

fn parse_value(raw: &str) -> Result<toml::Value, String> {
    if let Ok(b) = raw.parse::<bool>() {
        return Ok(toml::Value::Boolean(b));
    }
    if let Ok(i) = raw.parse::<i64>() {
        return Ok(toml::Value::Integer(i));
    }
    if let Ok(f) = raw.parse::<f64>() {
        return Ok(toml::Value::Float(f));
    }
    Ok(toml::Value::String(raw.to_string()))
}

fn toml_value_to_json(v: &toml::Value) -> serde_json::Value {
    match v {
        toml::Value::String(s) => serde_json::Value::String(s.clone()),
        toml::Value::Integer(i) => serde_json::Value::Number((*i).into()),
        toml::Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        toml::Value::Boolean(b) => serde_json::Value::Bool(*b),
        toml::Value::Datetime(d) => serde_json::Value::String(d.to_string()),
        toml::Value::Array(a) => {
            serde_json::Value::Array(a.iter().map(toml_value_to_json).collect())
        }
        toml::Value::Table(t) => serde_json::Value::Object(
            t.iter()
                .map(|(k, v)| (k.clone(), toml_value_to_json(v)))
                .collect(),
        ),
    }
}
