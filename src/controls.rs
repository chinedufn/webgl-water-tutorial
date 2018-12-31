use crate::canvas::APP_DIV_ID;
use crate::App;
use crate::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

pub fn append_controls(app: Rc<App>) -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let container: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into().expect("Html element"),
        None => document.body().expect("Document body"),
    };

    let controls = document.create_element("div")?;
    container.append_child(&controls)?;

    // Reflectivity
    {
        let app = Rc::clone(&app);
        let reflectivity_control = create_reflectivity_control(app)?;
        controls.append_child(&reflectivity_control)?;
    }

    // Fresnel Effect
    {
        let app = Rc::clone(&app);
        let fresnel_control = create_fresnel_control(app)?;
        controls.append_child(&fresnel_control)?;
    }

    // Wave Speed
    {
        let app = Rc::clone(&app);
        let wave_speed_control = create_wave_speed_control(app)?;
        controls.append_child(&wave_speed_control)?;
    }


    Ok(())
}

fn create_reflectivity_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let reflectivity = input_elem.value().parse().unwrap();

        app.store
            .borrow_mut()
            .msg(&Msg::SetReflectivity(reflectivity));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let reflectivity_control = Slider {
        min: 0.0,
        max: 1.0,
        step: 0.1,
        start: 0.5,
        label: "Reflectivity",
        closure,
    }
    .create_element()?;

    Ok(reflectivity_control)
}

fn create_fresnel_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let fresnel = input_elem.value().parse().unwrap();

        app.store
            .borrow_mut()
            .msg(&Msg::SetFresnel(fresnel));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let fresnel_control = Slider {
        min: 0.0,
        max: 10.0,
        step: 0.1,
        start: 1.5,
        label: "Fresnel Effect",
        closure,
    }
    .create_element()?;

    Ok(fresnel_control)
}

fn create_wave_speed_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let wave_speed = input_elem.value().parse().unwrap();

        app.store
            .borrow_mut()
            .msg(&Msg::SetWaveSpeed(wave_speed));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);

    let wave_speed_control = Slider {
        min: 0.02,
        max: 0.15,
        step: 0.01,
        start: 0.06,
        label: "Wave Speed",
        closure,
    }
    .create_element()?;

    Ok(wave_speed_control)
}


struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<FnMut(web_sys::Event)>,
}

impl Slider {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let slider: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        slider.set_type("range");
        slider.set_min(&format!("{}", self.min));
        slider.set_max(&format!("{}", self.max));
        slider.set_step(&format!("{}", self.step));
        slider.set_value(&format!("{}", self.start));

        let closure = self.closure;
        slider.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("div")?;
        label.set_inner_html(self.label);

        let container = document.create_element("div")?;
        container.append_child(&label)?;
        container.append_child(&slider)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "20px")?;

        Ok(container)
    }
}
