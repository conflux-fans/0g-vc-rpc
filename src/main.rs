mod rpc;
mod types;
use ark_bn254::Bn254;
use ark_groth16::Proof;
use jsonrpc_http_server::ServerBuilder;
use rpc::api::ZgVc;
use rpc::impls::RpcImpl;
use std::sync::mpsc;
use std::thread;
use vc_prove::{
    circuit::circom_builder, groth16::prove, params::load_proving_key, types::ProveInput,
};

fn main() {
    let (tx, rx) = mpsc::channel::<(ProveInput, mpsc::Sender<Proof<Bn254>>)>();
    let circom = circom_builder(&"output".into(), "check_vc");
    let pk = load_proving_key::<false>(&"output".into(), "check_vc")
        .expect("ProvingKey should load success");

    thread::spawn(move || loop {
        match rx.recv() {
            Ok((input, sender)) => {
                let proof = prove(&pk, &circom, input).expect("generate proof should success"); // TODO handle error
                sender.send(proof).expect("channel send should success"); // TODO handle error
            }
            Err(e) => println!("Channel Recv Error: {:?}", e),
        }
    });

    let mut io = jsonrpc_core::IoHandler::new();
    let rpc_impl = RpcImpl::new(tx);
    io.extend_with(rpc_impl.to_delegate());

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
