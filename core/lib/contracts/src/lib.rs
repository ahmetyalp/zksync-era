#![allow(clippy::derive_partial_eq_without_eq)]

use ethabi::ethereum_types::U256;
use ethabi::Contract;
use once_cell::sync::Lazy;
use std::fs::{self, File};
use std::path::Path;

use zksync_utils::bytecode::hash_bytecode;
use zksync_utils::{bytes_to_be_words, h256_to_u256};

const ZKSYNC_CONTRACT_FILE: &str =
    "contracts/ethereum/artifacts/cache/solpp-generated-contracts/zksync/interfaces/IZkSync.sol/IZkSync.json";
const VERIFIER_CONTRACT_FILE: &str =
    "contracts/ethereum/artifacts/cache/solpp-generated-contracts/zksync/Verifier.sol/Verifier.json";
const IERC20_CONTRACT_FILE: &str =
    "contracts/ethereum/artifacts/cache/solpp-generated-contracts/common/interfaces/IERC20.sol/IERC20.json";
const FAIL_ON_RECEIVE_CONTRACT_FILE: &str =
    "contracts/ethereum/artifacts/cache/solpp-generated-contracts/zksync/dev-contracts/FailOnReceive.sol/FailOnReceive.json";
const L2_BRIDGE_CONTRACT_FILE: &str =
    "contracts/zksync/artifacts-zk/cache-zk/solpp-generated-contracts/bridge/interfaces/IL2Bridge.sol/IL2Bridge.json";
const LOADNEXT_CONTRACT_FILE: &str =
    "etc/contracts-test-data/artifacts-zk/contracts/loadnext/loadnext_contract.sol/LoadnextContract.json";
const LOADNEXT_SIMPLE_CONTRACT_FILE: &str =
    "etc/contracts-test-data/artifacts-zk/contracts/loadnext/loadnext_contract.sol/Foo.json";

fn read_file_to_json_value(path: impl AsRef<Path>) -> serde_json::Value {
    let zksync_home = std::env::var("ZKSYNC_HOME").unwrap_or_else(|_| ".".into());
    let path = Path::new(&zksync_home).join(path);
    serde_json::from_reader(
        File::open(&path).unwrap_or_else(|e| panic!("Failed to open file {:?}: {}", path, e)),
    )
    .unwrap_or_else(|e| panic!("Failed to parse file {:?}: {}", path, e))
}

pub fn load_contract<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Contract {
    serde_json::from_value(read_file_to_json_value(&path)["abi"].take())
        .unwrap_or_else(|e| panic!("Failed to parse contract abi from file {:?}: {}", path, e))
}

pub fn load_sys_contract(contract_name: &str) -> Contract {
    load_contract(format!(
        "etc/system-contracts/artifacts-zk/cache-zk/solpp-generated-contracts/{0}.sol/{0}.json",
        contract_name
    ))
}

pub fn read_contract_abi(path: impl AsRef<Path>) -> String {
    read_file_to_json_value(path)["abi"]
        .as_str()
        .expect("Failed to parse abi")
        .to_string()
}

pub fn zksync_contract() -> Contract {
    load_contract(ZKSYNC_CONTRACT_FILE)
}

pub fn erc20_contract() -> Contract {
    load_contract(IERC20_CONTRACT_FILE)
}

pub fn l2_bridge_contract() -> Contract {
    load_contract(L2_BRIDGE_CONTRACT_FILE)
}

pub fn verifier_contract() -> Contract {
    load_contract(VERIFIER_CONTRACT_FILE)
}

#[derive(Debug, Clone)]
pub struct TestContract {
    /// Contract bytecode to be used for sending deploy transaction.
    pub bytecode: Vec<u8>,
    /// Contract ABI.
    pub contract: Contract,

    pub factory_deps: Vec<Vec<u8>>,
}

/// Reads test contract bytecode and its ABI.
pub fn get_loadnext_contract() -> TestContract {
    let bytecode = read_bytecode(LOADNEXT_CONTRACT_FILE);
    let dep = read_bytecode(LOADNEXT_SIMPLE_CONTRACT_FILE);

    TestContract {
        bytecode,
        contract: loadnext_contract(),
        factory_deps: vec![dep],
    }
}

// Returns loadnext contract and its factory dependencies
pub fn loadnext_contract() -> Contract {
    load_contract("etc/contracts-test-data/artifacts-zk/contracts/loadnext/loadnext_contract.sol/LoadnextContract.json")
}

pub fn loadnext_simple_contract() -> Contract {
    load_contract(
        "etc/contracts-test-data/artifacts-zk/contracts/loadnext/loadnext_contract.sol/Foo.json",
    )
}

pub fn fail_on_receive_contract() -> Contract {
    load_contract(FAIL_ON_RECEIVE_CONTRACT_FILE)
}

pub fn deployer_contract() -> Contract {
    load_sys_contract("ContractDeployer")
}

