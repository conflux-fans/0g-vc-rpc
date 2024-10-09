mod rpc;
mod types;

use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;

use ark_bn254::Bn254;
use ark_groth16::Proof;

use rpc::api::ZgVcServer;
use rpc::impls::RpcImpl;
use vc_prove::{
    circuit::circom_builder, groth16::prove, params::load_proving_key, types::VcProveInput,
};

use jsonrpsee::server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Loading circom, will take a while...");

    let circom = circom_builder(&"output".into(), "check_vc");
    let pk = load_proving_key::<false>(&"output".into(), "check_vc")
        .expect("ProvingKey should load success");

    // spawn a thread to handle proof generation
    let (tx, rx) = mpsc::channel::<(VcProveInput, mpsc::Sender<Proof<Bn254>>)>();
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok((input, sender)) => {
                    let proof = prove(&pk, &circom, input).expect("generate proof should success"); // TODO handle error
                    sender.send(proof).expect("channel send should success"); // TODO handle error
                }
                Err(e) => {
                    println!("Channel Recv Error: {:?}", e);
                    break;
                }
            }
        }
    });

    // start server
    let server = Server::builder()
        .build("127.0.0.1:3030".parse::<SocketAddr>()?)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcImpl::new(tx).into_rpc());
    println!("Server is listening on: {}", addr);

    // style 1
    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    // EG: monitor the exit signal and shutdown the server
    // handle.stopped().await;

    // style 2
    // Set up a signal handler for graceful shutdown
    // let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();

    // // Spawn a task to handle the shutdown signal
    // tokio::spawn(async move {
    //     tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    //     println!("Received shutdown signal. Shutting down...");
    //     shutdown_sender.send(()).expect("Failed to send shutdown signal");
    // });

    // // Wait for either the server to stop or the shutdown signal
    // tokio::select! {
    //     _ = handle.clone().stopped() => {
    //         println!("Server stopped unexpectedly");
    //     }
    //     _ = shutdown_receiver => {
    //         println!("Initiating graceful shutdown");
    //         handle.stop()?;
    //         println!("Server shut down successfully");
    //     }
    // }

    // style 3
    // tokio::signal::ctrl_c()
    //     .await
    //     .expect("Failed to listen for ctrl+c");
    // handle.stop()?;

    // style 4
    let handle_clone = handle.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl+c");
        // TODO graceful handle unfinished task
        handle_clone.stop().expect("Failed to stop server");
    });

    handle.stopped().await;
    println!("Server stopped");

    Ok(())
}
