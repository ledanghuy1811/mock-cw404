use std::vec;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw20::AllowanceResponse;
use cw721::Approval;
use cw_storage_plus::{Index, IndexList, Item, Map, MultiIndex, IndexedMap, Deque};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub admin: Addr,
    pub units: Uint128,
    pub base_token_uri: Option<String>
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
// marketing info, logo temporary don't consider
// cw20 info
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");

// cw721 info
pub const MAX_NFT_SUPPLY: Item<Uint128> = Item::new("max_nft_supply");
pub const NFT_COUNT: Item<u64> = Item::new("nft_count");
// Stored as (granter, operator) giving operator full control over granter's account
pub const OPERATORS: Map<(&Addr, &Addr), Expiration> = Map::new("operator");
const INDEXES: NftIndexes = NftIndexes {
    owner: MultiIndex::new(nft_owner_idx, "token", "token_owner")
};
pub const NFT_TOKENS: IndexedMap<&str, NftInfo, NftIndexes> = IndexedMap::new("token", INDEXES);

// nft queue using deque
pub const DEQUE_NFT: Deque<Uint128> = Deque::new("deque_nft");
pub const CW721_TRANSFER_EXEMPT: Map<&Addr, bool> = Map::new("cw721_traansfer_exempt");

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NftInfo {
    // The owner of newly minted Nft
    pub owner: Addr,
    // Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,
    // Universal resource identifier for this NFT
    // Should point to a JSON file that conforms to the ERC721
    // Metadata JSON Schema
    pub token_uri: Option<String>,
}

pub struct NftIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, NftInfo, String>,
}

impl<'a> IndexList<NftInfo> for NftIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo>> + '_> {
        let v: Vec<&dyn Index<NftInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn nft_owner_idx(_pk: &[u8], d: &NftInfo) -> Addr {
    d.owner.clone()
}