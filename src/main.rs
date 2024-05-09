mod templates;

use axum::{routing::get, Router};
use rs_docker::Docker;
use templates::{Container, Containers, Index};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/containers", get(containers))
        .nest_service("/css", ServeDir::new("static/css"));
    // .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"));

    let tcp_listener = TcpListener::bind(&"0.0.0.0:42069")
        .await
        .expect("couldn't bind to port 42069");

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .expect("couldn't host server");
}

async fn index() -> Index {
    Index {}
}

async fn containers() -> Containers {
    let mut docker = match Docker::connect("unix:///var/run/docker.sock") {
        Ok(docker) => docker,
        Err(e) => panic!("{}", e),
    };

    let docker_containers = tokio::task::spawn_blocking(move || {
        return match docker.get_containers(true) {
            Ok(c) => c,
            Err(e) => panic!("{}", e),
        };
    })
    .await
    .expect("Task panicked");
    println!("{:?}", docker_containers);

    let containers = docker_containers
        .iter()
        .map(|x| {
            return Container {
                id: x.Id.to_string(),
                description: x.Names.join(", ").to_string(),
            };
        })
        .collect();

    Containers { containers }
}
