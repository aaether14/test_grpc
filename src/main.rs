use tonic::{ transport::Server, Request, Response, Status };

mod hello {
    tonic::include_proto!("hello");
}
use hello::say_server::{ Say, SayServer };
use hello::{ SayResponse, SayRequest };

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        println!("requests: {:?}", request.get_ref());
        Ok(Response::new(SayResponse {
             message: format!("hello {}", request.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let say = MySay::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}