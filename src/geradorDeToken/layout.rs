use dioxus::prelude::*;

#[component]
pub fn layout_geradorDeToken() -> Element{
    rsx! (
        div {
            p {"aqui aparece o token"}
            button {"copia para a area de transferência"}
            button {"gerar token"}
        }
    )
}