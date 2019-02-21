use crate::sentence_generator::SentenceGenerator;
use crate::CliError;
use actix_web::{server, App, HttpRequest, Responder};
use std::sync::Arc;

struct AppState {
    sg: Arc<SentenceGenerator>,
}

pub fn new(port: u32, sg: SentenceGenerator) -> Result<(), CliError> {
    let sg = Arc::new(sg);
    server::new(move || {
        App::with_state(AppState { sg: sg.clone() }).resource("/", |r| r.f(sentence))
    })
    .bind(format!("127.0.0.1:{}", port))
    .map_err(|_| CliError::ServerPortBindingFailed(port))?
    .run();
    Ok(())
}

fn sentence(req: &HttpRequest<AppState>) -> impl Responder {
    let sg = &req.state().sg;
    format!("{}", sg.get_random_sentence())
}
