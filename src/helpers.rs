use deno_core::serde::{Deserialize, Serialize};
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::dpi::Size;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum WebViewStatus {
    Initialized,
    WindowCreated,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "Size")]
pub enum SizeDef {
    Physical(PhysicalSize<u32>),
    Logical(LogicalSize<f64>),
}
