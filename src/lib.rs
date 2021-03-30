use std::cell::RefCell;
use std::collections::HashMap;

use deno_core::error::anyhow;
use deno_core::error::AnyError;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;
use deno_core::serde_json::json;
use deno_core::serde_json::Value;
use deno_json_op::json_op;

use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use wry::webview::{WebView, WebViewBuilder};

mod event;
mod helpers;
use event::Event;

thread_local! {
  static INDEX: RefCell<u64> = RefCell::new(0);
  static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoop::new());
  static WEBVIEW_MAP: RefCell<HashMap<u64, WebView>> = RefCell::new(HashMap::new());
  static STACK_MAP: RefCell<HashMap<u64, Vec<event::Event>>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("wry_new", wry_new);
    interface.register_op("wry_loop", wry_loop);
    interface.register_op("wry_step", wry_step);
}

#[json_op]
fn wry_new(json: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
    let url = json["url"].as_str().unwrap();

    let mut id = 0;
    INDEX.with(|cell| {
        id = cell.replace_with(|&mut i| i + 1);
    });

    WEBVIEW_MAP.with(|cell| {
        let mut webviews = cell.borrow_mut();
        EVENT_LOOP.with(|cell| {
            let event_loop = cell.borrow();
            let window = Window::new(&event_loop)?;
            let webview = WebViewBuilder::new(window)
                .unwrap()
                .initialize_script("menacing = 'ゴ';")
                .load_url(url)?
                .build()?;

            webviews.insert(id, webview);
            STACK_MAP.with(|cell| {
                cell.borrow_mut().insert(id, Vec::new());
            });

            Ok(json!(id))
        })
    })
}

#[json_op]
fn wry_loop(json: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
    let id = json["id"].as_u64().unwrap();

    //println!("ID {}", id);
    let mut should_stop_loop = false;
    EVENT_LOOP.with(|cell| {
        let event_loop = &mut *cell.borrow_mut();
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Exit;

            WEBVIEW_MAP.with(|cell| {
                let webview_map = cell.borrow();

                if let Some(webview) = webview_map.get(&id) {
                    match event {
                        winit::event::Event::WindowEvent {
                            event: winit::event::WindowEvent::CloseRequested,
                            ..
                        } => {
                            *control_flow = ControlFlow::Exit;
                            should_stop_loop = true;
                        }
                        winit::event::Event::WindowEvent {
                            event: winit::event::WindowEvent::Resized(_),
                            ..
                        } => {
                            webview.resize().unwrap();
                        }
                        winit::event::Event::MainEventsCleared => {
                            webview.window().request_redraw();
                        }
                        winit::event::Event::RedrawRequested(_) => {}
                        _ => (),
                    };
                }
            });

            // add our event inside our stack to be pulled by the next step
            STACK_MAP.with(|cell| {
                let mut stack_map = cell.borrow_mut();
                if let Some(stack) = stack_map.get_mut(&id) {
                    stack.push(Event::from(event));
                } else {
                    panic!("Could not find stack with id {} to push onto stack", id);
                }
            });
        });
    });

    Ok(json!(should_stop_loop))
}

#[json_op]
fn wry_step(json: Value, _zero_copy: &mut [ZeroCopyBuf]) -> Result<Value, AnyError> {
    let id = json["id"].as_u64().unwrap();
    STACK_MAP.with(|cell| {
        let mut stack_map = cell.borrow_mut();
        if let Some(stack) = stack_map.get_mut(&id) {
            let ret = stack.clone();
            stack.clear();
            Ok(json!(ret))
        } else {
            Err(anyhow!("Could not find stack with id: {}", id))
        }
    })
}