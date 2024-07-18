use std::{collections::HashMap, ops::{Deref, DerefMut}, sync::{Arc, RwLock}, cell::RefCell};

use anyhow::{anyhow, Ok};
use automerge::{sync::{State, SyncDoc}, transaction::Transactable, AutoCommit, ObjId, ObjType, ReadDoc};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::fs;

struct Message {
    id: u64,
    uid: String,
    content: String,
    timestamp: i64,
}


#[derive(Debug, Clone)]
pub struct MessageStateItem {
    pub doc: AutoCommit,

    pub state: State,
}



impl TryFrom<&[u8]> for MessageStateItem {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(MessageStateItem {
            doc: AutoCommit::load(value)?,
            state: State::new(),
        })
    }
}

impl TryFrom<Vec<u8>> for MessageStateItem {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self {
            doc: AutoCommit::load(&value)?,
            state: State::new()
        })
    }
}


pub struct MessageState (Arc<RwLock<HashMap<String, MessageStateItem>>>);

impl Deref for MessageState {
    type Target = Arc<RwLock<HashMap<String, MessageStateItem>>>;
    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

impl DerefMut for MessageState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0
    }
}

impl MessageState {
    fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
}

#[derive(Serialize, Deserialize)]
struct MessageSerializableState {
    state: HashMap<String, Vec<u8>>
}

async fn init_state() -> anyhow::Result<MessageState> {
    let message_data_path = dirs::data_local_dir().ok_or(anyhow!("failed to get home dir"))?.join("messages.data");
    if !message_data_path.exists() {
        return Ok(MessageState::new())
    }
    let message_data = fs::read(message_data_path).await?;
    // TODO: serialize message state when exiting
    let deserialized: MessageSerializableState = serde_json::from_slice(&message_data)?;
    let state = MessageState::new();
    let mut locked_state = state.write().unwrap();

    for (k, v) in deserialized.state.into_iter() {
        locked_state.insert(k, v.as_slice().try_into()?);
    }

    Ok(MessageState::new())
}

fn insert_message_to_doc(doc: &mut AutoCommit, list: &ObjId, message: Message) -> anyhow::Result<()> {
    let obj = doc.insert_object(list, doc.length(list), ObjType::Map)?;
    doc.put(&obj, "id", message.id)?;
    doc.put(&obj, "uid", message.uid)?;
    doc.put(&obj, "content", message.content)?;
    doc.put(&obj, "timestamp", message.timestamp)?;

    Ok(())
}


impl MessageState {
    // FIXME: sync message maybe needs to be generated multiple times
    async fn commit_message_and_get_sync_state(self, uid: String, content: String) -> anyhow::Result<Option<Vec<u8>>>  {
        let ref mut locked_state = self.write().unwrap();
        match locked_state.get_mut(&uid) {
            Some(item) => {
                let message_list = match item.doc.get(automerge::ROOT, "messages")? {
                    Some((automerge::Value::Object(ObjType::List), message_list)) => message_list,
                    _ => panic!("no message list found"),
                };
                insert_message_to_doc(&mut item.doc, &message_list, Message{
                    id: 0,
                    uid,
                    content,
                    timestamp: Utc::now().timestamp(),
                })?;
                Ok(item.doc.sync().generate_sync_message(&mut item.state).map(|msg| msg.encode()))
            },
            None => Err(anyhow!("no matching uid found")),
        }
    }
}
