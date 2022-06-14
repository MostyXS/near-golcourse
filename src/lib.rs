use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, BorshStorageKey, PanicOnDefault, BlockHeight, AccountId,
};

near_sdk::setup_alloc!();



const WIDTH: usize = 64;
const HEIGHT: usize = 64;

const FIELD_LEN: usize = (WIDTH/8) * HEIGHT;
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Boards, 
    Accounts,
    AccountBoards {account_id: AccountId}
}
pub struct Account {

}

pub struct Board {
    pub field: Vec<u8>
}

impl Board {
    pub fn new() -> Self {
        Self { field: vec![0u8; FIELD_LEN] }
    }
    
    pub fn from(field: Vec<u8>) -> Self {
        assert_eq!(field.len(), FIELD_LEN);
        Self { 
            field
         }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub boards: Vector<Board>,
    pub accounts: UnorderedMap<AccountId, Account>,
}

pub struct BoardWithBlock {
    pub board: Board,
    pub current_block_height: BlockHeight,
    pub prev_block_height: BlockHeight,
}

impl BoardWithBlock {
    pub fn new(board: Board) -> Self {
        Self { board,
             current_block_height: env::block_index(),
             prev_block_height: 0 
            }
    }
}


pub type BoardIndex = u32

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            boards: Vector::new(StorageKey::Boards),
            accounts: UnorderedMap::new(StorageKey::Accounts),

        }
    }

    pub fn create_board(&self, field: Base64VecU8) -> BoardIndex {
        let board = Board::from(field)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .is_view(is_view)
            .build()
    }

    #[test]
    fn test_new() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = Contract::new();
    }
}
