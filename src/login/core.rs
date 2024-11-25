use std::collections::HashMap;

static mut GLOBAL_SESSION: Option<Session> = None;

// Estrutura da sessão
#[derive(Debug)]
pub struct Session {
    data: HashMap<String, String>,
    //status: i8,
}

impl Session {
    // Cria uma nova sessão vazia
    fn new() -> Self {
        Session {
            data: HashMap::new(),
            //status: 0,
        }
    }

    // Adiciona ou atualiza uma propriedade na sessão
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    // Obtém o valor de uma propriedade na sessão
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    // Remove uma propriedade da sessão
    pub fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }

    // Verifica o valor das propriedades
    pub fn has_value(&self, key: &str, value: &str) -> bool {
        self.get(key).map(|v| v == value).unwrap_or(false)
    }
}

// Inicializa a sessão global, caso ainda não esteja inicializada
pub fn initialize_session() {
    unsafe {
        if GLOBAL_SESSION.is_none() {
            GLOBAL_SESSION = Some(Session::new());
        }
    }
}

// Obtém uma referência mutável à sessão global
pub fn get_session() -> &'static mut Session {
    unsafe {
        GLOBAL_SESSION
            .as_mut()
            .expect("Sessão não inicializada. Chame initialize_session() primeiro.")
    }
}
