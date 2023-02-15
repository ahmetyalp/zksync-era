use zksync_basic_types::{AccountTreeId, Address, U256};
use zksync_config::constants::{BOOTLOADER_UTILITIES_ADDRESS, EVENT_WRITER_ADDRESS};
use zksync_contracts::read_sys_contract_bytecode;

use crate::{
    block::DeployedContract, ACCOUNT_CODE_STORAGE_ADDRESS, BOOTLOADER_ADDRESS,
    CONTRACT_DEPLOYER_ADDRESS, ECRECOVER_PRECOMPILE_ADDRESS, IMMUTABLE_SIMULATOR_STORAGE_ADDRESS,
    KECCAK256_PRECOMPILE_ADDRESS, KNOWN_CODES_STORAGE_ADDRESS, L1_MESSENGER_ADDRESS,
    L2_ETH_TOKEN_ADDRESS, MSG_VALUE_SIMULATOR_ADDRESS, NONCE_HOLDER_ADDRESS,
    SHA256_PRECOMPILE_ADDRESS, SYSTEM_CONTEXT_ADDRESS,
};
use once_cell::sync::Lazy;

// Note, that in the NONCE_HOLDER_ADDRESS's storage the nonces of accounts
// are stored in the following form:
// 2^128 * deployment_nonce + tx_nonce,
// where `tx_nonce` should be number of transactions, the account has processed
// and the `deployment_nonce` should be the number of contracts.
pub const TX_NONCE_INCREMENT: U256 = U256([1, 0, 0, 0]); // 1
pub const DEPLOYMENT_NONCE_INCREMENT: U256 = U256([0, 0, 1, 0]); // 2^128

static SYSTEM_CONTRACTS: Lazy<Vec<DeployedContract>> = Lazy::new(|| {
    let mut deployed_system_contracts = [
        ("", "AccountCodeStorage", ACCOUNT_CODE_STORAGE_ADDRESS),
        ("", "NonceHolder", NONCE_HOLDER_ADDRESS),
        ("", "KnownCodesStorage", KNOWN_CODES_STORAGE_ADDRESS),
        (
            "",
            "ImmutableSimulator",
            IMMUTABLE_SIMULATOR_STORAGE_ADDRESS,
        ),
        ("", "ContractDeployer", CONTRACT_DEPLOYER_ADDRESS),
        ("", "L1Messenger", L1_MESSENGER_ADDRESS),
        ("", "MsgValueSimulator", MSG_VALUE_SIMULATOR_ADDRESS),
        ("", "L2EthToken", L2_ETH_TOKEN_ADDRESS),
        ("precompiles/", "Keccak256", KECCAK256_PRECOMPILE_ADDRESS),
        ("precompiles/", "SHA256", SHA256_PRECOMPILE_ADDRESS),
        ("precompiles/", "Ecrecover", ECRECOVER_PRECOMPILE_ADDRESS),
        ("", "SystemContext", SYSTEM_CONTEXT_ADDRESS),
        ("", "EventWriter", EVENT_WRITER_ADDRESS),
        ("", "BootloaderUtilities", BOOTLOADER_UTILITIES_ADDRESS),
    ]
    .map(|(path, name, address)| DeployedContract {
        account_id: AccountTreeId::new(address),
        bytecode: read_sys_contract_bytecode(path, name),
    })
    .to_vec();

    let empty_bytecode = read_sys_contract_bytecode("", "EmptyContract");
    // For now, only zero address and the bootloader address have empty bytecode at the init
    // In the future, we might want to set all of the system contracts this way.
    let empty_system_contracts =
        [Address::zero(), BOOTLOADER_ADDRESS].map(|address| DeployedContract {
            account_id: AccountTreeId::new(address),
            bytecode: empty_bytecode.clone(),
        });

    deployed_system_contracts.extend(empty_system_contracts);
    deployed_system_contracts
});

pub fn get_system_smart_contracts() -> Vec<DeployedContract> {
    SYSTEM_CONTRACTS.clone()
}
