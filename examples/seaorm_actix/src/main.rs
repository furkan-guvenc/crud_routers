use std::io;
use std::net::TcpListener;
use seaorm_actix::run;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Could not bind TCP listener");
    run(listener).await?.await
}

#[cfg(test)]
mod tests {
    use crate::run;
    use std::net::TcpListener;
    use test_utils::{TestApp, e2e_test};

    async fn spawn_app() -> TestApp{
        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Could not bind TCP listener");
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await.expect("Failed to bind address");
        let _ = tokio::spawn(server);

        TestApp::new(format!("http://127.0.0.1:{}", port), "posts")
    }

    #[tokio::test]
    async fn e2e(){
        let app = spawn_app().await;

        e2e_test(app).await;
    }

}
