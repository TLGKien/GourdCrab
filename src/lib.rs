use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
// use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen,AccountId, Balance,
    collections::{ UnorderedMap },
    json_types::{ U128},
};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// information about bets allocation
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BetInfo {
    pub fish: u128, 
    pub prawn: u128, 
    pub crab: u128, 
    pub rooster: u128, 
    pub gourd: u128, 
    pub stag: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GourdCrab {
    pub owner_id: AccountId,
    pub accounts: UnorderedMap<AccountId, Balance>, // balance of user
    pub bets: UnorderedMap<AccountId,BetInfo>,      // bets in the room
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
            bets:UnorderedMap::new(b"b".to_vec()),
            bet_level: bet_level.into(),
        }
    }

    pub fn set_bet_level(&mut self, bet_level: U128){
        self.bet_level = bet_level.into();
    }

    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        let mut balance = self.accounts.get(&account_id).unwrap_or(0);
        balance = balance + deposit;
        self.accounts.insert(&account_id, &balance);
    }
    
    pub fn betting(
        &mut self, 
        num_fish: u128, 
        num_prawn: u128, 
        num_crab: u128, 
        num_rooster: u128, 
        num_gourd: u128, 
        num_stag: u128
    ){
        let account_id = env::signer_account_id();
        let mut balance = self.accounts.get(&account_id).unwrap_or(0);
        let total_bet = num_fish + num_prawn + num_crab + num_rooster + num_gourd + num_stag;
        assert!(balance > total_bet * self.bet_level, "Insufficient balance to bet");
        balance = balance - total_bet * self.bet_level;
        self.accounts.insert(&account_id, &balance);

        // thêm bet vào vector
        let bet = BetInfo{
            fish: num_fish, 
            prawn: num_prawn, 
            crab: num_crab, 
            rooster: num_rooster, 
            gourd: num_gourd, 
            stag: num_stag,
        };
        self.bets.insert(&account_id,&bet);
    }

    pub fn cancel_betting(&mut self){
        let account_id = env::signer_account_id();
        let mut balance = self.accounts.get(&account_id).unwrap_or(0);
        let _tmp = self.bets.get(&account_id).unwrap();
        
        balance = balance + (_tmp.fish + _tmp.prawn + _tmp.crab + _tmp.rooster + _tmp.gourd + _tmp.stag) * self.bet_level;
        self.accounts.insert(&account_id,&balance);
        self.bets.remove(&account_id);
    }

    pub fn rolling(&mut self)->[u128;6]{
        // rolling
        let _rand1: u8 = *env::random_seed().get(0).unwrap() % 6;
        let _rand2: u8 = *env::random_seed().get(1).unwrap() % 6;
        let _rand3: u8 = *env::random_seed().get(2).unwrap() % 6;

        let mut result: [u128;6] = [0,0,0,0,0,0];
        result[_rand1 as usize] += 1;
        result[_rand2 as usize] += 1;
        result[_rand3 as usize] += 1;

        
        // check the result
        for account_id in self.bets.keys() {
            let mut balance = self.accounts.get(&account_id).unwrap_or(0);
            let _tmp = self.bets.get(&account_id).unwrap();
            // let profit :u128 = result[0] as u128 * _tmp.fish + result[1] as u128 *_tmp.prawn + result[2] as u128 * _tmp.crab + result[3] as u128 * _tmp.rooster + result[4] as u128 * _tmp.gourd + result[5] as u128 * _tmp.stag;

            let mut profit : u128 = 0;
            if result[0] * _tmp.fish != 0{
                profit += result[0] * _tmp.fish;
                profit += 1;
            }
            if result[1] *_tmp.prawn != 0{
                profit += result[1] *_tmp.prawn;
                profit += 1;
            }
            if result[2] * _tmp.crab != 0{
                profit += result[2] * _tmp.crab;
                profit += 1;
            }
            if result[3] * _tmp.rooster != 0{
                profit += result[3] * _tmp.rooster;
                profit += 1;
            }
            if result[4] * _tmp.gourd != 0{
                profit += result[4] * _tmp.gourd;
                profit += 1;
            }
            if result[5] * _tmp.stag != 0{
                profit += result[5] * _tmp.stag;
                profit += 1;
            }
            
            balance = balance + profit * self.bet_level;

            self.accounts.insert(&account_id, &balance);            
        }

        self.bets.clear();
        result
    }
    
    pub fn get_balance(&self, account_id: AccountId) -> u128 {
        self.accounts.get(&account_id).unwrap_or(0)
    }

    pub fn get_bet(&self, account_id: AccountId) -> [u128;6] {
        let bet = self.bets.get(&account_id).unwrap();
        let betarr : [u128;6] = [bet.fish,bet.prawn,bet.crab, bet.rooster, bet.gourd, bet.stag];
        betarr
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};
    mod test_utils;
    use test_utils::*;
  
    fn account_casino() -> String {
        "casino".to_string()
    }

    fn accunt_bob() -> String {
        "bob".to_string()
    }

    fn accunt_alice() -> String {
        "alice".to_string()
    }

    const BET_LEVEL:u128 = 1_000_000_000_000_000_000_000_000;

    fn start_machine() -> GourdCrab {
        // initialize contract and deposit jackpod with 100 NEAR
        let context = VMContextBuilder::new()
            .current_account_id(account_casino())
            .predecessor_account_id(account_casino())
            .signer_account_id(account_casino())
            .attached_deposit(BET_LEVEL * 100)
            .finish();
        testing_env!(context.clone());
        let mut contract = GourdCrab::new(account_casino(), U128(BET_LEVEL));
        contract
    }

    #[test]
    fn test_rolling_successfully() {
        let mut contract = start_machine();

        

        let context = VMContextBuilder::new()
            .predecessor_account_id(accunt_bob())
            .signer_account_id(accunt_bob())
            .attached_deposit(10*BET_LEVEL)
            .finish();

        testing_env!(context.clone());    

        contract.deposit();
        assert_eq!(contract.get_balance(accunt_bob()),BET_LEVEL*10);

        contract.betting(2,0,0,0,0,0);
        contract.rolling();
        
    }
}