use crate::repo::create_repository;
use tonic::{Request, Response, Status};

pub mod cargo_ci {
    tonic::include_proto!("cargo_ci");
}

#[derive(Default)]
pub struct CargoCiService;

#[tonic::async_trait]
impl cargo_ci::cargo_ci_server::CargoCi for CargoCiService {
    async fn create_repo(
        &self,
        request: Request<cargo_ci::CreateRepoRequest>,
    ) -> Result<Response<cargo_ci::CreateRepoResponse>, Status> {
        let req = request.into_inner();

        match create_repository(&req.org, &req.project_name, &req.framework, "").await {
            Ok(repo_url) => Ok(Response::new(cargo_ci::CreateRepoResponse { repo_url })),
            Err(e) => Err(Status::internal(format!("Failed to create repo: {}", e))),
        }
    }
}
