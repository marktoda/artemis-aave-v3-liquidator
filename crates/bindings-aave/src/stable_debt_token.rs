pub use stable_debt_token::*;
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
pub mod stable_debt_token {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEBT_TOKEN_REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEBT_TOKEN_REVISION",
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
                    ::std::borrow::ToOwned::to_owned("DELEGATION_WITH_SIG_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DELEGATION_WITH_SIG_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EIP712_REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EIP712_REVISION"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UNDERLYING_ASSET_ADDRESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UNDERLYING_ASSET_ADDRESS",
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approveDelegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approveDelegation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("borrowAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fromUser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toUser"),
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
                    ::std::borrow::ToOwned::to_owned("burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegationWithSig"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegationWithSig"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("getAverageStableRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAverageStableRate",
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
                    ::std::borrow::ToOwned::to_owned("getIncentivesController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getIncentivesController",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAaveIncentivesController",
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
                    ::std::borrow::ToOwned::to_owned("getSupplyData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSupplyData"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalSupplyAndAvgRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalSupplyAndAvgRate",
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
                    ::std::borrow::ToOwned::to_owned("getTotalSupplyLastUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalSupplyLastUpdated",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserLastUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserLastUpdated"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getUserStableRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserStableRate"),
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
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
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
                                    name: ::std::borrow::ToOwned::to_owned("initializingPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "incentivesController",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAaveIncentivesController",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rate"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("principalBalanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("principalBalanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("setIncentivesController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setIncentivesController",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAaveIncentivesController",
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("BorrowAllowanceDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BorrowAllowanceDelegated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromUser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toUser"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
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
                                    name: ::std::borrow::ToOwned::to_owned("currentBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("balanceIncrease"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("avgStableRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTotalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("underlyingAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "incentivesController",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("currentBalance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("balanceIncrease"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("avgStableRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newTotalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
    pub static STABLEDEBTTOKEN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R`\0\x80U4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0#\xA08\x03\x80b\0#\xA0\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x024V[\x80`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7FSTABLE_DEBT_TOKEN_IMPL\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01\x7FSTABLE_DEBT_TOKEN_IMPL\0\0\0\0\0\0\0\0\0\0\x81RP`\0F`\x80\x81\x81RPP\x83`\x01`\x01`\xA0\x1B\x03\x16c\x05B\x97\\`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x14\x91\x90b\0\x024V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0R\x82Qb\0\x015\x90`;\x90` \x86\x01\x90b\0\x01uV[P\x81Qb\0\x01K\x90`<\x90` \x85\x01\x90b\0\x01uV[P`=\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01`\x01`\xA0\x1B\x03\x16`\xC0RPb\0\x02\x98V[\x82\x80Tb\0\x01\x83\x90b\0\x02[V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x01\xA7W`\0\x85Ub\0\x01\xF2V[\x82`\x1F\x10b\0\x01\xC2W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x01\xF2V[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x01\xF2W\x91\x82\x01[\x82\x81\x11\x15b\0\x01\xF2W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x01\xD5V[Pb\0\x02\0\x92\x91Pb\0\x02\x04V[P\x90V[[\x80\x82\x11\x15b\0\x02\0W`\0\x81U`\x01\x01b\0\x02\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x021W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x02GW`\0\x80\xFD[\x81Qb\0\x02T\x81b\0\x02\x1BV[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02pW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15b\0\x02\x92WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa \xBCb\0\x02\xE4`\09`\0\x81\x81a\x02\xE8\x01R\x81\x81a\t\xA3\x01R\x81\x81a\rB\x01R\x81\x81a\x11B\x01Ra\x12\x11\x01R`\0a\x12\xB6\x01R`\0a\x08q\x01Ra \xBC`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80c\x90\xF6\xFC\xF2\x11a\x01\x1AW\x80c\xC0J\x8A\x10\x11a\0\xADW\x80c\xE6U\xDB\xD8\x11a\0|W\x80c\xE6U\xDB\xD8\x14a\x04\xC7W\x80c\xE7HH\x90\x14a\x04\xDAW\x80c\xE7\x8C\x9B;\x14a\x04\xF0W\x80c\xF3\xBF\xC78\x14a\x05)W\x80c\xF71\xE9\xBE\x14a\x05PW`\0\x80\xFD[\x80c\xC0J\x8A\x10\x14a\x04\x80W\x80c\xC2\"\xEC\x8A\x14a\x04\x93W\x80c\xC64\xDF\xAA\x14a\x04\xA6W\x80c\xDDb\xED>\x14a\x04\xB9W`\0\x80\xFD[\x80c\xA9\x05\x9C\xBB\x11a\0\xE9W\x80c\xA9\x05\x9C\xBB\x14a\x02\x1EW\x80c\xB1j\x19\xDE\x14a\x047W\x80c\xB3\xF1\xC9=\x14a\x04HW\x80c\xB9\xA7\xB6\"\x14a\x04xW`\0\x80\xFD[\x80c\x90\xF6\xFC\xF2\x14a\x03\xF6W\x80c\x95\xD8\x9BA\x14a\x04\x07W\x80c\x9D\xC2\x9F\xAC\x14a\x04\x0FW\x80c\xA4W\xC2\xD7\x14a\x02\x1EW`\0\x80\xFD[\x80ck\xD7m$\x11a\x01\x92W\x80cx\x16\x03v\x11a\x01aW\x80cx\x16\x03v\x14a\x038W\x80cywC8\x14a\x03XW\x80cy\xCEk\x8C\x14a\x03\x87W\x80c~\xCE\xBE\0\x14a\x03\xCDW`\0\x80\xFD[\x80ck\xD7m$\x14a\x02\x97W\x80cp\xA0\x821\x14a\x02\xD0W\x80cu5\xD2F\x14a\x02\xE3W\x80cu\xD2d\x13\x14a\x03\"W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xCEW\x80c#\xB8r\xDD\x14a\x02lW\x80c1<\xE5g\x14a\x02zW\x80c6D\xE5\x15\x14a\x02\x8FW\x80c9P\x93Q\x14a\x02\x1EW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\0W\x80c\t^\xA7\xB3\x14a\x02\x1EW\x80c\x0BR\xD5X\x14a\x02AW\x80c\x18\x16\r\xDD\x14a\x02VW[`\0\x80\xFD[a\x02\x08a\x05XV[`@Qa\x02\x15\x91\x90a\x1B.V[`@Q\x80\x91\x03\x90\xF3[a\x021a\x02,6`\x04a\x1BdV[a\x05\xEAV[`@Q\x90\x15\x15\x81R` \x01a\x02\x15V[a\x02Ta\x02O6`\x04a\x1B\xA1V[a\x06%V[\0[a\x02^a\x08PV[`@Q\x90\x81R` \x01a\x02\x15V[a\x021a\x02,6`\x04a\x1C\x0FV[`=T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x15V[a\x02^a\x08mV[a\x02^a\x02\xA56`\x04a\x1CPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02^a\x02\xDE6`\x04a\x1C\x89V[a\x08\xA6V[a\x03\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x15V[`=Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[a\x02\x08`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x03`a\t-V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01a\x02\x15V[a\x03\xB7a\x03\x956`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`>` R`@\x90 Td\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x15V[a\x02^a\x03\xDB6`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[`?T`\x01`\x01`\x80\x1B\x03\x16a\x02^V[a\x02\x08a\tsV[a\x04\"a\x04\x1D6`\x04a\x1BdV[a\t\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x15V[`7T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[a\x04[a\x04V6`\x04a\x1C\xA6V[a\r4V[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x15V[a\x02^`\x01\x81V[a\x02Ta\x04\x8E6`\x04a\x1BdV[a\x10\x92V[a\x02Ta\x04\xA16`\x04a\x1D\xD1V[a\x10\xA1V[a\x02^a\x04\xB46`\x04a\x1C\x89V[a\x12\xA1V[a\x02^a\x02,6`\x04a\x1CPV[a\x02Ta\x04\xD56`\x04a\x1C\x89V[a\x12\xB2V[`?T`\x01`\x80\x1B\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16a\x03\xB7V[a\x02^a\x04\xFE6`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`8` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x02^\x7F2=\xB0A\x0F\xEC\xC1\x07\xE3\x9E*\xF5\x90\x86q\xF4\xC8\xD1\x06\x12;5\xA5\x15\x01\xBB\x80\\_\xA3j\xA0\x81V[a\x04\"a\x14\x03V[```;\x80Ta\x05g\x90a\x1E\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x93\x90a\x1E\xA6V[\x80\x15a\x05\xE0W\x80`\x1F\x10a\x05\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x02\x81Ra\x03\x83`\xF4\x1B` \x82\x01R\x90QbF\x1B\xCD`\xE5\x1B\x81R`\0\x91a\x06\x1C\x91`\x04\x01a\x1B.V[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra77`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16a\x06eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P\x83B\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x90a\x06\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x90a\x06\xC6a\x08mV[`@\x80Q\x7F2=\xB0A\x0F\xEC\xC1\x07\xE3\x9E*\xF5\x90\x86q\xF4\xC8\xD1\x06\x12;5\xA5\x15\x01\xBB\x80\\_\xA3j\xA0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x88\x90R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x07V\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83R\x81\x90R`\xFF\x88\x16\x91\x83\x01\x91\x90\x91R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\xBEW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x90a\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\x08!\x82`\x01a\x1E\xF7V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`4` R`@\x90 Ua\x08E\x89\x89\x89a\x14%V[PPPPPPPPPV[`?T`\0\x90a\x08h\x90`\x01`\x01`\x80\x1B\x03\x16a\x14\x8FV[\x90P\x90V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x08\x9EWP`5T\x90V[a\x08ha\x14\xDEV[`\0\x80a\x08\xB2\x83a\x15\x87V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x81a\x08\xEDWP`\0\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`>` R`@\x81 Ta\t\x18\x90\x83\x90d\xFF\xFF\xFF\xFF\xFF\x16a\x15\xABV[\x90Pa\t$\x83\x82a\x15\xBFV[\x95\x94PPPPPV[`?T`\0\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\x80\x1B\x03\x16a\tK`:T\x90V[a\tT\x82a\x14\x8FV[`?T\x91\x97\x90\x96P\x91\x94P`\x01`\x80\x1B\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x92P\x90PV[```<\x80Ta\x05g\x90a\x1E\xA6V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R`\0\x90\x81\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P`\0\x80a\t\xF6\x86a\x16\x03V[\x92P\x92PP`\0a\n\x05a\x08PV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`8` R`@\x81 T\x91\x92P\x90\x81\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x88\x84\x11a\nUW`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U`\0`:Ua\x0B\rV[a\n_\x89\x85a\x1F\x0FV[`:\x81\x90U\x91P`\0a\n\x86a\nt\x86a\x16SV[`?T`\x01`\x01`\x80\x1B\x03\x16\x90a\x15\xBFV[\x90P`\0a\n\x9Da\n\x96\x8Ca\x16SV[\x84\x90a\x15\xBFV[\x90P\x81\x81\x10a\n\xC4W`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U`\0`:\x81\x90U\x94Pa\x0B\nV[a\n\xE8a\n\xE3a\n\xD3\x86a\x16SV[a\n\xDD\x84\x86a\x1F\x0FV[\x90a\x16nV[a\x16\xADV[`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U\x94P[PP[\x85\x89\x14\x15a\x0BUW`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x90U`>\x90\x91R\x90 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0B\x83V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`>` R`@\x90 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16Bd\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U[`?\x80Td\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1BBd\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90U\x88\x85\x11\x15a\x0CnW`\0a\x0B\xB7\x8A\x87a\x1F\x0FV[\x90Pa\x0B\xC4\x8B\x82\x87a\x17\x1AV[`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x81\x90\x7F\xC1oNL\xA3My\r\xE4\xC6V\xC7/\xD0\x15\xC6g\xD6\x88\xF2\x0B\xE6N\xEA6\x06\x18T\\LS\x0F\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA3Pa\r$V[`\0a\x0Cz\x86\x8Ba\x1F\x0FV[\x90Pa\x0C\x87\x8B\x82\x87a\x18\x18V[`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7FD\xBD \xA7\x9E\x99;\xDC\xC7\xCB\xED\xF5J;M\x19\xFBxI\x01$\xB6\xB9\r\x04\xFE2B\xEE\xA5y\xE8\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA2P[P\x95P\x93PPPP[\x92P\x92\x90PV[`\0\x80\x80`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a23`\xF0\x1B\x81RP\x90a\r\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\r\xE0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x04Wa\x0E\x04\x87\x89\x88a\x18RV[`\0\x80a\x0E\x10\x89a\x16\x03V[\x92P\x92PPa\x0E\x1Da\x08PV[\x80\x84R`?T`\x01`\x01`\x80\x1B\x03\x16`\xA0\x85\x01Ra\x0E<\x90\x89\x90a\x1E\xF7V[`:\x81\x90U` \x84\x01Ra\x0EO\x88a\x16SV[`@\x84\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` R T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16``\x84\x01Ra\x0E\xC8a\x0E\x98a\x0E\x93\x8A\x85a\x1E\xF7V[a\x16SV[`@\x85\x01Qa\x0E\xA7\x90\x8Aa\x15\xBFV[a\x0E\xBEa\x0E\xB3\x86a\x16SV[``\x88\x01Q\x90a\x15\xBFV[a\n\xDD\x91\x90a\x1E\xF7V[`\x80\x84\x01\x81\x90Ra\x0E\xD8\x90a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x96\x90\x91\x16\x86\x02\x17\x90U`>\x82R\x90\x91 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16Bd\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x17\x90\x91U`?\x80Td\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16\x91\x90\x93\x02\x17\x90\x91U\x83\x01Qa\x0F\x83\x90a\n\xE3\x90a\x0FX\x90a\x16SV[`@\x86\x01Qa\x0Fh\x90\x8B\x90a\x15\xBFV[a\x0E\xBEa\x0Fx\x88`\0\x01Qa\x16SV[`\xA0\x89\x01Q\x90a\x15\xBFV[`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`\xA0\x84\x01R`\0a\x0F\xB3\x82\x8Aa\x1E\xF7V[\x90Pa\x0F\xC4\x8A\x82\x86`\0\x01Qa\x17\x1AV[`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`\x80\x80\x85\x01Q`\xA0\x80\x87\x01Q` \x80\x89\x01Q`@\x80Q\x88\x81R\x92\x83\x01\x8A\x90R\x82\x01\x88\x90R``\x82\x01\x94\x90\x94R\x93\x84\x01R\x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16\x91\x90\x8D\x16\x90\x7F\xC1oNL\xA3My\r\xE4\xC6V\xC7/\xD0\x15\xC6g\xD6\x88\xF2\x0B\xE6N\xEA6\x06\x18T\\LS\x0F\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA3PP` \x82\x01Q`\xA0\x90\x92\x01Q\x90\x15\x99\x91\x98P\x96P\x94PPPPPV[a\x10\x9D3\x83\x83a\x14%V[PPV[`\x01\x80T`\xFF\x16\x80a\x10\xB2WP0;\x15[\x80a\x10\xBEWP`\0T\x81\x11[a\x11!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\x1CV[`\x01T`\xFF\x16\x15\x80\x15a\x11@W`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x90a\x11\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\x11\xB8\x86a\x18\xF8V[a\x11\xC1\x85a\x19\x0BV[`=\x80T`7\x80T`\x01`\x01`\xA0\x1B\x03\x8D\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U\x8A\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\xFF\x8A\x16\x17\x17\x90Ua\x12\ta\x14\xDEV[`5\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F@%\x1F\xBF\xB6el\xFAe\xA0\rxy\x02\x9F\xEC\x1F\xAD!\xD2\x8F\xDC\xFF/Oh\xF5'\x95\xB7O,\x8A\x8A\x8A\x8A\x8A\x8A`@Qa\x12|\x96\x95\x94\x93\x92\x91\x90a\x1F&V[`@Q\x80\x91\x03\x90\xA3\x80\x15a\x12\x95W`\x01\x80T`\xFF\x19\x16\x90U[PPPPPPPPPPV[`\0a\x12\xAC\x82a\x15\x87V[\x92\x91PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x136\x91\x90a\x1F\x9DV[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA1\x91\x90a\x1F\xBAV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x13\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[PP`=\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`?T`\0\x90\x81\x90`\x01`\x01`\x80\x1B\x03\x16a\x14\x1D\x81a\x14\x8FV[\x93\x90\x92P\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x87\x86\x16\x80\x85R\x90\x83R\x92\x81\x90 \x86\x90U`7T\x90Q\x86\x81R\x94\x16\x93\x91\x92\x91\x7F\xDA\x91\x93`C2 \xE1;Q\xE8\xC2\x11\xE4\x90\xD1H\xE6\x1A;\xD5=\xE8\xC0\x97\x19NE\x8B\x97\xF3\xE1\x91\x01`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x80a\x14\x9B`:T\x90V[\x90P\x80a\x14\xABWP`\0\x92\x91PPV[`\0a\x14\xCA\x84`?`\x10\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16a\x15\xABV[\x90Pa\x14\xD6\x82\x82a\x15\xBFV[\x94\x93PPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x15\ta\x19\x1EV[\x80Q` \x91\x82\x01 `@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B\x90\x84\x01R\x80Q\x92\x83\x01\x93\x90\x93R\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`8` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16\x90V[`\0a\x15\xB8\x83\x83Ba\x19(V[\x93\x92PPPV[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x15\xE1W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x80`\0\x80a\x16\x12\x85a\x15\x87V[\x90P\x80a\x16*W`\0\x80`\0\x93P\x93P\x93PPa\x16LV[`\0a\x165\x86a\x08\xA6V[\x90P\x81\x81a\x16C\x82\x82a\x1F\x0FV[\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\x16iW`\0\x80\xFD[\x91\x90PV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x16\x92W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x06\x1CV[P\x90V[`\0a\x17%\x83a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x01`\x80\x1B\x03\x16a\x17T\x82\x82a\x1F\xDCV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`8` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`=Ta\x01\0\x90\x04\x16\x15a\x18\x11W`=T`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R`\x01`\x01`\x80\x1B\x03\x84\x16`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c1\x87>.\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08EW=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x18#\x83a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x01`\x80\x1B\x03\x16a\x17T\x82\x82a \x07V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 Ta\x18\x85\x90\x83\x90a\x1F\x0FV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x89\x86\x16\x80\x85R\x92R\x91\x82\x90 \x85\x90U`7T\x91Q\x94\x95P\x92\x16\x92\x7F\xDA\x91\x93`C2 \xE1;Q\xE8\xC2\x11\xE4\x90\xD1H\xE6\x1A;\xD5=\xE8\xC0\x97\x19NE\x8B\x97\xF3\xE1\x90a\x18\xEA\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[\x80Qa\x10\x9D\x90`;\x90` \x84\x01\x90a\x1AQV[\x80Qa\x10\x9D\x90`<\x90` \x84\x01\x90a\x1AQV[``a\x08ha\x05XV[`\0\x80a\x19<d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84a\x1F\x0FV[\x90P\x80a\x19XWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91PPa\x15\xB8V[`\0\x19\x81\x01`\0\x80\x80`\x02\x85\x11a\x19pW`\0a\x19uV[`\x02\x85\x03[\x92Pf\x03\x88\x82\x91\\@\0a\x19\x89\x8A\x80a\x15\xBFV[\x81a\x19\x96Wa\x19\x96a /V[\x04\x91Pc\x01\xE13\x80a\x19\xA8\x83\x8Ba\x15\xBFV[\x81a\x19\xB5Wa\x19\xB5a /V[\x04\x90P`\0\x82a\x19\xC5\x86\x88a EV[a\x19\xCF\x91\x90a EV[`\x02\x90\x04\x90P`\0\x82\x85a\x19\xE3\x88\x8Aa EV[a\x19\xED\x91\x90a EV[a\x19\xF7\x91\x90a EV[`\x06\x90\x04\x90P\x80\x82c\x01\xE13\x80a\x1A\x0E\x8A\x8Fa EV[a\x1A\x18\x91\x90a dV[a\x1A.\x90k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x1E\xF7V[a\x1A8\x91\x90a\x1E\xF7V[a\x1AB\x91\x90a\x1E\xF7V[\x9B\x9APPPPPPPPPPPV[\x82\x80Ta\x1A]\x90a\x1E\xA6V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x1A\x7FW`\0\x85Ua\x1A\xC5V[\x82`\x1F\x10a\x1A\x98W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x1A\xC5V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x1A\xC5W\x91\x82\x01[\x82\x81\x11\x15a\x1A\xC5W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1A\xAAV[Pa\x17\x16\x92\x91P[\x80\x82\x11\x15a\x17\x16W`\0\x81U`\x01\x01a\x1A\xCDV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1B\x07W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1A\xEBV[\x81\x81\x11\x15a\x1B\x19W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x15\xB8` \x83\x01\x84a\x1A\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1BVW`\0\x80\xFD[PV[\x805a\x16i\x81a\x1BAV[`\0\x80`@\x83\x85\x03\x12\x15a\x1BwW`\0\x80\xFD[\x825a\x1B\x82\x81a\x1BAV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\xFF\x81\x16\x81\x14a\x16iW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x1B\xBCW`\0\x80\xFD[\x875a\x1B\xC7\x81a\x1BAV[\x96P` \x88\x015a\x1B\xD7\x81a\x1BAV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x1B\xF3`\x80\x89\x01a\x1B\x90V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C$W`\0\x80\xFD[\x835a\x1C/\x81a\x1BAV[\x92P` \x84\x015a\x1C?\x81a\x1BAV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1CcW`\0\x80\xFD[\x825a\x1Cn\x81a\x1BAV[\x91P` \x83\x015a\x1C~\x81a\x1BAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1C\x9BW`\0\x80\xFD[\x815a\x15\xB8\x81a\x1BAV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C\xBCW`\0\x80\xFD[\x845a\x1C\xC7\x81a\x1BAV[\x93P` \x85\x015a\x1C\xD7\x81a\x1BAV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\x13W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D.Wa\x1D.a\x1C\xECV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1DVWa\x1DVa\x1C\xECV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x1DoW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x1D\xA1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xB9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r-W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a\x1D\xEDW`\0\x80\xFD[\x885a\x1D\xF8\x81a\x1BAV[\x97P` \x89\x015a\x1E\x08\x81a\x1BAV[\x96Pa\x1E\x16`@\x8A\x01a\x1BYV[\x95Pa\x1E$``\x8A\x01a\x1B\x90V[\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1EAW`\0\x80\xFD[a\x1EM\x8C\x83\x8D\x01a\x1D\x02V[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x1EcW`\0\x80\xFD[a\x1Eo\x8C\x83\x8D\x01a\x1D\x02V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15a\x1E\x85W`\0\x80\xFD[Pa\x1E\x92\x8B\x82\x8C\x01a\x1D\x8FV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\xBAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1E\xDBWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1F\nWa\x1F\na\x1E\xE1V[P\x01\x90V[`\0\x82\x82\x10\x15a\x1F!Wa\x1F!a\x1E\xE1V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xFF\x86\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90a\x1FS\x90\x83\x01\x87a\x1A\xE1V[\x82\x81\x03``\x84\x01Ra\x1Fe\x81\x87a\x1A\xE1V[\x90P\x82\x81\x03`\x80\x84\x01R\x83\x81R\x83\x85` \x83\x017`\0` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xAFW`\0\x80\xFD[\x81Qa\x15\xB8\x81a\x1BAV[`\0` \x82\x84\x03\x12\x15a\x1F\xCCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x15\xB8W`\0\x80\xFD[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\xFEWa\x1F\xFEa\x1E\xE1V[\x01\x94\x93PPPPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a 'Wa 'a\x1E\xE1V[\x03\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a _Wa _a\x1E\xE1V[P\x02\x90V[`\0\x82a \x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9C0g\xE7\xD0\xC9\xB6\xCF\xE6\x8F\xFE\x06\x1F\x8B-R\xD44gz\xA1\xA9\x81D\x9C\xC6\xA9\xD5\xC3\xFA\x9E\x15dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static STABLEDEBTTOKEN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80c\x90\xF6\xFC\xF2\x11a\x01\x1AW\x80c\xC0J\x8A\x10\x11a\0\xADW\x80c\xE6U\xDB\xD8\x11a\0|W\x80c\xE6U\xDB\xD8\x14a\x04\xC7W\x80c\xE7HH\x90\x14a\x04\xDAW\x80c\xE7\x8C\x9B;\x14a\x04\xF0W\x80c\xF3\xBF\xC78\x14a\x05)W\x80c\xF71\xE9\xBE\x14a\x05PW`\0\x80\xFD[\x80c\xC0J\x8A\x10\x14a\x04\x80W\x80c\xC2\"\xEC\x8A\x14a\x04\x93W\x80c\xC64\xDF\xAA\x14a\x04\xA6W\x80c\xDDb\xED>\x14a\x04\xB9W`\0\x80\xFD[\x80c\xA9\x05\x9C\xBB\x11a\0\xE9W\x80c\xA9\x05\x9C\xBB\x14a\x02\x1EW\x80c\xB1j\x19\xDE\x14a\x047W\x80c\xB3\xF1\xC9=\x14a\x04HW\x80c\xB9\xA7\xB6\"\x14a\x04xW`\0\x80\xFD[\x80c\x90\xF6\xFC\xF2\x14a\x03\xF6W\x80c\x95\xD8\x9BA\x14a\x04\x07W\x80c\x9D\xC2\x9F\xAC\x14a\x04\x0FW\x80c\xA4W\xC2\xD7\x14a\x02\x1EW`\0\x80\xFD[\x80ck\xD7m$\x11a\x01\x92W\x80cx\x16\x03v\x11a\x01aW\x80cx\x16\x03v\x14a\x038W\x80cywC8\x14a\x03XW\x80cy\xCEk\x8C\x14a\x03\x87W\x80c~\xCE\xBE\0\x14a\x03\xCDW`\0\x80\xFD[\x80ck\xD7m$\x14a\x02\x97W\x80cp\xA0\x821\x14a\x02\xD0W\x80cu5\xD2F\x14a\x02\xE3W\x80cu\xD2d\x13\x14a\x03\"W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x01\xCEW\x80c#\xB8r\xDD\x14a\x02lW\x80c1<\xE5g\x14a\x02zW\x80c6D\xE5\x15\x14a\x02\x8FW\x80c9P\x93Q\x14a\x02\x1EW`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\0W\x80c\t^\xA7\xB3\x14a\x02\x1EW\x80c\x0BR\xD5X\x14a\x02AW\x80c\x18\x16\r\xDD\x14a\x02VW[`\0\x80\xFD[a\x02\x08a\x05XV[`@Qa\x02\x15\x91\x90a\x1B.V[`@Q\x80\x91\x03\x90\xF3[a\x021a\x02,6`\x04a\x1BdV[a\x05\xEAV[`@Q\x90\x15\x15\x81R` \x01a\x02\x15V[a\x02Ta\x02O6`\x04a\x1B\xA1V[a\x06%V[\0[a\x02^a\x08PV[`@Q\x90\x81R` \x01a\x02\x15V[a\x021a\x02,6`\x04a\x1C\x0FV[`=T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x15V[a\x02^a\x08mV[a\x02^a\x02\xA56`\x04a\x1CPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02^a\x02\xDE6`\x04a\x1C\x89V[a\x08\xA6V[a\x03\n\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x15V[`=Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[a\x02\x08`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x03`a\t-V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x80\x01a\x02\x15V[a\x03\xB7a\x03\x956`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`>` R`@\x90 Td\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x15V[a\x02^a\x03\xDB6`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[`?T`\x01`\x01`\x80\x1B\x03\x16a\x02^V[a\x02\x08a\tsV[a\x04\"a\x04\x1D6`\x04a\x1BdV[a\t\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x15V[`7T`\x01`\x01`\xA0\x1B\x03\x16a\x03\nV[a\x04[a\x04V6`\x04a\x1C\xA6V[a\r4V[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x15V[a\x02^`\x01\x81V[a\x02Ta\x04\x8E6`\x04a\x1BdV[a\x10\x92V[a\x02Ta\x04\xA16`\x04a\x1D\xD1V[a\x10\xA1V[a\x02^a\x04\xB46`\x04a\x1C\x89V[a\x12\xA1V[a\x02^a\x02,6`\x04a\x1CPV[a\x02Ta\x04\xD56`\x04a\x1C\x89V[a\x12\xB2V[`?T`\x01`\x80\x1B\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16a\x03\xB7V[a\x02^a\x04\xFE6`\x04a\x1C\x89V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`8` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x02^\x7F2=\xB0A\x0F\xEC\xC1\x07\xE3\x9E*\xF5\x90\x86q\xF4\xC8\xD1\x06\x12;5\xA5\x15\x01\xBB\x80\\_\xA3j\xA0\x81V[a\x04\"a\x14\x03V[```;\x80Ta\x05g\x90a\x1E\xA6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x93\x90a\x1E\xA6V[\x80\x15a\x05\xE0W\x80`\x1F\x10a\x05\xB5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xE0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xC3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x82R`\x02\x81Ra\x03\x83`\xF4\x1B` \x82\x01R\x90QbF\x1B\xCD`\xE5\x1B\x81R`\0\x91a\x06\x1C\x91`\x04\x01a\x1B.V[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra77`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16a\x06eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P\x83B\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x90a\x06\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x90a\x06\xC6a\x08mV[`@\x80Q\x7F2=\xB0A\x0F\xEC\xC1\x07\xE3\x9E*\xF5\x90\x86q\xF4\xC8\xD1\x06\x12;5\xA5\x15\x01\xBB\x80\\_\xA3j\xA0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x88\x90R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x07V\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83R\x81\x90R`\xFF\x88\x16\x91\x83\x01\x91\x90\x91R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\xBEW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x90a\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\x08!\x82`\x01a\x1E\xF7V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`4` R`@\x90 Ua\x08E\x89\x89\x89a\x14%V[PPPPPPPPPV[`?T`\0\x90a\x08h\x90`\x01`\x01`\x80\x1B\x03\x16a\x14\x8FV[\x90P\x90V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x08\x9EWP`5T\x90V[a\x08ha\x14\xDEV[`\0\x80a\x08\xB2\x83a\x15\x87V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x81a\x08\xEDWP`\0\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`>` R`@\x81 Ta\t\x18\x90\x83\x90d\xFF\xFF\xFF\xFF\xFF\x16a\x15\xABV[\x90Pa\t$\x83\x82a\x15\xBFV[\x95\x94PPPPPV[`?T`\0\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\x80\x1B\x03\x16a\tK`:T\x90V[a\tT\x82a\x14\x8FV[`?T\x91\x97\x90\x96P\x91\x94P`\x01`\x80\x1B\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x92P\x90PV[```<\x80Ta\x05g\x90a\x1E\xA6V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R`\0\x90\x81\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[P`\0\x80a\t\xF6\x86a\x16\x03V[\x92P\x92PP`\0a\n\x05a\x08PV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`8` R`@\x81 T\x91\x92P\x90\x81\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x88\x84\x11a\nUW`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U`\0`:Ua\x0B\rV[a\n_\x89\x85a\x1F\x0FV[`:\x81\x90U\x91P`\0a\n\x86a\nt\x86a\x16SV[`?T`\x01`\x01`\x80\x1B\x03\x16\x90a\x15\xBFV[\x90P`\0a\n\x9Da\n\x96\x8Ca\x16SV[\x84\x90a\x15\xBFV[\x90P\x81\x81\x10a\n\xC4W`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x90U`\0`:\x81\x90U\x94Pa\x0B\nV[a\n\xE8a\n\xE3a\n\xD3\x86a\x16SV[a\n\xDD\x84\x86a\x1F\x0FV[\x90a\x16nV[a\x16\xADV[`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U\x94P[PP[\x85\x89\x14\x15a\x0BUW`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x16\x90U`>\x90\x91R\x90 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16\x90Ua\x0B\x83V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`>` R`@\x90 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16Bd\xFF\xFF\xFF\xFF\xFF\x16\x17\x90U[`?\x80Td\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1BBd\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90U\x88\x85\x11\x15a\x0CnW`\0a\x0B\xB7\x8A\x87a\x1F\x0FV[\x90Pa\x0B\xC4\x8B\x82\x87a\x17\x1AV[`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x81\x90\x7F\xC1oNL\xA3My\r\xE4\xC6V\xC7/\xD0\x15\xC6g\xD6\x88\xF2\x0B\xE6N\xEA6\x06\x18T\\LS\x0F\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA3Pa\r$V[`\0a\x0Cz\x86\x8Ba\x1F\x0FV[\x90Pa\x0C\x87\x8B\x82\x87a\x18\x18V[`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8D\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7FD\xBD \xA7\x9E\x99;\xDC\xC7\xCB\xED\xF5J;M\x19\xFBxI\x01$\xB6\xB9\r\x04\xFE2B\xEE\xA5y\xE8\x90`\xA0\x01`@Q\x80\x91\x03\x90\xA2P[P\x95P\x93PPPP[\x92P\x92\x90PV[`\0\x80\x80`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a23`\xF0\x1B\x81RP\x90a\r\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\r\xE0`@Q\x80`\xC0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x04Wa\x0E\x04\x87\x89\x88a\x18RV[`\0\x80a\x0E\x10\x89a\x16\x03V[\x92P\x92PPa\x0E\x1Da\x08PV[\x80\x84R`?T`\x01`\x01`\x80\x1B\x03\x16`\xA0\x85\x01Ra\x0E<\x90\x89\x90a\x1E\xF7V[`:\x81\x90U` \x84\x01Ra\x0EO\x88a\x16SV[`@\x84\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` R T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16``\x84\x01Ra\x0E\xC8a\x0E\x98a\x0E\x93\x8A\x85a\x1E\xF7V[a\x16SV[`@\x85\x01Qa\x0E\xA7\x90\x8Aa\x15\xBFV[a\x0E\xBEa\x0E\xB3\x86a\x16SV[``\x88\x01Q\x90a\x15\xBFV[a\n\xDD\x91\x90a\x1E\xF7V[`\x80\x84\x01\x81\x90Ra\x0E\xD8\x90a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`8` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x96\x90\x91\x16\x86\x02\x17\x90U`>\x82R\x90\x91 \x80Td\xFF\xFF\xFF\xFF\xFF\x19\x16Bd\xFF\xFF\xFF\xFF\xFF\x16\x90\x81\x17\x90\x91U`?\x80Td\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16\x91\x90\x93\x02\x17\x90\x91U\x83\x01Qa\x0F\x83\x90a\n\xE3\x90a\x0FX\x90a\x16SV[`@\x86\x01Qa\x0Fh\x90\x8B\x90a\x15\xBFV[a\x0E\xBEa\x0Fx\x88`\0\x01Qa\x16SV[`\xA0\x89\x01Q\x90a\x15\xBFV[`?\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`\xA0\x84\x01R`\0a\x0F\xB3\x82\x8Aa\x1E\xF7V[\x90Pa\x0F\xC4\x8A\x82\x86`\0\x01Qa\x17\x1AV[`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3`\x80\x80\x85\x01Q`\xA0\x80\x87\x01Q` \x80\x89\x01Q`@\x80Q\x88\x81R\x92\x83\x01\x8A\x90R\x82\x01\x88\x90R``\x82\x01\x94\x90\x94R\x93\x84\x01R\x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16\x91\x90\x8D\x16\x90\x7F\xC1oNL\xA3My\r\xE4\xC6V\xC7/\xD0\x15\xC6g\xD6\x88\xF2\x0B\xE6N\xEA6\x06\x18T\\LS\x0F\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA3PP` \x82\x01Q`\xA0\x90\x92\x01Q\x90\x15\x99\x91\x98P\x96P\x94PPPPPV[a\x10\x9D3\x83\x83a\x14%V[PPV[`\x01\x80T`\xFF\x16\x80a\x10\xB2WP0;\x15[\x80a\x10\xBEWP`\0T\x81\x11[a\x11!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\x1CV[`\x01T`\xFF\x16\x15\x80\x15a\x11@W`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8A`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x90a\x11\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[Pa\x11\xB8\x86a\x18\xF8V[a\x11\xC1\x85a\x19\x0BV[`=\x80T`7\x80T`\x01`\x01`\xA0\x1B\x03\x8D\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U\x8A\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\xFF\x8A\x16\x17\x17\x90Ua\x12\ta\x14\xDEV[`5\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F@%\x1F\xBF\xB6el\xFAe\xA0\rxy\x02\x9F\xEC\x1F\xAD!\xD2\x8F\xDC\xFF/Oh\xF5'\x95\xB7O,\x8A\x8A\x8A\x8A\x8A\x8A`@Qa\x12|\x96\x95\x94\x93\x92\x91\x90a\x1F&V[`@Q\x80\x91\x03\x90\xA3\x80\x15a\x12\x95W`\x01\x80T`\xFF\x19\x16\x90U[PPPPPPPPPPV[`\0a\x12\xAC\x82a\x15\x87V[\x92\x91PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x136\x91\x90a\x1F\x9DV[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xA1\x91\x90a\x1F\xBAV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x13\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x1C\x91\x90a\x1B.V[PP`=\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`?T`\0\x90\x81\x90`\x01`\x01`\x80\x1B\x03\x16a\x14\x1D\x81a\x14\x8FV[\x93\x90\x92P\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x87\x86\x16\x80\x85R\x90\x83R\x92\x81\x90 \x86\x90U`7T\x90Q\x86\x81R\x94\x16\x93\x91\x92\x91\x7F\xDA\x91\x93`C2 \xE1;Q\xE8\xC2\x11\xE4\x90\xD1H\xE6\x1A;\xD5=\xE8\xC0\x97\x19NE\x8B\x97\xF3\xE1\x91\x01`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x80a\x14\x9B`:T\x90V[\x90P\x80a\x14\xABWP`\0\x92\x91PPV[`\0a\x14\xCA\x84`?`\x10\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16a\x15\xABV[\x90Pa\x14\xD6\x82\x82a\x15\xBFV[\x94\x93PPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x15\ta\x19\x1EV[\x80Q` \x91\x82\x01 `@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B\x90\x84\x01R\x80Q\x92\x83\x01\x93\x90\x93R\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`8` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16\x90V[`\0a\x15\xB8\x83\x83Ba\x19(V[\x93\x92PPPV[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x15\xE1W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x80`\0\x80a\x16\x12\x85a\x15\x87V[\x90P\x80a\x16*W`\0\x80`\0\x93P\x93P\x93PPa\x16LV[`\0a\x165\x86a\x08\xA6V[\x90P\x81\x81a\x16C\x82\x82a\x1F\x0FV[\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\x16iW`\0\x80\xFD[\x91\x90PV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x16\x92W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x06\x1CV[P\x90V[`\0a\x17%\x83a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x01`\x80\x1B\x03\x16a\x17T\x82\x82a\x1F\xDCV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`8` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`=Ta\x01\0\x90\x04\x16\x15a\x18\x11W`=T`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R`\x01`\x01`\x80\x1B\x03\x84\x16`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c1\x87>.\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08EW=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x18#\x83a\x16\xADV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`8` R`@\x90 T\x90\x91P`\x01`\x01`\x80\x1B\x03\x16a\x17T\x82\x82a \x07V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 Ta\x18\x85\x90\x83\x90a\x1F\x0FV[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x89\x86\x16\x80\x85R\x92R\x91\x82\x90 \x85\x90U`7T\x91Q\x94\x95P\x92\x16\x92\x7F\xDA\x91\x93`C2 \xE1;Q\xE8\xC2\x11\xE4\x90\xD1H\xE6\x1A;\xD5=\xE8\xC0\x97\x19NE\x8B\x97\xF3\xE1\x90a\x18\xEA\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPV[\x80Qa\x10\x9D\x90`;\x90` \x84\x01\x90a\x1AQV[\x80Qa\x10\x9D\x90`<\x90` \x84\x01\x90a\x1AQV[``a\x08ha\x05XV[`\0\x80a\x19<d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84a\x1F\x0FV[\x90P\x80a\x19XWk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91PPa\x15\xB8V[`\0\x19\x81\x01`\0\x80\x80`\x02\x85\x11a\x19pW`\0a\x19uV[`\x02\x85\x03[\x92Pf\x03\x88\x82\x91\\@\0a\x19\x89\x8A\x80a\x15\xBFV[\x81a\x19\x96Wa\x19\x96a /V[\x04\x91Pc\x01\xE13\x80a\x19\xA8\x83\x8Ba\x15\xBFV[\x81a\x19\xB5Wa\x19\xB5a /V[\x04\x90P`\0\x82a\x19\xC5\x86\x88a EV[a\x19\xCF\x91\x90a EV[`\x02\x90\x04\x90P`\0\x82\x85a\x19\xE3\x88\x8Aa EV[a\x19\xED\x91\x90a EV[a\x19\xF7\x91\x90a EV[`\x06\x90\x04\x90P\x80\x82c\x01\xE13\x80a\x1A\x0E\x8A\x8Fa EV[a\x1A\x18\x91\x90a dV[a\x1A.\x90k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x1E\xF7V[a\x1A8\x91\x90a\x1E\xF7V[a\x1AB\x91\x90a\x1E\xF7V[\x9B\x9APPPPPPPPPPPV[\x82\x80Ta\x1A]\x90a\x1E\xA6V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x1A\x7FW`\0\x85Ua\x1A\xC5V[\x82`\x1F\x10a\x1A\x98W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x1A\xC5V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x1A\xC5W\x91\x82\x01[\x82\x81\x11\x15a\x1A\xC5W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1A\xAAV[Pa\x17\x16\x92\x91P[\x80\x82\x11\x15a\x17\x16W`\0\x81U`\x01\x01a\x1A\xCDV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1B\x07W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1A\xEBV[\x81\x81\x11\x15a\x1B\x19W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x15\xB8` \x83\x01\x84a\x1A\xE1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1BVW`\0\x80\xFD[PV[\x805a\x16i\x81a\x1BAV[`\0\x80`@\x83\x85\x03\x12\x15a\x1BwW`\0\x80\xFD[\x825a\x1B\x82\x81a\x1BAV[\x94` \x93\x90\x93\x015\x93PPPV[\x805`\xFF\x81\x16\x81\x14a\x16iW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x1B\xBCW`\0\x80\xFD[\x875a\x1B\xC7\x81a\x1BAV[\x96P` \x88\x015a\x1B\xD7\x81a\x1BAV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa\x1B\xF3`\x80\x89\x01a\x1B\x90V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C$W`\0\x80\xFD[\x835a\x1C/\x81a\x1BAV[\x92P` \x84\x015a\x1C?\x81a\x1BAV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1CcW`\0\x80\xFD[\x825a\x1Cn\x81a\x1BAV[\x91P` \x83\x015a\x1C~\x81a\x1BAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1C\x9BW`\0\x80\xFD[\x815a\x15\xB8\x81a\x1BAV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C\xBCW`\0\x80\xFD[\x845a\x1C\xC7\x81a\x1BAV[\x93P` \x85\x015a\x1C\xD7\x81a\x1BAV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\x13W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D.Wa\x1D.a\x1C\xECV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1DVWa\x1DVa\x1C\xECV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x1DoW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x1D\xA1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xB9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\r-W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x89\x8B\x03\x12\x15a\x1D\xEDW`\0\x80\xFD[\x885a\x1D\xF8\x81a\x1BAV[\x97P` \x89\x015a\x1E\x08\x81a\x1BAV[\x96Pa\x1E\x16`@\x8A\x01a\x1BYV[\x95Pa\x1E$``\x8A\x01a\x1B\x90V[\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1EAW`\0\x80\xFD[a\x1EM\x8C\x83\x8D\x01a\x1D\x02V[\x95P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x1EcW`\0\x80\xFD[a\x1Eo\x8C\x83\x8D\x01a\x1D\x02V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15a\x1E\x85W`\0\x80\xFD[Pa\x1E\x92\x8B\x82\x8C\x01a\x1D\x8FV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1E\xBAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1E\xDBWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x1F\nWa\x1F\na\x1E\xE1V[P\x01\x90V[`\0\x82\x82\x10\x15a\x1F!Wa\x1F!a\x1E\xE1V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81R`\xFF\x86\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90a\x1FS\x90\x83\x01\x87a\x1A\xE1V[\x82\x81\x03``\x84\x01Ra\x1Fe\x81\x87a\x1A\xE1V[\x90P\x82\x81\x03`\x80\x84\x01R\x83\x81R\x83\x85` \x83\x017`\0` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xAFW`\0\x80\xFD[\x81Qa\x15\xB8\x81a\x1BAV[`\0` \x82\x84\x03\x12\x15a\x1F\xCCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x15\xB8W`\0\x80\xFD[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x1F\xFEWa\x1F\xFEa\x1E\xE1V[\x01\x94\x93PPPPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a 'Wa 'a\x1E\xE1V[\x03\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a _Wa _a\x1E\xE1V[P\x02\x90V[`\0\x82a \x81WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9C0g\xE7\xD0\xC9\xB6\xCF\xE6\x8F\xFE\x06\x1F\x8B-R\xD44gz\xA1\xA9\x81D\x9C\xC6\xA9\xD5\xC3\xFA\x9E\x15dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static STABLEDEBTTOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StableDebtToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StableDebtToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StableDebtToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StableDebtToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StableDebtToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StableDebtToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StableDebtToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STABLEDEBTTOKEN_ABI.clone(),
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
                STABLEDEBTTOKEN_ABI.clone(),
                STABLEDEBTTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEBT_TOKEN_REVISION` (0xb9a7b622) function
        pub fn debt_token_revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 167, 182, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DELEGATION_WITH_SIG_TYPEHASH` (0xf3bfc738) function
        pub fn delegation_with_sig_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([243, 191, 199, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EIP712_REVISION` (0x78160376) function
        pub fn eip712_revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([120, 22, 3, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UNDERLYING_ASSET_ADDRESS` (0xb16a19de) function
        pub fn underlying_asset_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 106, 25, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveDelegation` (0xc04a8a10) function
        pub fn approve_delegation(
            &self,
            delegatee: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 74, 138, 16], (delegatee, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrowAllowance` (0x6bd76d24) function
        pub fn borrow_allowance(
            &self,
            from_user: ::ethers::core::types::Address,
            to_user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 215, 109, 36], (from_user, to_user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x9dc29fac) function
        pub fn burn(
            &self,
            from: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 194, 159, 172], (from, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationWithSig` (0x0b52d558) function
        pub fn delegation_with_sig(
            &self,
            delegator: ::ethers::core::types::Address,
            delegatee: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [11, 82, 213, 88],
                    (delegator, delegatee, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAverageStableRate` (0x90f6fcf2) function
        pub fn get_average_stable_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 246, 252, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getIncentivesController` (0x75d26413) function
        pub fn get_incentives_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 210, 100, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSupplyData` (0x79774338) function
        pub fn get_supply_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u64,
            ),
        > {
            self.0
                .method_hash([121, 119, 67, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalSupplyAndAvgRate` (0xf731e9be) function
        pub fn get_total_supply_and_avg_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([247, 49, 233, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalSupplyLastUpdated` (0xe7484890) function
        pub fn get_total_supply_last_updated(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 72, 72, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserLastUpdated` (0x79ce6b8c) function
        pub fn get_user_last_updated(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([121, 206, 107, 140], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserStableRate` (0xe78c9b3b) function
        pub fn get_user_stable_rate(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 140, 155, 59], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc222ec8a) function
        pub fn initialize(
            &self,
            initializing_pool: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
            incentives_controller: ::ethers::core::types::Address,
            debt_token_decimals: u8,
            debt_token_name: ::std::string::String,
            debt_token_symbol: ::std::string::String,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [194, 34, 236, 138],
                    (
                        initializing_pool,
                        underlying_asset,
                        incentives_controller,
                        debt_token_decimals,
                        debt_token_name,
                        debt_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0xb3f1c93d) function
        pub fn mint(
            &self,
            user: ::ethers::core::types::Address,
            on_behalf_of: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            rate: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([179, 241, 201, 61], (user, on_behalf_of, amount, rate))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `principalBalanceOf` (0xc634dfaa) function
        pub fn principal_balance_of(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 52, 223, 170], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setIncentivesController` (0xe655dbd8) function
        pub fn set_incentives_controller(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 85, 219, 216], controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BorrowAllowanceDelegated` event
        pub fn borrow_allowance_delegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BorrowAllowanceDelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StableDebtTokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StableDebtToken<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "BorrowAllowanceDelegated",
        abi = "BorrowAllowanceDelegated(address,address,address,uint256)"
    )]
    pub struct BorrowAllowanceDelegatedFilter {
        #[ethevent(indexed)]
        pub from_user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "Burn",
        abi = "Burn(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub current_balance: ::ethers::core::types::U256,
        pub balance_increase: ::ethers::core::types::U256,
        pub avg_stable_rate: ::ethers::core::types::U256,
        pub new_total_supply: ::ethers::core::types::U256,
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
        name = "Initialized",
        abi = "Initialized(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub incentives_controller: ::ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: ::std::string::String,
        pub debt_token_symbol: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
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
        name = "Mint",
        abi = "Mint(address,address,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub current_balance: ::ethers::core::types::U256,
        pub balance_increase: ::ethers::core::types::U256,
        pub new_rate: ::ethers::core::types::U256,
        pub avg_stable_rate: ::ethers::core::types::U256,
        pub new_total_supply: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StableDebtTokenEvents {
        ApprovalFilter(ApprovalFilter),
        BorrowAllowanceDelegatedFilter(BorrowAllowanceDelegatedFilter),
        BurnFilter(BurnFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for StableDebtTokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(StableDebtTokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BorrowAllowanceDelegatedFilter::decode_log(log) {
                return Ok(
                    StableDebtTokenEvents::BorrowAllowanceDelegatedFilter(decoded),
                );
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(StableDebtTokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StableDebtTokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(StableDebtTokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(StableDebtTokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StableDebtTokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowAllowanceDelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for StableDebtTokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BorrowAllowanceDelegatedFilter>
    for StableDebtTokenEvents {
        fn from(value: BorrowAllowanceDelegatedFilter) -> Self {
            Self::BorrowAllowanceDelegatedFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for StableDebtTokenEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for StableDebtTokenEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for StableDebtTokenEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for StableDebtTokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEBT_TOKEN_REVISION` function with signature `DEBT_TOKEN_REVISION()` and selector `0xb9a7b622`
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
    #[ethcall(name = "DEBT_TOKEN_REVISION", abi = "DEBT_TOKEN_REVISION()")]
    pub struct DebtTokenRevisionCall;
    ///Container type for all input parameters for the `DELEGATION_WITH_SIG_TYPEHASH` function with signature `DELEGATION_WITH_SIG_TYPEHASH()` and selector `0xf3bfc738`
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
        name = "DELEGATION_WITH_SIG_TYPEHASH",
        abi = "DELEGATION_WITH_SIG_TYPEHASH()"
    )]
    pub struct DelegationWithSigTypehashCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `EIP712_REVISION` function with signature `EIP712_REVISION()` and selector `0x78160376`
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
    #[ethcall(name = "EIP712_REVISION", abi = "EIP712_REVISION()")]
    pub struct Eip712RevisionCall;
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `0xb16a19de`
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
    #[ethcall(name = "UNDERLYING_ASSET_ADDRESS", abi = "UNDERLYING_ASSET_ADDRESS()")]
    pub struct UnderlyingAssetAddressCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `approveDelegation` function with signature `approveDelegation(address,uint256)` and selector `0xc04a8a10`
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
    #[ethcall(name = "approveDelegation", abi = "approveDelegation(address,uint256)")]
    pub struct ApproveDelegationCall {
        pub delegatee: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `borrowAllowance` function with signature `borrowAllowance(address,address)` and selector `0x6bd76d24`
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
    #[ethcall(name = "borrowAllowance", abi = "borrowAllowance(address,address)")]
    pub struct BorrowAllowanceCall {
        pub from_user: ::ethers::core::types::Address,
        pub to_user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(address,uint256)` and selector `0x9dc29fac`
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
    #[ethcall(name = "burn", abi = "burn(address,uint256)")]
    pub struct BurnCall {
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `delegationWithSig` function with signature `delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x0b52d558`
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
        name = "delegationWithSig",
        abi = "delegationWithSig(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegationWithSigCall {
        pub delegator: ::ethers::core::types::Address,
        pub delegatee: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `0x90f6fcf2`
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
    #[ethcall(name = "getAverageStableRate", abi = "getAverageStableRate()")]
    pub struct GetAverageStableRateCall;
    ///Container type for all input parameters for the `getIncentivesController` function with signature `getIncentivesController()` and selector `0x75d26413`
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
    #[ethcall(name = "getIncentivesController", abi = "getIncentivesController()")]
    pub struct GetIncentivesControllerCall;
    ///Container type for all input parameters for the `getSupplyData` function with signature `getSupplyData()` and selector `0x79774338`
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
    #[ethcall(name = "getSupplyData", abi = "getSupplyData()")]
    pub struct GetSupplyDataCall;
    ///Container type for all input parameters for the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `0xf731e9be`
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
    #[ethcall(name = "getTotalSupplyAndAvgRate", abi = "getTotalSupplyAndAvgRate()")]
    pub struct GetTotalSupplyAndAvgRateCall;
    ///Container type for all input parameters for the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `0xe7484890`
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
    #[ethcall(name = "getTotalSupplyLastUpdated", abi = "getTotalSupplyLastUpdated()")]
    pub struct GetTotalSupplyLastUpdatedCall;
    ///Container type for all input parameters for the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `0x79ce6b8c`
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
    #[ethcall(name = "getUserLastUpdated", abi = "getUserLastUpdated(address)")]
    pub struct GetUserLastUpdatedCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `0xe78c9b3b`
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
    #[ethcall(name = "getUserStableRate", abi = "getUserStableRate(address)")]
    pub struct GetUserStableRateCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint8,string,string,bytes)` and selector `0xc222ec8a`
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
        name = "initialize",
        abi = "initialize(address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub initializing_pool: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
        pub incentives_controller: ::ethers::core::types::Address,
        pub debt_token_decimals: u8,
        pub debt_token_name: ::std::string::String,
        pub debt_token_symbol: ::std::string::String,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `0xb3f1c93d`
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
    #[ethcall(name = "mint", abi = "mint(address,address,uint256,uint256)")]
    pub struct MintCall {
        pub user: ::ethers::core::types::Address,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `0xc634dfaa`
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
    #[ethcall(name = "principalBalanceOf", abi = "principalBalanceOf(address)")]
    pub struct PrincipalBalanceOfCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setIncentivesController` function with signature `setIncentivesController(address)` and selector `0xe655dbd8`
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
        name = "setIncentivesController",
        abi = "setIncentivesController(address)"
    )]
    pub struct SetIncentivesControllerCall {
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StableDebtTokenCalls {
        DebtTokenRevision(DebtTokenRevisionCall),
        DelegationWithSigTypehash(DelegationWithSigTypehashCall),
        DomainSeparator(DomainSeparatorCall),
        Eip712Revision(Eip712RevisionCall),
        Pool(PoolCall),
        UnderlyingAssetAddress(UnderlyingAssetAddressCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveDelegation(ApproveDelegationCall),
        BalanceOf(BalanceOfCall),
        BorrowAllowance(BorrowAllowanceCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DelegationWithSig(DelegationWithSigCall),
        GetAverageStableRate(GetAverageStableRateCall),
        GetIncentivesController(GetIncentivesControllerCall),
        GetSupplyData(GetSupplyDataCall),
        GetTotalSupplyAndAvgRate(GetTotalSupplyAndAvgRateCall),
        GetTotalSupplyLastUpdated(GetTotalSupplyLastUpdatedCall),
        GetUserLastUpdated(GetUserLastUpdatedCall),
        GetUserStableRate(GetUserStableRateCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        PrincipalBalanceOf(PrincipalBalanceOfCall),
        SetIncentivesController(SetIncentivesControllerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for StableDebtTokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DebtTokenRevisionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DebtTokenRevision(decoded));
            }
            if let Ok(decoded)
                = <DelegationWithSigTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DelegationWithSigTypehash(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <Eip712RevisionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Revision(decoded));
            }
            if let Ok(decoded)
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded)
                = <UnderlyingAssetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnderlyingAssetAddress(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <ApproveDelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ApproveDelegation(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BorrowAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BorrowAllowance(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <DelegationWithSigCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DelegationWithSig(decoded));
            }
            if let Ok(decoded)
                = <GetAverageStableRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAverageStableRate(decoded));
            }
            if let Ok(decoded)
                = <GetIncentivesControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetIncentivesController(decoded));
            }
            if let Ok(decoded)
                = <GetSupplyDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSupplyData(decoded));
            }
            if let Ok(decoded)
                = <GetTotalSupplyAndAvgRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTotalSupplyAndAvgRate(decoded));
            }
            if let Ok(decoded)
                = <GetTotalSupplyLastUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTotalSupplyLastUpdated(decoded));
            }
            if let Ok(decoded)
                = <GetUserLastUpdatedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUserLastUpdated(decoded));
            }
            if let Ok(decoded)
                = <GetUserStableRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUserStableRate(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PrincipalBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PrincipalBalanceOf(decoded));
            }
            if let Ok(decoded)
                = <SetIncentivesControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetIncentivesController(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StableDebtTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DebtTokenRevision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationWithSigTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Revision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnderlyingAssetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveDelegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationWithSig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAverageStableRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetIncentivesController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSupplyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalSupplyAndAvgRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalSupplyLastUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserLastUpdated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserStableRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrincipalBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetIncentivesController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StableDebtTokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DebtTokenRevision(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationWithSigTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Revision(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingAssetAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveDelegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationWithSig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAverageStableRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetIncentivesController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSupplyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalSupplyAndAvgRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalSupplyLastUpdated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserLastUpdated(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserStableRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrincipalBalanceOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetIncentivesController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DebtTokenRevisionCall> for StableDebtTokenCalls {
        fn from(value: DebtTokenRevisionCall) -> Self {
            Self::DebtTokenRevision(value)
        }
    }
    impl ::core::convert::From<DelegationWithSigTypehashCall> for StableDebtTokenCalls {
        fn from(value: DelegationWithSigTypehashCall) -> Self {
            Self::DelegationWithSigTypehash(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for StableDebtTokenCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<Eip712RevisionCall> for StableDebtTokenCalls {
        fn from(value: Eip712RevisionCall) -> Self {
            Self::Eip712Revision(value)
        }
    }
    impl ::core::convert::From<PoolCall> for StableDebtTokenCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<UnderlyingAssetAddressCall> for StableDebtTokenCalls {
        fn from(value: UnderlyingAssetAddressCall) -> Self {
            Self::UnderlyingAssetAddress(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for StableDebtTokenCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for StableDebtTokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<ApproveDelegationCall> for StableDebtTokenCalls {
        fn from(value: ApproveDelegationCall) -> Self {
            Self::ApproveDelegation(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for StableDebtTokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BorrowAllowanceCall> for StableDebtTokenCalls {
        fn from(value: BorrowAllowanceCall) -> Self {
            Self::BorrowAllowance(value)
        }
    }
    impl ::core::convert::From<BurnCall> for StableDebtTokenCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for StableDebtTokenCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for StableDebtTokenCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DelegationWithSigCall> for StableDebtTokenCalls {
        fn from(value: DelegationWithSigCall) -> Self {
            Self::DelegationWithSig(value)
        }
    }
    impl ::core::convert::From<GetAverageStableRateCall> for StableDebtTokenCalls {
        fn from(value: GetAverageStableRateCall) -> Self {
            Self::GetAverageStableRate(value)
        }
    }
    impl ::core::convert::From<GetIncentivesControllerCall> for StableDebtTokenCalls {
        fn from(value: GetIncentivesControllerCall) -> Self {
            Self::GetIncentivesController(value)
        }
    }
    impl ::core::convert::From<GetSupplyDataCall> for StableDebtTokenCalls {
        fn from(value: GetSupplyDataCall) -> Self {
            Self::GetSupplyData(value)
        }
    }
    impl ::core::convert::From<GetTotalSupplyAndAvgRateCall> for StableDebtTokenCalls {
        fn from(value: GetTotalSupplyAndAvgRateCall) -> Self {
            Self::GetTotalSupplyAndAvgRate(value)
        }
    }
    impl ::core::convert::From<GetTotalSupplyLastUpdatedCall> for StableDebtTokenCalls {
        fn from(value: GetTotalSupplyLastUpdatedCall) -> Self {
            Self::GetTotalSupplyLastUpdated(value)
        }
    }
    impl ::core::convert::From<GetUserLastUpdatedCall> for StableDebtTokenCalls {
        fn from(value: GetUserLastUpdatedCall) -> Self {
            Self::GetUserLastUpdated(value)
        }
    }
    impl ::core::convert::From<GetUserStableRateCall> for StableDebtTokenCalls {
        fn from(value: GetUserStableRateCall) -> Self {
            Self::GetUserStableRate(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for StableDebtTokenCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StableDebtTokenCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MintCall> for StableDebtTokenCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for StableDebtTokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for StableDebtTokenCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PrincipalBalanceOfCall> for StableDebtTokenCalls {
        fn from(value: PrincipalBalanceOfCall) -> Self {
            Self::PrincipalBalanceOf(value)
        }
    }
    impl ::core::convert::From<SetIncentivesControllerCall> for StableDebtTokenCalls {
        fn from(value: SetIncentivesControllerCall) -> Self {
            Self::SetIncentivesController(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for StableDebtTokenCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for StableDebtTokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for StableDebtTokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for StableDebtTokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DEBT_TOKEN_REVISION` function with signature `DEBT_TOKEN_REVISION()` and selector `0xb9a7b622`
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
    pub struct DebtTokenRevisionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DELEGATION_WITH_SIG_TYPEHASH` function with signature `DELEGATION_WITH_SIG_TYPEHASH()` and selector `0xf3bfc738`
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
    pub struct DelegationWithSigTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `EIP712_REVISION` function with signature `EIP712_REVISION()` and selector `0x78160376`
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
    pub struct Eip712RevisionReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
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
    pub struct PoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `UNDERLYING_ASSET_ADDRESS` function with signature `UNDERLYING_ASSET_ADDRESS()` and selector `0xb16a19de`
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
    pub struct UnderlyingAssetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `borrowAllowance` function with signature `borrowAllowance(address,address)` and selector `0x6bd76d24`
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
    pub struct BorrowAllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `burn` function with signature `burn(address,uint256)` and selector `0x9dc29fac`
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
    pub struct BurnReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `getAverageStableRate` function with signature `getAverageStableRate()` and selector `0x90f6fcf2`
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
    pub struct GetAverageStableRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getIncentivesController` function with signature `getIncentivesController()` and selector `0x75d26413`
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
    pub struct GetIncentivesControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSupplyData` function with signature `getSupplyData()` and selector `0x79774338`
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
    pub struct GetSupplyDataReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub u64,
    );
    ///Container type for all return fields from the `getTotalSupplyAndAvgRate` function with signature `getTotalSupplyAndAvgRate()` and selector `0xf731e9be`
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
    pub struct GetTotalSupplyAndAvgRateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getTotalSupplyLastUpdated` function with signature `getTotalSupplyLastUpdated()` and selector `0xe7484890`
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
    pub struct GetTotalSupplyLastUpdatedReturn(pub u64);
    ///Container type for all return fields from the `getUserLastUpdated` function with signature `getUserLastUpdated(address)` and selector `0x79ce6b8c`
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
    pub struct GetUserLastUpdatedReturn(pub u64);
    ///Container type for all return fields from the `getUserStableRate` function with signature `getUserStableRate(address)` and selector `0xe78c9b3b`
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
    pub struct GetUserStableRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `mint` function with signature `mint(address,address,uint256,uint256)` and selector `0xb3f1c93d`
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
    pub struct MintReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `principalBalanceOf` function with signature `principalBalanceOf(address)` and selector `0xc634dfaa`
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
    pub struct PrincipalBalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}
