pub use l2_pool::*;
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
pub mod l2_pool {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("provider"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IPoolAddressesProvider",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IPoolAddressesProvider",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BRIDGE_PROTOCOL_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BRIDGE_PROTOCOL_FEE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASHLOAN_PREMIUM_TOTAL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FLASHLOAN_PREMIUM_TOTAL",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASHLOAN_PREMIUM_TO_PROTOCOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FLASHLOAN_PREMIUM_TO_PROTOCOL",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_NUMBER_RESERVES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_NUMBER_RESERVES",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL_REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL_REVISION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("backUnbacked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("backUnbacked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("configureEModeCategory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "configureEModeCategory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("category"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.EModeCategory",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("dropReserve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dropReserve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("finalizeTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("finalizeTransfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balanceFromBefore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balanceToBefore"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateModes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("flashLoanSimple"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoanSimple"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiverAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("getConfiguration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getConfiguration"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.ReserveConfigurationMap",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEModeCategoryData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getEModeCategoryData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.EModeCategory",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveAddressById"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveAddressById",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("getReserveData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserveData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.ReserveData",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveNormalizedIncome"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveNormalizedIncome",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveNormalizedVariableDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveNormalizedVariableDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReservesList"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserAccountData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserAccountData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalCollateralBase",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebtBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "availableBorrowsBase",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentLiquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("healthFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserConfiguration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUserConfiguration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.UserConfigurationMap",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserEMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserEMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initReserve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initReserve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stableDebtAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableDebtAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "interestRateStrategyAddress",
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IPoolAddressesProvider",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("liquidationCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidationCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidationCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("mintToTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintToTreasury"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
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
                    ::std::borrow::ToOwned::to_owned("mintUnbacked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintUnbacked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("rebalanceStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rebalanceStableBorrowRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "rebalanceStableBorrowRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
                    ::std::borrow::ToOwned::to_owned("repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayWithATokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithATokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithATokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repayWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitV"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitS"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rescueTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rescueTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("resetIsolationModeTotalDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "resetIsolationModeTotalDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("setConfiguration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setConfiguration"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("configuration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.ReserveConfigurationMap",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "setReserveInterestRateStrategyAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setReserveInterestRateStrategyAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "rateStrategyAddress",
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
                    ::std::borrow::ToOwned::to_owned("setUserEMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUserEMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("categoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("setUserUseReserveAsCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUserUseReserveAsCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUserUseReserveAsCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("useAsCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("supplyWithPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitV"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitR"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitS"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyWithPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("swapBorrowRateMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapBorrowRateMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapBorrowRateMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("updateBridgeProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateBridgeProtocolFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("updateFlashloanPremiums"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateFlashloanPremiums",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "flashLoanPremiumTotal",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "flashLoanPremiumToProtocol",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
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
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("args"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BackUnbacked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BackUnbacked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("backer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("premium"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IsolationModeTotalDebtUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IsolationModeTotalDebtUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidationCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LiquidationCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidatedCollateralAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MintUnbacked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MintUnbacked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MintedToTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MintedToTreasury"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountMinted"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceStableBorrowRate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("repayer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("useATokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveDataUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveDataUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("stableBorrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableBorrowRate",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableBorrowIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveUsedAsCollateralDisabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveUsedAsCollateralDisabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveUsedAsCollateralEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveUsedAsCollateralEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Supply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Supply"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("referralCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapBorrowRateMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapBorrowRateMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("interestRateMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UserEModeSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UserEModeSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("categoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserve"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
    pub static L2POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct L2Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for L2Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for L2Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for L2Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for L2Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(L2Pool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> L2Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    L2POOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function
        pub fn addresses_provider(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BRIDGE_PROTOCOL_FEE` (0x272d9072) function
        pub fn bridge_protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([39, 45, 144, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASHLOAN_PREMIUM_TOTAL` (0x074b2e43) function
        pub fn flashloan_premium_total(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([7, 75, 46, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASHLOAN_PREMIUM_TO_PROTOCOL` (0x6a99c036) function
        pub fn flashloan_premium_to_protocol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([106, 153, 192, 54], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_NUMBER_RESERVES` (0xf8119d51) function
        pub fn max_number_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([248, 17, 157, 81], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` (0xe82fec2f) function
        pub fn max_stable_rate_borrow_size_percent(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 47, 236, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_REVISION` (0x0148170e) function
        pub fn pool_revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 72, 23, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `backUnbacked` (0xd65dc7a1) function
        pub fn back_unbacked(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([214, 93, 199, 161], (asset, amount, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrow` (0xa415bcad) function
        pub fn borrow_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
            referral_code: u16,
            on_behalf_of: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 21, 188, 173],
                    (asset, amount, interest_rate_mode, referral_code, on_behalf_of),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrow` (0xd5eed868) function
        pub fn borrow(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 238, 216, 104], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureEModeCategory` (0xd579ea7d) function
        pub fn configure_e_mode_category(
            &self,
            id: u8,
            category: EmodeCategory,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 121, 234, 125], (id, category))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xe8eda9df) function
        pub fn deposit(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 237, 169, 223],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropReserve` (0x63c9b860) function
        pub fn drop_reserve(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 201, 184, 96], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeTransfer` (0xd5ed3933) function
        pub fn finalize_transfer(
            &self,
            asset: ::ethers::core::types::Address,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            balance_from_before: ::ethers::core::types::U256,
            balance_to_before: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 237, 57, 51],
                    (asset, from, to, amount, balance_from_before, balance_to_before),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0xab9c4b5d) function
        pub fn flash_loan(
            &self,
            receiver_address: ::ethers::core::types::Address,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            interest_rate_modes: ::std::vec::Vec<::ethers::core::types::U256>,
            on_behalf_of: ::ethers::core::types::Address,
            params: ::ethers::core::types::Bytes,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 156, 75, 93],
                    (
                        receiver_address,
                        assets,
                        amounts,
                        interest_rate_modes,
                        on_behalf_of,
                        params,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoanSimple` (0x42b0b77c) function
        pub fn flash_loan_simple(
            &self,
            receiver_address: ::ethers::core::types::Address,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            params: ::ethers::core::types::Bytes,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 176, 183, 124],
                    (receiver_address, asset, amount, params, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfiguration` (0xc44b11f7) function
        pub fn get_configuration(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ReserveConfigurationMap> {
            self.0
                .method_hash([196, 75, 17, 247], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEModeCategoryData` (0x6c6f6ae1) function
        pub fn get_e_mode_category_data(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, EmodeCategory> {
            self.0
                .method_hash([108, 111, 106, 225], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveAddressById` (0x52751797) function
        pub fn get_reserve_address_by_id(
            &self,
            id: u16,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([82, 117, 23, 151], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveData` (0x35ea6a75) function
        pub fn get_reserve_data(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ReserveData> {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveNormalizedIncome` (0xd15e0053) function
        pub fn get_reserve_normalized_income(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([209, 94, 0, 83], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveNormalizedVariableDebt` (0x386497fd) function
        pub fn get_reserve_normalized_variable_debt(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([56, 100, 151, 253], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesList` (0xd1946dbc) function
        pub fn get_reserves_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([209, 148, 109, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserAccountData` (0xbf92857c) function
        pub fn get_user_account_data(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 146, 133, 124], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserConfiguration` (0x4417a583) function
        pub fn get_user_configuration(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, UserConfigurationMap> {
            self.0
                .method_hash([68, 23, 165, 131], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserEMode` (0xeddf1b79) function
        pub fn get_user_e_mode(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 223, 27, 121], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initReserve` (0x7a708e92) function
        pub fn init_reserve(
            &self,
            asset: ::ethers::core::types::Address,
            a_token_address: ::ethers::core::types::Address,
            stable_debt_address: ::ethers::core::types::Address,
            variable_debt_address: ::ethers::core::types::Address,
            interest_rate_strategy_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 112, 142, 146],
                    (
                        asset,
                        a_token_address,
                        stable_debt_address,
                        variable_debt_address,
                        interest_rate_strategy_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], provider)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationCall` (0x00a718a9) function
        pub fn liquidation_call_with_collateral_asset_and_debt_asset(
            &self,
            collateral_asset: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            receive_a_token: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 167, 24, 169],
                    (collateral_asset, debt_asset, user, debt_to_cover, receive_a_token),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationCall` (0xfd21ecff) function
        pub fn liquidation_call(
            &self,
            args_1: [u8; 32],
            args_2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 33, 236, 255], (args_1, args_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintToTreasury` (0x9cd19996) function
        pub fn mint_to_treasury(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 209, 153, 150], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintUnbacked` (0x69a933a5) function
        pub fn mint_unbacked(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 169, 51, 165],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceStableBorrowRate` (0x427da177) function
        pub fn rebalance_stable_borrow_rate(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 125, 161, 119], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceStableBorrowRate` (0xcd112382) function
        pub fn rebalance_stable_borrow_rate_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 17, 35, 130], (asset, user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repay` (0x563dd613) function
        pub fn repay(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 61, 214, 19], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repay` (0x573ade81) function
        pub fn repay_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [87, 58, 222, 129],
                    (asset, amount, interest_rate_mode, on_behalf_of),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithATokens` (0x2dad97d4) function
        pub fn repay_with_a_tokens_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 173, 151, 212], (asset, amount, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithATokens` (0xdc7c0bff) function
        pub fn repay_with_a_tokens(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 124, 11, 255], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithPermit` (0x94b576de) function
        pub fn repay_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 181, 118, 222], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayWithPermit` (0xee3e210b) function
        pub fn repay_with_permit_with_asset_and_amount_and_interest_rate_mode(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            interest_rate_mode: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
            deadline: ::ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [238, 62, 33, 11],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        on_behalf_of,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueTokens` (0xcea9d26f) function
        pub fn rescue_tokens(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 169, 210, 111], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetIsolationModeTotalDebt` (0xe43e88a1) function
        pub fn reset_isolation_mode_total_debt(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 62, 136, 161], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfiguration` (0xf51e435b) function
        pub fn set_configuration(
            &self,
            asset: ::ethers::core::types::Address,
            configuration: ReserveConfigurationMap,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 30, 67, 91], (asset, configuration))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            asset: ::ethers::core::types::Address,
            rate_strategy_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (asset, rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUserEMode` (0x28530a47) function
        pub fn set_user_e_mode(
            &self,
            category_id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 83, 10, 71], category_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUserUseReserveAsCollateral` (0x4d013f03) function
        pub fn set_user_use_reserve_as_collateral(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 1, 63, 3], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUserUseReserveAsCollateral` (0x5a3b74b9) function
        pub fn set_user_use_reserve_as_collateral_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 59, 116, 185], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supply` (0x617ba037) function
        pub fn supply_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
            referral_code: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [97, 123, 160, 55],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supply` (0xf7a73840) function
        pub fn supply(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 167, 56, 64], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyWithPermit` (0x02c205f0) function
        pub fn supply_with_permit_with_asset_and_amount_and_on_behalf_of(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            on_behalf_of: ::ethers::core::types::Address,
            referral_code: u16,
            deadline: ::ethers::core::types::U256,
            permit_v: u8,
            permit_r: [u8; 32],
            permit_s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [2, 194, 5, 240],
                    (
                        asset,
                        amount,
                        on_behalf_of,
                        referral_code,
                        deadline,
                        permit_v,
                        permit_r,
                        permit_s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyWithPermit` (0x680dd47c) function
        pub fn supply_with_permit(
            &self,
            args: [u8; 32],
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 13, 212, 124], (args, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapBorrowRateMode` (0x1fe3c6f3) function
        pub fn swap_borrow_rate_mode(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 227, 198, 243], args)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapBorrowRateMode` (0x94ba89a2) function
        pub fn swap_borrow_rate_mode_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            interest_rate_mode: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 186, 137, 162], (asset, interest_rate_mode))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBridgeProtocolFee` (0x3036b439) function
        pub fn update_bridge_protocol_fee(
            &self,
            protocol_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 54, 180, 57], protocol_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFlashloanPremiums` (0xbcb6e522) function
        pub fn update_flashloan_premiums(
            &self,
            flash_loan_premium_total: u128,
            flash_loan_premium_to_protocol: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [188, 182, 229, 34],
                    (flash_loan_premium_total, flash_loan_premium_to_protocol),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x69328dec) function
        pub fn withdraw_with_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 50, 141, 236], (asset, amount, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x8e19899e) function
        pub fn withdraw(
            &self,
            args: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 25, 137, 158], args)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BackUnbacked` event
        pub fn back_unbacked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BackUnbackedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Borrow` event
        pub fn borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BorrowFilter> {
            self.0.event()
        }
        ///Gets the contract's `FlashLoan` event
        pub fn flash_loan_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FlashLoanFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IsolationModeTotalDebtUpdated` event
        pub fn isolation_mode_total_debt_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IsolationModeTotalDebtUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidationCall` event
        pub fn liquidation_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MintUnbacked` event
        pub fn mint_unbacked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintUnbackedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MintedToTreasury` event
        pub fn minted_to_treasury_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintedToTreasuryFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RebalanceStableBorrowRate` event
        pub fn rebalance_stable_borrow_rate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RebalanceStableBorrowRateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Repay` event
        pub fn repay_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RepayFilter> {
            self.0.event()
        }
        ///Gets the contract's `ReserveDataUpdated` event
        pub fn reserve_data_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveDataUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveUsedAsCollateralDisabled` event
        pub fn reserve_used_as_collateral_disabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveUsedAsCollateralDisabledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveUsedAsCollateralEnabled` event
        pub fn reserve_used_as_collateral_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveUsedAsCollateralEnabledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Supply` event
        pub fn supply_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SupplyFilter> {
            self.0.event()
        }
        ///Gets the contract's `SwapBorrowRateMode` event
        pub fn swap_borrow_rate_mode_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapBorrowRateModeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UserEModeSet` event
        pub fn user_e_mode_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserEModeSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, L2PoolEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for L2Pool<M> {
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
    #[ethevent(
        name = "BackUnbacked",
        abi = "BackUnbacked(address,address,uint256,uint256)"
    )]
    pub struct BackUnbackedFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub backer: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
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
        name = "Borrow",
        abi = "Borrow(address,address,address,uint256,uint8,uint256,uint16)"
    )]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: u8,
        pub borrow_rate: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "FlashLoan",
        abi = "FlashLoan(address,address,address,uint256,uint8,uint256,uint16)"
    )]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        pub initiator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: u8,
        pub premium: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "IsolationModeTotalDebtUpdated",
        abi = "IsolationModeTotalDebtUpdated(address,uint256)"
    )]
    pub struct IsolationModeTotalDebtUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub total_debt: ::ethers::core::types::U256,
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
        name = "LiquidationCall",
        abi = "LiquidationCall(address,address,address,uint256,uint256,address,bool)"
    )]
    pub struct LiquidationCallFilter {
        #[ethevent(indexed)]
        pub collateral_asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub debt_asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub liquidated_collateral_amount: ::ethers::core::types::U256,
        pub liquidator: ::ethers::core::types::Address,
        pub receive_a_token: bool,
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
        name = "MintUnbacked",
        abi = "MintUnbacked(address,address,address,uint256,uint16)"
    )]
    pub struct MintUnbackedFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
    #[ethevent(name = "MintedToTreasury", abi = "MintedToTreasury(address,uint256)")]
    pub struct MintedToTreasuryFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        pub amount_minted: ::ethers::core::types::U256,
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
        name = "RebalanceStableBorrowRate",
        abi = "RebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(name = "Repay", abi = "Repay(address,address,address,uint256,bool)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub repayer: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub use_a_tokens: bool,
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
        name = "ReserveDataUpdated",
        abi = "ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ReserveDataUpdatedFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        pub liquidity_rate: ::ethers::core::types::U256,
        pub stable_borrow_rate: ::ethers::core::types::U256,
        pub variable_borrow_rate: ::ethers::core::types::U256,
        pub liquidity_index: ::ethers::core::types::U256,
        pub variable_borrow_index: ::ethers::core::types::U256,
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
        name = "ReserveUsedAsCollateralDisabled",
        abi = "ReserveUsedAsCollateralDisabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralDisabledFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
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
        name = "ReserveUsedAsCollateralEnabled",
        abi = "ReserveUsedAsCollateralEnabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralEnabledFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(name = "Supply", abi = "Supply(address,address,address,uint256,uint16)")]
    pub struct SupplyFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral_code: u16,
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
        name = "SwapBorrowRateMode",
        abi = "SwapBorrowRateMode(address,address,uint8)"
    )]
    pub struct SwapBorrowRateModeFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub interest_rate_mode: u8,
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
    #[ethevent(name = "UserEModeSet", abi = "UserEModeSet(address,uint8)")]
    pub struct UserEModeSetFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub category_id: u8,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub reserve: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2PoolEvents {
        BackUnbackedFilter(BackUnbackedFilter),
        BorrowFilter(BorrowFilter),
        FlashLoanFilter(FlashLoanFilter),
        IsolationModeTotalDebtUpdatedFilter(IsolationModeTotalDebtUpdatedFilter),
        LiquidationCallFilter(LiquidationCallFilter),
        MintUnbackedFilter(MintUnbackedFilter),
        MintedToTreasuryFilter(MintedToTreasuryFilter),
        RebalanceStableBorrowRateFilter(RebalanceStableBorrowRateFilter),
        RepayFilter(RepayFilter),
        ReserveDataUpdatedFilter(ReserveDataUpdatedFilter),
        ReserveUsedAsCollateralDisabledFilter(ReserveUsedAsCollateralDisabledFilter),
        ReserveUsedAsCollateralEnabledFilter(ReserveUsedAsCollateralEnabledFilter),
        SupplyFilter(SupplyFilter),
        SwapBorrowRateModeFilter(SwapBorrowRateModeFilter),
        UserEModeSetFilter(UserEModeSetFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for L2PoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BackUnbackedFilter::decode_log(log) {
                return Ok(L2PoolEvents::BackUnbackedFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(L2PoolEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(L2PoolEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = IsolationModeTotalDebtUpdatedFilter::decode_log(log) {
                return Ok(L2PoolEvents::IsolationModeTotalDebtUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LiquidationCallFilter::decode_log(log) {
                return Ok(L2PoolEvents::LiquidationCallFilter(decoded));
            }
            if let Ok(decoded) = MintUnbackedFilter::decode_log(log) {
                return Ok(L2PoolEvents::MintUnbackedFilter(decoded));
            }
            if let Ok(decoded) = MintedToTreasuryFilter::decode_log(log) {
                return Ok(L2PoolEvents::MintedToTreasuryFilter(decoded));
            }
            if let Ok(decoded) = RebalanceStableBorrowRateFilter::decode_log(log) {
                return Ok(L2PoolEvents::RebalanceStableBorrowRateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(L2PoolEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = ReserveDataUpdatedFilter::decode_log(log) {
                return Ok(L2PoolEvents::ReserveDataUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralDisabledFilter::decode_log(log) {
                return Ok(L2PoolEvents::ReserveUsedAsCollateralDisabledFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralEnabledFilter::decode_log(log) {
                return Ok(L2PoolEvents::ReserveUsedAsCollateralEnabledFilter(decoded));
            }
            if let Ok(decoded) = SupplyFilter::decode_log(log) {
                return Ok(L2PoolEvents::SupplyFilter(decoded));
            }
            if let Ok(decoded) = SwapBorrowRateModeFilter::decode_log(log) {
                return Ok(L2PoolEvents::SwapBorrowRateModeFilter(decoded));
            }
            if let Ok(decoded) = UserEModeSetFilter::decode_log(log) {
                return Ok(L2PoolEvents::UserEModeSetFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(L2PoolEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for L2PoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BackUnbackedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoanFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsolationModeTotalDebtUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintUnbackedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintedToTreasuryFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceStableBorrowRateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveDataUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveUsedAsCollateralDisabledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveUsedAsCollateralEnabledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupplyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapBorrowRateModeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserEModeSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BackUnbackedFilter> for L2PoolEvents {
        fn from(value: BackUnbackedFilter) -> Self {
            Self::BackUnbackedFilter(value)
        }
    }
    impl ::core::convert::From<BorrowFilter> for L2PoolEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<FlashLoanFilter> for L2PoolEvents {
        fn from(value: FlashLoanFilter) -> Self {
            Self::FlashLoanFilter(value)
        }
    }
    impl ::core::convert::From<IsolationModeTotalDebtUpdatedFilter> for L2PoolEvents {
        fn from(value: IsolationModeTotalDebtUpdatedFilter) -> Self {
            Self::IsolationModeTotalDebtUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationCallFilter> for L2PoolEvents {
        fn from(value: LiquidationCallFilter) -> Self {
            Self::LiquidationCallFilter(value)
        }
    }
    impl ::core::convert::From<MintUnbackedFilter> for L2PoolEvents {
        fn from(value: MintUnbackedFilter) -> Self {
            Self::MintUnbackedFilter(value)
        }
    }
    impl ::core::convert::From<MintedToTreasuryFilter> for L2PoolEvents {
        fn from(value: MintedToTreasuryFilter) -> Self {
            Self::MintedToTreasuryFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceStableBorrowRateFilter> for L2PoolEvents {
        fn from(value: RebalanceStableBorrowRateFilter) -> Self {
            Self::RebalanceStableBorrowRateFilter(value)
        }
    }
    impl ::core::convert::From<RepayFilter> for L2PoolEvents {
        fn from(value: RepayFilter) -> Self {
            Self::RepayFilter(value)
        }
    }
    impl ::core::convert::From<ReserveDataUpdatedFilter> for L2PoolEvents {
        fn from(value: ReserveDataUpdatedFilter) -> Self {
            Self::ReserveDataUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveUsedAsCollateralDisabledFilter> for L2PoolEvents {
        fn from(value: ReserveUsedAsCollateralDisabledFilter) -> Self {
            Self::ReserveUsedAsCollateralDisabledFilter(value)
        }
    }
    impl ::core::convert::From<ReserveUsedAsCollateralEnabledFilter> for L2PoolEvents {
        fn from(value: ReserveUsedAsCollateralEnabledFilter) -> Self {
            Self::ReserveUsedAsCollateralEnabledFilter(value)
        }
    }
    impl ::core::convert::From<SupplyFilter> for L2PoolEvents {
        fn from(value: SupplyFilter) -> Self {
            Self::SupplyFilter(value)
        }
    }
    impl ::core::convert::From<SwapBorrowRateModeFilter> for L2PoolEvents {
        fn from(value: SwapBorrowRateModeFilter) -> Self {
            Self::SwapBorrowRateModeFilter(value)
        }
    }
    impl ::core::convert::From<UserEModeSetFilter> for L2PoolEvents {
        fn from(value: UserEModeSetFilter) -> Self {
            Self::UserEModeSetFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for L2PoolEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
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
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    ///Container type for all input parameters for the `BRIDGE_PROTOCOL_FEE` function with signature `BRIDGE_PROTOCOL_FEE()` and selector `0x272d9072`
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
    #[ethcall(name = "BRIDGE_PROTOCOL_FEE", abi = "BRIDGE_PROTOCOL_FEE()")]
    pub struct BridgeProtocolFeeCall;
    ///Container type for all input parameters for the `FLASHLOAN_PREMIUM_TOTAL` function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `0x074b2e43`
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
    #[ethcall(name = "FLASHLOAN_PREMIUM_TOTAL", abi = "FLASHLOAN_PREMIUM_TOTAL()")]
    pub struct FlashloanPremiumTotalCall;
    ///Container type for all input parameters for the `FLASHLOAN_PREMIUM_TO_PROTOCOL` function with signature `FLASHLOAN_PREMIUM_TO_PROTOCOL()` and selector `0x6a99c036`
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
        name = "FLASHLOAN_PREMIUM_TO_PROTOCOL",
        abi = "FLASHLOAN_PREMIUM_TO_PROTOCOL()"
    )]
    pub struct FlashloanPremiumToProtocolCall;
    ///Container type for all input parameters for the `MAX_NUMBER_RESERVES` function with signature `MAX_NUMBER_RESERVES()` and selector `0xf8119d51`
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
    #[ethcall(name = "MAX_NUMBER_RESERVES", abi = "MAX_NUMBER_RESERVES()")]
    pub struct MaxNumberReservesCall;
    ///Container type for all input parameters for the `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `0xe82fec2f`
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
        name = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
        abi = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT()"
    )]
    pub struct MaxStableRateBorrowSizePercentCall;
    ///Container type for all input parameters for the `POOL_REVISION` function with signature `POOL_REVISION()` and selector `0x0148170e`
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
    #[ethcall(name = "POOL_REVISION", abi = "POOL_REVISION()")]
    pub struct PoolRevisionCall;
    ///Container type for all input parameters for the `backUnbacked` function with signature `backUnbacked(address,uint256,uint256)` and selector `0xd65dc7a1`
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
    #[ethcall(name = "backUnbacked", abi = "backUnbacked(address,uint256,uint256)")]
    pub struct BackUnbackedCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `borrow` function with signature `borrow(address,uint256,uint256,uint16,address)` and selector `0xa415bcad`
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
    #[ethcall(name = "borrow", abi = "borrow(address,uint256,uint256,uint16,address)")]
    pub struct BorrowWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
        pub referral_code: u16,
        pub on_behalf_of: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `borrow` function with signature `borrow(bytes32)` and selector `0xd5eed868`
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
    #[ethcall(name = "borrow", abi = "borrow(bytes32)")]
    pub struct BorrowCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `configureEModeCategory` function with signature `configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))` and selector `0xd579ea7d`
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
        name = "configureEModeCategory",
        abi = "configureEModeCategory(uint8,(uint16,uint16,uint16,address,string))"
    )]
    pub struct ConfigureEModeCategoryCall {
        pub id: u8,
        pub category: EmodeCategory,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,uint256,address,uint16)` and selector `0xe8eda9df`
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
    #[ethcall(name = "deposit", abi = "deposit(address,uint256,address,uint16)")]
    pub struct DepositCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `dropReserve` function with signature `dropReserve(address)` and selector `0x63c9b860`
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
    #[ethcall(name = "dropReserve", abi = "dropReserve(address)")]
    pub struct DropReserveCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `finalizeTransfer` function with signature `finalizeTransfer(address,address,address,uint256,uint256,uint256)` and selector `0xd5ed3933`
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
        name = "finalizeTransfer",
        abi = "finalizeTransfer(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct FinalizeTransferCall {
        pub asset: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub balance_from_before: ::ethers::core::types::U256,
        pub balance_to_before: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)` and selector `0xab9c4b5d`
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
        name = "flashLoan",
        abi = "flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)"
    )]
    pub struct FlashLoanCall {
        pub receiver_address: ::ethers::core::types::Address,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub interest_rate_modes: ::std::vec::Vec<::ethers::core::types::U256>,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub params: ::ethers::core::types::Bytes,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `flashLoanSimple` function with signature `flashLoanSimple(address,address,uint256,bytes,uint16)` and selector `0x42b0b77c`
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
        name = "flashLoanSimple",
        abi = "flashLoanSimple(address,address,uint256,bytes,uint16)"
    )]
    pub struct FlashLoanSimpleCall {
        pub receiver_address: ::ethers::core::types::Address,
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub params: ::ethers::core::types::Bytes,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `getConfiguration` function with signature `getConfiguration(address)` and selector `0xc44b11f7`
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
    #[ethcall(name = "getConfiguration", abi = "getConfiguration(address)")]
    pub struct GetConfigurationCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getEModeCategoryData` function with signature `getEModeCategoryData(uint8)` and selector `0x6c6f6ae1`
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
    #[ethcall(name = "getEModeCategoryData", abi = "getEModeCategoryData(uint8)")]
    pub struct GetEModeCategoryDataCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `getReserveAddressById` function with signature `getReserveAddressById(uint16)` and selector `0x52751797`
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
    #[ethcall(name = "getReserveAddressById", abi = "getReserveAddressById(uint16)")]
    pub struct GetReserveAddressByIdCall {
        pub id: u16,
    }
    ///Container type for all input parameters for the `getReserveData` function with signature `getReserveData(address)` and selector `0x35ea6a75`
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
    #[ethcall(name = "getReserveData", abi = "getReserveData(address)")]
    pub struct GetReserveDataCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `0xd15e0053`
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
        name = "getReserveNormalizedIncome",
        abi = "getReserveNormalizedIncome(address)"
    )]
    pub struct GetReserveNormalizedIncomeCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `0x386497fd`
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
        name = "getReserveNormalizedVariableDebt",
        abi = "getReserveNormalizedVariableDebt(address)"
    )]
    pub struct GetReserveNormalizedVariableDebtCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReservesList` function with signature `getReservesList()` and selector `0xd1946dbc`
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
    #[ethcall(name = "getReservesList", abi = "getReservesList()")]
    pub struct GetReservesListCall;
    ///Container type for all input parameters for the `getUserAccountData` function with signature `getUserAccountData(address)` and selector `0xbf92857c`
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
    #[ethcall(name = "getUserAccountData", abi = "getUserAccountData(address)")]
    pub struct GetUserAccountDataCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserConfiguration` function with signature `getUserConfiguration(address)` and selector `0x4417a583`
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
    #[ethcall(name = "getUserConfiguration", abi = "getUserConfiguration(address)")]
    pub struct GetUserConfigurationCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserEMode` function with signature `getUserEMode(address)` and selector `0xeddf1b79`
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
    #[ethcall(name = "getUserEMode", abi = "getUserEMode(address)")]
    pub struct GetUserEModeCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initReserve` function with signature `initReserve(address,address,address,address,address)` and selector `0x7a708e92`
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
        name = "initReserve",
        abi = "initReserve(address,address,address,address,address)"
    )]
    pub struct InitReserveCall {
        pub asset: ::ethers::core::types::Address,
        pub a_token_address: ::ethers::core::types::Address,
        pub stable_debt_address: ::ethers::core::types::Address,
        pub variable_debt_address: ::ethers::core::types::Address,
        pub interest_rate_strategy_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub provider: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidationCall` function with signature `liquidationCall(address,address,address,uint256,bool)` and selector `0x00a718a9`
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
        name = "liquidationCall",
        abi = "liquidationCall(address,address,address,uint256,bool)"
    )]
    pub struct LiquidationCallWithCollateralAssetAndDebtAssetCall {
        pub collateral_asset: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub receive_a_token: bool,
    }
    ///Container type for all input parameters for the `liquidationCall` function with signature `liquidationCall(bytes32,bytes32)` and selector `0xfd21ecff`
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
    #[ethcall(name = "liquidationCall", abi = "liquidationCall(bytes32,bytes32)")]
    pub struct LiquidationCallCall {
        pub args_1: [u8; 32],
        pub args_2: [u8; 32],
    }
    ///Container type for all input parameters for the `mintToTreasury` function with signature `mintToTreasury(address[])` and selector `0x9cd19996`
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
    #[ethcall(name = "mintToTreasury", abi = "mintToTreasury(address[])")]
    pub struct MintToTreasuryCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `mintUnbacked` function with signature `mintUnbacked(address,uint256,address,uint16)` and selector `0x69a933a5`
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
        name = "mintUnbacked",
        abi = "mintUnbacked(address,uint256,address,uint16)"
    )]
    pub struct MintUnbackedCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `rebalanceStableBorrowRate` function with signature `rebalanceStableBorrowRate(bytes32)` and selector `0x427da177`
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(bytes32)"
    )]
    pub struct RebalanceStableBorrowRateCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `rebalanceStableBorrowRate` function with signature `rebalanceStableBorrowRate(address,address)` and selector `0xcd112382`
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `repay` function with signature `repay(bytes32)` and selector `0x563dd613`
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
    #[ethcall(name = "repay", abi = "repay(bytes32)")]
    pub struct RepayCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `repay` function with signature `repay(address,uint256,uint256,address)` and selector `0x573ade81`
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
    #[ethcall(name = "repay", abi = "repay(address,uint256,uint256,address)")]
    pub struct RepayWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `repayWithATokens` function with signature `repayWithATokens(address,uint256,uint256)` and selector `0x2dad97d4`
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
        name = "repayWithATokens",
        abi = "repayWithATokens(address,uint256,uint256)"
    )]
    pub struct RepayWithATokensWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `repayWithATokens` function with signature `repayWithATokens(bytes32)` and selector `0xdc7c0bff`
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
    #[ethcall(name = "repayWithATokens", abi = "repayWithATokens(bytes32)")]
    pub struct RepayWithATokensCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `repayWithPermit` function with signature `repayWithPermit(bytes32,bytes32,bytes32)` and selector `0x94b576de`
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
        name = "repayWithPermit",
        abi = "repayWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct RepayWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `repayWithPermit` function with signature `repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `0xee3e210b`
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
        name = "repayWithPermit",
        abi = "repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct RepayWithPermitWithAssetAndAmountAndInterestRateModeCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub interest_rate_mode: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub deadline: ::ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    ///Container type for all input parameters for the `rescueTokens` function with signature `rescueTokens(address,address,uint256)` and selector `0xcea9d26f`
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
    #[ethcall(name = "rescueTokens", abi = "rescueTokens(address,address,uint256)")]
    pub struct RescueTokensCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `resetIsolationModeTotalDebt` function with signature `resetIsolationModeTotalDebt(address)` and selector `0xe43e88a1`
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
        name = "resetIsolationModeTotalDebt",
        abi = "resetIsolationModeTotalDebt(address)"
    )]
    pub struct ResetIsolationModeTotalDebtCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setConfiguration` function with signature `setConfiguration(address,(uint256))` and selector `0xf51e435b`
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
    #[ethcall(name = "setConfiguration", abi = "setConfiguration(address,(uint256))")]
    pub struct SetConfigurationCall {
        pub asset: ::ethers::core::types::Address,
        pub configuration: ReserveConfigurationMap,
    }
    ///Container type for all input parameters for the `setReserveInterestRateStrategyAddress` function with signature `setReserveInterestRateStrategyAddress(address,address)` and selector `0x1d2118f9`
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
        name = "setReserveInterestRateStrategyAddress",
        abi = "setReserveInterestRateStrategyAddress(address,address)"
    )]
    pub struct SetReserveInterestRateStrategyAddressCall {
        pub asset: ::ethers::core::types::Address,
        pub rate_strategy_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setUserEMode` function with signature `setUserEMode(uint8)` and selector `0x28530a47`
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
    #[ethcall(name = "setUserEMode", abi = "setUserEMode(uint8)")]
    pub struct SetUserEModeCall {
        pub category_id: u8,
    }
    ///Container type for all input parameters for the `setUserUseReserveAsCollateral` function with signature `setUserUseReserveAsCollateral(bytes32)` and selector `0x4d013f03`
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(bytes32)"
    )]
    pub struct SetUserUseReserveAsCollateralCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `setUserUseReserveAsCollateral` function with signature `setUserUseReserveAsCollateral(address,bool)` and selector `0x5a3b74b9`
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(address,bool)"
    )]
    pub struct SetUserUseReserveAsCollateralWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub use_as_collateral: bool,
    }
    ///Container type for all input parameters for the `supply` function with signature `supply(address,uint256,address,uint16)` and selector `0x617ba037`
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
    #[ethcall(name = "supply", abi = "supply(address,uint256,address,uint16)")]
    pub struct SupplyWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub referral_code: u16,
    }
    ///Container type for all input parameters for the `supply` function with signature `supply(bytes32)` and selector `0xf7a73840`
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
    #[ethcall(name = "supply", abi = "supply(bytes32)")]
    pub struct SupplyCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `supplyWithPermit` function with signature `supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)` and selector `0x02c205f0`
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
        name = "supplyWithPermit",
        abi = "supplyWithPermit(address,uint256,address,uint16,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SupplyWithPermitWithAssetAndAmountAndOnBehalfOfCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub referral_code: u16,
        pub deadline: ::ethers::core::types::U256,
        pub permit_v: u8,
        pub permit_r: [u8; 32],
        pub permit_s: [u8; 32],
    }
    ///Container type for all input parameters for the `supplyWithPermit` function with signature `supplyWithPermit(bytes32,bytes32,bytes32)` and selector `0x680dd47c`
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
        name = "supplyWithPermit",
        abi = "supplyWithPermit(bytes32,bytes32,bytes32)"
    )]
    pub struct SupplyWithPermitCall {
        pub args: [u8; 32],
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `swapBorrowRateMode` function with signature `swapBorrowRateMode(bytes32)` and selector `0x1fe3c6f3`
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
    #[ethcall(name = "swapBorrowRateMode", abi = "swapBorrowRateMode(bytes32)")]
    pub struct SwapBorrowRateModeCall {
        pub args: [u8; 32],
    }
    ///Container type for all input parameters for the `swapBorrowRateMode` function with signature `swapBorrowRateMode(address,uint256)` and selector `0x94ba89a2`
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
    #[ethcall(name = "swapBorrowRateMode", abi = "swapBorrowRateMode(address,uint256)")]
    pub struct SwapBorrowRateModeWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub interest_rate_mode: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateBridgeProtocolFee` function with signature `updateBridgeProtocolFee(uint256)` and selector `0x3036b439`
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
        name = "updateBridgeProtocolFee",
        abi = "updateBridgeProtocolFee(uint256)"
    )]
    pub struct UpdateBridgeProtocolFeeCall {
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateFlashloanPremiums` function with signature `updateFlashloanPremiums(uint128,uint128)` and selector `0xbcb6e522`
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
        name = "updateFlashloanPremiums",
        abi = "updateFlashloanPremiums(uint128,uint128)"
    )]
    pub struct UpdateFlashloanPremiumsCall {
        pub flash_loan_premium_total: u128,
        pub flash_loan_premium_to_protocol: u128,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `0x69328dec`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256,address)")]
    pub struct WithdrawWithAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(bytes32)` and selector `0x8e19899e`
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
    #[ethcall(name = "withdraw", abi = "withdraw(bytes32)")]
    pub struct WithdrawCall {
        pub args: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum L2PoolCalls {
        AddressesProvider(AddressesProviderCall),
        BridgeProtocolFee(BridgeProtocolFeeCall),
        FlashloanPremiumTotal(FlashloanPremiumTotalCall),
        FlashloanPremiumToProtocol(FlashloanPremiumToProtocolCall),
        MaxNumberReserves(MaxNumberReservesCall),
        MaxStableRateBorrowSizePercent(MaxStableRateBorrowSizePercentCall),
        PoolRevision(PoolRevisionCall),
        BackUnbacked(BackUnbackedCall),
        BorrowWithAsset(BorrowWithAssetCall),
        Borrow(BorrowCall),
        ConfigureEModeCategory(ConfigureEModeCategoryCall),
        Deposit(DepositCall),
        DropReserve(DropReserveCall),
        FinalizeTransfer(FinalizeTransferCall),
        FlashLoan(FlashLoanCall),
        FlashLoanSimple(FlashLoanSimpleCall),
        GetConfiguration(GetConfigurationCall),
        GetEModeCategoryData(GetEModeCategoryDataCall),
        GetReserveAddressById(GetReserveAddressByIdCall),
        GetReserveData(GetReserveDataCall),
        GetReserveNormalizedIncome(GetReserveNormalizedIncomeCall),
        GetReserveNormalizedVariableDebt(GetReserveNormalizedVariableDebtCall),
        GetReservesList(GetReservesListCall),
        GetUserAccountData(GetUserAccountDataCall),
        GetUserConfiguration(GetUserConfigurationCall),
        GetUserEMode(GetUserEModeCall),
        InitReserve(InitReserveCall),
        Initialize(InitializeCall),
        LiquidationCallWithCollateralAssetAndDebtAsset(
            LiquidationCallWithCollateralAssetAndDebtAssetCall,
        ),
        LiquidationCall(LiquidationCallCall),
        MintToTreasury(MintToTreasuryCall),
        MintUnbacked(MintUnbackedCall),
        RebalanceStableBorrowRate(RebalanceStableBorrowRateCall),
        RebalanceStableBorrowRateWithAsset(RebalanceStableBorrowRateWithAssetCall),
        Repay(RepayCall),
        RepayWithAsset(RepayWithAssetCall),
        RepayWithATokensWithAsset(RepayWithATokensWithAssetCall),
        RepayWithATokens(RepayWithATokensCall),
        RepayWithPermit(RepayWithPermitCall),
        RepayWithPermitWithAssetAndAmountAndInterestRateMode(
            RepayWithPermitWithAssetAndAmountAndInterestRateModeCall,
        ),
        RescueTokens(RescueTokensCall),
        ResetIsolationModeTotalDebt(ResetIsolationModeTotalDebtCall),
        SetConfiguration(SetConfigurationCall),
        SetReserveInterestRateStrategyAddress(SetReserveInterestRateStrategyAddressCall),
        SetUserEMode(SetUserEModeCall),
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateralCall),
        SetUserUseReserveAsCollateralWithAsset(
            SetUserUseReserveAsCollateralWithAssetCall,
        ),
        SupplyWithAsset(SupplyWithAssetCall),
        Supply(SupplyCall),
        SupplyWithPermitWithAssetAndAmountAndOnBehalfOf(
            SupplyWithPermitWithAssetAndAmountAndOnBehalfOfCall,
        ),
        SupplyWithPermit(SupplyWithPermitCall),
        SwapBorrowRateMode(SwapBorrowRateModeCall),
        SwapBorrowRateModeWithAsset(SwapBorrowRateModeWithAssetCall),
        UpdateBridgeProtocolFee(UpdateBridgeProtocolFeeCall),
        UpdateFlashloanPremiums(UpdateFlashloanPremiumsCall),
        WithdrawWithAsset(WithdrawWithAssetCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for L2PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddressesProvider(decoded));
            }
            if let Ok(decoded)
                = <BridgeProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BridgeProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <FlashloanPremiumTotalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashloanPremiumTotal(decoded));
            }
            if let Ok(decoded)
                = <FlashloanPremiumToProtocolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashloanPremiumToProtocol(decoded));
            }
            if let Ok(decoded)
                = <MaxNumberReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxNumberReserves(decoded));
            }
            if let Ok(decoded)
                = <MaxStableRateBorrowSizePercentCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxStableRateBorrowSizePercent(decoded));
            }
            if let Ok(decoded)
                = <PoolRevisionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolRevision(decoded));
            }
            if let Ok(decoded)
                = <BackUnbackedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BackUnbacked(decoded));
            }
            if let Ok(decoded)
                = <BorrowWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BorrowWithAsset(decoded));
            }
            if let Ok(decoded)
                = <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <ConfigureEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigureEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <DropReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropReserve(decoded));
            }
            if let Ok(decoded)
                = <FinalizeTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FinalizeTransfer(decoded));
            }
            if let Ok(decoded)
                = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded)
                = <FlashLoanSimpleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashLoanSimple(decoded));
            }
            if let Ok(decoded)
                = <GetConfigurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetConfiguration(decoded));
            }
            if let Ok(decoded)
                = <GetEModeCategoryDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetEModeCategoryData(decoded));
            }
            if let Ok(decoded)
                = <GetReserveAddressByIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveAddressById(decoded));
            }
            if let Ok(decoded)
                = <GetReserveDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserveData(decoded));
            }
            if let Ok(decoded)
                = <GetReserveNormalizedIncomeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveNormalizedIncome(decoded));
            }
            if let Ok(decoded)
                = <GetReserveNormalizedVariableDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveNormalizedVariableDebt(decoded));
            }
            if let Ok(decoded)
                = <GetReservesListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReservesList(decoded));
            }
            if let Ok(decoded)
                = <GetUserAccountDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUserAccountData(decoded));
            }
            if let Ok(decoded)
                = <GetUserConfigurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUserConfiguration(decoded));
            }
            if let Ok(decoded)
                = <GetUserEModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetUserEMode(decoded));
            }
            if let Ok(decoded)
                = <InitReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitReserve(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <LiquidationCallWithCollateralAssetAndDebtAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LiquidationCallWithCollateralAssetAndDebtAsset(decoded));
            }
            if let Ok(decoded)
                = <LiquidationCallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LiquidationCall(decoded));
            }
            if let Ok(decoded)
                = <MintToTreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintToTreasury(decoded));
            }
            if let Ok(decoded)
                = <MintUnbackedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintUnbacked(decoded));
            }
            if let Ok(decoded)
                = <RebalanceStableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <RebalanceStableBorrowRateWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RebalanceStableBorrowRateWithAsset(decoded));
            }
            if let Ok(decoded)
                = <RepayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Repay(decoded));
            }
            if let Ok(decoded)
                = <RepayWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RepayWithAsset(decoded));
            }
            if let Ok(decoded)
                = <RepayWithATokensWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RepayWithATokensWithAsset(decoded));
            }
            if let Ok(decoded)
                = <RepayWithATokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RepayWithATokens(decoded));
            }
            if let Ok(decoded)
                = <RepayWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RepayWithPermit(decoded));
            }
            if let Ok(decoded)
                = <RepayWithPermitWithAssetAndAmountAndInterestRateModeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::RepayWithPermitWithAssetAndAmountAndInterestRateMode(decoded),
                );
            }
            if let Ok(decoded)
                = <RescueTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescueTokens(decoded));
            }
            if let Ok(decoded)
                = <ResetIsolationModeTotalDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ResetIsolationModeTotalDebt(decoded));
            }
            if let Ok(decoded)
                = <SetConfigurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetConfiguration(decoded));
            }
            if let Ok(decoded)
                = <SetReserveInterestRateStrategyAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded)
                = <SetUserEModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetUserEMode(decoded));
            }
            if let Ok(decoded)
                = <SetUserUseReserveAsCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded)
                = <SetUserUseReserveAsCollateralWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetUserUseReserveAsCollateralWithAsset(decoded));
            }
            if let Ok(decoded)
                = <SupplyWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SupplyWithAsset(decoded));
            }
            if let Ok(decoded)
                = <SupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Supply(decoded));
            }
            if let Ok(decoded)
                = <SupplyWithPermitWithAssetAndAmountAndOnBehalfOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::SupplyWithPermitWithAssetAndAmountAndOnBehalfOf(decoded),
                );
            }
            if let Ok(decoded)
                = <SupplyWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupplyWithPermit(decoded));
            }
            if let Ok(decoded)
                = <SwapBorrowRateModeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapBorrowRateMode(decoded));
            }
            if let Ok(decoded)
                = <SwapBorrowRateModeWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapBorrowRateModeWithAsset(decoded));
            }
            if let Ok(decoded)
                = <UpdateBridgeProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateBridgeProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <UpdateFlashloanPremiumsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateFlashloanPremiums(decoded));
            }
            if let Ok(decoded)
                = <WithdrawWithAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawWithAsset(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for L2PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashloanPremiumTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashloanPremiumToProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxNumberReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxStableRateBorrowSizePercent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolRevision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BackUnbacked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Borrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConfigureEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DropReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoanSimple(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEModeCategoryData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveAddressById(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveNormalizedIncome(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveNormalizedVariableDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserAccountData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserEMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationCallWithCollateralAssetAndDebtAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidationCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintToTreasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintUnbacked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceStableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceStableBorrowRateWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Repay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RepayWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayWithATokensWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayWithATokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayWithPermitWithAssetAndAmountAndInterestRateMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetIsolationModeTotalDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveInterestRateStrategyAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUserEMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUserUseReserveAsCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUserUseReserveAsCollateralWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Supply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupplyWithPermitWithAssetAndAmountAndOnBehalfOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapBorrowRateMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapBorrowRateModeWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateBridgeProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFlashloanPremiums(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for L2PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::BridgeProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashloanPremiumTotal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashloanPremiumToProtocol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxNumberReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxStableRateBorrowSizePercent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PoolRevision(element) => ::core::fmt::Display::fmt(element, f),
                Self::BackUnbacked(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowWithAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigureEModeCategory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoanSimple(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEModeCategoryData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReserveAddressById(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReserveData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveNormalizedIncome(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReserveNormalizedVariableDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReservesList(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUserAccountData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserConfiguration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserEMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationCallWithCollateralAssetAndDebtAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintToTreasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintUnbacked(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceStableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceStableBorrowRateWithAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Repay(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithATokensWithAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayWithATokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayWithPermitWithAssetAndAmountAndInterestRateMode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RescueTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetIsolationModeTotalDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveInterestRateStrategyAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUserEMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUserUseReserveAsCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUserUseReserveAsCollateralWithAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupplyWithAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Supply(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyWithPermitWithAssetAndAmountAndOnBehalfOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupplyWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapBorrowRateMode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapBorrowRateModeWithAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateBridgeProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateFlashloanPremiums(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawWithAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for L2PoolCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<BridgeProtocolFeeCall> for L2PoolCalls {
        fn from(value: BridgeProtocolFeeCall) -> Self {
            Self::BridgeProtocolFee(value)
        }
    }
    impl ::core::convert::From<FlashloanPremiumTotalCall> for L2PoolCalls {
        fn from(value: FlashloanPremiumTotalCall) -> Self {
            Self::FlashloanPremiumTotal(value)
        }
    }
    impl ::core::convert::From<FlashloanPremiumToProtocolCall> for L2PoolCalls {
        fn from(value: FlashloanPremiumToProtocolCall) -> Self {
            Self::FlashloanPremiumToProtocol(value)
        }
    }
    impl ::core::convert::From<MaxNumberReservesCall> for L2PoolCalls {
        fn from(value: MaxNumberReservesCall) -> Self {
            Self::MaxNumberReserves(value)
        }
    }
    impl ::core::convert::From<MaxStableRateBorrowSizePercentCall> for L2PoolCalls {
        fn from(value: MaxStableRateBorrowSizePercentCall) -> Self {
            Self::MaxStableRateBorrowSizePercent(value)
        }
    }
    impl ::core::convert::From<PoolRevisionCall> for L2PoolCalls {
        fn from(value: PoolRevisionCall) -> Self {
            Self::PoolRevision(value)
        }
    }
    impl ::core::convert::From<BackUnbackedCall> for L2PoolCalls {
        fn from(value: BackUnbackedCall) -> Self {
            Self::BackUnbacked(value)
        }
    }
    impl ::core::convert::From<BorrowWithAssetCall> for L2PoolCalls {
        fn from(value: BorrowWithAssetCall) -> Self {
            Self::BorrowWithAsset(value)
        }
    }
    impl ::core::convert::From<BorrowCall> for L2PoolCalls {
        fn from(value: BorrowCall) -> Self {
            Self::Borrow(value)
        }
    }
    impl ::core::convert::From<ConfigureEModeCategoryCall> for L2PoolCalls {
        fn from(value: ConfigureEModeCategoryCall) -> Self {
            Self::ConfigureEModeCategory(value)
        }
    }
    impl ::core::convert::From<DepositCall> for L2PoolCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DropReserveCall> for L2PoolCalls {
        fn from(value: DropReserveCall) -> Self {
            Self::DropReserve(value)
        }
    }
    impl ::core::convert::From<FinalizeTransferCall> for L2PoolCalls {
        fn from(value: FinalizeTransferCall) -> Self {
            Self::FinalizeTransfer(value)
        }
    }
    impl ::core::convert::From<FlashLoanCall> for L2PoolCalls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<FlashLoanSimpleCall> for L2PoolCalls {
        fn from(value: FlashLoanSimpleCall) -> Self {
            Self::FlashLoanSimple(value)
        }
    }
    impl ::core::convert::From<GetConfigurationCall> for L2PoolCalls {
        fn from(value: GetConfigurationCall) -> Self {
            Self::GetConfiguration(value)
        }
    }
    impl ::core::convert::From<GetEModeCategoryDataCall> for L2PoolCalls {
        fn from(value: GetEModeCategoryDataCall) -> Self {
            Self::GetEModeCategoryData(value)
        }
    }
    impl ::core::convert::From<GetReserveAddressByIdCall> for L2PoolCalls {
        fn from(value: GetReserveAddressByIdCall) -> Self {
            Self::GetReserveAddressById(value)
        }
    }
    impl ::core::convert::From<GetReserveDataCall> for L2PoolCalls {
        fn from(value: GetReserveDataCall) -> Self {
            Self::GetReserveData(value)
        }
    }
    impl ::core::convert::From<GetReserveNormalizedIncomeCall> for L2PoolCalls {
        fn from(value: GetReserveNormalizedIncomeCall) -> Self {
            Self::GetReserveNormalizedIncome(value)
        }
    }
    impl ::core::convert::From<GetReserveNormalizedVariableDebtCall> for L2PoolCalls {
        fn from(value: GetReserveNormalizedVariableDebtCall) -> Self {
            Self::GetReserveNormalizedVariableDebt(value)
        }
    }
    impl ::core::convert::From<GetReservesListCall> for L2PoolCalls {
        fn from(value: GetReservesListCall) -> Self {
            Self::GetReservesList(value)
        }
    }
    impl ::core::convert::From<GetUserAccountDataCall> for L2PoolCalls {
        fn from(value: GetUserAccountDataCall) -> Self {
            Self::GetUserAccountData(value)
        }
    }
    impl ::core::convert::From<GetUserConfigurationCall> for L2PoolCalls {
        fn from(value: GetUserConfigurationCall) -> Self {
            Self::GetUserConfiguration(value)
        }
    }
    impl ::core::convert::From<GetUserEModeCall> for L2PoolCalls {
        fn from(value: GetUserEModeCall) -> Self {
            Self::GetUserEMode(value)
        }
    }
    impl ::core::convert::From<InitReserveCall> for L2PoolCalls {
        fn from(value: InitReserveCall) -> Self {
            Self::InitReserve(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for L2PoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LiquidationCallWithCollateralAssetAndDebtAssetCall>
    for L2PoolCalls {
        fn from(value: LiquidationCallWithCollateralAssetAndDebtAssetCall) -> Self {
            Self::LiquidationCallWithCollateralAssetAndDebtAsset(value)
        }
    }
    impl ::core::convert::From<LiquidationCallCall> for L2PoolCalls {
        fn from(value: LiquidationCallCall) -> Self {
            Self::LiquidationCall(value)
        }
    }
    impl ::core::convert::From<MintToTreasuryCall> for L2PoolCalls {
        fn from(value: MintToTreasuryCall) -> Self {
            Self::MintToTreasury(value)
        }
    }
    impl ::core::convert::From<MintUnbackedCall> for L2PoolCalls {
        fn from(value: MintUnbackedCall) -> Self {
            Self::MintUnbacked(value)
        }
    }
    impl ::core::convert::From<RebalanceStableBorrowRateCall> for L2PoolCalls {
        fn from(value: RebalanceStableBorrowRateCall) -> Self {
            Self::RebalanceStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<RebalanceStableBorrowRateWithAssetCall> for L2PoolCalls {
        fn from(value: RebalanceStableBorrowRateWithAssetCall) -> Self {
            Self::RebalanceStableBorrowRateWithAsset(value)
        }
    }
    impl ::core::convert::From<RepayCall> for L2PoolCalls {
        fn from(value: RepayCall) -> Self {
            Self::Repay(value)
        }
    }
    impl ::core::convert::From<RepayWithAssetCall> for L2PoolCalls {
        fn from(value: RepayWithAssetCall) -> Self {
            Self::RepayWithAsset(value)
        }
    }
    impl ::core::convert::From<RepayWithATokensWithAssetCall> for L2PoolCalls {
        fn from(value: RepayWithATokensWithAssetCall) -> Self {
            Self::RepayWithATokensWithAsset(value)
        }
    }
    impl ::core::convert::From<RepayWithATokensCall> for L2PoolCalls {
        fn from(value: RepayWithATokensCall) -> Self {
            Self::RepayWithATokens(value)
        }
    }
    impl ::core::convert::From<RepayWithPermitCall> for L2PoolCalls {
        fn from(value: RepayWithPermitCall) -> Self {
            Self::RepayWithPermit(value)
        }
    }
    impl ::core::convert::From<RepayWithPermitWithAssetAndAmountAndInterestRateModeCall>
    for L2PoolCalls {
        fn from(
            value: RepayWithPermitWithAssetAndAmountAndInterestRateModeCall,
        ) -> Self {
            Self::RepayWithPermitWithAssetAndAmountAndInterestRateMode(value)
        }
    }
    impl ::core::convert::From<RescueTokensCall> for L2PoolCalls {
        fn from(value: RescueTokensCall) -> Self {
            Self::RescueTokens(value)
        }
    }
    impl ::core::convert::From<ResetIsolationModeTotalDebtCall> for L2PoolCalls {
        fn from(value: ResetIsolationModeTotalDebtCall) -> Self {
            Self::ResetIsolationModeTotalDebt(value)
        }
    }
    impl ::core::convert::From<SetConfigurationCall> for L2PoolCalls {
        fn from(value: SetConfigurationCall) -> Self {
            Self::SetConfiguration(value)
        }
    }
    impl ::core::convert::From<SetReserveInterestRateStrategyAddressCall>
    for L2PoolCalls {
        fn from(value: SetReserveInterestRateStrategyAddressCall) -> Self {
            Self::SetReserveInterestRateStrategyAddress(value)
        }
    }
    impl ::core::convert::From<SetUserEModeCall> for L2PoolCalls {
        fn from(value: SetUserEModeCall) -> Self {
            Self::SetUserEMode(value)
        }
    }
    impl ::core::convert::From<SetUserUseReserveAsCollateralCall> for L2PoolCalls {
        fn from(value: SetUserUseReserveAsCollateralCall) -> Self {
            Self::SetUserUseReserveAsCollateral(value)
        }
    }
    impl ::core::convert::From<SetUserUseReserveAsCollateralWithAssetCall>
    for L2PoolCalls {
        fn from(value: SetUserUseReserveAsCollateralWithAssetCall) -> Self {
            Self::SetUserUseReserveAsCollateralWithAsset(value)
        }
    }
    impl ::core::convert::From<SupplyWithAssetCall> for L2PoolCalls {
        fn from(value: SupplyWithAssetCall) -> Self {
            Self::SupplyWithAsset(value)
        }
    }
    impl ::core::convert::From<SupplyCall> for L2PoolCalls {
        fn from(value: SupplyCall) -> Self {
            Self::Supply(value)
        }
    }
    impl ::core::convert::From<SupplyWithPermitWithAssetAndAmountAndOnBehalfOfCall>
    for L2PoolCalls {
        fn from(value: SupplyWithPermitWithAssetAndAmountAndOnBehalfOfCall) -> Self {
            Self::SupplyWithPermitWithAssetAndAmountAndOnBehalfOf(value)
        }
    }
    impl ::core::convert::From<SupplyWithPermitCall> for L2PoolCalls {
        fn from(value: SupplyWithPermitCall) -> Self {
            Self::SupplyWithPermit(value)
        }
    }
    impl ::core::convert::From<SwapBorrowRateModeCall> for L2PoolCalls {
        fn from(value: SwapBorrowRateModeCall) -> Self {
            Self::SwapBorrowRateMode(value)
        }
    }
    impl ::core::convert::From<SwapBorrowRateModeWithAssetCall> for L2PoolCalls {
        fn from(value: SwapBorrowRateModeWithAssetCall) -> Self {
            Self::SwapBorrowRateModeWithAsset(value)
        }
    }
    impl ::core::convert::From<UpdateBridgeProtocolFeeCall> for L2PoolCalls {
        fn from(value: UpdateBridgeProtocolFeeCall) -> Self {
            Self::UpdateBridgeProtocolFee(value)
        }
    }
    impl ::core::convert::From<UpdateFlashloanPremiumsCall> for L2PoolCalls {
        fn from(value: UpdateFlashloanPremiumsCall) -> Self {
            Self::UpdateFlashloanPremiums(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAssetCall> for L2PoolCalls {
        fn from(value: WithdrawWithAssetCall) -> Self {
            Self::WithdrawWithAsset(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for L2PoolCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
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
    pub struct AddressesProviderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BRIDGE_PROTOCOL_FEE` function with signature `BRIDGE_PROTOCOL_FEE()` and selector `0x272d9072`
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
    pub struct BridgeProtocolFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `FLASHLOAN_PREMIUM_TOTAL` function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `0x074b2e43`
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
    pub struct FlashloanPremiumTotalReturn(pub u128);
    ///Container type for all return fields from the `FLASHLOAN_PREMIUM_TO_PROTOCOL` function with signature `FLASHLOAN_PREMIUM_TO_PROTOCOL()` and selector `0x6a99c036`
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
    pub struct FlashloanPremiumToProtocolReturn(pub u128);
    ///Container type for all return fields from the `MAX_NUMBER_RESERVES` function with signature `MAX_NUMBER_RESERVES()` and selector `0xf8119d51`
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
    pub struct MaxNumberReservesReturn(pub u16);
    ///Container type for all return fields from the `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `0xe82fec2f`
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
    pub struct MaxStableRateBorrowSizePercentReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `POOL_REVISION` function with signature `POOL_REVISION()` and selector `0x0148170e`
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
    pub struct PoolRevisionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `backUnbacked` function with signature `backUnbacked(address,uint256,uint256)` and selector `0xd65dc7a1`
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
    pub struct BackUnbackedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getConfiguration` function with signature `getConfiguration(address)` and selector `0xc44b11f7`
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
    pub struct GetConfigurationReturn(pub ReserveConfigurationMap);
    ///Container type for all return fields from the `getEModeCategoryData` function with signature `getEModeCategoryData(uint8)` and selector `0x6c6f6ae1`
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
    pub struct GetEModeCategoryDataReturn(pub EmodeCategory);
    ///Container type for all return fields from the `getReserveAddressById` function with signature `getReserveAddressById(uint16)` and selector `0x52751797`
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
    pub struct GetReserveAddressByIdReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getReserveData` function with signature `getReserveData(address)` and selector `0x35ea6a75`
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
    pub struct GetReserveDataReturn(pub ReserveData);
    ///Container type for all return fields from the `getReserveNormalizedIncome` function with signature `getReserveNormalizedIncome(address)` and selector `0xd15e0053`
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
    pub struct GetReserveNormalizedIncomeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReserveNormalizedVariableDebt` function with signature `getReserveNormalizedVariableDebt(address)` and selector `0x386497fd`
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
    pub struct GetReserveNormalizedVariableDebtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReservesList` function with signature `getReservesList()` and selector `0xd1946dbc`
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
    pub struct GetReservesListReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getUserAccountData` function with signature `getUserAccountData(address)` and selector `0xbf92857c`
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
    pub struct GetUserAccountDataReturn {
        pub total_collateral_base: ::ethers::core::types::U256,
        pub total_debt_base: ::ethers::core::types::U256,
        pub available_borrows_base: ::ethers::core::types::U256,
        pub current_liquidation_threshold: ::ethers::core::types::U256,
        pub ltv: ::ethers::core::types::U256,
        pub health_factor: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getUserConfiguration` function with signature `getUserConfiguration(address)` and selector `0x4417a583`
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
    pub struct GetUserConfigurationReturn(pub UserConfigurationMap);
    ///Container type for all return fields from the `getUserEMode` function with signature `getUserEMode(address)` and selector `0xeddf1b79`
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
    pub struct GetUserEModeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repay` function with signature `repay(bytes32)` and selector `0x563dd613`
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
    pub struct RepayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repay` function with signature `repay(address,uint256,uint256,address)` and selector `0x573ade81`
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
    pub struct RepayWithAssetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithATokens` function with signature `repayWithATokens(address,uint256,uint256)` and selector `0x2dad97d4`
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
    pub struct RepayWithATokensWithAssetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithATokens` function with signature `repayWithATokens(bytes32)` and selector `0xdc7c0bff`
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
    pub struct RepayWithATokensReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithPermit` function with signature `repayWithPermit(bytes32,bytes32,bytes32)` and selector `0x94b576de`
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
    pub struct RepayWithPermitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `repayWithPermit` function with signature `repayWithPermit(address,uint256,uint256,address,uint256,uint8,bytes32,bytes32)` and selector `0xee3e210b`
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
    pub struct RepayWithPermitWithAssetAndAmountAndInterestRateModeReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(address,uint256,address)` and selector `0x69328dec`
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
    pub struct WithdrawWithAssetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(bytes32)` and selector `0x8e19899e`
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
    pub struct WithdrawReturn(pub ::ethers::core::types::U256);
}
