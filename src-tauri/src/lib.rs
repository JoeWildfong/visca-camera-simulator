use std::net::{Ipv4Addr, SocketAddrV4};
use tauri::{Emitter, State};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, sync::{mpsc::{Receiver, Sender, channel, error::TryRecvError}, watch}};
use tracing::{Level, event, span};

async fn accept_loop(mut outgoing: Receiver<Vec<u8>>, incoming: Sender<Vec<u8>>, connected: watch::Sender<bool>) -> std::io::Result<()> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 41414)).await?;
    loop {
        let _ = connected.send(false);
        let (mut connection, _addr) = {
            let span = span!(Level::INFO, "waiting for connection");
            let _entered = span.enter();
            listener.accept().await?
        };
        let _ = connected.send(true);
        loop {
            let span = span!(Level::DEBUG, "draining waiting messages");
            let _entered = span.enter();
            match outgoing.try_recv() {
                Ok(_) => {},
                Err(TryRecvError::Disconnected) => return Ok(()),
                Err(TryRecvError::Empty) => break,
            }
        }
        let mut buf = [0_u8; 1024];
        let span = span!(Level::INFO, "connected");
        let _entered = span.enter();
        loop {
            event!(Level::TRACE, "io ready");
            tokio::select! {
                outbound = outgoing.recv() => {
                    let Some(msg) = outbound else {
                        return Ok(());
                    };
                    event!(Level::INFO, "out: {:x?}", msg.as_slice());
                    if let Err(e) = connection.write_all(&msg).await {
                        event!(Level::ERROR, "write error {e}, closing connection");
                        break;
                    }
                }
                // todo: use an actual parser instead of assuming one packet == one message
                inbound = connection.read(&mut buf) => {
                    let n = match inbound {
                        Ok(0) => {
                            event!(Level::INFO, "connection closed");
                            break;
                        },
                        Ok(n) => n,
                        Err(e) => {
                            event!(Level::ERROR, "read error {e}, closing connection");
                            break;
                        }
                    };
                    let msg = buf[..n].to_vec();
                    event!(Level::INFO, "in: {:x?}", msg.as_slice());
                    let _ = incoming.send(msg).await;
                }
            }
        }
    }
}

struct AppState {
    outgoing: Sender<Vec<u8>>,
    connected: watch::Receiver<bool>,
}

#[tauri::command]
fn send_data(data: Vec<u8>, state: State<'_, AppState>) {
    let _ = state.outgoing.try_send(data);
}

#[tauri::command]
async fn stream_connected(channel: tauri::ipc::Channel<bool>, state: State<'_, AppState>) -> tauri::Result<()> {
    let mut receiver = state.connected.clone();
    drop(state);
    loop {
        let Ok(()) = receiver.changed().await else {
            break;
        };
        let connected_state = *receiver.borrow_and_update();
        channel.send(connected_state)?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (send_outgoing, recv_outgoing) = channel(2);
    let (send_incoming, mut recv_incoming) = channel(2);
    let (send_connected, recv_connected) = watch::channel(false);
    let _handle = tauri::async_runtime::spawn(accept_loop(recv_outgoing, send_incoming, send_connected));

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Some(msg) = recv_incoming.recv().await {
                    event!(Level::INFO, "emitting incoming-msg event {:x?}", msg.as_slice());
                    if let Err(e) = handle.emit("incoming-msg", msg) {
                        event!(Level::ERROR, "failed to emit incoming-msg event: {e}");
                    }
                };
            });
            Ok(())
        })
        .manage(AppState {
            outgoing: send_outgoing,
            connected: recv_connected,
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            send_data,
            stream_connected,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
