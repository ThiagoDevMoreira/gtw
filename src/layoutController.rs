use crate::login::layoutEntrar::*;
use crate::login::core::get_session;
use crate::geradorDeToken::layout::*;

pub fn TelaFuncionalidadesPrincipais(){
    let session = get_session();

    if session.has_value("status", "0") {
        layout_realizarLogin();
    } else {
        layout_geradorDeToken();
    }
}
