use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

use super::swgl::graphics_2d::color::Color;
use super::swgl::runtime_error::SWGLResult;
use super::swgl::utils::web_helpers;

// -----------------------------------------------------------------------------------------

pub fn initialize_webgl_context(
    canvas_selector: &str,
    clear_color: Color,
) -> SWGLResult<(web_sys::HtmlCanvasElement, swgl::AppContext)> {
    let (canvas, context) = web_helpers::app_handler(canvas_selector)?;

    context.clear_color(
        clear_color.red,
        clear_color.green,
        clear_color.blue,
        clear_color.alpha,
    );
    context.clear_depth(1.);

    attach_mouse_down_handler(&canvas).unwrap();
    attach_mouse_up_handler(&canvas).unwrap();
    attach_mouse_move_handler(&canvas).unwrap();

    Ok((canvas, context))
}

// -----------------------------------------------------------------------------------------
// event handlers

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_down(event.offset_x() as f32, event.offset_y() as f32, true);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

// -----------------------------------------------------------------------------------------

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_down(
            event.offset_x() as f32,
            event.offset_y() as f32,
            false,
        );
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

// -----------------------------------------------------------------------------------------

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_position(event.offset_x() as f32, event.offset_y() as f32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
