use deno_core::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum Event {
    WindowCreated,
    DomContentLoaded,
    Undefined,
    Close,
    Suspended,
    Resumed,
}

#[cfg(not(target_os = "linux"))]
impl From<winit::event::Event<'_, ()>> for Event {
    fn from(event: winit::event::Event<()>) -> Self {
        match event {
            winit::event::Event::Suspended => Event::Suspended,
            winit::event::Event::Resumed => Event::Resumed,
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => Event::Close,
            _ => Event::Undefined,
        }
    }
}
