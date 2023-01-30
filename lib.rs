#![cfg_attr(not(feature = "std"), no_std)]

mod FT;
mod follow;
mod message;
mod metadata;
mod post;
mod profile;

use ink_lang as ink;

#[ink::contract]
mod astar_sns_contract_drei {

    use ink_env::debug_println;
    use ink_lang::codegen::Env;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use openbrush::storage::Mapping;
    use openbrush::test_utils::*;

    pub use crate::follow::*;
    pub use crate::message::*;
    pub use crate::metadata::*;
    pub use crate::post::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AstarSnsContractDrei {
        pub profile_map: Mapping<AccountId, Profile>,
        pub post_map: Mapping<u128, Post>,
        pub post_map_counter: u128,
        pub message_list_map: Mapping<u128, Vec<Message>>,
        pub message_list_map_counter: u128,
        pub asset_mapping: Mapping<AccountId, u128>,
    }

    impl AstarSnsContractDrei {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                profile_map: Mapping::default(),
                post_map: Mapping::default(),
                post_map_counter: 0,
                message_list_map: Mapping::default(),
                message_list_map_counter: 0,
                asset_mapping: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn release_post(
            &mut self,
            description: String,
            created_time: String,
            post_img_url: String,
        ) {
            let caller: AccountId = self.env().caller();
            self.release_post_fn(caller, description, created_time, post_img_url);
        }

        #[ink(message)]
        pub fn get_general_post(&self, num: u128) -> Vec<Post> {
            let general_post_list = self.get_general_post_fn(num);
            general_post_list
        }

        #[ink(message)]
        pub fn get_individual_post(&self, num: u128, account_id: AccountId) -> Vec<Post> {
            let individual_post_list = self.get_individual_post_fn(num, account_id);
            individual_post_list
        }

        #[ink(message)]
        pub fn add_likes(&mut self, post_id: u128) {
            self.add_likes_fn(post_id);
        }

        #[ink(message)]
        pub fn send_message(
            &mut self,
            message: String,
            message_list_id: u128,
            created_time: String,
        ) {
            let caller: AccountId = self.env().caller();
            self.send_message_fn(message, message_list_id, caller, created_time);
        }

        #[ink(message)]
        pub fn get_message_list(&self, message_list_id: u128, num: u128) -> Vec<Message> {
            let message_list: Vec<Message> =
                self.get_message_list_fn(message_list_id, num as usize);
            message_list
        }

        #[ink(message)]
        pub fn get_last_message(&self, message_list_id: u128) -> Message {
            let message: Message = self.get_last_message_fn(message_list_id);
            message
        }

        #[ink(message)]
        pub fn create_profile(&mut self) {
            let caller: AccountId = self.env().caller();
            self.create_profile_fn(caller);
        }

        #[ink(message)]
        pub fn set_profile_info(&mut self, name: String, img_url: String) {
            let caller: AccountId = self.env().caller();
            self.set_profile_info_fn(caller, name, img_url);
        }

        #[ink(message)]
        pub fn follow(&mut self, followed_account_id: AccountId) {
            let caller: AccountId = self.env().caller();
            self.follow_fn(caller, followed_account_id);
        }

        #[ink(message)]
        pub fn get_following_list(&self, account_id: AccountId) -> Vec<AccountId> {
            let following_list: Vec<AccountId> = self.get_following_list_fn(account_id);
            following_list
        }

        #[ink(message)]
        pub fn get_follower_list(&self, account_id: AccountId) -> Vec<AccountId> {
            let follower_list: Vec<AccountId> = self.get_follower_list_fn(account_id);
            follower_list
        }

        #[ink(message)]
        pub fn get_profile_info(&self, account_id: AccountId) -> Profile {
            let profile: Profile = self.get_profile_info_fn(account_id);
            profile
        }

        #[ink(message)]
        pub fn check_created_info(&self, account_id: AccountId) -> bool {
            let is_already_connected: bool = self.check_created_profile_fn(account_id);
            is_already_connected
        }

        #[ink(message)]
        pub fn get_total_likes(&self, account_id: AccountId) -> u128 {
            let num_of_likes = self.get_total_likes_fn(account_id);
            num_of_likes
        }

        #[ink(message)]
        pub fn balance_of(&self, account_id: AccountId) -> u128 {
            let asset = self.balance_of_fn(account_id);
            asset
        }

        #[ink(message)]
        pub fn transfer(&mut self, to_id: AccountId, amount: u128) {
            let caller: AccountId = self.env().caller();
            self.transfer_fn(caller, to_id, amount);
        }

