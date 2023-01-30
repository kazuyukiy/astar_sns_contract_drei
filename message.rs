use crate::metadata::*;
use ink_env::AccountId;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

use crate::astar_sns_contract_drei::AstarSnsContractDrei;

impl AstarSnsContractDrei {
    pub fn send_message_fn(
        &mut self,
        message: String,
        message_list_id: u128,
        sender_id: AccountId,
        created_time: String,
    ) {
        let mut message_list: Vec<Message> = self
            .message_list_map
            .get(&message_list_id)
            .unwrap_or(Vec::default());

        message_list.push(Message {
            message,
            sender_id,
            created_time,
        });

        self.message_list_map
            .insert(&message_list_id, &message_list);
    }

    pub fn get_message_list_fn(&self, message_list_id: u128, num: usize) -> Vec<Message> {
        let self_message_list: Vec<Message> = self.message_list_map.get(&message_list_id).unwrap();

        let mut message_list: Vec<Message> = Vec::new();

        let amount_index: usize = 5;

        let list_length: usize = self_message_list.len();

        if list_length < amount_index + 1 {
            for m in 0..(list_length) {
                message_list.push(self_message_list[m].clone());
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                message_list.push(self_message_list[n].clone());
            }
        }
        message_list
    }

    pub fn get_last_message_fn(&self, message_list_id: u128) -> Message {
        let last_message: Message = self
            .message_list_map
            .get(&message_list_id)
            .unwrap()
            .last()
            .unwrap()
            .clone();
        last_message
    }
}
