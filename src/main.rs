use std::sync::Arc;

use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use rand::distributions::{Distribution, Uniform};
use tokio::sync::RwLock;

#[derive(Debug)]
struct SharedState {
    rust: usize,
    python: usize
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(RwLock::new(SharedState { rust: 0, python: 0}));    


    // build our app
    let app = Router::new()
        .route("/", get(index))
        .route("/rust", get(get_rust))
        .route("/inc-rust", post(increment_count_rust))
        .route("/python", get(get_python))
        .route("/inc-python", post(increment_count_python))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}


async fn index() -> Html<String> {
    Html(include_str!("./index.html").to_string())
}

async fn increment_count_rust(State(state): State<Arc<RwLock<SharedState>>>) {
    
    tracing::info!("Correct answer!");

    {
        let mut lock = state.write().await;
        lock.rust += 1;
        lock.rust.clone().to_string()
    };
}

async fn get_rust(State(state): State<Arc<RwLock<SharedState>>>) -> Html<String> {

    let return_val = {
        let lock = state.read().await;
        lock.rust.clone().to_string()
    };
    Html(return_val)
}


async fn increment_count_python(State(state): State<Arc<RwLock<SharedState>>>) {
    tracing::info!("Are you really sure you want to do this?");

    let fate = {
        let mut range = rand::thread_rng();
        let possibilities = Uniform::from(1..100);
        possibilities.sample(&mut range)
    };

    if fate < 10 {
        tracing::error!("You lucky bugger");
        // this will destroy
        // let null = cve_rs::null_mut::<u8>();
        // *null = 42;

        let mut lock = state.write().await;
        lock.python += 1;
    } else {
        tracing::info!("Sanity prevails");
        let mut lock = state.write().await;
        lock.rust += 1;
    };
}


async fn get_python(State(state): State<Arc<RwLock<SharedState>>>) -> Html<String> {

    let return_val = {
        let lock = state.read().await;
        lock.python.clone().to_string()
    };
    Html(return_val)
}
