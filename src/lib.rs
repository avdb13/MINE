// mod bar_items;
// mod commands;
// mod completions;
// mod config;
// mod connection;
// mod debug;
// mod render;
// mod room;
// mod server;
// mod utils;
mod plugin;

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use plugin::Plugin;
use tokio::runtime::{Handle, Runtime};
use tracing_subscriber::layer::SubscriberExt;

use weechat::{
    buffer::{Buffer, BufferHandle},
    hooks::{SignalCallback, SignalData, SignalHook},
    Args, ReturnCode, Weechat,
};

// use crate::{
//     bar_items::BarItems, commands::Commands, completions::Completions,
//     config::ConfigHandle, room::RoomHandle, server::MatrixServer,
// };

weechat::plugin!(
    Plugin,
    name: "MINE",
    author: "[Damir Jelić <poljar@termina.org.uk>, Mikoto <avdb@keemail.me>]",
    description: "Matrix",
    version: "0.0.0",
    license: "ISC AND AGPL-3.0-or-later"
);

//#[derive(Clone, Debug)]
//pub struct Servers {
//    inner: Rc<RefCell<HashMap<String, MatrixServer>>>,
//    runtime: Handle,
//}

//#[allow(clippy::large_enum_variant)]
//pub enum Buffer {
//    Server(MatrixServer),
//    Space(MatrixServer),
//    Room(MatrixServer, RoomHandle),
//    None,
//}

//impl BufferOwner {
//    fn into_server(self) -> Option<MatrixServer> {
//        match self {
//            BufferOwner::Server(s) => Some(s),
//            BufferOwner::Room(s, _) => Some(s),
//            BufferOwner::None => None,
//        }
//    }

//    fn into_room(self) -> Option<RoomHandle> {
//        if let BufferOwner::Room(_, r) = self {
//            Some(r)
//        } else {
//            None
//        }
//    }
//}

//impl Servers {
//    fn new(handle: tokio::runtime::Handle) -> Self {
//        Servers {
//            inner: Rc::new(RefCell::new(HashMap::new())),
//            runtime: handle,
//        }
//    }

//    fn borrow(&self) -> Ref<'_, HashMap<String, MatrixServer>> {
//        self.inner.borrow()
//    }

//    pub fn runtime(&self) -> &Handle {
//        &self.runtime
//    }

//    pub fn is_empty(&self) -> bool {
//        self.inner.borrow().is_empty()
//    }

//    pub fn contains(&self, server_name: &str) -> bool {
//        self.inner.borrow().contains_key(server_name)
//    }

//    pub fn clear(&self) {
//        self.inner.borrow_mut().clear();
//    }

//    pub fn insert(&self, server: MatrixServer) {
//        self.inner
//            .borrow_mut()
//            .insert(server.name().to_string(), server);
//    }

//    pub fn get(&self, server_name: &str) -> Option<MatrixServer> {
//        self.inner.borrow().get(server_name).cloned()
//    }

//    pub fn remove(&self, server_name: &str) -> Option<MatrixServer> {
//        self.inner.borrow_mut().remove(server_name)
//    }

//    pub fn buffer_owner(&self, buffer: &Buffer) -> BufferOwner {
//        let servers = self.borrow();

//        for server in servers.values() {
//            if let Some(b) = &*server.server_buffer() {
//                if b.upgrade().map_or(false, |b| &b == buffer) {
//                    return BufferOwner::Server(server.clone());
//                }
//            }

//            for room in server.rooms() {
//                let buffer_handle = room.buffer_handle();

//                if let Ok(b) = buffer_handle.upgrade() {
//                    if buffer == &b {
//                        return BufferOwner::Room(server.clone(), room);
//                    }
//                }
//            }
//        }

//        BufferOwner::None
//    }

//    /// Find a `MatrixServer` that the given buffer belongs to.
//    ///
//    /// Returns None if the buffer doesn't belong to any of our servers of
//    /// rooms.
//    pub fn find_server(&self, buffer: &Buffer) -> Option<MatrixServer> {
//        self.buffer_owner(buffer).into_server()
//    }

//    /// Find a `RoomHandle` that the given buffer belongs to.
//    ///
//    /// Returns None if the buffer doesn't belong to any of our servers of
//    /// rooms.
//    pub fn find_room(&self, buffer: &Buffer) -> Option<RoomHandle> {
//        self.buffer_owner(buffer).into_room()
//    }
//}

//impl SignalCallback for Servers {
//    fn callback(
//        &mut self,
//        _: &Weechat,
//        _signal_name: &str,
//        data: Option<SignalData>,
//    ) -> ReturnCode {
//        if let Some(SignalData::Buffer(buffer)) = data {
//            if let Some(room) = self.find_room(&buffer) {
//                room.update_typing_notice();
//            }
//        }
//        ReturnCode::Ok
//    }
//}

// impl std::fmt::Debug for Matrix {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut fmt = f.debug_struct("Matrix");
//         fmt.field("servers", &self.servers).finish()
//     }
// }
