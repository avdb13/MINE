use std::{cell::RefCell, collections::HashSet, io, rc::Rc};

use tracing_subscriber::{fmt::Layer, Registry};
use weechat::{
    config::{BooleanOptionSettings, Config, ConfigSectionSettings},
    Args, Weechat,
};

use crate::server::Server;

pub struct Plugin {
    rt: tokio::runtime::Runtime,
    servers: Rc<RefCell<HashSet<Server>>>,
    config: Config,
    // commands: Commands,
    // bar_items: BarItems,
    // typing_notice_signal: SignalHook,
    // completions: Completions,
    // debug_buffer: RefCell<Option<BufferHandle>>,
}

impl Plugin {
    // fn autoconnect(servers: &HashMap<String, MatrixServer>) {
    //     for server in servers.values() {
    //         if server.autoconnect() {
    //             match server.connect() {
    //                 Ok(_) => (),
    //                 Err(e) => Weechat::print(&format!("{:?}", e)),
    //             }
    //         }
    //     }
    // }

    // fn create_default_server(servers: Servers, config: &ConfigHandle) {
    //     // TODO change this to matrix.org.
    //     let server_name = "localhost";

    //     let mut config_borrow = config.borrow_mut();
    //     let mut section = config_borrow
    //         .search_section_mut("server")
    //         .expect("Can't get server section");

    //     let server = MatrixServer::new(
    //         server_name,
    //         config,
    //         &mut section,
    //         servers.clone(),
    //     );
    //     servers.insert(server);
    // }
}

impl weechat::Plugin for Plugin {
    fn init(_: &Weechat, _args: Args) -> io::Result<Self> {
        let rt = tokio::runtime::Runtime::new()?;

        let subscriber = Registry::default().with(Layer::new());
        tracing::subscriber::set_global_default(subscriber)
            .expect("Could not initialize tracing");

        let config = Rc::new(RefCell::new(
            Config::new("mine").expect("Can't create new config"),
        ));

        // let servers = Servers::new(global_runtime.handle().to_owned());
        // let config = ConfigHandle::new(&servers);
        // let commands = Commands::hook_all(&servers, &config)?;

        // let bar_items = BarItems::hook_all(servers.clone())?;
        // let completions = Completions::hook_all(servers.clone())?;

        // let subscriber = tracing_subscriber::registry()
        //     .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        //     .with(tracing_subscriber::fmt::layer().with_writer(debug::Debug));

        // let _ = tracing::subscriber::set_global_default(subscriber).map_err(
        //     |_err| Weechat::print("Unable to set global default subscriber"),
        // );

        // {
        //     let config_borrow = config.borrow();
        //     if config_borrow.read().is_err() {
        //         return Err(());
        //     }
        // }

        // if servers.is_empty() {
        //     Matrix::create_default_server(servers.clone(), &config)
        // }

        // let typing = SignalHook::new("input_text_changed", servers.clone())
        //     .expect("Can't create signal hook for the typing notice cb");

        // let plugin = Matrix {
        //     global_runtime,
        //     servers: servers.clone(),
        //     commands,
        //     config,
        //     bar_items,
        //     completions,
        //     debug_buffer: RefCell::new(None),
        //     typing_notice_signal: typing,
        // };

        // Weechat::spawn(async move {
        //     let servers = servers.borrow();
        //     Matrix::autoconnect(&servers);
        // })
        // .detach();

        Ok(Plugin {
            rt,
            servers,
            config,
        })
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        //let servers = self.servers.borrow();

        //// Buffer close callbacks get called after this, so disconnect here so
        //// we don't leave all our rooms.
        ////
        //// TODO set a flag on the server as well so we don't even try to leave
        //// the rooms, once leaving the rooms is implemented when the buffer gets
        //// closed.
        //for server in servers.values() {
        //    server.disconnect();
        //}

        //drop(servers);

        //self.servers.clear();
    }
}
