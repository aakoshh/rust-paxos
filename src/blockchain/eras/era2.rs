use crate::blockchain::eras::era1::{EpochId, Ledger, SlotId, Transaction, ValidatorId};
use crate::blockchain::property;
use crate::blockchain::{
    ecdsa::{PublicKey, Signature},
    CryptoHash,
};

use super::{era1, Crossing};

#[derive(Clone)]
pub struct InputBlockHash(CryptoHash);

#[derive(Clone)]
pub struct RankingBlockHash(CryptoHash);

impl From<RankingBlockHash> for CryptoHash {
    fn from(h: RankingBlockHash) -> Self {
        h.0
    }
}

impl From<InputBlockHash> for CryptoHash {
    fn from(h: InputBlockHash) -> Self {
        h.0
    }
}

pub struct RankingBlock {
    pub parent_hash: Crossing<era1::BlockHash, RankingBlockHash>,
    pub epoch_id: EpochId,
    pub slot_id: SlotId,
    pub height: u64,
    pub input_block_hashes: Vec<InputBlockHash>,
    pub validator_id: ValidatorId,
    pub signature: Signature<ValidatorId, RankingBlock>,
}

impl<'a> property::HasHash<'a> for RankingBlock {
    type Hash = RankingBlockHash;

    fn hash(&self) -> Self::Hash {
        todo!()
    }
}

impl<'a> property::RankingBlock<'a> for RankingBlock {
    type PrevEraHash = era1::BlockHash;
    type InputBlockHash = InputBlockHash;
    fn parent_hash(&self) -> Crossing<Self::PrevEraHash, Self::Hash> {
        self.parent_hash.clone()
    }

    fn height(&self) -> u64 {
        self.height
    }

    fn input_block_hashes(&self) -> Vec<Self::InputBlockHash> {
        self.input_block_hashes.clone()
    }
}

#[derive(Clone)]
pub struct MinerId(PublicKey);

#[derive(Clone)]
pub struct InputBlockHeader {
    pub content_hash: CryptoHash,
    pub nonce: [u8; 32],
    pub miner_id: MinerId,
    pub signature: Signature<MinerId, InputBlockHeader>,
}

impl InputBlockHeader {
    /// Verify that `hash(hash_without_nonce ++ nonce)` starts with `target_difficulty` number of zeroes.
    pub fn verify_pow(&self, target_difficulty: u8) -> bool {
        todo!()
    }

    fn hash_without_nonce(&self) -> CryptoHash {
        todo!()
    }
}

impl<'a> property::HasHash<'a> for InputBlockHeader {
    type Hash = InputBlockHash;

    fn hash(&self) -> Self::Hash {
        todo!()
    }
}

pub struct InputBlock {
    pub header: InputBlockHeader,
    pub transactions: Vec<Transaction>,
}

impl<'a> property::HasTransactions<'a> for InputBlock {
    type Transaction = Transaction;

    fn fold_transactions<F, R>(&'a self, init: R, f: F) -> R
    where
        F: Fn(R, &Self::Transaction) -> R,
    {
        self.transactions.iter().fold(init, f)
    }
}

impl property::HasHeader for InputBlock {
    type Header = InputBlockHeader;

    fn header(&self) -> Self::Header {
        self.header.clone()
    }
}

pub struct Era2;

impl property::Era for Era2 {
    type RankingBlock<'a> = RankingBlock;
    type InputBlock<'a> = InputBlock;
    type Transaction<'a> = Transaction;
    type Ledger<'a> = Ledger;
}