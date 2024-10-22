#[cfg(feature = "axum")]
mod axum;
#[cfg(feature = "actix")]
mod actix;

#[cfg(feature = "axum")]
pub use axum::AxumServer;
#[cfg(feature = "actix")]
pub use actix::ActixServer;

pub trait ApiServer {
    fn get_path(prefix: &str) -> String {
        format!("/{}", prefix)
    }

    fn get_id_path(prefix: &str) -> String;
}
