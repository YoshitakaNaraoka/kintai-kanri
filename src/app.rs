use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use wasm_bindgen::prelude::*;


#[derive(Serialize, Deserialize)]
struct LoginResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct LoginArgs {
    mail: String,
    pass: String,
}

// MessageComponent用のプロパティ型
#[derive(Properties, Clone, PartialEq)]
struct MessageComponentProps {
    message: String,
}

// メッセージを表示するコンポーネント
#[function_component(MessageComponent)]
fn message_component(props: &MessageComponentProps) -> Html {
    html! {
        <p><b>{ &props.message }</b></p>
    }
}


#[function_component(App)]
pub fn app() -> Html {
    let login_input_ref = NodeRef::default();
    let login_msg = use_state(String::new);

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
                match serde_wasm_bindgen::to_value(&login_args) {
                    Ok(js_value) => {
                        match invoke("login", &js_value).await {
                            Ok(js_value) => {
                                let login_response: LoginResponse = js_value.into_serde().unwrap();
                                login_msg.set(login_response.message);
                            },
                            Err(_) => {
                                login_msg.set("Failed to communicate with server".to_string());
                            }
                        }
                    },
                    Err(_) => {
                        login_msg.set("Failed to serialize login arguments".to_string());
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

            // MessageComponentを呼び出す際にPropsを渡す
            <MessageComponent message=login_msg.clone() />
        </main>
    }
}
