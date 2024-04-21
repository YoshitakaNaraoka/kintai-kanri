use webview2::{Environment, WebviewBuilder};
use yew::prelude::*;

// サーバーからの応答に基づくログイン結果を表す列挙型
enum LoginResult {
    Success(String),
    Failure(String),
}

// ログインフォームとメッセージコンポーネントを含むアプリケーションのコンポーネント
struct App {
    login_form_link: ComponentLink<LoginForm>,
    message: Option<String>,
}

// ログインフォームのコンポーネント
struct LoginForm {
    link: ComponentLink<Self>,
    on_login: Callback<LoginResult>,
}

impl Component for LoginForm {
    type Message = ();
    type Properties = Callback<LoginResult>;

    fn create(on_login: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginForm {
            link,
            on_login,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        // ログイン要求をサーバーに送信（ここではダミーの非同期処理を模擬）
        let future = async {
            // サーバーとの通信や認証が成功したと仮定
            let mail_value = "user@example.com"; // 仮の値
            let pass_value = "password"; // 仮の値
            if mail_value == "user@example.com" && pass_value == "password" {
                LoginResult::Success("Login successful!".to_string())
            } else {
                LoginResult::Failure("Login failed!".to_string())
            }
        };

        // 非同期処理の結果を処理
        async {
            match future.await {
                LoginResult::Success(msg) => self.on_login.emit(LoginResult::Success(msg)),
                LoginResult::Failure(msg) => self.on_login.emit(LoginResult::Failure(msg)),
            }
        };

        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <form onsubmit=self.link.callback(|e: FocusEvent| {
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
struct MessageComponent;

impl Component for MessageComponent {
    type Message = Option<String>;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        MessageComponent
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Some(message) = msg {
            // ログイン成功時のメッセージを受信
            // ここでページの遷移などの処理を行う
            true // 再描画をトリガー
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <p><b>{ self.props }</b></p>
        }
    }
}

impl Component for App {
    type Message = LoginResult;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            login_form_link: ComponentLink::default(),
            message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginResult::Success(msg) => {
                self.message = Some(msg);
                true // 再描画をトリガー
            }
            LoginResult::Failure(msg) => {
                self.message = Some(msg);
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
                <LoginForm on_login=self.login_form_link.callback(|result| result) />

                // メッセージコンポーネントの表示
                <MessageComponent message={ self.message.clone() } />
            </main>
        }
    }
}

// アプリケーションのエントリーポイント
fn main() {
    let environment = Environment::builder().build().unwrap();
    let webview = WebviewBuilder::new(environment)
        .title("Tauri Yew App")
        .content(Content::Html(include_str!("index.html")))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .build()
        .unwrap();
    let mut runtime = webview2::Runtime::new().unwrap();
    runtime.run();
    yew::start_app::<App>();
}
