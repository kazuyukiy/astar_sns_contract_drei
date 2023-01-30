use crate::astar_sns_contract_drei::AstarSnsContractDrei;
use ink_env::AccountId;

impl AstarSnsContractDrei {
    pub fn balance_of_fn(&self, account_id: AccountId) -> u128 {
        let asset = self.asset_mapping.get(&account_id).unwrap_or(0 as u128);
        asset
    }

    pub fn transfer_fn(&mut self, from_id: AccountId, to_id: AccountId, amount: u128) {
        let mut to_asset = self.asset_mapping.get(&to_id).unwrap_or(0 as u128);
        let mut from_asset = self.asset_mapping.get(&from_id).unwrap_or(0 as u128);
        if from_asset < amount {
            return;
        }
        to_asset = to_asset + amount;
        from_asset = from_asset - amount;
        self.asset_mapping.insert(&to_id, &to_asset);
        self.asset_mapping.insert(&from_id, &from_asset);
    }

    pub fn distribute_fn(&mut self, to_id: AccountId, amount: u128) {
        let mut to_asset = self.asset_mapping.get(&to_id).unwrap_or(0 as u128);
        to_asset = to_asset + amount;
        self.asset_mapping.insert(&to_id, &to_asset);
    }
}
