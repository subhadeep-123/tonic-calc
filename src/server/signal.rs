use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::oneshot;
use tracing::info;

// Shutdown controller with sender and receiver
pub struct Shutdown {
    pub receiver: oneshot::Receiver<()>,
    pub task: tokio::task::JoinHandle<()>,
}

/// Create a shutdown signal handler that listens for SIGINT/SIGTERM
pub fn setup_shutdown_handler() -> Shutdown {
    // Create a shutdown channel
    let (tx, rx) = oneshot::channel::<()>();

    let task = tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to bind SIGTERM");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to bind SIGINT");

        tokio::select! {
            _ = sigterm.recv() => info!("Received SIGTERM"),
            _ = sigint.recv() => info!("Received SIGINT"),
        }

        info!("Sending shutdown signal");
        let _ = tx.send(());
    });

    Shutdown { receiver: rx, task }
}
