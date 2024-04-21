use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub trait Component {
    type Message;
    type Properties;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self;
    fn update(&mut self, _: Self::Message) -> ShouldRender;
    fn view(&self) -> Html;
}

pub enum ShouldRender {
    Render,
    NoRender,
}



// サーバーからのレスポンスを受け取る構造体
#[derive(Serialize, Deserialize)]
struct LoginResponse {
    message: String,
}

// ログイン情報を表す構造体
#[derive(Serialize, Deserialize)]
struct LoginArgs {
    mail: String,
    pass: String,
}

// メッセージを表示するコンポーネント
struct MessageComponent {
    message: String,
}

struct MessageComponent {
    message: String,
}

impl Component for MessageComponent {
    type Message = ();
    type Properties = ();

    // コンポーネントの初期化
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MessageComponent { message: String::new() }
    }

    // メッセージの更新
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    // 表示の更新
    fn view(&self) -> Html {
        html! {
            <p><b>{ &self.message }</b></p>
        }
    }
}


// アプリケーションのエントリーポイント
#[function_component(App)]
pub fn app() -> Html {
    // 入力フィールドの参照を作成
    let login_input_ref = use_node_ref::<web_sys::HtmlInputElement>();

    // ログインメッセージの状態を管理
    let login_msg = use_state(|| String::new());

    // ログインのイベントハンドラ
    let on_login_submit = {
        let login_input_ref = login_input_ref.clone();
        let login_msg = login_msg.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();

            // 入力値の取得
            let mail_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let pass_value = login_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();

            // 空の場合は処理をスキップ
            if mail_value.is_empty() || pass_value.is_empty() {
                return;
            }

            // ログイン情報を作成
            let login_args = LoginArgs { mail: mail_value, pass: pass_value };

            // ログインリクエストの送信
            spawn_local(async move {
                match invoke("login", JsValue::from_serde(&login_args).unwrap()).await {
                    Ok(js_value) => {
                        let login_response: LoginResponse = js_value.into_serde().unwrap();
                        login_msg.set(login_response.message);
                    },
                    Err(_) => {
                        login_msg.set("Failed to communicate with server".to_string());
                    }
                }
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

            <form class="row" onsubmit={on_login_submit}>
                <input id="login-input" ref={login_input_ref} placeholder="Your mail address" />
                <input id="password-input" type="password" placeholder="Password" />
                <button type="submit">{"Login"}</button>
            </form>

            <MessageComponent message=&login_msg />
        </main>
    }
}
