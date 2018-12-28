use crate::app::App;
use crate::app::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

static CANVAS_CONTAINER: &'static str = "webgl-water-tutorial";

static MOUSE_DOWN: &'static str = "mousedown";
static MOUSE_UP: &'static str = "mouseup";
static MOUSE_MOVE: &'static str = "mousemove";
static WHEEL: &'static str = "wheel";

pub static CANVAS_WIDTH: i32 = 512;
pub static CANVAS_HEIGHT: i32 = 512;


pub fn create_webgl_context(app: Rc<App>) -> Result<WebGlRenderingContext, JsValue> {
    let canvas = init_canvas(app)?;

    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

fn init_canvas (app: Rc<App>) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(CANVAS_WIDTH as u32);
    canvas.set_height(CANVAS_HEIGHT as u32);

    attach_mouse_down_handler(&canvas, Rc::clone(&app))?;

    attach_mouse_up_handler(&canvas, Rc::clone(&app))?;

    attach_mouse_move_handler(&canvas, Rc::clone(&app))?;

    attach_mouse_wheel_handler(&canvas, Rc::clone(&app))?;

    match document.get_element_by_id(CANVAS_CONTAINER) {
        Some(container) => container.append_child(&canvas)?,
        None => document.body().expect("Body").append_child(&canvas)?
    };

    Ok(canvas)
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        app.store.borrow_mut().msg(&Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback(MOUSE_DOWN, handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        app.store.borrow_mut().msg(&Msg::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback(MOUSE_UP, handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        app.store.borrow_mut().msg(&Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback(MOUSE_MOVE, handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::WheelEvent| {
        let zoom_amount = event.delta_y() / 50.;

        app.store.borrow_mut().msg(&Msg::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    canvas.add_event_listener_with_callback(WHEEL, handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
