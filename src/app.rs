// src/lib.rs

use yew::prelude::*;

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

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::GoToHome)>{"Go to Home"}</button>
                <button onclick=self.link.callback(|_| Msg::GoToDetail)>{"Go to Detail"}</button>
            </div>
        }
    }
}
