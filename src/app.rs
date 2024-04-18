use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
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

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GoToHome,
    GoToDetail,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToHome => {
                // Tauriコマンドを呼び出してホーム画面へ遷移
                tauri::command::spawn("go_to_home".to_string());
            }
            Msg::GoToDetail => {
                // Tauriコマンドを呼び出して詳細画面へ遷移
                tauri::command::spawn("go_to_detail".to_string());
            }
        }
        false
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let login_input_ref = use_node_ref();
    let mail = use_state(|| String::new());
    let login_msg = use_state(|| String::new());

    {
        let login_msg = login_msg.clone();
        let mail = mail.clone();
        let pass = mail.clone(); // str の中身は一つのものを.clone()で使いまわせる
        use_effect_with(mail.clone(), move |_| {
            spawn_local(async move {
                if mail.is_empty() {
                    return;
                }

                if pass.is_empty() {
                    return;
                }

                let args = to_value(&LoginArgs {
                    mail: &mail,
                    pass: &pass,
                })
                .unwrap();
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let new_msg = invoke("login", args).await.as_string().unwrap();
                login_msg.set(new_msg);
            });

            || {}
        });
    }

    let login = {
        let mail = mail.clone();
        let pass = mail.clone();
        let login_input_ref = login_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            mail.set(
                login_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );

            pass.set(
                login_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
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
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <form class="row" onsubmit={login}>
                <input id="login-input" ref={&login_input_ref} placeholder="Your mail address" />
                <input id="login-input" ref={&login_input_ref} placeholder="Password" /> // 参照がいる
                <button type="submit">{"Login"}</button>
            </form>

            <p><b>{ &*login_msg }</b></p>
        </main>
    }
}

fn view(&self) -> Html {
    html! {
        <div>
            <button onclick=self.link.callback(|_| Msg::GoToHome)>{"Go to Home"}</button>
            <button onclick=self.link.callback(|_| Msg::GoToDetail)>{"Go to Detail"}</button>
        </div>
    }
};