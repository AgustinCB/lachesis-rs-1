use lachesis_rs::HttpServer;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("lachesis_server");
    HttpServer::start();
    let _ = sys.run();
}
