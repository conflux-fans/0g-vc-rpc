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
        .build("0.0.0.0:3030".parse::<SocketAddr>()?)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcImpl::new(tx).into_rpc());
    println!("Server is listening on: {}", addr);

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
