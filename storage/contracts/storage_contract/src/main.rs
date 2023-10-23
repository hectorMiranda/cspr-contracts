#![no_main]
#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, format, string::String, vec};

use casper_contract::{
    contract_api::{
        runtime::{self},
        storage::{dictionary_put, new_contract, new_dictionary},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPoints, Parameter};

#[no_mangle]
pub extern "C" fn store_batch() {
    let data_type: String = runtime::get_named_arg("data_type");
    let batch_id_data: BTreeMap<String, String> = runtime::get_named_arg("data_batch");
    let dictionary_uref = match runtime::get_key(&data_type) {
        None => new_dictionary(&data_type).unwrap_or_revert(),
        Some(dict_uref) => *dict_uref.as_uref().unwrap_or_revert(),
    };
    for (key, value) in batch_id_data {
        dictionary_put(dictionary_uref, &key, value);
    }
}

#[no_mangle]
pub extern "C" fn store_data() {
    let data_type: String = runtime::get_named_arg("data_type");
    let data_id: String = runtime::get_named_arg("data_id");
    let data: String = runtime::get_named_arg("data");
    let dictionary_uref = match runtime::get_key(&data_type) {
        None => new_dictionary(&data_type).unwrap_or_revert(),
        Some(dict_uref) => *dict_uref.as_uref().unwrap_or_revert(),
    };
    dictionary_put(dictionary_uref, &data_id, data);
}

#[no_mangle]
pub extern "C" fn call() {
    let name: String = runtime::get_named_arg("name");
    let entry_points = {
        let mut eps = EntryPoints::new();
        eps.add_entry_point(EntryPoint::new(
            "store_data",
            vec![
                Parameter::new("data_type", CLType::String),
                Parameter::new("data_id", CLType::String),
                Parameter::new("data", CLType::String),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps.add_entry_point(EntryPoint::new(
            "store_batch",
            vec![
                Parameter::new("data_type", CLType::String),
                Parameter::new("data_batch", BTreeMap::<String, String>::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            casper_types::EntryPointType::Contract,
        ));
        eps
    };

    let (contract_hash, _version) = new_contract(
        entry_points,
        None,
        Some(format! {"{name}_contract_package_hash"}),
        Some(format! {"{name}_access_token"}),
    );
    runtime::put_key(&format! {"{name}_contract_hash"}, contract_hash.into());
}
