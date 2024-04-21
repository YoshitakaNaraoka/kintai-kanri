
use js_sys::Reflect::apply;
use serde::{Deserialize, Serialize};


use wasm_bindgen::*;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct LoginArgs {
    mail: String,
    pass: String,
}

#[derive(Properties, Clone, PartialEq)]
struct MessageComponentProps {
    message: String,
}

#[function_component(MessageComponent)]
fn message_component(props: &MessageComponentProps) -> Html {
    html! {
        <p><b>{ &props.message }</b></p>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let login_input_ref = NodeRef::default();
    let login_msg = use_state(|| String::new());

    let on_login_submit = {
        let login_input_ref = login_input_ref.clone();
        let login_msg = login_msg.clone();
        Callback::from(move |e: yew::events::SubmitEvent| (
            e.prevent_default();

            let mail_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let pass_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();

            if mail_value.is_empty() || pass_value.is_empty() {
                return;
            }

            let login_args = LoginArgs { mail: mail_value, pass: pass_value };

            let login_future = async move {
                let js_value = serde_wasm_bindgen::to_value(&login_args).unwrap();
                let window = web_sys::window().unwrap();
                let login_future = async move {
                    let js_value = serde_wasm_bindgen::to_value(&login_args).unwrap();
                    let window = web_sys::window().unwrap();
                    let promise = {
                        let promise = apply(
                            &JsValue::from(window),
                            &JsValue::from_str("invoke"),
                            &JsValue::from_str("login"),
                            &js_sys::Array::of1(&js_value),
                        );
                        JsFuture::from(promise)
                    };
        )

                let result = wasm_bindgen_futures::JsFuture::from(promise).await;

                let message = match result {
                    Ok(js_value) => {
                        let login_response: Result<LoginResponse, _> = serde_wasm_bindgen::from_value(js_value);
                        match login_response {
                            Ok(response) => response.message,
                            Err(_) => "Failed to deserialize login response".to_string(),
                        }
                    }
                    Err(_) => "Failed to communicate with server".to_string(),
                };
                

                login_msg.set(message);
            };

            spawn_local(login_future);
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
                <a href="https://www.google.com/intl/ja/chrome/" target="_blank">
                    <img src="public/chrome-logo-m100.svg" class="logo chrome" alt="Chrome logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={on_login_submit}>
                <input id="login-input" ref={login_input_ref.clone()} placeholder="Your mail address" />
                <input id="password-input" type="password" placeholder="Password" />
                <button type="submit">{"Login"}</button>
            </form>

            <MessageComponent message={login_msg.get(0..).map(|s| s.to_string()).unwrap_or_default()} />




        </main>
    }
}