pub fn eth_contract() -> Contract {
    load_sys_contract("L2EthToken")
}

pub fn known_codes_contract() -> Contract {
    load_sys_contract("KnownCodesStorage")
}

pub fn read_bytecode(path: impl AsRef<Path>) -> Vec<u8> {
    let zksync_home = std::env::var("ZKSYNC_HOME").unwrap_or_else(|_| ".".into());
    let artifact_path = Path::new(&zksync_home).join(path);
    let artifact = read_file_to_json_value(artifact_path.clone());

    let bytecode = artifact["bytecode"]
        .as_str()
        .unwrap_or_else(|| panic!("Bytecode not found in {:?}", artifact_path))
        .strip_prefix("0x")
        .unwrap_or_else(|| panic!("Bytecode in {:?} is not hex", artifact_path));

    hex::decode(bytecode)
        .unwrap_or_else(|err| panic!("Can't decode bytecode in {:?}: {}", artifact_path, err))
}

pub fn default_erc20_bytecode() -> Vec<u8> {
    read_bytecode("etc/ERC20/artifacts-zk/contracts/ZkSyncERC20.sol/ZkSyncERC20.json")
}

pub fn read_sys_contract_bytecode(directory: &str, name: &str) -> Vec<u8> {
    read_bytecode(format!(
        "etc/system-contracts/artifacts-zk/cache-zk/solpp-generated-contracts/{0}{1}.sol/{1}.json",
        directory, name
    ))
}

pub fn read_bootloader_code(bootloader_type: &str) -> Vec<u8> {
    read_zbin_bytecode(format!(
        "etc/system-contracts/bootloader/build/artifacts/{}.yul/{}.yul.zbin",
        bootloader_type, bootloader_type
    ))
}

pub fn read_proved_block_bootloader_bytecode() -> Vec<u8> {
    read_bootloader_code("proved_block")
}

pub fn read_playground_block_bootloader_bytecode() -> Vec<u8> {
    read_bootloader_code("playground_block")
}

pub fn get_loadnext_test_contract_path(file_name: &str, contract_name: &str) -> String {
    format!(
        "core/tests/loadnext/test-contracts/loadnext_contract/artifacts/loadnext_contract.sol/{}.sol:{}.abi",
        file_name, contract_name
    )
}

pub fn get_loadnext_test_contract_bytecode(file_name: &str, contract_name: &str) -> String {
    format!(
        "core/tests/loadnext/test-contracts/loadnext_contract/artifacts/loadnext_contract.sol/{}.sol:{}.zbin",
        file_name, contract_name
    )
}
pub fn read_zbin_bytecode(zbin_path: impl AsRef<Path>) -> Vec<u8> {
    let zksync_home = std::env::var("ZKSYNC_HOME").unwrap_or_else(|_| ".".into());
    let bytecode_path = Path::new(&zksync_home).join(zbin_path);
    fs::read(&bytecode_path)
        .unwrap_or_else(|err| panic!("Can't read .zbin bytecode at {:?}: {}", bytecode_path, err))
}

pub fn read_bootloader_bytecode() -> Vec<u8> {
    read_zbin_bytecode("etc/system-contracts/bootloader/artifacts/bootloader/bootloader.yul.zbin")
}

/// Hash of code and code which consists of 32 bytes words
#[derive(Debug)]
pub struct SystemContractCode {
    pub code: Vec<U256>,
    pub hash: U256,
}

pub static PROVED_BLOCK_BOOTLOADER_CODE: Lazy<SystemContractCode> = Lazy::new(|| {
    let bytecode = read_proved_block_bootloader_bytecode();
    let hash = hash_bytecode(&bytecode);

    SystemContractCode {
        code: bytes_to_be_words(bytecode),
        hash: h256_to_u256(hash),
    }
});

pub static PLAYGROUND_BLOCK_BOOTLOADER_CODE: Lazy<SystemContractCode> = Lazy::new(|| {
    let bytecode = read_playground_block_bootloader_bytecode();
    let hash = hash_bytecode(&bytecode);

    SystemContractCode {
        code: bytes_to_be_words(bytecode),
        hash: h256_to_u256(hash),
    }
});

pub static ESTIMATE_FEE_BLOCK_CODE: Lazy<SystemContractCode> = Lazy::new(|| {
    let bytecode = read_bootloader_code("fee_estimate");
    let hash = hash_bytecode(&bytecode);

    SystemContractCode {
        code: bytes_to_be_words(bytecode),
        hash: h256_to_u256(hash),
    }
});

pub static DEFAULT_ACCOUNT_CODE: Lazy<SystemContractCode> = Lazy::new(|| {
    let bytecode = read_sys_contract_bytecode("", "DefaultAccount");
    let hash = hash_bytecode(&bytecode);

    SystemContractCode {
        code: bytes_to_be_words(bytecode),
        hash: h256_to_u256(hash),
    }
});
