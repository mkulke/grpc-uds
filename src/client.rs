use attester::{attester_client::AttesterClient, EvidenceRequest, TeeRequest};
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

pub mod attester {
    tonic::include_proto!("attester");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = "/tmp/bla";
            UnixStream::connect(path)
        }))
        .await?;
    let mut client = AttesterClient::new(channel);
    let request = tonic::Request::new(TeeRequest {});
    let response = client.tee(request).await?;
    println!("Got: '{}' from service", response.into_inner().tee);
    let request = tonic::Request::new(EvidenceRequest {
        challenge: "omg".into(),
    });
    let response = client.evidence(request).await?;
    println!("Got: '{}' from service", response.into_inner().evidence);
    Ok(())
}
