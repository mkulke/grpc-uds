use attester::{
    attester_server::{Attester, AttesterServer},
    EvidenceRequest, EvidenceResponse, TeeRequest, TeeResponse,
};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod attester {
    tonic::include_proto!("attester");
}

#[derive(Debug, Default)]
pub struct AttesterService;

#[tonic::async_trait]
impl Attester for AttesterService {
    async fn tee(&self, _request: Request<TeeRequest>) -> Result<Response<TeeResponse>, Status> {
        let res = TeeResponse {
            tee: "sometee".into(),
        };
        Ok(Response::new(res))
    }

    async fn evidence(
        &self,
        request: Request<EvidenceRequest>,
    ) -> Result<Response<EvidenceResponse>, Status> {
        let request = request.into_inner();
        let evidence = format!("{} + hello", request.challenge);
        let res = EvidenceResponse { evidence };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/bla";
    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);
    let attester_service = AttesterService::default();

    Server::builder()
        .add_service(AttesterServer::new(attester_service))
        .serve_with_incoming(uds_stream)
        .await?;
    Ok(())
}
