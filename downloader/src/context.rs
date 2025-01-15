use crate::conversation::Conversation;
use discord::model::message::Message;

pub struct Context {

}

impl Context {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn add(&mut self, message: &Message) {
        println!("message added... (not really)");
    }

    pub fn add_chunk(&mut self, messages: &[Message]) {
        for msg in messages {
            self.add(msg);
        }
    }
}