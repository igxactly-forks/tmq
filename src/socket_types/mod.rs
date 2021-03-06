/// Dealer Sockets
pub mod dealer;
/// Publish Sockets
pub mod publish;
/// Pull Sockets
pub mod pull;
/// Push Sockets
pub mod push;
/// Request/Reply Sockets
pub mod request_reply;
/// Router Sockets
pub mod router;
/// Subscribe Sockets
pub mod subscribe;
/// Pair Sockets
pub mod pair;

pub use dealer::dealer;
pub use publish::publish;
pub use pull::pull;
pub use push::push;
pub use request_reply::reply;
pub use request_reply::request;
pub use router::router;
pub use subscribe::subscribe;
pub use pair::pair;

#[doc(hidden)]
pub trait FromZmqSocket<T> {
    fn from_zmq_socket(socket: zmq::Socket) -> crate::Result<T>;
}
