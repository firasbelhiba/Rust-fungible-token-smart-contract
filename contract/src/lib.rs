use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    user_accounts: LookupMap<AccountId, u128>,
    total_supply: u128,
}

impl Default for FungibleToken {
    fn default() -> FungibleToken {
        let mut contract = FungibleToken {
            user_accounts: LookupMap::new(b'm'),
            total_supply: 100,
        };
        let account_id = env::signer_account_id();
        contract
            .user_accounts
            .insert(&account_id, &contract.total_supply);
        return contract;
    }
}

#[near_bindgen]
impl FungibleToken {
    pub fn get_total_supply(&self) -> u128 {
        return self.total_supply.clone();
    }

    pub fn get_balance_of(&self, account_id: AccountId) -> u128 {
        if let None = self.user_accounts.get(&account_id) {
            return 0;
        }
        return self.user_accounts.get(&account_id).unwrap();
    }

    pub fn transfer(&mut self, receiver_id: AccountId, tokens: u128) {
        let sender_id = env::signer_account_id();

        let initial_sender_amount;
        if let None = self.user_accounts.get(&sender_id) {
            initial_sender_amount = 0;
        } else {
            initial_sender_amount = self.user_accounts.get(&sender_id).unwrap();
        }

        assert!(
            initial_sender_amount >= tokens,
            "Sender does not have enough tokens."
        );
        self.user_accounts
            .insert(&sender_id, &(initial_sender_amount - tokens));

        let initial_receiver_amount;
        if let None = self.user_accounts.get(&receiver_id) {
            initial_receiver_amount = 0;
        } else {
            initial_receiver_amount = self.user_accounts.get(&receiver_id).unwrap();
        }
        self.user_accounts
            .insert(&receiver_id, &(initial_receiver_amount + tokens));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_total_supply() {
        let contract = FungibleToken::default();
        // We need to check if the supply is the default value
        assert_eq!(contract.get_total_supply(), 100);
    }

    #[test]
    fn get_balance_of() {
        let contract = FungibleToken::default();
        // The signer account id is in this case the account who depolyed the smart contract . In this case he owns 100 tokens in default
        assert_eq!(contract.get_balance_of(env::signer_account_id()), 100);

        // You can also test it with your account if you own some tokens of this smart contract
    }

    #[test]
    fn transfer() {
        let mut contract = FungibleToken::default();
        contract.transfer("firas.testnet".parse().unwrap(), 10); // don't forget to cast it from a string to an account id ( by parsing and unwrapping)
        assert_eq!(contract.get_balance_of(env::signer_account_id()), 90); // the current contract now should have only 90 tokens left ( 100 - 10)

        assert_eq!(
            contract.get_balance_of("firas.testnet".parse().unwrap()),
            10
        ) // We can also test the receiver_id
    }
}
