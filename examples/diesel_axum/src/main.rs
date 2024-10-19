use diesel_axum::run;
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8008").await
        .expect("Could not bind TCP listener");
    run(listener).await
}

#[cfg(test)]
mod tests {
    use crate::run;
    use tokio::net::TcpListener;
    use test_utils::{TestApp, e2e_test};

    async fn spawn_app() -> TestApp{
        let listener = TcpListener::bind("127.0.0.1:0").await
            .expect("Could not bind TCP listener");
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            run(listener).await.unwrap();
        });

        TestApp::new(format!("http://127.0.0.1:{}", port), "base/api")
    }

    #[tokio::test]
    async fn e2e(){
        let app = spawn_app().await;

        e2e_test(app).await;
    }

}
