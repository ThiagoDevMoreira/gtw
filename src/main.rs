#![allow(non_snake_case, unused)]
mod layoutController;
mod geradorDeToken;
mod login;

use dioxus::prelude::*;
use layoutController::*;
use login::core::initialize_session;

fn main() {
    initialize_session();
    launch(App);
}

#[component]
//tela inicial da aplicação.
fn App() -> Element {
    rsx! {
        header{
            h1{"Gerador de token para whatsapp"}}
        main{
            div{{TelaFuncionalidadesPrincipais()}}
        }
        footer{"v.0.0.3 - 24-11-24"}
    }
}