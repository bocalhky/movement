use alloy_primitives::U256;
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::{Context, Result};
use publisher::{BonsaiProver, TxSender};
use clap::Parser;
use methods::IS_EVEN_ELF;

// `IEvenNumber` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface IEvenNumber {
        function set(uint256 x, bytes32 post_state_digest, bytes calldata seal);
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: String,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: String,

    /// The input to provide to the guest binary
    #[clap(short, long)]
    input: U256,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Create a new `TxSender`.
    let tx_sender = TxSender::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &args.contract,
    )?;

    // ABI encode the input for the guest binary, to match what the `is_even` guest
    // code expects.
    let input = args.input.abi_encode();

    // Send an off-chain proof request to the Bonsai proving service.
    let (journal, post_state_digest, seal) = BonsaiProver::prove(IS_EVEN_ELF, &input)?;

    // Decode the journal. Must match what was written in the guest with
    // `env::commit_slice`.
    let x = U256::abi_decode(&journal, true).context("decoding journal data")?;

    // Encode the function call for `IEvenNumber.set(x)`.
    let calldata = IEvenNumber::IEvenNumberCalls::set(IEvenNumber::setCall {
        x,
        post_state_digest,
        seal,
    })
    .abi_encode();

    // Send the calldata to Ethereum.
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}