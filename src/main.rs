mod http_server;

use http_server::log::debug;
use http_server::server;

fn main() {
    debug("Starting server");
    match server::run() {
        Ok(()) => {
            debug("Stopping server");
        }
        Err(_e) => {
            debug("Error running server");
        }
    }
}