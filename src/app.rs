use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
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

#[derive(Properties, Clone, PartialEq)]
struct AppProps {}

#[function_component(App)]
pub fn app() -> Html {
    let login_input_ref = NodeRef::default();
    let login_msg = use_state(|| String::new());

    let on_login_submit = {
        let login_input_ref = login_input_ref.clone();
        let login_msg = login_msg.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();

            let mail_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let pass_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            
            if mail_value.is_empty() || pass_value.is_empty() {
                return;
            }

            let login_args = LoginArgs { mail: mail_value, pass: pass_value };

            let login_future = async move {
                match send_login_request(&login_args).await {
                    Ok(js_value) => {
                        handle_login_response(js_value, &login_msg).await;
                    },
                    Err(_) => {
                        login_msg.set("Failed to communicate with server".to_string());
                    }
                }
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

            <MessageComponent message=login_msg.clone() />
        </main>
    }
}

async fn send_login_request(login_args: &LoginArgs) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let body = serde_wasm_bindgen::to_value(login_args)?;

    let request = Request::new_with_str_and_init("/login", &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.body(Some(&body.into()))?;

    let resp_value = JsFuture::from(window().unwrap().fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().expect("not a Response");
    let resp_json = JsFuture::from(resp.json()?).await?;

    Ok(resp_json)
}

async fn handle_login_response(js_value: JsValue, login_msg: &Callback<String>) {
    let login_response: Result<LoginResponse, _> = js_value.into_serde();
    match login_response {
        Ok(login_response) => {
            login_msg.emit(login_response.message);
        }
        Err(_) => {
            login_msg.emit("Failed to communicate with server".to_string());
        }
    }
}
