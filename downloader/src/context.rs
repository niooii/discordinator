use std::path::PathBuf;

use crate::conversation::{Conversation, Turn};
use discord::model::message::Message;
use serde::Serialize;
use tokio::fs;

#[derive(Serialize)]
pub struct Context(Vec<Conversation>);

impl Context {
    pub fn new() -> Self {
        // TODO! sorting into multiple convos how
        let thing = vec![Conversation::new()];
        Self(thing)
    }

    pub fn add(&mut self, message: &Message) {
        let turn = Turn::new(message);
        if let Some(t) = turn {
            self.0[0].add_turn(t);
        }
    }

    pub fn add_chunk(&mut self, messages: &[Message]) {
        for msg in messages {
            self.add(msg);
        }
    }

    pub async fn save_to(&self, path: PathBuf) {
        match serde_json::to_string_pretty(self) {
            Ok(v) => {
                if let Err(e) = fs::write(path, v).await {
                    eprintln!("Failed to write file: {e}");
                } else {
                    println!("Save successful.");
                }
            },
            Err(e) => {
                eprintln!("FAIL: {e}");
            },
        }
    }
}