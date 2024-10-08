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

    let (tx, rx) = mpsc::channel::<(VcProveInput, mpsc::Sender<Proof<Bn254>>)>();
    thread::spawn(move || loop {
        match rx.recv() {
            Ok((input, sender)) => {
                let proof = prove(&pk, &circom, input).expect("generate proof should success"); // TODO handle error
                sender.send(proof).expect("channel send should success"); // TODO handle error
            }
            Err(e) => println!("Channel Recv Error: {:?}", e),
        }
    });

    let server = Server::builder()
        .build("127.0.0.1:3030".parse::<SocketAddr>()?)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(RpcImpl::new(tx).into_rpc());
    println!("Server is listening on: {}", addr);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    // EG: monitor the exit signal and shutdown the server
    handle.stopped().await;

    // tokio::spawn(handle.stopped());  // spawn a thread to wait for stopped

    Ok(())
}
