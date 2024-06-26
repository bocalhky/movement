use crate::send_eth_transaction::InsufficentFunds;
use crate::send_eth_transaction::SendTransactionErrorRule;
use crate::send_eth_transaction::UnderPriced;
use crate::send_eth_transaction::VerifyRule;
use crate::{CommitmentStream, McrSettlementClientOperations};
use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_provider::fillers::ChainIdFiller;
use alloy_provider::fillers::FillProvider;
use alloy_provider::fillers::GasFiller;
use alloy_provider::fillers::JoinFill;
use alloy_provider::fillers::NonceFiller;
use alloy_provider::fillers::SignerFiller;
use std::array::TryFromSliceError;
use std::str::FromStr;
//use alloy_provider::fillers::TransactionFiller;
use alloy_provider::{ProviderBuilder, RootProvider};
use alloy_transport::Transport;
use movement_types::{Commitment, Id};
use std::marker::PhantomData;
use tokio_stream::StreamExt;
//use alloy_network::Network;
use alloy_provider::Provider;
use thiserror::Error;
//use alloy_network::EthereumSigner;
use alloy_primitives::U256;
//use alloy_provider::ProviderBuilder;
use alloy_sol_types::sol;
//use alloy_transport_http::Http;
use alloy::pubsub::PubSubFrontend;
use alloy_network::EthereumSigner;
use alloy_signer_wallet::LocalWallet;
use alloy_transport::BoxTransport;
use alloy_transport_ws::WsConnect;
use movement_types::BlockCommitment;
use std::env;
