use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

// サーバーからの応答に基づくログイン結果を表す列挙型
enum LoginResult {
    Success(String),
    Failure(String),
}

// アプリケーションのコンポーネント
struct App {
    link: ComponentLink<Self>,
    login_msg: String,
}

impl Component for App {
    type Message = LoginResult;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            login_msg: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginResult::Success(msg) => {
                // ログイン成功時のメッセージを受信
                self.login_msg = msg;
            }
            LoginResult::Failure(msg) => {
                // ログイン失敗時のメッセージを受信
                self.login_msg = msg;
            }
        }
        true // 再描画をトリガー
    }

    fn view(&self) -> Html {
        html! {
            <main class="container">
                <div class="row">
                    <a href="https://">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                    <a href="https://">
                        <img src="public/chrome-logo-m100.svg" class="logo chrome" alt="Chrome logo"/>
                    </a>
                </div>

                <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                // ログインフォームの表示
                <div>
                    <form onsubmit=self.link.callback(|e: FocusEvent| {
                        e.prevent_default();
                        LoginResult::Success("Login successful!".to_string())
                    })>
                        <label for="login-input">{"Email: "}</label>
                        <input id="login-input" type="text" />

                        <label for="password-input">{"Password: "}</label>
                        <input id="password-input" type="password" />

                        <button type="submit">{"Login"}</button>
                    </form>
                </div>
                
                // メッセージコンポーネントの表示
                <p><b>{ &self.login_msg }</b></p>
            </main>
        }
    }
}

// アプリのエントリーポイント
fn main() {
    yew::start_app::<App>();
}
