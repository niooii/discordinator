/// So this whoel thing kinda depedns on the sequential order of messages
/// being retained when getting info from discord's api.

use std::{cmp::{Ordering, Reverse}, collections::BinaryHeap};
use serde::Serialize;
use discord::model::{message::{DefaultMessageData, Message}, user::UserData, Snowflake};
use time::OffsetDateTime;
use std::vec::IntoIter;
use std::iter::Rev;
/// Represents a turn in a conversation 
#[derive(Serialize, Debug)]
pub struct Turn {
    pub content: String,
    pub author: String,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub start_time: OffsetDateTime,
}

impl Turn {
    /// This fails if the type of the message should not be included in the training context
    pub fn new(msg: &Message) -> Option<Self> {
        let (data, replied_to)
        : (&DefaultMessageData, &Option<Box<Message>>) = match msg {
            Message::Default(default_message_data) => {
                (default_message_data, &None)
            },
            Message::Reply(reply_message_data) => {
                (&reply_message_data.message, &reply_message_data.referenced_message)
            },
            _ => return None
        };

        Some(
            Self {
                start_time: data.general.timestamp.clone(),
                author: data.author.username.clone(),
                content: data.content.clone()
            }
        )
    }
}

pub struct Conversation(Vec<Turn>);

impl Serialize for Conversation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let reversed: Vec<_> = self.0.iter().rev().collect();
        
        reversed.serialize(serializer)
    }
}

impl Conversation {
    pub fn new() -> Self {
        Self (Vec::new())
    }

    pub fn add_turn(&mut self, turn: Turn) {
        // TODO! a lot
        // For now combine everything into a single conversation and dont
        // attempt organizing by context
        let turns = &mut self.0;
        
        if let Some(t) = turns.last_mut() {
            if t.author == turn.author {
                t.content = format!("{}\n{}", turn.content, t.content);
                return;
            }
        } 
        turns.push(turn);
    }

    pub fn print(&self) {
        for turn in self {
            println!("\n{} at {}", turn.author, turn.start_time.time());
            println!("{}", turn.content);
        }
    }
}

impl<'a> IntoIterator for &'a Conversation {
    type Item = &'a Turn;
    type IntoIter = Rev<std::slice::Iter<'a, Turn>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().rev()
    }
}

impl IntoIterator for Conversation {
    type Item = Turn;
    type IntoIter = Rev<IntoIter<Turn>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().rev()
    }
}

impl Ord for Turn {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time
    }
}

impl Eq for Turn {}