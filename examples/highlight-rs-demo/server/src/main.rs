#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use debug::Env;

mod server;

use server::Server;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    Env::setup().await;

    Server::setup().await;
}
