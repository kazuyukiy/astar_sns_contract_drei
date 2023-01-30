use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::string::ToString;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract_drei::AstarSnsContractDrei;

impl AstarSnsContractDrei {
    pub fn release_post_fn(
        &mut self,
        account_id: AccountId,
        description: String,
        created_time: String,
        post_img_url: String,
    ) {
        // profile related with wallet address
        let profile_info: Profile = self.profile_map.get(&account_id).unwrap();

        // add post data related with post id
        self.post_map.insert(
            &(self.post_map_counter),
            &Post {
                name: profile_info.name.unwrap_or("unknown".to_string()),
                user_id: profile_info.user_id,
                created_time,
                img_url: post_img_url,
                user_img_url: profile_info.img_url.unwrap(),
                description: description,
                num_of_likes: 0,
                post_id: self.post_map_counter,
            },
        );

        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();

        // add the post id
        profile.post_id_list.push(self.post_map_counter);

        // overwrite profile
        self.profile_map.insert(&account_id, &profile);

        // count up post id
        self.post_map_counter = &self.post_map_counter + 1;
    }

    // get the last post
    pub fn get_general_post_fn(&self, num: u128) -> Vec<Post> {
        // post list to be returned
        let mut post_list: Vec<Post> = Vec::new();

        let length: u128 = self.post_map_counter;

        let amount_index: u128 = 5;

        // distinguish how to get up to amount of the posts
        if length < amount_index + 1 {
            for m in 0..(length + 1) {
                let post: Option<Post> = self.post_map.get(&m);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                let post: Option<Post> = self.post_map.get(&(length - n - 1));
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        post_list
    }

    pub fn get_individual_post_fn(&self, num: u128, account_id: AccountId) -> Vec<Post> {
        // list to be returned;
        let mut post_list: Vec<Post> = Vec::new();

        let post_id_list: Vec<u128> = self.profile_map.get(&account_id).unwrap().post_id_list;

        let amount_index: u128 = 5;

        let length: u128 = post_id_list.len() as u128;

        if length < amount_index + 1 {
            for m in 0..(length) {
                let post: Option<Post> = self.post_map.get(&post_id_list[m as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1)..(amount_index * num)) {
                let post: Option<Post> =
                    self.post_map.get(&post_id_list[(length - n - 1) as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }

        post_list
    }

    // add iine
    pub fn add_likes_fn(&mut self, post_id: u128) {
        let mut post: Post = self.post_map.get(&post_id).unwrap();

        post.num_of_likes = &post.num_of_likes + 1;

        self.post_map.insert(&post_id, &post);
    }

    pub fn get_total_likes_fn(&self, account_id: AccountId) -> u128 {
        let post_id_list = self.profile_map.get(&account_id).unwrap().post_id_list;

        let mut num_of_likes = 0;

        for id in post_id_list {
            let likes_of_post = self.post_map.get(&id).unwrap().num_of_likes;
            num_of_likes = num_of_likes + likes_of_post;
        }

        num_of_likes
    }
}
