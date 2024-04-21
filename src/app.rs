use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

// サーバーからの応答に基づくログイン結果を表す列挙型
enum LoginResult {
    Success(String),
    Failure(String),
}

// ログインフォームのコンポーネント
struct LoginForm;

impl Component for LoginForm {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        LoginForm
    }

    fn update(&mut self, _: &yew::Context<LoginForm>, msg: Self::Message) -> bool {
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
                    yew::Callback::noop().emit(result);
                });
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <form onsubmit=|e: FocusEvent| { e.prevent_default(); }>
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

    fn create(_: &yew::Context<Self>) -> Self {
        MessageComponent
    }

    fn update(&mut self, _: &yew::Context<MessageComponent>, msg: Self::Message) -> bool {
        if let Some(message) = msg {
            // ログイン成功時のメッセージを受信
            // メッセージを表示する等の処理を行う
        } else {
            // ログイン失敗時のメッセージを受信
            // エラーメッセージを表示する等の処理を行う
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p>{"Message Component"}</p>
        }
    }
}

// アプリケーションのコンポーネント
struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _: &yew::Context<App>, msg: Self::Message) -> bool {
        match msg {
            () => {
                // ログインフォームからのログイン結果を受け取る等の処理を行う
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                // ログインフォームの表示
                <LoginForm />

                // メッセージコンポーネントの表示
                <MessageComponent />
            </main>
        }
    }
}
