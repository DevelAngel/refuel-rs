mod helloworld;

use self::helloworld::hello_world::greeter_server::GreeterServer;
use self::helloworld::MyGreeter;

use tonic::transport::{Error, Server};

use tracing::info;

pub(crate) async fn service() -> Result<(), Error> {
    let addr = "[::1]:50051".parse().unwrap();

    let greeter = MyGreeter::default();
    let service = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr);
    info!("GreeterServer listening on {}", addr);
    service.await
}
