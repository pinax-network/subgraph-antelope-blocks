use substreams_antelope::pb::{ActionTrace, PermissionLevel, TransactionTrace};
use substreams_entity_change::tables::Tables;

use crate::{keys::{action_key, authorization_key}, utils::is_match};

// https://github.com/pinax-network/firehose-antelope/blob/534ca5bf2aeda67e8ef07a1af8fc8e0fe46473ee/proto/sf/antelope/type/v1/type.proto#L616
pub fn insert_authorization(params: &String, tables: &mut Tables, action: &ActionTrace, transaction: &TransactionTrace, authorization: &PermissionLevel) {
    // transaction
    let tx_hash = &transaction.id;

    // action
    let action_index = &action.execution_index;

    // authorization
    let actor = &authorization.actor;
    let permission = &authorization.permission;

    // TABLE::PermissionLevel
    let action_key = action_key(tx_hash, action_index);
    let key = authorization_key(&action_key, actor, permission);
    if is_match(Vec::from(["table:Authorization"]), params) {
        tables
            .create_row("Authorization", key)
            // pointers
            .set("transaction", tx_hash)
            .set("action", action_key)

            // authorization
            .set("actor", actor.to_string())
            .set("permission", permission.to_string())
        ;
    }
}