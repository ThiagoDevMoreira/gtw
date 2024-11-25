use dioxus::prelude::*;

#[component]
pub fn layout_realizarLogin() -> Element{
    rsx! (
        div {
            h2{"faça login"}
            input {"escreva sua senha aqui"}
            button {"aperte este botão"}
        }
    )
}