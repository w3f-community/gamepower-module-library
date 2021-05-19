// This file is part of GamePower Network.

// Copyright (C) 2021 GamePower Network.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_support::{
  decl_module, decl_storage,
  traits::{Currency, Get, ReservableCurrency},
  Parameter,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{
  traits::{AtLeast32BitUnsigned, Member},
    DispatchResult, ModuleId, RuntimeDebug,
};
use sp_std::vec::Vec;
use orml_nft::Pallet as NftModule;
use gamepower_traits::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type Token = u64;
type Class = u128;
type Price = u128;

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Listing<AccountId> {
    pub asset: (Class, Token),
    pub seller: AccountId,
    pub price: Price,
}

#[derive(Encode, Decode, Default, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Order<AccountId> {
    pub listing: Listing<AccountId>,
    pub buyer: AccountId,
}

/// The module configuration trait.
pub trait Config: system::Config + orml_nft::Config {
    /// The class ID type
    type ClassId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
    /// The token ID type
    type TokenId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy;
    /// Allow assets to be transferred through the wallet
    type AllowTransfer: Get<bool>;
    /// Allow assets to be burned from the wallet
    type AllowBurn: Get<bool>;
    /// Allow assets to be listed on the market
    type AllowMarketListing: Get<bool>;
    /// Allow asset claiming
    type AllowClaim: Get<bool>;
    /// Currency type for reserve/unreserve balance
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    /// Wallet Module Id
    type ModuleId: Get<ModuleId>;
}

decl_storage! {
  trait Store for Module<T: Config> as GamePowerWallet {

      pub ListingByOwner get(fn get_listing_by_owner): map hasher(blake2_128_concat) T::AccountId => Listing<T::AccountId>;
      pub OrderByOwner get(fn get_order_by_owner): map hasher(blake2_128_concat) T::AccountId => Order<T::AccountId>;
      pub AllListings get(fn all_listings_count): u64;
      pub AllOrders get(fn all_orders_count): u64;
      pub NextListingId get(fn next_listing_id): u64;
      pub NextOrderId get(fn next_order_id): u64;
  }
}


decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

      const AllowTransfer: bool = T::AllowTransfer::get();
      const AllowBurn: bool = T::AllowBurn::get();
      const AllowMarketListing: bool = T::AllowMarketListing::get();
      const AllowClaim: bool = T::AllowClaim::get();

      #[weight = 10_000]
      pub fn transfer(origin, asset:(<T as orml_nft::Config>::ClassId, <T as orml_nft::Config>::TokenId), to: T::AccountId) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          NftModule::<T>::transfer(&sender, &to, asset)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn burn(origin, token_id:Token) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn list(origin, token_id:Token, price:Price) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn buy(origin, token_id:Token, price:Price) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn emote(origin, token_id:Token, emote: Vec<u8>) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

      #[weight = 10_000]
      pub fn claim(origin, token_id:Token) -> DispatchResult{

          let sender = ensure_signed(origin)?;

          Ok(())
      }

    }
}


impl<T: Config> Module<T> {
    
}