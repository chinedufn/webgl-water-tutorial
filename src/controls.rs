use crate::canvas::APP_DIV_ID;
use crate::App;
use crate::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Element;
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
    let controls: HtmlElement = controls.dyn_into()?;
    controls.style().set_property("padding-left", "5px")?;
    let controls: Element = controls.dyn_into()?;

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

    // Use Refraction
    {
        let app = Rc::clone(&app);
        let use_refraction_control = create_use_refraction_checkbox(app)?;
        controls.append_child(&use_refraction_control)?;
    }

    // Use Reflection
    {
        let app = Rc::clone(&app);
        let use_reflection_control = create_use_reflection_checkbox(app)?;
        controls.append_child(&use_reflection_control)?;
    }

    // Render Scenery
    {
        let app = Rc::clone(&app);
        let show_scenery_control = create_show_scenery_control(app)?;
        controls.append_child(&show_scenery_control)?;
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
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

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

        app.store.borrow_mut().msg(&Msg::SetFresnel(fresnel));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

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

        app.store.borrow_mut().msg(&Msg::SetWaveSpeed(wave_speed));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let wave_speed_control = Slider {
        min: 0.0,
        max: 0.15,
        step: 0.01,
        start: 0.06,
        label: "Wave Speed",
        closure,
    }
    .create_element()?;

    Ok(wave_speed_control)
}

fn create_use_refraction_checkbox(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let use_refraction = input_elem.checked();

        app.store
            .borrow_mut()
            .msg(&Msg::UseRefraction(use_refraction));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let use_refraction_control = Checkbox {
        start_checked: true,
        label: "Use Refraction",
        closure,
    }
    .create_element()?;

    Ok(use_refraction_control)
}

fn create_use_reflection_checkbox(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let use_reflection = input_elem.checked();

        app.store
            .borrow_mut()
            .msg(&Msg::UseReflection(use_reflection));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let use_reflection_control = Checkbox {
        start_checked: true,
        label: "Use Reflection",
        closure,
    }
    .create_element()?;

    Ok(use_reflection_control)
}

fn create_show_scenery_control(app: Rc<App>) -> Result<HtmlElement, JsValue> {
    let handler = move |event: web_sys::Event| {
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let show_scenery = input_elem.checked();

        app.store.borrow_mut().msg(&Msg::ShowScenery(show_scenery));
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    let show_scenery_control = Checkbox {
        start_checked: true,
        label: "Show Scenery",
        closure,
    }
    .create_element()?;

    Ok(show_scenery_control)
}

struct Slider {
    min: f32,
    max: f32,
    step: f32,
    start: f32,
    label: &'static str,
    closure: Closure<dyn FnMut(web_sys::Event)>,
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
        container.style().set_property("margin-bottom", "15px")?;

        Ok(container)
    }
}

struct Checkbox {
    start_checked: bool,
    label: &'static str,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl Checkbox {
    fn create_element(self) -> Result<HtmlElement, JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let checkbox: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        checkbox.set_type("checkbox");
        checkbox.set_checked(self.start_checked);

        let closure = self.closure;
        checkbox.set_oninput(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        let label = document.create_element("label")?;
        label.set_inner_html(self.label);
        label.append_child(&checkbox)?;

        let container = document.create_element("div")?;
        container.append_child(&label)?;

        let container: HtmlElement = container.dyn_into()?;
        container.style().set_property("margin-bottom", "15px")?;
        container.style().set_property("display", "flex")?;
        container.style().set_property("align-items", "center")?;
        container.style().set_property("cursor", "pointer")?;

        Ok(container)
    }
}
