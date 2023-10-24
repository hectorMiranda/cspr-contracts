#[cfg(test)]
mod test {
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
    use casper_types::{
        bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, Key, RuntimeArgs,
    };
    use std::collections::BTreeMap;

    #[test]
    fn test_store_data() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        let install_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            "storage_contract.wasm",
            runtime_args! {
                "name" => "storage"
            },
        )
        .build();

        builder.exec(install_request).expect_success().commit();

        let contract_key: Key = get_contract_hash(&builder, "storage").into();

        let store_request = ExecuteRequestBuilder::contract_call_by_name(
            *DEFAULT_ACCOUNT_ADDR,
            "storage_contract_hash",
            "store_data",
            runtime_args! {
                "data_type" => "simple".to_string(),
                "data_id" => "id_0".to_string(),
                "data" => "{simple data}".to_string(),
            },
        )
        .build();

        builder.exec(store_request).expect_success().commit();

        let stored_data: String =
            get_dictionary_value_from_key(&builder, &contract_key, "simple", "id_0");
        assert_eq!("{simple data}", stored_data);
    }

    #[test]
    fn test_store_batch() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        let install_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            "storage_contract.wasm",
            runtime_args! {
                "name" => "storage"
            },
        )
        .build();

        builder.exec(install_request).expect_success().commit();

        let contract_key: Key = get_contract_hash(&builder, "storage").into();

        let mut batch = BTreeMap::new();
        batch.insert("id_0", "{data_0}");
        batch.insert("id_1", "{data_1}");
        batch.insert("id_2", "{data_2}");
        batch.insert("id_3", "{data_3}");
        batch.insert("id_4", "{data_4}");
        batch.insert("id_5", "{data_5}");

        let store_request = ExecuteRequestBuilder::contract_call_by_name(
            *DEFAULT_ACCOUNT_ADDR,
            "storage_contract_hash",
            "store_batch",
            runtime_args! {
                "data_type" => "simple".to_string(),
                "data_batch" => batch
            },
        )
        .build();

        builder.exec(store_request).expect_success().commit();

        let stored_data: String =
            get_dictionary_value_from_key(&builder, &contract_key, "simple", "id_3");
        assert_eq!("{data_3}", stored_data);
    }

    pub(crate) fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
        contract_key: &Key,
        dictionary_name: &str,
        dictionary_key: &str,
    ) -> T {
        let seed_uref = *builder
            .query(None, *contract_key, &[])
            .expect("must have contract")
            .as_contract()
            .expect("must convert contract")
            .named_keys()
            .get(dictionary_name)
            .expect("must have key")
            .as_uref()
            .expect("must convert to seed uref");

        builder
            .query_dictionary_item(None, seed_uref, dictionary_key)
            .expect("should have dictionary value")
            .as_cl_value()
            .expect("T should be CLValue")
            .to_owned()
            .into_t()
            .unwrap()
    }

    pub(crate) fn get_contract_hash(
        builder: &WasmTestBuilder<InMemoryGlobalState>,
        name: &str,
    ) -> ContractHash {
        let hash_addr = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(&format! {"{name}_contract_hash"})
            .expect("must have this entry in named keys")
            .into_hash()
            .expect("must get hash_addr");

        ContractHash::new(hash_addr)
    }
}
