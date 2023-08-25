///`InitReserveInput(address,address,address,uint8,address,address,address,address,string,string,string,string,string,string,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct InitReserveInput {
    pub a_token_impl: ::ethers::core::types::Address,
    pub stable_debt_token_impl: ::ethers::core::types::Address,
    pub variable_debt_token_impl: ::ethers::core::types::Address,
    pub underlying_asset_decimals: u8,
    pub interest_rate_strategy_address: ::ethers::core::types::Address,
    pub underlying_asset: ::ethers::core::types::Address,
    pub treasury: ::ethers::core::types::Address,
    pub incentives_controller: ::ethers::core::types::Address,
    pub a_token_name: ::std::string::String,
    pub a_token_symbol: ::std::string::String,
    pub variable_debt_token_name: ::std::string::String,
    pub variable_debt_token_symbol: ::std::string::String,
    pub stable_debt_token_name: ::std::string::String,
    pub stable_debt_token_symbol: ::std::string::String,
    pub params: ::ethers::core::types::Bytes,
}
///`UpdateATokenInput(address,address,address,string,string,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct UpdateATokenInput {
    pub asset: ::ethers::core::types::Address,
    pub treasury: ::ethers::core::types::Address,
    pub incentives_controller: ::ethers::core::types::Address,
    pub name: ::std::string::String,
    pub symbol: ::std::string::String,
    pub implementation: ::ethers::core::types::Address,
    pub params: ::ethers::core::types::Bytes,
}
///`UpdateDebtTokenInput(address,address,string,string,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct UpdateDebtTokenInput {
    pub asset: ::ethers::core::types::Address,
    pub incentives_controller: ::ethers::core::types::Address,
    pub name: ::std::string::String,
    pub symbol: ::std::string::String,
    pub implementation: ::ethers::core::types::Address,
    pub params: ::ethers::core::types::Bytes,
}
///`CalculateInterestRatesParams(uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct CalculateInterestRatesParams {
    pub unbacked: ::ethers::core::types::U256,
    pub liquidity_added: ::ethers::core::types::U256,
    pub liquidity_taken: ::ethers::core::types::U256,
    pub total_stable_debt: ::ethers::core::types::U256,
    pub total_variable_debt: ::ethers::core::types::U256,
    pub average_stable_borrow_rate: ::ethers::core::types::U256,
    pub reserve_factor: ::ethers::core::types::U256,
    pub reserve: ::ethers::core::types::Address,
    pub a_token: ::ethers::core::types::Address,
}
///`EmodeCategory(uint16,uint16,uint16,address,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct EmodeCategory {
    pub ltv: u16,
    pub liquidation_threshold: u16,
    pub liquidation_bonus: u16,
    pub price_source: ::ethers::core::types::Address,
    pub label: ::std::string::String,
}
///`ReserveConfigurationMap(uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ReserveConfigurationMap {
    pub data: ::ethers::core::types::U256,
}
///`ReserveData((uint256),uint128,uint128,uint128,uint128,uint128,uint40,uint16,address,address,address,address,uint128,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ReserveData {
    pub configuration: ReserveConfigurationMap,
    pub liquidity_index: u128,
    pub current_liquidity_rate: u128,
    pub variable_borrow_index: u128,
    pub current_variable_borrow_rate: u128,
    pub current_stable_borrow_rate: u128,
    pub last_update_timestamp: u64,
    pub id: u16,
    pub a_token_address: ::ethers::core::types::Address,
    pub stable_debt_token_address: ::ethers::core::types::Address,
    pub variable_debt_token_address: ::ethers::core::types::Address,
    pub interest_rate_strategy_address: ::ethers::core::types::Address,
    pub accrued_to_treasury: u128,
    pub unbacked: u128,
    pub isolation_mode_total_debt: u128,
}
///`UserConfigurationMap(uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct UserConfigurationMap {
    pub data: ::ethers::core::types::U256,
}
///`TokenData(string,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct TokenData {
    pub symbol: ::std::string::String,
    pub token_address: ::ethers::core::types::Address,
}
