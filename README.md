# What is weechat-matrix?

[Weechat](https://weechat.org/) is an extensible chat client.

[Matrix](https://matrix.org/blog/home) is an open network for secure,
decentralized communication.

weechat-matrix-rs is a Rust plugin for Weechat that lets Weechat communicate
over the Matrix protocol. This is a Rust rewrite of the
[weechat-matrix](https://github.com/poljar/weechat-matrix) Python script.

# Project status

This fork is an attempt at reviving the original project, [weechat-matrix-rs](https://github.com/poljar/weechat-matrix-rs).

# Build

After Rust is installed the plugin can be compiled with:

    cargo build

On Linux this creates a `libmatrix.so` file in the `target/debug/` folder, this
file needs to be renamed to `matrix.so` and copied to your Weechat plugin
directory. A plugin directory can be created in your `$WEECHAT_HOME` folder, by
default `.weechat/plugins/`.

Alternatively, `make install` will build and install the plugin in your
`$WEECHAT_HOME` as well.
