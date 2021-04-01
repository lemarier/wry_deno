use deno_core::serde::{Deserialize, Serialize};
#[cfg(not(target_os = "linux"))]
use winit::dpi::{LogicalSize, PhysicalSize, Size};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum WebViewStatus {
    Initialized,
    WindowCreated,
}
#[cfg(not(target_os = "linux"))]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "Size")]
pub enum SizeDef {
    Physical(PhysicalSize<u32>),
    Logical(LogicalSize<f64>),
}
