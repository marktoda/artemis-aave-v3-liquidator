pub use pool_addresses_provider::*;
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
pub mod pool_addresses_provider {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("marketId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static POOLADDRESSESPROVIDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1F\xB88\x03\x80b\0\x1F\xB8\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03\xAAV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91`\0\x80Q` b\0\x1F\x98\x839\x81Q\x91R\x90\x82\x90\xA3Pb\0\0o\x82b\0\0\x82V[b\0\0z\x81b\0\x01\x8DV[PPb\0\x04\xD2V[`\0`\x01\x80Tb\0\0\x93\x90b\0\x04wV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\0\xC1\x90b\0\x04wV[\x80\x15b\0\x01\x12W\x80`\x1F\x10b\0\0\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x01\x12V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\0\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PP\x85Q\x93\x94Pb\0\x010\x93`\x01\x93P` \x87\x01\x92P\x90Pb\0\x02\x9EV[P\x81`@Qb\0\x01A\x91\x90b\0\x04\xB4V[`@Q\x80\x91\x03\x90 \x81`@Qb\0\x01Y\x91\x90b\0\x04\xB4V[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xE6\x85\xC8\xCD\xEC\xC6\x03\x0CE\x03\x0F\xD5Gx\x81,\xB8N\xD8\xE4F|8)D\x03\xD6\x8B\xA7\x86\x08#\x90`\0\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x01\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x02TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x01\xE4V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91`\0\x80Q` b\0\x1F\x98\x839\x81Q\x91R\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82\x80Tb\0\x02\xAC\x90b\0\x04wV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x02\xD0W`\0\x85Ub\0\x03\x1BV[\x82`\x1F\x10b\0\x02\xEBW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x03\x1BV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x03\x1BW\x91\x82\x01[\x82\x81\x11\x15b\0\x03\x1BW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x02\xFEV[Pb\0\x03)\x92\x91Pb\0\x03-V[P\x90V[[\x80\x82\x11\x15b\0\x03)W`\0\x81U`\x01\x01b\0\x03.V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x03wW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x03]V[\x83\x81\x11\x15b\0\x03\x87W`\0\x84\x84\x01R[PPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\xA5W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\xBEW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\xD6W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x03\xEBW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x04\0Wb\0\x04\0b\0\x03DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x04+Wb\0\x04+b\0\x03DV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15b\0\x04EW`\0\x80\xFD[b\0\x04X\x83` \x83\x01` \x88\x01b\0\x03ZV[\x80\x96PPPPPPb\0\x04n` \x84\x01b\0\x03\x8DV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x04\x8CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15b\0\x04\xAEWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x82Qb\0\x04\xC8\x81\x84` \x87\x01b\0\x03ZV[\x91\x90\x91\x01\x92\x91PPV[a\x1A\xB6\x80b\0\x04\xE2`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cv\xD8O\xFC\x11a\0\xB8W\x80c\xE4\xCA(\xB7\x11a\0|W\x80c\xE4\xCA(\xB7\x14a\x02TW\x80c\xE8`\xAC\xCB\x14a\x02gW\x80c\xED0\x1C\xA9\x14a\x02oW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x82W\x80c\xF6{\x18G\x14a\x02\x95W\x80c\xFC\xA5\x13\xA8\x14a\x02\xA8W`\0\x80\xFD[\x80cv\xD8O\xFC\x14a\x01\xF7W\x80c\x8D\xA5\xCB[\x14a\x02\nW\x80c\xA1VD\x06\x14a\x02\x1BW\x80c\xCADm\xD9\x14a\x02.W\x80c\xE4N\x9E\xD1\x14a\x02AW`\0\x80\xFD[\x80c]\xCCR\x8C\x11a\x01\nW\x80c]\xCCR\x8C\x14a\x01\xB1W\x80c^\xB8\x8D=\x14a\x01\xC4W\x80cc\x1A\xDF\xCA\x14a\x01\xCCW\x80cp|\xD7\x16\x14a\x01\xD4W\x80cqP\x18\xA6\x14a\x01\xDCW\x80ct\x94L\xEC\x14a\x01\xE4W`\0\x80\xFD[\x80c\x02k\x1D_\x14a\x01GW\x80c\x0Eg\x17\x8C\x14a\x01lW\x80c!\xF8\xA7!\x14a\x01tW\x80cS\x0ExO\x14a\x01\x87W\x80cV\x8E\xF4p\x14a\x01\x9CW[`\0\x80\xFD[a\x01Oa\x02\xB0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Oa\x02\xC7V[a\x01Oa\x01\x826`\x04a\x0F\xB7V[a\x02\xDAV[a\x01\x9Aa\x01\x956`\x04a\x0F\xE5V[a\x02\xF5V[\0[a\x01\xA4a\x03\xB0V[`@Qa\x01c\x91\x90a\x10eV[a\x01\x9Aa\x01\xBF6`\x04a\x10xV[a\x04BV[a\x01Oa\x04\xE7V[a\x01Oa\x05\nV[a\x01Oa\x05)V[a\x01\x9Aa\x05BV[a\x01\x9Aa\x01\xF26`\x04a\x0F\xE5V[a\x05\xB6V[a\x01\x9Aa\x02\x056`\x04a\x0F\xE5V[a\x06qV[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x01OV[a\x01\x9Aa\x02)6`\x04a\x0F\xE5V[a\x07 V[a\x01\x9Aa\x02<6`\x04a\x10xV[a\x07\xB3V[a\x01\x9Aa\x02O6`\x04a\x0F\xE5V[a\x08;V[a\x01\x9Aa\x02b6`\x04a\x0F\xE5V[a\x08\xEEV[a\x01Oa\t\x9BV[a\x01\x9Aa\x02}6`\x04a\x0F\xE5V[a\t\xB6V[a\x01\x9Aa\x02\x906`\x04a\x0F\xE5V[a\ngV[a\x01\x9Aa\x02\xA36`\x04a\x10\xBEV[a\x0BQV[a\x01Oa\x0B\x87V[`\0a\x02\xC2c\x14\x13\xD3\xD3`\xE2\x1Ba\x02\xDAV[\x90P\x90V[`\0a\x02\xC2h \xA1\xA6/\xA0\xA2&\xA4\xA7`\xB9\x1B[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`@Q\x80\x91\x03\x90\xFD[kPRICE_ORACLE`\xA0\x1B`\0\x90\x81R`\x02` R\x7Ft\x0Fq\x06f\xBDz\x12\xAFB\xDF\x981\x1ET\x1EG\xF7\xFD3\xD3\x82\xD1\x16\x02EzmT\x0C\xBDc\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7FV\xB5\xF8\r\x8C\xAC\x14yi\x8A\xA7\xD0\x16\x05\xFDa\x11\xE9\x0B\x15\xFCM+7t\x17\xF4`4\x87l\xBD\x91\x90\xA3PPV[```\x01\x80Ta\x03\xBF\x90a\x11\xA4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xEB\x90a\x11\xA4V[\x80\x15a\x048W\x80`\x1F\x10a\x04\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x048V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\x04\x8E\x84a\x0B\xA1V[\x90Pa\x04\x9A\x84\x84a\x0C>V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x85\x16\x91\x90\x84\x16\x90\x86\x90\x7F;\xBDE\xB5B\x9B8^?\xB3z\xD5\xCD\x1C\xD1CZ<\x8E\xC3!\x96\xC7\x93u\x976Z?\xD3\xE9\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPPV[`\0a\x02\xC2t\x14\x14\x92P\xD1W\xD3\xD4\x90P\xD3\x11W\xD4\xD1S\x95\x12S\x91S`Z\x1Ba\x02\xDAV[`\0a\x02\xC2p('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1Ba\x02\xDAV[`\0a\x02\xC2j \xA1\xA6/\xA6\xA0\xA7 \xA3\xA2\xA9`\xA9\x1Ba\x02\xDAV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[t\x14\x14\x92P\xD1W\xD3\xD4\x90P\xD3\x11W\xD4\xD1S\x95\x12S\x91S`Z\x1B`\0\x90\x81R`\x02` R\x7F\r,\x1B\xCE\xE5dG\xB4\xF4bH'/4 zX\n\\@\xF6f\xA3\x1FN/\xBBG\x0E\xA5:\xB8\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7FS&QN\xEC\xA9\x04\x94\xA1K\xED\xAB\xCF\xF8\x12\xA0\xE6\x83\x02\x9E\xE8]\x1E#\x82MD\xFD\x14\xCDj\xE7\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[h \xA1\xA6/\xA0\xA2&\xA4\xA7`\xB9\x1B`\0\x90\x81R`\x02` R\x7F\xFA\xB1g\xAD \t\xDC\xB8\x0E\xE3yp\x0B\xB4\xBD\x02\x9D\x97\xC1\x18\x1E\xD9\xD9abV2\xC8\xA6\xF0Q\xC6\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xE9\xCFS\x97\"d\xDC\x950O\xD4$E\x87E\x01\x9D\xDF\xCA\x0E7\xAE\x8Fp=tw,A\xAD\x11[\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0a\x07\\c\x14\x13\xD3\xD3`\xE2\x1Ba\x0B\xA1V[\x90Pa\x07oc\x14\x13\xD3\xD3`\xE2\x1B\x83a\x0C>V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x90\xAF\xFC\x16?\x1A-\xFE\xDC\xD3j\xA0.\xD9\x92\xEE\xEB\xA8\x10\n@\x14\xF0\xB4\xCD\xC2\x0E\xA2e\xA6f'`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x82\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x82\x17\x90\x93U\x92Q\x91\x16\x92\x83\x91\x86\x91\x7F\x9E\xF0\xE8\xC8\xE5'C\xBB8\xB8;\x17\xD9B\x91A\xD4\x94\xB8\x04\x1C\xA6\xD6\x16\xA6\xC7|\xEB\xAE\x9C\xD8\xB7\x91\xA4PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[l\" \xAA \xAF\xA8)'\xAB$\xA2\"\xA9`\x99\x1B`\0\x90\x81R`\x02` R\x7F\xCDyD`\x1A\xAA\\\xD7\xCC\xDA\xE1\xBE\xBE\xC6Y\xE9\x8Cj\xAC\x8F\x12Hk0\xE5\x9D\xB0\xD3\x96\x98\x05\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xC8S\x97L\xFB\xF8\x14\x87\xA1J#VY\x17\xBE\xE6?RxS\xBC\xB5\xFAT\xF2\xAE\x1C\xDF\x8A85m\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0a\t7p('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1Ba\x0B\xA1V[\x90Pa\tWp('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1B\x83a\x0C>V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x892\x89%i\xEB\xA5\x9C\x83\x82\xA0\x89\xD9\xB72\xD1\xF4\x92r\x87\x87u#Wa\xA2\xA6\xB00\x9C\xD4e`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0a\x02\xC2l\" \xAA \xAF\xA8)'\xAB$\xA2\"\xA9`\x99\x1Ba\x02\xDAV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[j \xA1\xA6/\xA6\xA0\xA7 \xA3\xA2\xA9`\xA9\x1B`\0\x90\x81R`\x02` R\x7F\x9E\xDE\xF2f\xEF5\xFD\x0Cn\x13\x1D\xF0\xF3\x1A3\x0F=\xD4\xC4\xD1\x9D\xD3\x1E\xD6\x15\xC2\x1D\0\\h\x11k\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xB3\x0E\xFA\x042{\xB8\xA57\xD6\x1C\xC1\xE5\xC4\x80\x954Z\xD1\x8E\xF7\xCC\x04\xE6\xBA\xCF}\xFBl\xAA\xF5\x07\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03\x1FV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[a\x0B\x84\x81a\x0E\x14V[PV[`\0a\x02\xC2kPRICE_ORACLE`\xA0\x1Ba\x02\xDAV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x0B\xC7WP`\0\x92\x91PPV[`\0\x81\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C0\x91\x90a\x11\xD9V[\x94\x93PPPPV[P\x91\x90PV[`\0\x82\x81R`\x02` R`@\x80\x82 T\x90Q0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x90\x81\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x9A\xCD\xBD`\xE3\x1B\x17\x90R\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\r\xA9W0`@Qa\x0C\xAF\x90a\x0F\x11V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x0C\xDBW=`\0\x80>=`\0\xFD[P`\0\x86\x81R`\x02` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x90Qc4}^%`\xE2\x1B\x81R\x91\x94P\x84\x93P\x90c\xD1\xF5x\x94\x90a\r1\x90\x87\x90\x85\x90`\x04\x01a\x11\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rKW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r_W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x86\x7FJFZ\x9B\xD8\x19\xD9f%c\xC1\xE1\x1A\xE9X\xF8\x10\x9EC~\x7FK\xF1\xC6\xEF\x0B\x9A{?5\xD4x`@Q`@Q\x80\x91\x03\x90\xA4a\x0E\rV[`@Qc'\x8FyC`\xE1\x1B\x81R\x83\x92P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cO\x1E\xF2\x86\x90a\r\xDA\x90\x87\x90\x85\x90`\x04\x01a\x11\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x08W=`\0\x80>=`\0\xFD[PPPP[PPPPPV[`\0`\x01\x80Ta\x0E#\x90a\x11\xA4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0EO\x90a\x11\xA4V[\x80\x15a\x0E\x9CW\x80`\x1F\x10a\x0EqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PP\x85Q\x93\x94Pa\x0E\xB8\x93`\x01\x93P` \x87\x01\x92P\x90Pa\x0F\x1EV[P\x81`@Qa\x0E\xC7\x91\x90a\x12\x1AV[`@Q\x80\x91\x03\x90 \x81`@Qa\x0E\xDD\x91\x90a\x12\x1AV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xE6\x85\xC8\xCD\xEC\xC6\x03\x0CE\x03\x0F\xD5Gx\x81,\xB8N\xD8\xE4F|8)D\x03\xD6\x8B\xA7\x86\x08#\x90`\0\x90\xA3PPV[a\x08J\x80a\x127\x839\x01\x90V[\x82\x80Ta\x0F*\x90a\x11\xA4V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x0FLW`\0\x85Ua\x0F\x92V[\x82`\x1F\x10a\x0FeW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x0F\x92V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x0F\x92W\x91\x82\x01[\x82\x81\x11\x15a\x0F\x92W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x0FwV[Pa\x0F\x9E\x92\x91Pa\x0F\xA2V[P\x90V[[\x80\x82\x11\x15a\x0F\x9EW`\0\x81U`\x01\x01a\x0F\xA3V[`\0` \x82\x84\x03\x12\x15a\x0F\xC9W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x84W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\xF7W`\0\x80\xFD[\x815a\x10\x02\x81a\x0F\xD0V[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x10$W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\x0CV[\x83\x81\x11\x15a\x103W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x10Q\x81` \x86\x01` \x86\x01a\x10\tV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x10\x02` \x83\x01\x84a\x109V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x8BW`\0\x80\xFD[\x825\x91P` \x83\x015a\x10\x9D\x81a\x0F\xD0V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xD0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xE8W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x10\xFCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\x0EWa\x11\x0Ea\x10\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x116Wa\x116a\x10\xA8V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x11OW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x11\xB8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x0C8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11\xEBW`\0\x80\xFD[\x81Qa\x10\x02\x81a\x0F\xD0V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0C0\x90\x83\x01\x84a\x109V[`\0\x82Qa\x12,\x81\x84` \x87\x01a\x10\tV[\x91\x90\x91\x01\x92\x91PPV\xFE`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08J8\x03\x80a\x08J\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x07\x9Ca\0\xAE`\09`\0\x81\x81a\x01\x13\x01R\x81\x81a\x01X\x01R\x81\x81a\x02\x11\x01R\x81\x81a\x03Q\x01R\x81\x81a\x03z\x01Ra\x04\x9E\x01Ra\x07\x9C`\0\xF3\xFE`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80c6Y\xCF\xE6\x14a\0TW\x80cO\x1E\xF2\x86\x14a\0tW\x80c\\`\xDA\x1B\x14a\0\x87W\x80c\xD1\xF5x\x94\x14a\0\xB8W\x80c\xF8Q\xA4@\x14a\0\xCBW[a\0Ra\0\xE0V[\0[4\x80\x15a\0`W`\0\x80\xFD[Pa\0Ra\0o6`\x04a\x05CV[a\x01\x08V[a\0Ra\0\x826`\x04a\x05eV[a\x01MV[4\x80\x15a\0\x93W`\0\x80\xFD[Pa\0\x9Ca\x02\x04V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Ra\0\xC66`\x04a\x05\xFEV[a\x02VV[4\x80\x15a\0\xD7W`\0\x80\xFD[Pa\0\x9Ca\x03DV[a\0\xE8a\x03\x9CV[a\x01\x06a\x01\x01`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[a\x03\xA4V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01EWa\x01B\x81a\x03\xC8V[PV[a\x01Ba\0\xE0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01\xF7Wa\x01\x87\x83a\x03\xC8V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x01\xA3\x92\x91\x90a\x06\xC0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xE3V[``\x91P[PP\x90P\x80a\x01\xF1W`\0\x80\xFD[PPPPV[a\x01\xFFa\0\xE0V[PPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x02KWP`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[a\x02Sa\0\xE0V[\x90V[`\0a\x02n`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02\x81W`\0\x80\xFD[a\x02\xAC`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x06\xD0V[`\0\x80Q` a\x07G\x839\x81Q\x91R\x14a\x02\xC8Wa\x02\xC8a\x06\xF5V[a\x02\xD1\x82a\x04\x08V[\x80Q\x15a\x03@W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x02\xF2\x91\x90a\x07\x0BV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03-W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x032V[``\x91P[PP\x90P\x80a\x01\xFFW`\0\x80\xFD[PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x02KWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x01\x06a\x04\x93V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x03\xC3W=`\0\xF3[=`\0\xFD[a\x03\xD1\x81a\x04\x08V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[\x80;a\x04\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FCannot set a proxy implementatio`D\x82\x01R\x7Fn to a non-contract address\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a\x07G\x839\x81Q\x91RUV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FCannot call fallback function fr`D\x82\x01Rq7\xB6\x90:42\x90897\xBC<\x900\xB26\xB4\xB7`q\x1B`d\x82\x01R`\x84\x01a\x04xV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05>W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05UW`\0\x80\xFD[a\x05^\x82a\x05'V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x05zW`\0\x80\xFD[a\x05\x83\x84a\x05'V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xA0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xB4W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xC3W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x05\xD5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x11W`\0\x80\xFD[a\x06\x1A\x83a\x05'V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x067W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06KW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06]Wa\x06]a\x05\xE8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x06\x85Wa\x06\x85a\x05\xE8V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x06\x9EW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82\x82\x10\x15a\x06\xF0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x07,W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x07\x12V[\x81\x81\x11\x15a\x07;W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xA7V\x11\xEC\xC8\xE7\x8B\xF05W\xF8\xF7\x1Bf28\xE9\xEC\x81\xFC\xABgx7\t\x80\xDB\xCE\xD1\xA8\xBD\x07dsolcC\0\x08\n\x003\xA2dipfsX\"\x12 q\x19\xDEyQ\x0E\xBF\"\xBD\x10\xEA0\\\xA7\0\x04\x1EX\x94\x10\xFAO@*\xB5\xC9\t2\x06~\xA9\xA3dsolcC\0\x08\n\x003\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0";
    /// The bytecode of the contract.
    pub static POOLADDRESSESPROVIDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01BW`\x005`\xE0\x1C\x80cv\xD8O\xFC\x11a\0\xB8W\x80c\xE4\xCA(\xB7\x11a\0|W\x80c\xE4\xCA(\xB7\x14a\x02TW\x80c\xE8`\xAC\xCB\x14a\x02gW\x80c\xED0\x1C\xA9\x14a\x02oW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x82W\x80c\xF6{\x18G\x14a\x02\x95W\x80c\xFC\xA5\x13\xA8\x14a\x02\xA8W`\0\x80\xFD[\x80cv\xD8O\xFC\x14a\x01\xF7W\x80c\x8D\xA5\xCB[\x14a\x02\nW\x80c\xA1VD\x06\x14a\x02\x1BW\x80c\xCADm\xD9\x14a\x02.W\x80c\xE4N\x9E\xD1\x14a\x02AW`\0\x80\xFD[\x80c]\xCCR\x8C\x11a\x01\nW\x80c]\xCCR\x8C\x14a\x01\xB1W\x80c^\xB8\x8D=\x14a\x01\xC4W\x80cc\x1A\xDF\xCA\x14a\x01\xCCW\x80cp|\xD7\x16\x14a\x01\xD4W\x80cqP\x18\xA6\x14a\x01\xDCW\x80ct\x94L\xEC\x14a\x01\xE4W`\0\x80\xFD[\x80c\x02k\x1D_\x14a\x01GW\x80c\x0Eg\x17\x8C\x14a\x01lW\x80c!\xF8\xA7!\x14a\x01tW\x80cS\x0ExO\x14a\x01\x87W\x80cV\x8E\xF4p\x14a\x01\x9CW[`\0\x80\xFD[a\x01Oa\x02\xB0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Oa\x02\xC7V[a\x01Oa\x01\x826`\x04a\x0F\xB7V[a\x02\xDAV[a\x01\x9Aa\x01\x956`\x04a\x0F\xE5V[a\x02\xF5V[\0[a\x01\xA4a\x03\xB0V[`@Qa\x01c\x91\x90a\x10eV[a\x01\x9Aa\x01\xBF6`\x04a\x10xV[a\x04BV[a\x01Oa\x04\xE7V[a\x01Oa\x05\nV[a\x01Oa\x05)V[a\x01\x9Aa\x05BV[a\x01\x9Aa\x01\xF26`\x04a\x0F\xE5V[a\x05\xB6V[a\x01\x9Aa\x02\x056`\x04a\x0F\xE5V[a\x06qV[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x01OV[a\x01\x9Aa\x02)6`\x04a\x0F\xE5V[a\x07 V[a\x01\x9Aa\x02<6`\x04a\x10xV[a\x07\xB3V[a\x01\x9Aa\x02O6`\x04a\x0F\xE5V[a\x08;V[a\x01\x9Aa\x02b6`\x04a\x0F\xE5V[a\x08\xEEV[a\x01Oa\t\x9BV[a\x01\x9Aa\x02}6`\x04a\x0F\xE5V[a\t\xB6V[a\x01\x9Aa\x02\x906`\x04a\x0F\xE5V[a\ngV[a\x01\x9Aa\x02\xA36`\x04a\x10\xBEV[a\x0BQV[a\x01Oa\x0B\x87V[`\0a\x02\xC2c\x14\x13\xD3\xD3`\xE2\x1Ba\x02\xDAV[\x90P\x90V[`\0a\x02\xC2h \xA1\xA6/\xA0\xA2&\xA4\xA7`\xB9\x1B[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`@Q\x80\x91\x03\x90\xFD[kPRICE_ORACLE`\xA0\x1B`\0\x90\x81R`\x02` R\x7Ft\x0Fq\x06f\xBDz\x12\xAFB\xDF\x981\x1ET\x1EG\xF7\xFD3\xD3\x82\xD1\x16\x02EzmT\x0C\xBDc\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7FV\xB5\xF8\r\x8C\xAC\x14yi\x8A\xA7\xD0\x16\x05\xFDa\x11\xE9\x0B\x15\xFCM+7t\x17\xF4`4\x87l\xBD\x91\x90\xA3PPV[```\x01\x80Ta\x03\xBF\x90a\x11\xA4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xEB\x90a\x11\xA4V[\x80\x15a\x048W\x80`\x1F\x10a\x04\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x048V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\x04\x8E\x84a\x0B\xA1V[\x90Pa\x04\x9A\x84\x84a\x0C>V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x85\x16\x91\x90\x84\x16\x90\x86\x90\x7F;\xBDE\xB5B\x9B8^?\xB3z\xD5\xCD\x1C\xD1CZ<\x8E\xC3!\x96\xC7\x93u\x976Z?\xD3\xE9\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPPV[`\0a\x02\xC2t\x14\x14\x92P\xD1W\xD3\xD4\x90P\xD3\x11W\xD4\xD1S\x95\x12S\x91S`Z\x1Ba\x02\xDAV[`\0a\x02\xC2p('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1Ba\x02\xDAV[`\0a\x02\xC2j \xA1\xA6/\xA6\xA0\xA7 \xA3\xA2\xA9`\xA9\x1Ba\x02\xDAV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[t\x14\x14\x92P\xD1W\xD3\xD4\x90P\xD3\x11W\xD4\xD1S\x95\x12S\x91S`Z\x1B`\0\x90\x81R`\x02` R\x7F\r,\x1B\xCE\xE5dG\xB4\xF4bH'/4 zX\n\\@\xF6f\xA3\x1FN/\xBBG\x0E\xA5:\xB8\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7FS&QN\xEC\xA9\x04\x94\xA1K\xED\xAB\xCF\xF8\x12\xA0\xE6\x83\x02\x9E\xE8]\x1E#\x82MD\xFD\x14\xCDj\xE7\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[h \xA1\xA6/\xA0\xA2&\xA4\xA7`\xB9\x1B`\0\x90\x81R`\x02` R\x7F\xFA\xB1g\xAD \t\xDC\xB8\x0E\xE3yp\x0B\xB4\xBD\x02\x9D\x97\xC1\x18\x1E\xD9\xD9abV2\xC8\xA6\xF0Q\xC6\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xE9\xCFS\x97\"d\xDC\x950O\xD4$E\x87E\x01\x9D\xDF\xCA\x0E7\xAE\x8Fp=tw,A\xAD\x11[\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0a\x07\\c\x14\x13\xD3\xD3`\xE2\x1Ba\x0B\xA1V[\x90Pa\x07oc\x14\x13\xD3\xD3`\xE2\x1B\x83a\x0C>V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x90\xAF\xFC\x16?\x1A-\xFE\xDC\xD3j\xA0.\xD9\x92\xEE\xEB\xA8\x10\n@\x14\xF0\xB4\xCD\xC2\x0E\xA2e\xA6f'`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0\x82\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x82\x17\x90\x93U\x92Q\x91\x16\x92\x83\x91\x86\x91\x7F\x9E\xF0\xE8\xC8\xE5'C\xBB8\xB8;\x17\xD9B\x91A\xD4\x94\xB8\x04\x1C\xA6\xD6\x16\xA6\xC7|\xEB\xAE\x9C\xD8\xB7\x91\xA4PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[l\" \xAA \xAF\xA8)'\xAB$\xA2\"\xA9`\x99\x1B`\0\x90\x81R`\x02` R\x7F\xCDyD`\x1A\xAA\\\xD7\xCC\xDA\xE1\xBE\xBE\xC6Y\xE9\x8Cj\xAC\x8F\x12Hk0\xE5\x9D\xB0\xD3\x96\x98\x05\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xC8S\x97L\xFB\xF8\x14\x87\xA1J#VY\x17\xBE\xE6?RxS\xBC\xB5\xFAT\xF2\xAE\x1C\xDF\x8A85m\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\0a\t7p('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1Ba\x0B\xA1V[\x90Pa\tWp('\xA7\xA6/\xA1\xA7\xA7#$\xA3\xAA\xA9 \xAA'\xA9`y\x1B\x83a\x0C>V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x892\x89%i\xEB\xA5\x9C\x83\x82\xA0\x89\xD9\xB72\xD1\xF4\x92r\x87\x87u#Wa\xA2\xA6\xB00\x9C\xD4e`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0a\x02\xC2l\" \xAA \xAF\xA8)'\xAB$\xA2\"\xA9`\x99\x1Ba\x02\xDAV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[j \xA1\xA6/\xA6\xA0\xA7 \xA3\xA2\xA9`\xA9\x1B`\0\x90\x81R`\x02` R\x7F\x9E\xDE\xF2f\xEF5\xFD\x0Cn\x13\x1D\xF0\xF3\x1A3\x0F=\xD4\xC4\xD1\x9D\xD3\x1E\xD6\x15\xC2\x1D\0\\h\x11k\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x92\x83\x91\x7F\xB3\x0E\xFA\x042{\xB8\xA57\xD6\x1C\xC1\xE5\xC4\x80\x954Z\xD1\x8E\xF7\xCC\x04\xE6\xBA\xCF}\xFBl\xAA\xF5\x07\x91\x90\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03\x1FV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x1F\x90a\x11oV[a\x0B\x84\x81a\x0E\x14V[PV[`\0a\x02\xC2kPRICE_ORACLE`\xA0\x1Ba\x02\xDAV[`\0\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x0B\xC7WP`\0\x92\x91PPV[`\0\x81\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\\`\xDA\x1B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C0\x91\x90a\x11\xD9V[\x94\x93PPPPV[P\x91\x90PV[`\0\x82\x81R`\x02` R`@\x80\x82 T\x90Q0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x90\x81\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x18\x9A\xCD\xBD`\xE3\x1B\x17\x90R\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\r\xA9W0`@Qa\x0C\xAF\x90a\x0F\x11V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x0C\xDBW=`\0\x80>=`\0\xFD[P`\0\x86\x81R`\x02` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U\x90Qc4}^%`\xE2\x1B\x81R\x91\x94P\x84\x93P\x90c\xD1\xF5x\x94\x90a\r1\x90\x87\x90\x85\x90`\x04\x01a\x11\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rKW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r_W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x86\x7FJFZ\x9B\xD8\x19\xD9f%c\xC1\xE1\x1A\xE9X\xF8\x10\x9EC~\x7FK\xF1\xC6\xEF\x0B\x9A{?5\xD4x`@Q`@Q\x80\x91\x03\x90\xA4a\x0E\rV[`@Qc'\x8FyC`\xE1\x1B\x81R\x83\x92P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cO\x1E\xF2\x86\x90a\r\xDA\x90\x87\x90\x85\x90`\x04\x01a\x11\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xF4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x08W=`\0\x80>=`\0\xFD[PPPP[PPPPPV[`\0`\x01\x80Ta\x0E#\x90a\x11\xA4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0EO\x90a\x11\xA4V[\x80\x15a\x0E\x9CW\x80`\x1F\x10a\x0EqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PP\x85Q\x93\x94Pa\x0E\xB8\x93`\x01\x93P` \x87\x01\x92P\x90Pa\x0F\x1EV[P\x81`@Qa\x0E\xC7\x91\x90a\x12\x1AV[`@Q\x80\x91\x03\x90 \x81`@Qa\x0E\xDD\x91\x90a\x12\x1AV[`@Q\x90\x81\x90\x03\x81 \x90\x7F\xE6\x85\xC8\xCD\xEC\xC6\x03\x0CE\x03\x0F\xD5Gx\x81,\xB8N\xD8\xE4F|8)D\x03\xD6\x8B\xA7\x86\x08#\x90`\0\x90\xA3PPV[a\x08J\x80a\x127\x839\x01\x90V[\x82\x80Ta\x0F*\x90a\x11\xA4V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x0FLW`\0\x85Ua\x0F\x92V[\x82`\x1F\x10a\x0FeW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x0F\x92V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x0F\x92W\x91\x82\x01[\x82\x81\x11\x15a\x0F\x92W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x0FwV[Pa\x0F\x9E\x92\x91Pa\x0F\xA2V[P\x90V[[\x80\x82\x11\x15a\x0F\x9EW`\0\x81U`\x01\x01a\x0F\xA3V[`\0` \x82\x84\x03\x12\x15a\x0F\xC9W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x84W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\xF7W`\0\x80\xFD[\x815a\x10\x02\x81a\x0F\xD0V[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x10$W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\x0CV[\x83\x81\x11\x15a\x103W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x10Q\x81` \x86\x01` \x86\x01a\x10\tV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x10\x02` \x83\x01\x84a\x109V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x8BW`\0\x80\xFD[\x825\x91P` \x83\x015a\x10\x9D\x81a\x0F\xD0V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xD0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xE8W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x10\xFCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\x0EWa\x11\x0Ea\x10\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x116Wa\x116a\x10\xA8V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x11OW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x11\xB8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x0C8WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11\xEBW`\0\x80\xFD[\x81Qa\x10\x02\x81a\x0F\xD0V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0C0\x90\x83\x01\x84a\x109V[`\0\x82Qa\x12,\x81\x84` \x87\x01a\x10\tV[\x91\x90\x91\x01\x92\x91PPV\xFE`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08J8\x03\x80a\x08J\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x07\x9Ca\0\xAE`\09`\0\x81\x81a\x01\x13\x01R\x81\x81a\x01X\x01R\x81\x81a\x02\x11\x01R\x81\x81a\x03Q\x01R\x81\x81a\x03z\x01Ra\x04\x9E\x01Ra\x07\x9C`\0\xF3\xFE`\x80`@R`\x046\x10a\0JW`\x005`\xE0\x1C\x80c6Y\xCF\xE6\x14a\0TW\x80cO\x1E\xF2\x86\x14a\0tW\x80c\\`\xDA\x1B\x14a\0\x87W\x80c\xD1\xF5x\x94\x14a\0\xB8W\x80c\xF8Q\xA4@\x14a\0\xCBW[a\0Ra\0\xE0V[\0[4\x80\x15a\0`W`\0\x80\xFD[Pa\0Ra\0o6`\x04a\x05CV[a\x01\x08V[a\0Ra\0\x826`\x04a\x05eV[a\x01MV[4\x80\x15a\0\x93W`\0\x80\xFD[Pa\0\x9Ca\x02\x04V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Ra\0\xC66`\x04a\x05\xFEV[a\x02VV[4\x80\x15a\0\xD7W`\0\x80\xFD[Pa\0\x9Ca\x03DV[a\0\xE8a\x03\x9CV[a\x01\x06a\x01\x01`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[a\x03\xA4V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01EWa\x01B\x81a\x03\xC8V[PV[a\x01Ba\0\xE0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01\xF7Wa\x01\x87\x83a\x03\xC8V[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x83\x83`@Qa\x01\xA3\x92\x91\x90a\x06\xC0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xDEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xE3V[``\x91P[PP\x90P\x80a\x01\xF1W`\0\x80\xFD[PPPPV[a\x01\xFFa\0\xE0V[PPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x02KWP`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[a\x02Sa\0\xE0V[\x90V[`\0a\x02n`\0\x80Q` a\x07G\x839\x81Q\x91RT\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02\x81W`\0\x80\xFD[a\x02\xAC`\x01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBDa\x06\xD0V[`\0\x80Q` a\x07G\x839\x81Q\x91R\x14a\x02\xC8Wa\x02\xC8a\x06\xF5V[a\x02\xD1\x82a\x04\x08V[\x80Q\x15a\x03@W`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Qa\x02\xF2\x91\x90a\x07\x0BV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x03-W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x032V[``\x91P[PP\x90P\x80a\x01\xFFW`\0\x80\xFD[PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x02KWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x01\x06a\x04\x93V[6`\0\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x03\xC3W=`\0\xF3[=`\0\xFD[a\x03\xD1\x81a\x04\x08V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2PV[\x80;a\x04\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FCannot set a proxy implementatio`D\x82\x01R\x7Fn to a non-contract address\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80Q` a\x07G\x839\x81Q\x91RUV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x01\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FCannot call fallback function fr`D\x82\x01Rq7\xB6\x90:42\x90897\xBC<\x900\xB26\xB4\xB7`q\x1B`d\x82\x01R`\x84\x01a\x04xV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05>W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05UW`\0\x80\xFD[a\x05^\x82a\x05'V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x05zW`\0\x80\xFD[a\x05\x83\x84a\x05'V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xA0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x05\xB4W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xC3W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x05\xD5W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x11W`\0\x80\xFD[a\x06\x1A\x83a\x05'V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x067W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06KW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06]Wa\x06]a\x05\xE8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x06\x85Wa\x06\x85a\x05\xE8V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x06\x9EW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0\x82\x82\x10\x15a\x06\xF0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x07,W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x07\x12V[\x81\x81\x11\x15a\x07;W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xA7V\x11\xEC\xC8\xE7\x8B\xF05W\xF8\xF7\x1Bf28\xE9\xEC\x81\xFC\xABgx7\t\x80\xDB\xCE\xD1\xA8\xBD\x07dsolcC\0\x08\n\x003\xA2dipfsX\"\x12 q\x19\xDEyQ\x0E\xBF\"\xBD\x10\xEA0\\\xA7\0\x04\x1EX\x94\x10\xFAO@*\xB5\xC9\t2\x06~\xA9\xA3dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static POOLADDRESSESPROVIDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PoolAddressesProvider<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoolAddressesProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoolAddressesProvider<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoolAddressesProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoolAddressesProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PoolAddressesProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoolAddressesProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POOLADDRESSESPROVIDER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                POOLADDRESSESPROVIDER_ABI.clone(),
                POOLADDRESSESPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
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
            PoolAddressesProviderEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PoolAddressesProvider<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum PoolAddressesProviderEvents {
        AcladminUpdatedFilter(AcladminUpdatedFilter),
        AclmanagerUpdatedFilter(AclmanagerUpdatedFilter),
        AddressSetFilter(AddressSetFilter),
        AddressSetAsProxyFilter(AddressSetAsProxyFilter),
        MarketIdSetFilter(MarketIdSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PoolConfiguratorUpdatedFilter(PoolConfiguratorUpdatedFilter),
        PoolDataProviderUpdatedFilter(PoolDataProviderUpdatedFilter),
        PoolUpdatedFilter(PoolUpdatedFilter),
        PriceOracleSentinelUpdatedFilter(PriceOracleSentinelUpdatedFilter),
        PriceOracleUpdatedFilter(PriceOracleUpdatedFilter),
        ProxyCreatedFilter(ProxyCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PoolAddressesProviderEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AcladminUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AcladminUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AclmanagerUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AclmanagerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = AddressSetAsProxyFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::AddressSetAsProxyFilter(decoded));
            }
            if let Ok(decoded) = MarketIdSetFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::MarketIdSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PoolConfiguratorUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderEvents::PoolConfiguratorUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PoolDataProviderUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderEvents::PoolDataProviderUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = PoolUpdatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::PoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PriceOracleSentinelUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderEvents::PriceOracleSentinelUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderEvents::PriceOracleUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = ProxyCreatedFilter::decode_log(log) {
                return Ok(PoolAddressesProviderEvents::ProxyCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PoolAddressesProviderEvents {
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
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<AcladminUpdatedFilter> for PoolAddressesProviderEvents {
        fn from(value: AcladminUpdatedFilter) -> Self {
            Self::AcladminUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AclmanagerUpdatedFilter> for PoolAddressesProviderEvents {
        fn from(value: AclmanagerUpdatedFilter) -> Self {
            Self::AclmanagerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AddressSetFilter> for PoolAddressesProviderEvents {
        fn from(value: AddressSetFilter) -> Self {
            Self::AddressSetFilter(value)
        }
    }
    impl ::core::convert::From<AddressSetAsProxyFilter> for PoolAddressesProviderEvents {
        fn from(value: AddressSetAsProxyFilter) -> Self {
            Self::AddressSetAsProxyFilter(value)
        }
    }
    impl ::core::convert::From<MarketIdSetFilter> for PoolAddressesProviderEvents {
        fn from(value: MarketIdSetFilter) -> Self {
            Self::MarketIdSetFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for PoolAddressesProviderEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PoolConfiguratorUpdatedFilter>
    for PoolAddressesProviderEvents {
        fn from(value: PoolConfiguratorUpdatedFilter) -> Self {
            Self::PoolConfiguratorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PoolDataProviderUpdatedFilter>
    for PoolAddressesProviderEvents {
        fn from(value: PoolDataProviderUpdatedFilter) -> Self {
            Self::PoolDataProviderUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PoolUpdatedFilter> for PoolAddressesProviderEvents {
        fn from(value: PoolUpdatedFilter) -> Self {
            Self::PoolUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PriceOracleSentinelUpdatedFilter>
    for PoolAddressesProviderEvents {
        fn from(value: PriceOracleSentinelUpdatedFilter) -> Self {
            Self::PriceOracleSentinelUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PriceOracleUpdatedFilter>
    for PoolAddressesProviderEvents {
        fn from(value: PriceOracleUpdatedFilter) -> Self {
            Self::PriceOracleUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ProxyCreatedFilter> for PoolAddressesProviderEvents {
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
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolAddressesProviderCalls {
        GetACLAdmin(GetACLAdminCall),
        GetACLManager(GetACLManagerCall),
        GetAddress(GetAddressCall),
        GetMarketId(GetMarketIdCall),
        GetPool(GetPoolCall),
        GetPoolConfigurator(GetPoolConfiguratorCall),
        GetPoolDataProvider(GetPoolDataProviderCall),
        GetPriceOracle(GetPriceOracleCall),
        GetPriceOracleSentinel(GetPriceOracleSentinelCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
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
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolAddressesProviderCalls {
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
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
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
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolAddressesProviderCalls {
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
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
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
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoolAddressesProviderCalls {
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
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetACLAdminCall> for PoolAddressesProviderCalls {
        fn from(value: GetACLAdminCall) -> Self {
            Self::GetACLAdmin(value)
        }
    }
    impl ::core::convert::From<GetACLManagerCall> for PoolAddressesProviderCalls {
        fn from(value: GetACLManagerCall) -> Self {
            Self::GetACLManager(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for PoolAddressesProviderCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetMarketIdCall> for PoolAddressesProviderCalls {
        fn from(value: GetMarketIdCall) -> Self {
            Self::GetMarketId(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for PoolAddressesProviderCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetPoolConfiguratorCall> for PoolAddressesProviderCalls {
        fn from(value: GetPoolConfiguratorCall) -> Self {
            Self::GetPoolConfigurator(value)
        }
    }
    impl ::core::convert::From<GetPoolDataProviderCall> for PoolAddressesProviderCalls {
        fn from(value: GetPoolDataProviderCall) -> Self {
            Self::GetPoolDataProvider(value)
        }
    }
    impl ::core::convert::From<GetPriceOracleCall> for PoolAddressesProviderCalls {
        fn from(value: GetPriceOracleCall) -> Self {
            Self::GetPriceOracle(value)
        }
    }
    impl ::core::convert::From<GetPriceOracleSentinelCall>
    for PoolAddressesProviderCalls {
        fn from(value: GetPriceOracleSentinelCall) -> Self {
            Self::GetPriceOracleSentinel(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PoolAddressesProviderCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PoolAddressesProviderCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetACLAdminCall> for PoolAddressesProviderCalls {
        fn from(value: SetACLAdminCall) -> Self {
            Self::SetACLAdmin(value)
        }
    }
    impl ::core::convert::From<SetACLManagerCall> for PoolAddressesProviderCalls {
        fn from(value: SetACLManagerCall) -> Self {
            Self::SetACLManager(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for PoolAddressesProviderCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<SetAddressAsProxyCall> for PoolAddressesProviderCalls {
        fn from(value: SetAddressAsProxyCall) -> Self {
            Self::SetAddressAsProxy(value)
        }
    }
    impl ::core::convert::From<SetMarketIdCall> for PoolAddressesProviderCalls {
        fn from(value: SetMarketIdCall) -> Self {
            Self::SetMarketId(value)
        }
    }
    impl ::core::convert::From<SetPoolConfiguratorImplCall>
    for PoolAddressesProviderCalls {
        fn from(value: SetPoolConfiguratorImplCall) -> Self {
            Self::SetPoolConfiguratorImpl(value)
        }
    }
    impl ::core::convert::From<SetPoolDataProviderCall> for PoolAddressesProviderCalls {
        fn from(value: SetPoolDataProviderCall) -> Self {
            Self::SetPoolDataProvider(value)
        }
    }
    impl ::core::convert::From<SetPoolImplCall> for PoolAddressesProviderCalls {
        fn from(value: SetPoolImplCall) -> Self {
            Self::SetPoolImpl(value)
        }
    }
    impl ::core::convert::From<SetPriceOracleCall> for PoolAddressesProviderCalls {
        fn from(value: SetPriceOracleCall) -> Self {
            Self::SetPriceOracle(value)
        }
    }
    impl ::core::convert::From<SetPriceOracleSentinelCall>
    for PoolAddressesProviderCalls {
        fn from(value: SetPriceOracleSentinelCall) -> Self {
            Self::SetPriceOracleSentinel(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PoolAddressesProviderCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
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
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
