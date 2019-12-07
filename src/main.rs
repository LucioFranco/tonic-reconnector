use hyper::client::service::Connect; 
use hyper::client::conn::Builder;
use hyper::client::connect::HttpConnector;
use tower_service::Service;
use tower_make::MakeService;
use hyper::Uri;

mod reconn;
mod add_origin;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Builder::new().http2_only(true).clone();
    let connector = HttpConnector::new();

    // This is a MakeService that can take a Target -> Connection
    let maker = Connect::new(connector, settings); 
    
    let dst = Uri::from_static("[::1]:50051");

    // Setup a reconnect service that takes a MakeService (Service factory)
    // and a Target, and will attempt to lazily connect and reestablish if 
    // the connection gets interupted.
    //
    // Something that is not so obvious is that this Reconnect Service
    // doesn't implement `MakeService` but implements just a Service that
    // returns the regualr connections request/response type. This is because
    // reconnect is designed to take some connection factory then act as a layer
    // in between them to make it easier to deal with connecting. This will end up
    // wrapping your connect into the same future that you used to dispatch the request.
    let conn = reconn::Reconnect::new(maker, dst.clone());

    // Hyper needs this to add a Origin header to each outbound http request.
    let svc = add_origin::AddOrigin::new(conn, dst);

    // Construct our greeter client from the service.
    let mut greeter = pb::client::GreeterClient::new(svc); 

    let res = greeter.say_hello(pb::HelloRequest { name: "me".into()}).await?;

    println!("response {:?}", res);

    Ok(())
}

pub mod pb {
    tonic::include_proto!("helloworld");
}