pub use i_pool_addresses_provider::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_pool_addresses_provider {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getACLAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getACLAdmin"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getACLManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getACLManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarketId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMarketId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolConfigurator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPoolConfigurator",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolDataProvider"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPoolDataProvider",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPriceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPriceOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPriceOracleSentinel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPriceOracleSentinel",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setACLAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setACLAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAclAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setACLManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setACLManager"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAclManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAddressAsProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAddressAsProxy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newImplementationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMarketId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMarketId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMarketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolConfiguratorImpl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPoolConfiguratorImpl",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newPoolConfiguratorImpl",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolDataProvider"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPoolDataProvider",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDataProvider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPoolImpl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolImpl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPoolImpl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPriceOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPriceOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPriceOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPriceOracleSentinel"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setPriceOracleSentinel",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newPriceOracleSentinel",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACLAdminUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ACLAdminUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ACLManagerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ACLManagerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddressSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressSetAsProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AddressSetAsProxy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxyAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldImplementationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newImplementationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MarketIdSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MarketIdSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldMarketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newMarketId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolConfiguratorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolConfiguratorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolDataProviderUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PoolDataProviderUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PoolUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PriceOracleSentinelUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PriceOracleSentinelUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PriceOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PriceOracleUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProxyCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProxyCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxyAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "implementationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IPOOLADDRESSESPROVIDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPoolAddressesProvider<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPoolAddressesProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPoolAddressesProvider<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPoolAddressesProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPoolAddressesProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPoolAddressesProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPoolAddressesProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOOLADDRESSESPROVIDER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getACLAdmin` (0x0e67178c) function
        pub fn get_acl_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([14, 103, 23, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getACLManager` (0x707cd716) function
        pub fn get_acl_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([112, 124, 215, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddress` (0x21f8a721) function
        pub fn get_address(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 248, 167, 33], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarketId` (0x568ef470) function
        pub fn get_market_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 142, 244, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x026b1d5f) function
        pub fn get_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 107, 29, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolConfigurator` (0x631adfca) function
        pub fn get_pool_configurator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 26, 223, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolDataProvider` (0xe860accb) function
        pub fn get_pool_data_provider(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([232, 96, 172, 203], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceOracle` (0xfca513a8) function
        pub fn get_price_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceOracleSentinel` (0x5eb88d3d) function
        pub fn get_price_oracle_sentinel(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([94, 184, 141, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setACLAdmin` (0x76d84ffc) function
        pub fn set_acl_admin(
            &self,
            new_acl_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 216, 79, 252], new_acl_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setACLManager` (0xed301ca9) function
        pub fn set_acl_manager(
            &self,
            new_acl_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 48, 28, 169], new_acl_manager)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddress` (0xca446dd9) function
        pub fn set_address(
            &self,
            id: [u8; 32],
            new_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 68, 109, 217], (id, new_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddressAsProxy` (0x5dcc528c) function
        pub fn set_address_as_proxy(
            &self,
            id: [u8; 32],
            new_implementation_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 204, 82, 140], (id, new_implementation_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMarketId` (0xf67b1847) function
        pub fn set_market_id(
            &self,
            new_market_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 123, 24, 71], new_market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolConfiguratorImpl` (0xe4ca28b7) function
        pub fn set_pool_configurator_impl(
            &self,
            new_pool_configurator_impl: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 202, 40, 183], new_pool_configurator_impl)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolDataProvider` (0xe44e9ed1) function
        pub fn set_pool_data_provider(
            &self,
            new_data_provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 78, 158, 209], new_data_provider)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolImpl` (0xa1564406) function
        pub fn set_pool_impl(
            &self,
            new_pool_impl: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 86, 68, 6], new_pool_impl)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPriceOracle` (0x530e784f) function
        pub fn set_price_oracle(
            &self,
            new_price_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], new_price_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPriceOracleSentinel` (0x74944cec) function
        pub fn set_price_oracle_sentinel(
            &self,
            new_price_oracle_sentinel: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 148, 76, 236], new_price_oracle_sentinel)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ACLAdminUpdated` event
        pub fn acl_admin_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AcladminUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ACLManagerUpdated` event
        pub fn acl_manager_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AclmanagerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AddressSet` event
        pub fn address_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddressSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AddressSetAsProxy` event
        pub fn address_set_as_proxy_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddressSetAsProxyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MarketIdSet` event
        pub fn market_id_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketIdSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolConfiguratorUpdated` event
        pub fn pool_configurator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolConfiguratorUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolDataProviderUpdated` event
        pub fn pool_data_provider_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolDataProviderUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PoolUpdated` event
        pub fn pool_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PriceOracleSentinelUpdated` event
        pub fn price_oracle_sentinel_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceOracleSentinelUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PriceOracleUpdated` event
        pub fn price_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceOracleUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProxyCreated` event
        pub fn proxy_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProxyCreatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPoolAddressesProviderEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPoolAddressesProvider<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ACLAdminUpdated", abi = "ACLAdminUpdated(address,address)")]
    pub struct AcladminUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ACLManagerUpdated", abi = "ACLManagerUpdated(address,address)")]
    pub struct AclmanagerUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AddressSetAsProxy",
        abi = "AddressSetAsProxy(bytes32,address,address,address)"
    )]
    pub struct AddressSetAsProxyFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ::ethers::core::types::Address,
        pub old_implementation_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_implementation_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MarketIdSet", abi = "MarketIdSet(string,string)")]
    pub struct MarketIdSetFilter {
        #[ethevent(indexed)]
        pub old_market_id: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub new_market_id: ::ethers::core::types::H256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PoolConfiguratorUpdated",
        abi = "PoolConfiguratorUpdated(address,address)"
    )]
    pub struct PoolConfiguratorUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PoolDataProviderUpdated",
        abi = "PoolDataProviderUpdated(address,address)"
    )]
    pub struct PoolDataProviderUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PoolUpdated", abi = "PoolUpdated(address,address)")]
    pub struct PoolUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PriceOracleSentinelUpdated",
        abi = "PriceOracleSentinelUpdated(address,address)"
    )]
    pub struct PriceOracleSentinelUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PriceOracleUpdated", abi = "PriceOracleUpdated(address,address)")]
    pub struct PriceOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub old_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ProxyCreated", abi = "ProxyCreated(bytes32,address,address)")]
    pub struct ProxyCreatedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub proxy_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPoolAddressesProviderEvents {
        AcladminUpdatedFilter(AcladminUpdatedFilter),
        AclmanagerUpdatedFilter(AclmanagerUpdatedFilter),
        AddressSetFilter(AddressSetFilter),
        AddressSetAsProxyFilter(AddressSetAsProxyFilter),
        MarketIdSetFilter(MarketIdSetFilter),
        PoolConfiguratorUpdatedFilter(PoolConfiguratorUpdatedFilter),
        PoolDataProviderUpdatedFilter(PoolDataProviderUpdatedFilter),
        PoolUpdatedFilter(PoolUpdatedFilter),
        PriceOracleSentinelUpdatedFilter(PriceOracleSentinelUpdatedFilter),
        PriceOracleUpdatedFilter(PriceOracleUpdatedFilter),
        ProxyCreatedFilter(ProxyCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPoolAddressesProviderEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AcladminUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AcladminUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AclmanagerUpdatedFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::AclmanagerUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = AddressSetAsProxyFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::AddressSetAsProxyFilter(decoded),
                );
            }
            if let Ok(decoded) = MarketIdSetFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::MarketIdSetFilter(decoded));
            }
            if let Ok(decoded) = PoolConfiguratorUpdatedFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PoolDataProviderUpdatedFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PoolUpdatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::PoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleSentinelUpdatedFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderEvents::PriceOracleUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = ProxyCreatedFilter::decode_log(log) {
                return Ok(IPoolAddressesProviderEvents::ProxyCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPoolAddressesProviderEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcladminUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AclmanagerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressSetAsProxyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MarketIdSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolConfiguratorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolDataProviderUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceOracleSentinelUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProxyCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AcladminUpdatedFilter> for IPoolAddressesProviderEvents {
        fn from(value: AcladminUpdatedFilter) -> Self {
            Self::AcladminUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AclmanagerUpdatedFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: AclmanagerUpdatedFilter) -> Self {
            Self::AclmanagerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AddressSetFilter> for IPoolAddressesProviderEvents {
        fn from(value: AddressSetFilter) -> Self {
            Self::AddressSetFilter(value)
        }
    }
    impl ::core::convert::From<AddressSetAsProxyFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: AddressSetAsProxyFilter) -> Self {
            Self::AddressSetAsProxyFilter(value)
        }
    }
    impl ::core::convert::From<MarketIdSetFilter> for IPoolAddressesProviderEvents {
        fn from(value: MarketIdSetFilter) -> Self {
            Self::MarketIdSetFilter(value)
        }
    }
    impl ::core::convert::From<PoolConfiguratorUpdatedFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: PoolConfiguratorUpdatedFilter) -> Self {
            Self::PoolConfiguratorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PoolDataProviderUpdatedFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: PoolDataProviderUpdatedFilter) -> Self {
            Self::PoolDataProviderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PoolUpdatedFilter> for IPoolAddressesProviderEvents {
        fn from(value: PoolUpdatedFilter) -> Self {
            Self::PoolUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PriceOracleSentinelUpdatedFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: PriceOracleSentinelUpdatedFilter) -> Self {
            Self::PriceOracleSentinelUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PriceOracleUpdatedFilter>
    for IPoolAddressesProviderEvents {
        fn from(value: PriceOracleUpdatedFilter) -> Self {
            Self::PriceOracleUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ProxyCreatedFilter> for IPoolAddressesProviderEvents {
        fn from(value: ProxyCreatedFilter) -> Self {
            Self::ProxyCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `getACLAdmin` function with signature `getACLAdmin()` and selector `0x0e67178c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getACLAdmin", abi = "getACLAdmin()")]
    pub struct GetACLAdminCall;
    ///Container type for all input parameters for the `getACLManager` function with signature `getACLManager()` and selector `0x707cd716`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getACLManager", abi = "getACLManager()")]
    pub struct GetACLManagerCall;
    ///Container type for all input parameters for the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAddress", abi = "getAddress(bytes32)")]
    pub struct GetAddressCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `getMarketId` function with signature `getMarketId()` and selector `0x568ef470`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMarketId", abi = "getMarketId()")]
    pub struct GetMarketIdCall;
    ///Container type for all input parameters for the `getPool` function with signature `getPool()` and selector `0x026b1d5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPool", abi = "getPool()")]
    pub struct GetPoolCall;
    ///Container type for all input parameters for the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `0x631adfca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolConfigurator", abi = "getPoolConfigurator()")]
    pub struct GetPoolConfiguratorCall;
    ///Container type for all input parameters for the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `0xe860accb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPoolDataProvider", abi = "getPoolDataProvider()")]
    pub struct GetPoolDataProviderCall;
    ///Container type for all input parameters for the `getPriceOracle` function with signature `getPriceOracle()` and selector `0xfca513a8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPriceOracle", abi = "getPriceOracle()")]
    pub struct GetPriceOracleCall;
    ///Container type for all input parameters for the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `0x5eb88d3d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPriceOracleSentinel", abi = "getPriceOracleSentinel()")]
    pub struct GetPriceOracleSentinelCall;
    ///Container type for all input parameters for the `setACLAdmin` function with signature `setACLAdmin(address)` and selector `0x76d84ffc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setACLAdmin", abi = "setACLAdmin(address)")]
    pub struct SetACLAdminCall {
        pub new_acl_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setACLManager` function with signature `setACLManager(address)` and selector `0xed301ca9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setACLManager", abi = "setACLManager(address)")]
    pub struct SetACLManagerCall {
        pub new_acl_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAddress` function with signature `setAddress(bytes32,address)` and selector `0xca446dd9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAddress", abi = "setAddress(bytes32,address)")]
    pub struct SetAddressCall {
        pub id: [u8; 32],
        pub new_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAddressAsProxy` function with signature `setAddressAsProxy(bytes32,address)` and selector `0x5dcc528c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAddressAsProxy", abi = "setAddressAsProxy(bytes32,address)")]
    pub struct SetAddressAsProxyCall {
        pub id: [u8; 32],
        pub new_implementation_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setMarketId` function with signature `setMarketId(string)` and selector `0xf67b1847`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setMarketId", abi = "setMarketId(string)")]
    pub struct SetMarketIdCall {
        pub new_market_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `setPoolConfiguratorImpl` function with signature `setPoolConfiguratorImpl(address)` and selector `0xe4ca28b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setPoolConfiguratorImpl",
        abi = "setPoolConfiguratorImpl(address)"
    )]
    pub struct SetPoolConfiguratorImplCall {
        pub new_pool_configurator_impl: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPoolDataProvider` function with signature `setPoolDataProvider(address)` and selector `0xe44e9ed1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPoolDataProvider", abi = "setPoolDataProvider(address)")]
    pub struct SetPoolDataProviderCall {
        pub new_data_provider: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPoolImpl` function with signature `setPoolImpl(address)` and selector `0xa1564406`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPoolImpl", abi = "setPoolImpl(address)")]
    pub struct SetPoolImplCall {
        pub new_pool_impl: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPriceOracle` function with signature `setPriceOracle(address)` and selector `0x530e784f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPriceOracle", abi = "setPriceOracle(address)")]
    pub struct SetPriceOracleCall {
        pub new_price_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPriceOracleSentinel` function with signature `setPriceOracleSentinel(address)` and selector `0x74944cec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPriceOracleSentinel", abi = "setPriceOracleSentinel(address)")]
    pub struct SetPriceOracleSentinelCall {
        pub new_price_oracle_sentinel: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPoolAddressesProviderCalls {
        GetACLAdmin(GetACLAdminCall),
        GetACLManager(GetACLManagerCall),
        GetAddress(GetAddressCall),
        GetMarketId(GetMarketIdCall),
        GetPool(GetPoolCall),
        GetPoolConfigurator(GetPoolConfiguratorCall),
        GetPoolDataProvider(GetPoolDataProviderCall),
        GetPriceOracle(GetPriceOracleCall),
        GetPriceOracleSentinel(GetPriceOracleSentinelCall),
        SetACLAdmin(SetACLAdminCall),
        SetACLManager(SetACLManagerCall),
        SetAddress(SetAddressCall),
        SetAddressAsProxy(SetAddressAsProxyCall),
        SetMarketId(SetMarketIdCall),
        SetPoolConfiguratorImpl(SetPoolConfiguratorImplCall),
        SetPoolDataProvider(SetPoolDataProviderCall),
        SetPoolImpl(SetPoolImplCall),
        SetPriceOracle(SetPriceOracleCall),
        SetPriceOracleSentinel(SetPriceOracleSentinelCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolAddressesProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetACLAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetACLAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetACLManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetACLManager(decoded));
            }
            if let Ok(decoded)
                = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAddress(decoded));
            }
            if let Ok(decoded)
                = <GetMarketIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMarketId(decoded));
            }
            if let Ok(decoded)
                = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded)
                = <GetPoolConfiguratorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPoolConfigurator(decoded));
            }
            if let Ok(decoded)
                = <GetPoolDataProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPoolDataProvider(decoded));
            }
            if let Ok(decoded)
                = <GetPriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPriceOracle(decoded));
            }
            if let Ok(decoded)
                = <GetPriceOracleSentinelCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPriceOracleSentinel(decoded));
            }
            if let Ok(decoded)
                = <SetACLAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetACLAdmin(decoded));
            }
            if let Ok(decoded)
                = <SetACLManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetACLManager(decoded));
            }
            if let Ok(decoded)
                = <SetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAddress(decoded));
            }
            if let Ok(decoded)
                = <SetAddressAsProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetAddressAsProxy(decoded));
            }
            if let Ok(decoded)
                = <SetMarketIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetMarketId(decoded));
            }
            if let Ok(decoded)
                = <SetPoolConfiguratorImplCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPoolConfiguratorImpl(decoded));
            }
            if let Ok(decoded)
                = <SetPoolDataProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPoolDataProvider(decoded));
            }
            if let Ok(decoded)
                = <SetPoolImplCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPoolImpl(decoded));
            }
            if let Ok(decoded)
                = <SetPriceOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPriceOracle(decoded));
            }
            if let Ok(decoded)
                = <SetPriceOracleSentinelCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetPriceOracleSentinel(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPoolAddressesProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetACLAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetACLManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarketId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolConfigurator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolDataProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceOracleSentinel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetACLAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetACLManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAddressAsProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMarketId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolConfiguratorImpl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolDataProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolImpl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPriceOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPriceOracleSentinel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPoolAddressesProviderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetACLAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetACLManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarketId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolConfigurator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPoolDataProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPriceOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceOracleSentinel(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetACLAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetACLManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddressAsProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMarketId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPoolConfiguratorImpl(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolDataProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolImpl(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriceOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriceOracleSentinel(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetACLAdminCall> for IPoolAddressesProviderCalls {
        fn from(value: GetACLAdminCall) -> Self {
            Self::GetACLAdmin(value)
        }
    }
    impl ::core::convert::From<GetACLManagerCall> for IPoolAddressesProviderCalls {
        fn from(value: GetACLManagerCall) -> Self {
            Self::GetACLManager(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for IPoolAddressesProviderCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetMarketIdCall> for IPoolAddressesProviderCalls {
        fn from(value: GetMarketIdCall) -> Self {
            Self::GetMarketId(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for IPoolAddressesProviderCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolConfiguratorCall> for IPoolAddressesProviderCalls {
        fn from(value: GetPoolConfiguratorCall) -> Self {
            Self::GetPoolConfigurator(value)
        }
    }
    impl ::core::convert::From<GetPoolDataProviderCall> for IPoolAddressesProviderCalls {
        fn from(value: GetPoolDataProviderCall) -> Self {
            Self::GetPoolDataProvider(value)
        }
    }
    impl ::core::convert::From<GetPriceOracleCall> for IPoolAddressesProviderCalls {
        fn from(value: GetPriceOracleCall) -> Self {
            Self::GetPriceOracle(value)
        }
    }
    impl ::core::convert::From<GetPriceOracleSentinelCall>
    for IPoolAddressesProviderCalls {
        fn from(value: GetPriceOracleSentinelCall) -> Self {
            Self::GetPriceOracleSentinel(value)
        }
    }
    impl ::core::convert::From<SetACLAdminCall> for IPoolAddressesProviderCalls {
        fn from(value: SetACLAdminCall) -> Self {
            Self::SetACLAdmin(value)
        }
    }
    impl ::core::convert::From<SetACLManagerCall> for IPoolAddressesProviderCalls {
        fn from(value: SetACLManagerCall) -> Self {
            Self::SetACLManager(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for IPoolAddressesProviderCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<SetAddressAsProxyCall> for IPoolAddressesProviderCalls {
        fn from(value: SetAddressAsProxyCall) -> Self {
            Self::SetAddressAsProxy(value)
        }
    }
    impl ::core::convert::From<SetMarketIdCall> for IPoolAddressesProviderCalls {
        fn from(value: SetMarketIdCall) -> Self {
            Self::SetMarketId(value)
        }
    }
    impl ::core::convert::From<SetPoolConfiguratorImplCall>
    for IPoolAddressesProviderCalls {
        fn from(value: SetPoolConfiguratorImplCall) -> Self {
            Self::SetPoolConfiguratorImpl(value)
        }
    }
    impl ::core::convert::From<SetPoolDataProviderCall> for IPoolAddressesProviderCalls {
        fn from(value: SetPoolDataProviderCall) -> Self {
            Self::SetPoolDataProvider(value)
        }
    }
    impl ::core::convert::From<SetPoolImplCall> for IPoolAddressesProviderCalls {
        fn from(value: SetPoolImplCall) -> Self {
            Self::SetPoolImpl(value)
        }
    }
    impl ::core::convert::From<SetPriceOracleCall> for IPoolAddressesProviderCalls {
        fn from(value: SetPriceOracleCall) -> Self {
            Self::SetPriceOracle(value)
        }
    }
    impl ::core::convert::From<SetPriceOracleSentinelCall>
    for IPoolAddressesProviderCalls {
        fn from(value: SetPriceOracleSentinelCall) -> Self {
            Self::SetPriceOracleSentinel(value)
        }
    }
    ///Container type for all return fields from the `getACLAdmin` function with signature `getACLAdmin()` and selector `0x0e67178c`
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
    pub struct GetACLAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getACLManager` function with signature `getACLManager()` and selector `0x707cd716`
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
    pub struct GetACLManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
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
    pub struct GetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getMarketId` function with signature `getMarketId()` and selector `0x568ef470`
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
    pub struct GetMarketIdReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getPool` function with signature `getPool()` and selector `0x026b1d5f`
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
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolConfigurator` function with signature `getPoolConfigurator()` and selector `0x631adfca`
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
    pub struct GetPoolConfiguratorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolDataProvider` function with signature `getPoolDataProvider()` and selector `0xe860accb`
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
    pub struct GetPoolDataProviderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPriceOracle` function with signature `getPriceOracle()` and selector `0xfca513a8`
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
    pub struct GetPriceOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPriceOracleSentinel` function with signature `getPriceOracleSentinel()` and selector `0x5eb88d3d`
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
    pub struct GetPriceOracleSentinelReturn(pub ::ethers::core::types::Address);
}
