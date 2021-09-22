use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen,AccountId, Balance, PublicKey, Promise,
    collections::{ Vector, UnorderedMap },
    json_types::{ U128, Base58PublicKey },
};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ONE_NEAR:u128 = 1_000_000_000_000_000_000_000_000;
const PROB:u8 = 128;


// information about bets allocation
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BetInfo {
    pub user: AccountId,  
    pub bet: UnorderedMap<String,u8>,
}

impl BetInfo{
    #[init]
    pub fn new(
        num_fish: u8, 
        num_prawn: u8, 
        num_crab: u8, 
        num_rooster: u8, 
        num_gourd: u8, 
        num_stag: u8,
    ) -> Self{

    }
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GourdCrab {
    pub owner_id: AccountId,
    pub accounts: UnorderedMap<AccountId, Balance>, // balance of user
    pub bets: Vector<BetInfo>,                      // bets in the room
    pub bet_level: Balance, 
}

impl Default for GourdCrab {
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl GourdCrab {
    #[init]
    pub fn new(
        owner_id: AccountId,
        bet_level: U128, 
    ) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            accounts:UnorderedMap::new(b"a".to_vec()),
            bets:Vector::new(b"b".to_vec()),
            bet_level: bet_level.into(),
        }
    }


    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        let mut balance = self.accounts.get(&account_id).unwrap_or(0);
        balance = balance + deposit;
        self.accounts.insert(&account_id, &balance);
    }

    pub fn betting(&mut self, num_fish: u8, num_prawn: u8, num_crab: u8, num_rooster: u8, num_gourd: u8, num_stag: u8){
        let account_id = env::signer_account_id();
        let mut balance = self.accounts.get(&account_id).unwrap_or(0);
        let total_bet = (num_fish + num_prawn + num_crab + num_rooster + num_gourd + num_stag);
        assert!(balance > total_bet * ONE_NEAR, "Insufficient balance to bet");
        balance = balance - total_bet * ONE_NEAR;




        self.bet.push(&bet);
    }

    pub fn play(){

    }


    
    pub fn get_balance(&self, account_id: AccountId) -> U128 {
        self.accounts.get(&account_id).unwrap_or(0).into()
    }
}






// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

  
//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "alice.testnet".to_string(),
//             signer_account_id: "robert.testnet".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "jane.testnet".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 0,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }


//     #[test]
//     fn increment() {

//     }
// }