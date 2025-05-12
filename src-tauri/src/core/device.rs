use rdev::{exit_grab, grab, is_grabbed, Event, EventType};
use serde::Serialize;
use serde_json::{json, Value};
use tauri::{command, AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub enum DeviceKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceKind,
    value: Value,
}

#[command]
pub fn start_listening(app_handle: AppHandle) -> Result<bool, String> {
    if is_grabbed() {
        return Err("Device listener is already running".to_string());
    }

    let callback = move |event: Event| {
        let device = match event.event_type {
            EventType::ButtonPress(button) => DeviceEvent {
                kind: DeviceKind::MousePress,
                value: json!(format!("{:?}", button)),
            },
            EventType::ButtonRelease(button) => DeviceEvent {
                kind: DeviceKind::MouseRelease,
                value: json!(format!("{:?}", button)),
            },
            EventType::MouseMove { x, y } => DeviceEvent {
                kind: DeviceKind::MouseMove,
                value: json!({ "x": x, "y": y }),
            },
            EventType::KeyPress(key) => DeviceEvent {
                kind: DeviceKind::KeyboardPress,
                value: json!(format!("{:?}", key)),
            },
            EventType::KeyRelease(key) => DeviceEvent {
                kind: DeviceKind::KeyboardRelease,
                value: json!(format!("{:?}", key)),
            },
            _ => return Some(event),
        };

        if let Err(e) = app_handle.emit("device-changed", device) {
            eprintln!("Failed to emit event: {:?}", e);
        }

        Some(event)
    };

    grab(callback).map_err(|err| format!("{:?}", err))?;

    Ok(true)
}

#[command]
pub fn stop_listening() -> Result<bool, String> {
    if !is_grabbed() {
        return Err("Device listener is not running".to_string());
    }

    exit_grab().map_err(|err| format!("{:?}", err))?;

    Ok(true)
}
