use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct LoginArgs<'a> {
    mail: &'a str,
    pass: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let login_input_ref = use_node_ref();
    let mail = use_state(String::new);
    let login_msg = use_state(String::new);

    let login = {
        let login_input_ref = login_input_ref.clone();
        let login_msg = login_msg.clone();
        Callback::from(move |e: &SubmitEvent| {
            e.prevent_default();
            let mail_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let pass_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            if mail_value.is_empty() || pass_value.is_empty() {
                return;
            }
    
            let args = serde_wasm_bindgen::to_value(&LoginArgs {
                mail: &mail_value,
                pass: &pass_value,
            }).unwrap();
            
            spawn_local(async move {
                let new_msg = invoke("login", args).await.as_string().unwrap_or_else(|| String::from(""));
                login_msg.set(new_msg);
            });
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

            <form class="row" onsubmit={Callback::from(on_login_submit)}>
                <input id="login-input" ref={&login_input_ref} placeholder="Your mail address" />
                <input id="login-input" ref={&login_input_ref} placeholder="Password" />
                <button type="submit">{"Login"}</button>
            </form>

            <p><b>{ &*login_msg }</b></p>
        </main>
    }
}