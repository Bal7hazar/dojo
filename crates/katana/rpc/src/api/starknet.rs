use jsonrpsee::core::Error;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::{CallError, ErrorObject};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::core::serde::unsigned_field_element::UfeHex;
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
    ContractClass, DeclareTransactionResult, DeployAccountTransactionResult, EventFilterWithPage,
    EventsPage, FeeEstimate, FieldElement, FunctionCall, InvokeTransactionResult,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt,
    MsgFromL1, StateUpdate, Transaction,
};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Felt(#[serde_as(as = "UfeHex")] pub FieldElement);

// TODO: implement From<SequencerError> for StarknetApiError
#[derive(thiserror::Error, Clone, Copy, Debug)]
pub enum StarknetApiError {
    #[error("Failed to write transaction")]
    FailedToReceiveTxn = 1,
    #[error("Contract not found")]
    ContractNotFound = 20,
    #[error("Invalid message selector")]
    InvalidMessageSelector = 21,
    #[error("Invalid call data")]
    InvalidCallData = 22,
    #[error("Block not found")]
    BlockNotFound = 24,
    #[error("Transaction hash not found")]
    TxnHashNotFound = 29,
    #[error("Invalid transaction index in a block")]
    InvalidTxnIndex = 27,
    #[error("Class hash not found")]
    ClassHashNotFound = 28,
    #[error("Requested page size is too big")]
    PageSizeTooBig = 31,
    #[error("There are no blocks")]
    NoBlocks = 32,
    #[error("The supplied continuation token is invalid or unknown")]
    InvalidContinuationToken = 33,
    #[error("Contract error")]
    ContractError = 40,
    #[error("Invalid contract class")]
    InvalidContractClass = 50,
    #[error("Class already declared")]
    ClassAlreadyDeclared = 51,
    #[error("Invalid transaction nonce")]
    InvalidTransactionNonce = 52,
    #[error("Max fee is smaller than the minimal transaction cost (validation plus fee transfer)")]
    InsufficientMaxFee = 53,
    #[error("Account balance is smaller than the transaction's max_fee")]
    InsufficientAccountBalance = 54,
    #[error("Account validation failed")]
    ValidationFailure = 55,
    #[error("Compilation failed")]
    CompilationFailed = 56,
    #[error("Contract class size is too large")]
    ContractClassSizeIsTooLarge = 57,
    #[error("Sender address in not an account contract")]
    NonAccount = 58,
    #[error("A transaction with the same hash already exists in the mempool")]
    DuplicateTransaction = 59,
    #[error("The compiled class hash did not match the one supplied in the transaction")]
    CompiledClassHashMismatch = 60,
    #[error("The transaction version is not supported")]
    UnsupportedTransactionVersion = 61,
    #[error("The contract class version is not supported")]
    UnsupportedContractClassVersion = 62,
    #[error("An unexpected error occured")]
    UnexpectedError = 63,
    #[error("Too many storage keys requested")]
    ProofLimitExceeded = 10000,
    #[error("Too many keys provided in a filter")]
    TooManyKeysInFilter = 34,
    #[error("Failed to fetch pending transactions")]
    FailedToFetchPendingTransactions = 38,
}

impl From<StarknetApiError> for Error {
    fn from(err: StarknetApiError) -> Self {
        Error::Call(CallError::Custom(ErrorObject::owned(err as i32, err.to_string(), None::<()>)))
    }
}

#[rpc(server, namespace = "starknet")]
pub trait StarknetApi {
    // Read API

    #[method(name = "chainId")]
    async fn chain_id(&self) -> Result<String, Error>;

    #[method(name = "getNonce")]
    async fn nonce(&self, block_id: BlockId, contract_address: FieldElement)
    -> Result<Felt, Error>;

    #[method(name = "blockNumber")]
    async fn block_number(&self) -> Result<u64, Error>;

    #[method(name = "getTransactionByHash")]
    async fn transaction_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<Transaction, Error>;

    #[method(name = "getBlockTransactionCount")]
    async fn block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error>;

    #[method(name = "getClassAt")]
    async fn class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, Error>;

    #[method(name = "blockHashAndNumber")]
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error>;

    #[method(name = "getBlockWithTxHashes")]
    async fn block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error>;

    #[method(name = "getTransactionByBlockIdAndIndex")]
    async fn transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: usize,
    ) -> Result<Transaction, Error>;

    #[method(name = "getBlockWithTxs")]
    async fn block_with_txs(&self, block_id: BlockId) -> Result<MaybePendingBlockWithTxs, Error>;

    #[method(name = "getStateUpdate")]
    async fn state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error>;

    #[method(name = "getTransactionReceipt")]
    async fn transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, Error>;

    #[method(name = "getClassHashAt")]
    async fn class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<Felt, Error>;

    #[method(name = "getClass")]
    async fn class(
        &self,
        block_id: BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass, Error>;

    #[method(name = "getEvents")]
    async fn events(&self, filter: EventFilterWithPage) -> Result<EventsPage, Error>;

    #[method(name = "pendingTransactions")]
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, Error>;

    #[method(name = "estimateFee")]
    async fn estimate_fee(
        &self,
        request: Vec<BroadcastedTransaction>,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, Error>;

    #[method(name = "estimateMessageFee")]
    async fn estimate_message_fee(
        &self,
        message: MsgFromL1,
        block_id: BlockId,
    ) -> Result<FeeEstimate, Error>;

    #[method(name = "call")]
    async fn call(&self, request: FunctionCall, block_id: BlockId) -> Result<Vec<Felt>, Error>;

    #[method(name = "getStorageAt")]
    async fn storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<Felt, Error>;

    // Write API

    #[method(name = "addDeployAccountTransaction")]
    async fn add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, Error>;

    #[method(name = "addDeclareTransaction")]
    async fn add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, Error>;

    #[method(name = "addInvokeTransaction")]
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error>;
}
