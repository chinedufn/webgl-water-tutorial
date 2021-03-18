use crate::app::App;
use crate::app::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub static APP_DIV_ID: &'static str = "webgl-water-tutorial";

pub static CANVAS_WIDTH: i32 = 512;
pub static CANVAS_HEIGHT: i32 = 512;

pub fn create_webgl_context(app: Rc<App>) -> Result<WebGlRenderingContext, JsValue> {
    let canvas = init_canvas(app)?;

    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

fn init_canvas(app: Rc<App>) -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(CANVAS_WIDTH as u32);
    canvas.set_height(CANVAS_HEIGHT as u32);

    attach_mouse_down_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_up_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_move_handler(&canvas, Rc::clone(&app))?;
    attach_mouse_wheel_handler(&canvas, Rc::clone(&app))?;

    attach_touch_start_handler(&canvas, Rc::clone(&app))?;
    attach_touch_move_handler(&canvas, Rc::clone(&app))?;
    attach_touch_end_handler(&canvas, Rc::clone(&app))?;

    let app_div: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into()?,
        None => {
            let app_div = document.create_element("div")?;
            app_div.set_id(APP_DIV_ID);
            app_div.dyn_into()?
        }
    };

    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        app.store.borrow_mut().msg(&Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::MouseEvent| {
        app.store.borrow_mut().msg(&Msg::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();
    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        event.prevent_default();
        let x = event.client_x();
        let y = event.client_y();
        app.store.borrow_mut().msg(&Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_wheel_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::WheelEvent| {
        event.prevent_default();

        let zoom_amount = event.delta_y() / 50.;

        app.store.borrow_mut().msg(&Msg::Zoom(zoom_amount as f32));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_start_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        app.store.borrow_mut().msg(&Msg::MouseDown(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_move_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |event: web_sys::TouchEvent| {
        event.prevent_default();
        let touch = event.touches().item(0).expect("First Touch");
        let x = touch.client_x();
        let y = touch.client_y();
        app.store.borrow_mut().msg(&Msg::MouseMove(x, y));
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_touch_end_handler(canvas: &HtmlCanvasElement, app: Rc<App>) -> Result<(), JsValue> {
    let handler = move |_event: web_sys::TouchEvent| {
        app.store.borrow_mut().msg(&Msg::MouseUp);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("touchend", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
