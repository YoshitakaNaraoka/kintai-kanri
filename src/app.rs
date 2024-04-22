use yew::prelude::*;

// サーバーからの応答に基づくログイン結果を表す列挙型
enum LoginResult {
    Success(String),
    Failure(String),
}

// アプリケーションのルートとなるルーティングを定義する列挙型
pub enum AppRoute {
    Login,
    Hello,
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
    
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    
    fn prepare_state(&self) -> Option<String> {
        None
    }
    
    fn destroy(&mut self, ctx: &Context<Self>) {}
}

// メッセージコンポーネント
struct MessageComponent;

impl Component for MessageComponent {
    type Message = Option<String>;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        MessageComponent
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> ShouldRender {
        // メッセージコンポーネントの更新ロジックをここに実装
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <p>{"Message Component"}</p>
        }
    }
    
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    
    fn prepare_state(&self) -> Option<String> {
        None
    }
    
    fn destroy(&mut self, ctx: &Context<Self>) {}
}

// アプリケーションのコンポーネント
pub struct App {
    login_msg: String, // ログインメッセージを保持するためのフィールド
}

impl Component for App {
    type Message = AppRoute;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            login_msg: String::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> ShouldRender {
        match msg {
            AppRoute::Login => {
                // ログイン要求をサーバーに送信する処理などを実装し、結果をlogin_msgに反映する
                self.login_msg = "Logging in...".to_string();
            }
            AppRoute::Hello => {
                // Helloページに遷移する処理などを実装し、結果をlogin_msgに反映する
                self.login_msg = "Hello Page".to_string();
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
                            AppRoute::Login => {
                                html! {
                                    <LoginForm />
                                }
                            },
                            AppRoute::Hello => html! {
                                <div>{"Hello, World!"}</div>
                            },
                        }
                    })
                />
                <p><b>{ &self.login_msg }</b></p>
            </main>
        }
    }
    
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    
    fn prepare_state(&self) -> Option<String> {
        None
    }
    
    fn destroy(&mut self, ctx: &Context<Self>) {}
}
