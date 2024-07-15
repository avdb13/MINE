// mod bar_items;
// mod commands;
// mod completions;
// mod config;
// mod connection;
// mod debug;
// mod render;
// mod room;
mod server;
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

weechat::plugin!(
    Plugin,
    name: "MINE",
    author: "[Damir Jelić <poljar@termina.org.uk>, Mikoto <avdb@keemail.me>]",
    description: "Matrix",
    version: "0.0.0",
    license: "ISC AND AGPL-3.0-or-later"
);
