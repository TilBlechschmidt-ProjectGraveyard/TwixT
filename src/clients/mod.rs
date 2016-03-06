use structures::*;

pub trait Client {
    fn run(&self, _b: &Board, _l: &Links, player: u8) -> Move;
}

#[cfg(feature = "simple_client")]
pub mod simple_client;
#[cfg(feature = "simple_client")]
pub use self::simple_client::*;

#[cfg(feature = "neural_network")]
pub mod neural_network;
#[cfg(feature = "neural_network")]
pub use self::neural_network::*;
