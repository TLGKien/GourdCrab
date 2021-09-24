Near Cetificate Devoloper - Demo
================================

Vietnamese gambling game: 
Gourd-Fish-Shrimp-Crab On NEAR

## How to play

Instead of showing one to six pips, the sides of the dice have pictures of a fish, a prawn, a crab, a rooster, a calabash gourd,and a stag.We have 3 dice.
Players place wagers on a board that has the six pictures
The bettor will get their money back plus the corresponding amount of profit if the bet is correct
If two (or three) dice correspond with a bet, the bettor receives tow (or three) times their money
For instance, if one were to place $3 on fish, and the dealer rolls 1 fish, 1 crab and 1 stag, then the bettor would receive $3 while keeping the $3 they had bet


About Contract
====================

It's need to be mentioned that it is a pure dapp project, which means there is no centralized backend nor data server, all persistent information is stored and mananged on NEAR chain by a contract.

## Contract Structure

```rust
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
```

## Contract Interface


```rust
//****************/
//***** INIT *****/
//****************/

/// initialization of this contract
#[init]
    pub fn new(
        owner_id: AccountId,
        bet_level: U128, 
    ) -> Self;


//***************************/
//***** OWNER FUNCTIONS *****/
//***************************/

pub fn set_bet_level(&mut self, bet_level: U128);

pub fn deposit(&mut self); 

pub fn betting(
    &mut self, 
    num_fish: u128, 
    num_prawn: u128,
    num_crab: u128,
    num_rooster: u128, 
    num_gourd: u128,
    num_stag: u128);

pub fn cancel_betting(&mut self);

// roll the dice and calculate the payoff
pub fn rolling(&mut self);

// get account's balance
pub fn get_balance(&self, account_id: AccountId) -> u128;

```

Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

The "contract" code lives in the `/src` folder. See the README there for more info.

Preparation
===========

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Install near-cli (optional)
---------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

How to run
==========

After you clone the project, go to the project folder

Step 1: Login
-------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `NCD-GroupA-Demo.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `NCD-GroupA-Demo.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      `near login`

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      `near create-account NCD-GroupA-Demo.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet`

3. Export account to variable
      `export ACCOUNT_ID=<YOUR_ACCOUNT_ID>`

Step 2: build and testing
---------------------------------

    ./build.sh
    ./test.sh


Step 3: deploy!
---------------

    ./deploy.sh

Step 4: Call contract from Near CLI

    ./call.sh <METHOD_NAME> <ARGs>

Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [Vue]: https://vuejs.org/
  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
