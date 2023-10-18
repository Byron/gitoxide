/// The name of the `git` client in a format suitable for presentation to a `git` server, using `name` as user-defined portion of the value.
pub fn agent(name: impl Into<String>) -> String {
    let mut name = name.into();
    if !name.starts_with("git/") {
        name.insert_str(0, "git/");
    }
    name
}

/// Send a message to indicate the remote side that there is nothing more to expect from us, indicating a graceful shutdown.
/// If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate.
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
#[maybe_async::maybe_async]
pub async fn indicate_end_of_interaction(
    mut transport: impl gix_transport::client::Transport,
    trace: bool,
) -> Result<(), gix_transport::client::Error> {
    // An empty request marks the (early) end of the interaction. Only relevant in stateful transports though.
    if transport.connection_persists_across_multiple_requests() {
        transport
            .request(
                gix_transport::client::WriteMode::Binary,
                gix_transport::client::MessageKind::Flush,
                trace,
            )?
            .into_read()
            .await?;
    }
    Ok(())
}
