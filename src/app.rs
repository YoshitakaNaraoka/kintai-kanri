use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

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

    fn create(_: &Context<Self>) -> Self {
        LoginForm
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> ShouldRender {
        // ログインフォームの更新ロジックをここに実装
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <form class="row" onsubmit=|e: SubmitEvent| { e.prevent_default(); }>
                <input id="login-input" placeholder="Your mail address" />
                <input id="password-input" placeholder="Password" type="password" />
                <button type="submit">{"Login"}</button>
            </form>
        }
    }
}

// アプリケーションのコンポーネント
pub struct App;

impl Component for App {
    type Message = AppRoute;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            AppRoute::Login => {
                // ログイン要求をサーバーに送信する処理などを実装
            }
            AppRoute::Hello => {
                // Helloページに遷移する処理などを実装
            }
        }
        true // コンポーネントの再描画が必要
    }

    fn view(&self, _: &Context<Self>) -> Html {
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

                <Router<AppRoute, ()>
                    render=Router::render(move |switch: AppRoute| {
                        match switch {
                            AppRoute::Login => html! { <LoginForm /> },
                            AppRoute::Hello => html! { <div>{"Hello, World!"}</div> },
                        }
                    })
                />
            </main>
        }
    }
}
