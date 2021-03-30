use deno_core::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum Event {
    Undefined,
    Close,
    Suspended,
    Resumed,
    MainEventsCleared,
    RedrawRequested,
    RedrawEventsCleared,
    LoopDestroyed,
}

impl From<winit::event::Event<'_, ()>> for Event {
    fn from(event: winit::event::Event<()>) -> Self {
        match event {
            winit::event::Event::Suspended => Event::Suspended,
            winit::event::Event::Resumed => Event::Resumed,
            winit::event::Event::MainEventsCleared => Event::MainEventsCleared,
            winit::event::Event::RedrawRequested(_) => Event::RedrawRequested,
            winit::event::Event::RedrawEventsCleared => Event::RedrawEventsCleared,
            winit::event::Event::LoopDestroyed => Event::LoopDestroyed,
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => Event::Close,
            _ => Event::Undefined,
        }
    }
}
