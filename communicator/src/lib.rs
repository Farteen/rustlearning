
pub mod client;
pub mod network;

// mod client {
//     fn connect() {

//     }
// }

#[cfg(test)]
mod tests {
	use super::client;
    #[test]
    fn it_works() {
    	// ::client::connect();
    	client::connect();
    }
}