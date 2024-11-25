#![allow(non_snake_case, unused)]
mod layoutController;
mod geradorDeToken;
mod login;

use dioxus::prelude::*;
use layoutController::TelaFuncionalidadesPrincipais;
use login::core::*;

fn main() {
    initialize_session();
    let session = get_session();
    session.set("status", "0");
    launch(App);
}

#[component]
//tela inicial da aplicação.
fn App() -> Element {
    rsx! {
        header{
            h1{"Gerador de token para whatsapp"}}
        main{
            div{
                TelaFuncionalidadesPrincipais{}
            }
        }
        footer{"v.0.0.3 - 24-11-24"}
    }
}