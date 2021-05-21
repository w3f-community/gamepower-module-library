#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct WalletClassData {
    // Metadata from ipfs
    pub metadata: Vec<u8>,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct WalletAssetData {
    //Deposit balance to create each token
    pub metadata: Vec<u8>,
}