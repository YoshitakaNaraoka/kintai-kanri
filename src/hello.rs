// src/app.rs

use yew::prelude::*;

pub mod hello {
  pub struct App {}

  pub enum Msg {
      // ログインボタンがクリックされた時のメッセージ
      LoginButtonClicked,
  }

  impl Component for App {
      type Message = Msg;
      type Properties = ();

      // コンポーネントの生成
      fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
          Self {}
      }

      // メッセージのハンドリング
      fn update(&mut self, msg: Self::Message) -> ShouldRender {
          match msg {
              Msg::LoginButtonClicked => {
                  // ログインボタンがクリックされたときの処理
                  // Tauriのコマンドを呼び出してUIを表示する
                  tauri::command::spawn("show_ui".to_string());
              }
          }
          false
      }

      // UIのレンダリング
      fn view(&self) -> Html {
          html! {
              <div>
                  <h1>{"Hello, world!"}</h1>
                  <button onclick=|_| Msg::LoginButtonClicked>{"Login"}</button>
              </div>
          }
      }
  }
}