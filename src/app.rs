use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use tauri::api::{webview::WebviewBuilder, Environment, Content};

// サーバーからの応答に基づくログイン結果を表す列挙型
enum LoginResult {
    Success(String),
    Failure(String),
}

// ログインフォームのコンポーネント
struct LoginForm {
    on_login: Callback<LoginResult>, // プロパティを直接定義
}

#[derive(Properties, Clone, PartialEq)]
struct LoginFormProps {
    on_login: Callback<LoginResult>,
}

impl Component for LoginForm {
    type Message = ();
    type Properties = LoginFormProps;

    fn create(props: &yew::Context<LoginForm>) -> Self {
        LoginForm {
            on_login: props.on_login,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            () => {
                let document = window().unwrap().document().unwrap();
                let mail_input = document.get_element_by_id("login-input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let pass_input = document.get_element_by_id("password-input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let mail_value = mail_input.value();
                let pass_value = pass_input.value();

                // ログイン要求をサーバーに送信（ここではダミーの非同期処理を模擬）
                let future = async move {
                    // サーバーとの通信や認証が成功したと仮定
                    if mail_value == "user@example.com" && pass_value == "password" {
                        LoginResult::Success("Login successful!".to_string())
                    } else {
                        LoginResult::Failure("Login failed!".to_string())
                    }
                };

                // 非同期処理の結果を処理
                wasm_bindgen_futures::spawn_local(async move {
                    let result = future.await;
                    self.on_login.emit(result);
                });
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <form onsubmit=self.on_login.reform(|e: FocusEvent| {
                    e.prevent_default();
                    ()
                })>
                    <label for="login-input">{"Email: "}</label>
                    <input id="login-input" type="text" />

                    <label for="password-input">{"Password: "}</label>
                    <input id="password-input" type="password" />

                    <button type="submit">{"Login"}</button>
                </form>
            </div>
        }
    }
}

// メッセージコンポーネント
#[derive(Properties, Clone, PartialEq)]
struct MessageComponentProps {
    message: String,
}

fn message_component(props: &MessageComponentProps) -> Html {
    html! {
        <p><b>{ &props.message }</b></p>
    }
}

// アプリケーションのエントリーポイント
fn main() {
    let environment = Environment::builder().build().unwrap(); // Tauriの環境を設定
    let webview = WebviewBuilder::new(environment) // TauriのWebViewを構築
        .title("Tauri Yew App")
        .content(Content::Html(include_str!("hello.html")))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .build()
        .unwrap(); // WebViewのビルド
    let mut runtime = webview2::Runtime::new().unwrap();
    runtime.run(); // WebViewのランタイムを実行
    yew::start_app::<App>();
}

// アプリケーションのコンポーネント
struct App {
    message: Option<String>, // Option<String>を使ってメッセージを管理
}

impl Component for App {
    type Message = LoginResult;
    type Properties = ();

    fn create(_: Self::Properties) -> Self {
        App {
            message: None, // メッセージを初期化
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginResult::Success(msg) => {
                // ログイン成功時のメッセージを受信
                self.message = Some(msg); // メッセージをセット
                true // 再描画をトリガー
            }
            LoginResult::Failure(msg) => {
                // ログイン失敗時のメッセージを受信
                self.message = Some(msg); // メッセージをセット
                true // 再描画をトリガー
            }
        }
    }

    fn view(&self) -> Html {
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

                // ログインフォームの表示
                <LoginForm on_login=self.link.callback(|result| result) />

                // メッセージコンポーネントの表示
                <MessageComponent message={ self.message.clone() } />
            </main>
        }
    }
}
