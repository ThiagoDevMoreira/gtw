#[allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    // Estado para a senha
    let mut password = use_signal(|| String::new());
    let mut error_message = use_signal(|| None::<String>);

    // Função para validação
    let handle_submit = move |_| {
        password.with(|pwd| {
            Some(pwd)
                .filter(|pwd| *pwd == "senha123") // Valida se a senha está correta
                .map(|_| println!("Senha correta. Acesso concedido.")) // Ação para senha correta
                .or_else(|| {
                    error_message.set(Some("Senha incorreta!".to_string())); // Define mensagem de erro
                    None
                })
        });
    };

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-gray-200",
            div {
                class: "bg-white p-6 rounded-lg shadow-lg w-80",
                h1 {
                    class: "text-2xl font-bold mb-4 text-center text-gray-700",
                    "Login"
                }
                input {
                    class: "border border-gray-300 rounded-md w-full px-3 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500",
                    r#type: "password",
                    placeholder: "Digite a senha",
                    value: "{password}",
                    oninput: move |evt| password.set(evt.value().clone()),
                }
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded w-full",
                    onclick: handle_submit,
                    "Entrar"
                }
                {error_message.as_ref().map(|msg| rsx! {
                    p {
                        class: "text-red-500 text-sm mt-2 text-center",
                        "{msg}"
                    }
                })}
            }
        }
    }
}
