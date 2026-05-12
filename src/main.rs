use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::{thread, time};
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
pub user_id: String,
pub user_name: String
}
pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
fn handle(&self, message: Box<UserCreatedEventMessage>
) -> Result<(), HandleError> {
let ten_millis = time::Duration::from_millis(1000);
let now = time::Instant::now();
// thread::sleep(ten_millis);
println!("In Fakhri’s Computer [2406436972]. Message received: {:?}",
message);
Ok(())
}