        #[ink(message)]
        pub fn distrubute_refer_likes(&mut self) {
            let caller: AccountId = self.env().caller();
            let total_likes = self.get_total_likes_fn(caller);
            let asset = self.balance_of_fn(caller);
            let calculated_amount = total_likes * 20;
            if asset < calculated_amount {
                self.distribute_fn(caller, calculated_amount - asset);
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        use ink_env::debug_println;

        use ink_lang as ink;

        #[ink::test]
        fn test_profile_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            let alice_account_id = accounts().alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);

            astar_sns_contract.create_profile();

            astar_sns_contract
                .set_profile_info("Tony Stark".to_string(), "https//ff...".to_string());

            debug_println!(
                "profile_list: {:?}",
                astar_sns_contract
                    .profile_map
                    .get(&alice_account_id)
                    .unwrap()
            );
        }

        //
        #[ink::test]
        fn test_post_release_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            let alice_account_id = accounts().alice;

            // profile, creating
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();

            // profile, data set
            astar_sns_contract.set_profile_info("Alice".to_string(), "https//ff...".to_string());

            // post
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );

            // post info
            let post_info: Post = astar_sns_contract
                .post_map
                .get(&(astar_sns_contract.post_map_counter - 1))
                .unwrap();
            debug_println!("post: {:?}\n", post_info);
        }

        //
        #[ink::test]
        fn test_general_post_get_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            // accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // profiles alice/bob/charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.create_profile();

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.create_profile();

            // set profile_info - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.set_profile_info("Alice".to_string(), "https//ff...".to_string());

            // posts alice
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );
            astar_sns_contract.release_post(
                "I come to Thailand".to_string(),
                "12:35".to_string(),
                "https://gsdef...".to_string(),
            );
            astar_sns_contract.release_post(
                "Hello YouTube".to_string(),
                "12:40".to_string(),
                "https://fafds...".to_string(),
            );
            astar_sns_contract.release_post(
                "Oh baby, come on!".to_string(),
                "12:45".to_string(),
                "https://fsdfee...".to_string(),
            );
            astar_sns_contract.release_post(
                "Don't mention it!".to_string(),
                "12:50".to_string(),
                "https://fase...".to_string(),
            );
            astar_sns_contract.release_post(
                "Do what you want".to_string(),
                "12:55".to_string(),
                "https://fasdfgeg...".to_string(),
            );

            // print post list
            let post_list: Vec<Post> = astar_sns_contract.get_general_post(1);
            debug_println!("General post get test\n",);
            for n in 0..(post_list.len()) {
                debug_println!("{:?}\n", post_list[n]);
            }
        }

        //
        #[ink::test]
        fn test_individual_post_get_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            // accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // create profiles alice/bob/charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.create_profile();

            // set profile_info - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.set_profile_info("Alice".to_string(), "https//ff...".to_string());
            // set profile_info - bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.set_profile_info("Bob".to_string(), "https//ff...".to_string());
            // set profile_info - charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.set_profile_info("Charlie".to_string(), "https//ff...".to_string());

            // post - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );
            astar_sns_contract.release_post(
                "Oh baby, come on!".to_string(),
                "12:45".to_string(),
                "https://fsdfee...".to_string(),
            );

            // post - bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.release_post(
                "I come to Thailand".to_string(),
                "12:35".to_string(),
                "https://gsdef...".to_string(),
            );
            astar_sns_contract.release_post(
                "Don't mention it!".to_string(),
                "12:50".to_string(),
                "https://fasee...".to_string(),
            );

            // post - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.release_post(
                "Hello YouTube".to_string(),
                "12:40".to_string(),
                "https://fafds...".to_string(),
            );
            astar_sns_contract.release_post(
                "Do whta you want".to_string(),
                "12:55".to_string(),
                "https://fasdfgeg...".to_string(),
            );

            let alice_post_list: Vec<Post> =
                astar_sns_contract.get_individual_post(1, alice_account_id);
            let bob_post_list: Vec<Post> =
                astar_sns_contract.get_individual_post(1, bob_account_id);
            let charlie_post_list: Vec<Post> =
                astar_sns_contract.get_individual_post(1, charlie_account_id);
            debug_println!("Individual post get test");
            for n in 0..(alice_post_list.len()) {
                debug_println!("{:?}", alice_post_list[n]);
            }
            for n in 0..(bob_post_list.len()) {
                debug_println!("{:?}", bob_post_list[n]);
            }
            for n in 0..(charlie_post_list.len()) {
                debug_println!("{:?}", charlie_post_list[n]);
            }
        }

        #[ink::test]
        fn test_add_likes_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            let alice_account_id = accounts().alice;
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();

            astar_sns_contract.set_profile_info("Alice".to_string(), "https//ff...".to_string());

            // post,  same psot third times
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );
            astar_sns_contract.release_post(
                "Today, it was so rainy!".to_string(),
                "12:30".to_string(),
                "https://sdfds...".to_string(),
            );

            // like
            astar_sns_contract.add_likes(0);
            debug_println!(
                "Number of likes: {}",
                astar_sns_contract.post_map.get(&0).unwrap().num_of_likes
            );
            astar_sns_contract.add_likes(0);
            astar_sns_contract.add_likes(1);
            astar_sns_contract.add_likes(2);
            debug_println!(
                "Number of likes: {}",
                astar_sns_contract.post_map.get(&0).unwrap().num_of_likes
            );

            //
            let total_likes = astar_sns_contract.get_total_likes(alice_account_id);
            debug_println!("alice total num of likes: {}", total_likes)
        }

        #[ink::test]
        fn test_follow_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            // accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // create profiles alice/bob/charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.create_profile();

            // cunter at this point
            debug_println!(
                "message_list_map_counter: {}",
                astar_sns_contract.message_list_map_counter
            );

            // following
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.follow(bob_account_id);
            debug_println!(
                "following_list: {:?}\nfollower_list: {:?}",
                astar_sns_contract.get_following_list(alice_account_id),
                astar_sns_contract.get_follower_list(bob_account_id)
            );

            // message_list counter
            debug_println!(
                "message_list_map_id: {}",
                astar_sns_contract.message_list_map_counter
            );

            // following (bob to alice)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.follow(alice_account_id);
            debug_println!(
                "message_list_map_counter: {}",
                astar_sns_contract.message_list_map_counter
            );

            // following (alice to bob)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.follow(bob_account_id);
            // the counter shoul not be upped
            debug_println!(
                "message_list_map_counter: {}",
                astar_sns_contract.message_list_map_counter
            );

            // following (alice to charlie)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.follow(charlie_account_id);
            // the counter shoul not be upped
            debug_println!(
                "following_list: {:?}\nfollower_list: {:?}",
                astar_sns_contract.get_following_list(alice_account_id),
                astar_sns_contract.get_follower_list(charlie_account_id)
            );

            // updated
            debug_println!(
                "message_list_map_counter: {}",
                astar_sns_contract.message_list_map_counter
            );
        }

        //
        #[ink::test]
        fn test_message_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            // accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // create profiles alice/bob/charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.create_profile();

            // set profile_info - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.set_profile_info("Alice".to_string(), "https//ff...".to_string());
            // set profile_info - bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.set_profile_info("Bob".to_string(), "https//ff...".to_string());
            // set profile_info - charlie
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.set_profile_info("Charlie".to_string(), "https//ff...".to_string());

            // message send - alice
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.send_message(
                "Sorry Bro, I can't go today.".to_string(),
                0,
                "12:30".to_string(),
            );
            astar_sns_contract.send_message(
                "Why don't we go there tomorrow?".to_string(),
                0,
                "12:33".to_string(),
            );
            astar_sns_contract.send_message(
                "Hey, charlie will come".to_string(),
                0,
                "12:35".to_string(),
            );
            astar_sns_contract.send_message(
                "He seems so muscular, so he would teach us.".to_string(),
                0,
                "12:36".to_string(),
            );
            astar_sns_contract.send_message(
                "Why don't we go there tommorow?".to_string(),
                0,
                "12:37".to_string(),
            );

            // message send - bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            astar_sns_contract.send_message(
                "I'm so looking forward that!".to_string(),
                0,
                "12:38".to_string(),
            );
            astar_sns_contract.send_message(
                "Hey bro! Tomorrow I and Bob go to gym. Don't you join up?".to_string(),
                1,
                "12:34".to_string(),
            );

            debug_println!(
                "message_list_alice_bob: {:?}\n",
                astar_sns_contract.get_message_list(0, 1)
            );
            debug_println!(
                "last_message_alice_bob: {:?}\n",
                astar_sns_contract.get_last_message(0)
            );
            debug_println!(
                "message_list_alice_charlie: {:?}\n",
                astar_sns_contract.get_message_list(1, 1)
            );
        }

        //
        #[ink::test]
        fn FT_fn_works() {
            let mut astar_sns_contract = AstarSnsContractDrei::new();

            // accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // send tokens
            astar_sns_contract.distribute_fn(alice_account_id, 100);
            astar_sns_contract.distribute_fn(bob_account_id, 100);
            astar_sns_contract.distribute_fn(charlie_account_id, 100);

            // send tokens from alice to bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            astar_sns_contract.transfer(bob_account_id, 50);
            // send tokens from alice to bob
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            astar_sns_contract.transfer(bob_account_id, 50);

            // balance
            let alice_asset = astar_sns_contract.balance_of(alice_account_id);
            let bob_asset = astar_sns_contract.balance_of(bob_account_id);
            let charlie_asset = astar_sns_contract.balance_of(charlie_account_id);
            debug_println!("alice_asset:{} ", alice_asset);
            debug_println!("bob_asset:{} ", bob_asset);
            debug_println!("charlie_asset:{} ", charlie_asset);
        }
    }
}
