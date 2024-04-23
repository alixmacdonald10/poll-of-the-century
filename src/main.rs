use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
struct SharedState {
    alix: usize,
    other: usize
}



#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // define some state shared between all threads
    let shared_state = Arc::new(RwLock::new(SharedState { alix: 0, other: 0}));

    // build our app
    let app = Router::new()
        .route("/", get(index))
        .route("/alix", get(get_alix))
        .route("/inc-alix", post(increment_count_alix))
        .route("/other", get(get_other))
        .route("/inc-other", post(increment_count_other))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let listener = match tokio::net::TcpListener::bind("0.0.0.0").await {
    //     Ok(listener) => {
    //         tracing::info!("cool beans");
    //         listener
    //     },
    //     Err(err) => {
    //         tracing::error!("{:#}", err);
    //         panic!("This wont work!");
    //     }
    // };
    // let listener = tokio::net::TcpListener::bind("0.0.0.0").await
    //     .unwrap_or({
    //         tracing::error!("there was an error binding listener, binding to available port.");
    //         tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap()
    //     });

    // run the app
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    Html(include_str!("./index.html").to_string())
}

async fn increment_count_alix(State(state): State<Arc<RwLock<SharedState>>>) {
    
    tracing::info!("Correct answer!");

    {
        let mut lock = state.write().await;
        lock.alix += 1;
        lock.alix.clone().to_string()
    };
}

async fn get_alix(State(state): State<Arc<RwLock<SharedState>>>) -> Html<String> {

    let return_val = {
        let lock = state.read().await;
        lock.alix.clone().to_string()
    };
    Html(return_val)
}


async fn increment_count_other(State(state): State<Arc<RwLock<SharedState>>>) {

    tracing::warn!("Are you sure you want to do this?");
    
    let fate = {
        // range is not Send so make sure its out of context prior to any awaits
        let mut range = rand::thread_rng();
        let possibilities = Uniform::from(1..100);
        possibilities.sample(&mut range)
    };

    if fate < 10 {
        tracing::error!("You lucky bugger"); 
        // let null: &mut u8 = cve_rs::null_mut::<u8>();
        // *null = 42;
        
        let mut lock = state.write().await;
        lock.other += 1;
    } else {
        tracing::info!("Sanity prevails!");
        let mut lock = state.write().await;
        lock.alix += 1;
    }; 

}


async fn get_other(State(state): State<Arc<RwLock<SharedState>>>) -> Html<String> {

    let return_val = {
        let lock = state.read().await;
        lock.other.clone().to_string()
    };
    Html(return_val)
}

