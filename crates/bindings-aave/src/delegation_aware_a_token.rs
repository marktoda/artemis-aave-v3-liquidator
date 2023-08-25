pub use delegation_aware_a_token::*;
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
pub mod delegation_aware_a_token {
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
                    ::std::borrow::ToOwned::to_owned("ATOKEN_REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ATOKEN_REVISION"),
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
                    ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
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
                    ::std::borrow::ToOwned::to_owned("RESERVE_TREASURY_ADDRESS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RESERVE_TREASURY_ADDRESS",
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "receiverOfUnderlying",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                    ::std::borrow::ToOwned::to_owned("delegateUnderlyingTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "delegateUnderlyingTo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
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
                    ::std::borrow::ToOwned::to_owned("getPreviousIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPreviousIndex"),
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
                    ::std::borrow::ToOwned::to_owned("getScaledUserBalanceAndSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getScaledUserBalanceAndSupply",
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
                    ::std::borrow::ToOwned::to_owned("handleRepayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("handleRepayment"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                                    name: ::std::borrow::ToOwned::to_owned("treasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("aTokenDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenSymbol"),
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
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("mintToTreasury"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintToTreasury"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("scaledBalanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scaledBalanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("scaledTotalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scaledTotalSupply"),
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
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                    ::std::borrow::ToOwned::to_owned("transferOnLiquidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferOnLiquidation",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("transferUnderlyingTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferUnderlyingTo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                    ::std::borrow::ToOwned::to_owned("BalanceTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BalanceTransfer"),
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("balanceIncrease"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("DelegateUnderlyingTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DelegateUnderlyingTo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("delegatee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                                    name: ::std::borrow::ToOwned::to_owned("treasury"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "incentivesController",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aTokenSymbol"),
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
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("onBehalfOf"),
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("balanceIncrease"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
    pub static DELEGATIONAWAREATOKEN_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R`\0\x80U4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0->8\x03\x80b\0->\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\x02\x1FV[\x80\x80`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x10U\x13\xD2\xD1S\x97\xD2ST\x13`\xAA\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01j\x10U\x13\xD2\xD1S\x97\xD2ST\x13`\xAA\x1B\x81RP`\0\x83\x83\x83\x83\x83\x83\x83\x83\x83`\x01`\x01`\xA0\x1B\x03\x16c\x05B\x97\\`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xF1\x91\x90b\0\x02\x1FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80R\x82Qb\0\x01\x12\x90`7\x90` \x86\x01\x90b\0\x01`V[P\x81Qb\0\x01(\x90`8\x90` \x85\x01\x90b\0\x01`V[P`9\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01`\x01`\xA0\x1B\x03\x16`\xA0RPPF`\xC0RPb\0\x02\x83\x96PPPPPPPV[\x82\x80Tb\0\x01n\x90b\0\x02FV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0\x01\x92W`\0\x85Ub\0\x01\xDDV[\x82`\x1F\x10b\0\x01\xADW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0\x01\xDDV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0\x01\xDDW\x91\x82\x01[\x82\x81\x11\x15b\0\x01\xDDW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0\x01\xC0V[Pb\0\x01\xEB\x92\x91Pb\0\x01\xEFV[P\x90V[[\x80\x82\x11\x15b\0\x01\xEBW`\0\x81U`\x01\x01b\0\x01\xF0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\x1CW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x022W`\0\x80\xFD[\x81Qb\0\x02?\x81b\0\x02\x06V[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02[W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15b\0\x02}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa*\"b\0\x03\x1C`\09`\0a\x17R\x01R`\0\x81\x81a\x03\x80\x01R\x81\x81a\x06S\x01R\x81\x81a\x07q\x01R\x81\x81a\x08\xC7\x01R\x81\x81a\x0B\xFC\x01R\x81\x81a\x0Cz\x01R\x81\x81a\x0C\xEF\x01R\x81\x81a\r\x87\x01R\x81\x81a\r\xE0\x01R\x81\x81a\x0E\xC6\x01R\x81\x81a\x12\xF8\x01R\x81\x81a\x14\xDF\x01R\x81\x81a\x1B\xE8\x01Ra\x1C\xCF\x01R`\0\x81\x81a\t\xDF\x01R\x81\x81a\x0F&\x01Ra\x13v\x01Ra*\"`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cu\xD2d\x13\x11a\x01%W\x80c\xB1\xBF\x96-\x11a\0\xADW\x80c\xD7\x02\r\n\x11a\0|W\x80c\xD7\x02\r\n\x14a\x04\xA7W\x80c\xDDb\xED>\x14a\x04\xBAW\x80c\xE0u9\x86\x14a\x04\xF3W\x80c\xE6U\xDB\xD8\x14a\x05,W\x80c\xF8f\xC3\x19\x14a\x05?W`\0\x80\xFD[\x80c\xB1\xBF\x96-\x14a\x04fW\x80c\xB3\xF1\xC9=\x14a\x04nW\x80c\xCE\xA9\xD2o\x14a\x04\x81W\x80c\xD5\x05\xAC\xCF\x14a\x04\x94W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xF4W\x80c\x95\xD8\x9BA\x14a\x04\x16W\x80c\xA4W\xC2\xD7\x14a\x04\x1EW\x80c\xA9\x05\x9C\xBB\x14a\x041W\x80c\xAE\x16s5\x14a\x04DW\x80c\xB1j\x19\xDE\x14a\x04UW`\0\x80\xFD[\x80cu\xD2d\x13\x14a\x03\xBAW\x80cx\x16\x03v\x14a\x03\xD0W\x80c}\xF5\xBD;\x14a\x03\xF0W\x80c~\xCE\xBE\0\x14a\x04\x03W`\0\x80\xFD[\x80c/\x11F\x18\x11a\x01\xA8W\x80c9P\x93Q\x11a\x01wW\x80c9P\x93Q\x14a\x03/W\x80cN\xFE\xCA\xA5\x14a\x03BW\x80co\xD9vv\x14a\x03UW\x80cp\xA0\x821\x14a\x03hW\x80cu5\xD2F\x14a\x03{W`\0\x80\xFD[\x80c/\x11F\x18\x14a\x02\xD8W\x80c0\xAD\xF8\x1F\x14a\x02\xEBW\x80c1<\xE5g\x14a\x03\x12W\x80c6D\xE5\x15\x14a\x03'W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x01\xE4W\x80c\x18\x16\r\xDD\x14a\x02\x95W\x80c\x18?\xB4\x13\x14a\x02\x9DW\x80c\x1D\xA2O>\x14a\x02\xB2W\x80c#\xB8r\xDD\x14a\x02\xC5W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\x16W\x80c\t^\xA7\xB3\x14a\x024W\x80c\n\xFB\xCD\xC9\x14a\x02WW\x80c\x0B\xD7\xAD;\x14a\x02\x7FW[`\0\x80\xFD[a\x02\x1Ea\x05RV[`@Qa\x02+\x91\x90a$\x9DV[`@Q\x80\x91\x03\x90\xF3[a\x02Ga\x02B6`\x04a$\xDFV[a\x05\xE4V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[a\x02ja\x02e6`\x04a%\x0BV[a\x05\xFAV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02+V[a\x02\x87`\x01\x81V[`@Q\x90\x81R` \x01a\x02+V[a\x02\x87a\x06\x12V[a\x02\xB0a\x02\xAB6`\x04a%\x82V[a\x06\xCBV[\0[a\x02\x87a\x02\xC06`\x04a%\x0BV[a\t`V[a\x02Ga\x02\xD36`\x04a&vV[a\tqV[a\x02\xB0a\x02\xE66`\x04a%\x0BV[a\t\xDBV[a\x02\x87\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`9T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02+V[a\x02\x87a\x0B\x9AV[a\x02Ga\x03=6`\x04a$\xDFV[a\x0B\xA9V[a\x02\xB0a\x03P6`\x04a$\xDFV[a\x0B\xE0V[a\x02\xB0a\x03c6`\x04a&vV[a\x0C^V[a\x02\x87a\x03v6`\x04a%\x0BV[a\x0C\xC6V[a\x03\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[`9Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[a\x02\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x02\xB0a\x03\xFE6`\x04a&\xB7V[a\rkV[a\x02\x87a\x04\x116`\x04a%\x0BV[a\x0E\x15V[a\x02\x1Ea\x0E3V[a\x02Ga\x04,6`\x04a$\xDFV[a\x0EBV[a\x02Ga\x04?6`\x04a$\xDFV[a\x0EyV[`<T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[`=T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[a\x02\x87a\x0E\x9CV[a\x02Ga\x04|6`\x04a&\xD9V[a\x0E\xA7V[a\x02\xB0a\x04\x8F6`\x04a&vV[a\x0F\"V[a\x02\xB0a\x04\xA26`\x04a'\x1FV[a\x10\xA8V[a\x02\xB0a\x04\xB56`\x04a&\xD9V[a\x12\xDCV[a\x02\x87a\x04\xC86`\x04a'\x8DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`5` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02\x87a\x05\x016`\x04a%\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x02\xB0a\x05:6`\x04a%\x0BV[a\x13rV[a\x02\xB0a\x05M6`\x04a&vV[a\x14\xC3V[```7\x80Ta\x05a\x90a'\xC6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x8D\x90a'\xC6V[\x80\x15a\x05\xDAW\x80`\x1F\x10a\x05\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xDAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x05\xF13\x84\x84a\x153V[P`\x01\x92\x91PPV[`\0\x80a\x06\x06\x83a\x15\x94V[`6T\x91P\x91P\x91P\x91V[`\0\x80a\x06\x1E`6T\x90V[\x90P\x80a\x06-W`\0\x91PP\x90V[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x06\xC5\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xBE\x91\x90a'\xFBV[\x82\x90a\x15\xB8V[\x91PP\x90V[`\x01\x80T`\xFF\x16\x80a\x06\xDCWP0;\x15[\x80a\x06\xE8WP`\0T\x81\x11[a\x07PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x07oW`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x90a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x08\x1D\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x15\xFC\x92PPPV[a\x08\\\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x16\x0F\x92PPPV[`9\x80T`\xFF\x19\x16`\xFF\x8B\x16\x17\x90U`<\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`=\x80T\x8E\x84\x16\x92\x16\x91\x90\x91\x17\x90U`9\x80T\x91\x8C\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xBFa\x16\"V[`;\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB1\x9E\x05\x1F\x8A\xF4\x11P\xCC\xCC\xB3\xFC,-\x8D\x15\xF4\xA4\xCFCO2\xA5Y\xBAu\xFEs\xD6\xEE\xA2\x0B\x8E\x8D\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\t8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a(=V[`@Q\x80\x91\x03\x90\xA3\x80\x15a\tQW`\x01\x80T`\xFF\x19\x16\x90U[PPPPPPPPPPPPPV[`\0a\tk\x82a\x15\x94V[\x92\x91PPV[`\0\x80a\t}\x83a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`5` \x90\x81R`@\x80\x83 3\x80\x85R\x92R\x90\x91 T\x91\x92Pa\t\xC5\x91\x87\x91\x90a\t\xC0\x90`\x01`\x01`\x80\x1B\x03\x86\x16\x90a(\xC1V[a\x153V[a\t\xD0\x85\x85\x83a\x178V[P`\x01\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCA\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=T`@Qc\x17\x06jW`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\\\x19\xA9\\\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0BJW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B^W=`\0\x80>=`\0\xFD[PP`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x92P\x7F\xC7\xA5R;\xFD\trO\xD5iP\xE7\x08\xE5#\xADja\xAB\x16[\x8E\x99z1\xD4#W\xA7\x7F\x0E\x0F\x91P`\0\x90\xA2PPV[`\0a\x0B\xA4a\x17NV[\x90P\x90V[3`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x81 T\x90\x91a\x05\xF1\x91\x85\x90a\t\xC0\x90\x86\x90a)\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=Ta\x0CZ\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a\x17\x87V[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[PPPPV[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x91a\tk\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\\\x91\x90a'\xFBV[a\re\x84a\x15\x94V[\x90a\x15\xB8V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P\x81a\r\xD7WPPV[`<Ta\x0E\x10\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x84a\x18\x12V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`:` R`@\x81 Ta\tkV[```8\x80Ta\x05a\x90a'\xC6V[3`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x81 T\x90\x91a\x05\xF1\x91\x85\x90a\t\xC0\x90\x86\x90a(\xC1V[`\0\x80a\x0E\x85\x83a\x16\xCBV[\x90Pa\x0E\x923\x85\x83a\x178V[P`\x01\x93\x92PPPV[`\0a\x0B\xA4`6T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R`\0\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0F\x19\x85\x85\x85\x85a\x18\x12V[\x95\x94PPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA6\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x11\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x10IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra85`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14\x15a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0C\xC0`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84a\x17\x87V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra77`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16a\x10\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P\x83B\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x90a\x11&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`:` R`@\x81 T\x90a\x11Ia\x0B\x9AV[`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16\x92\x82\x01\x92\x90\x92R\x90\x8A\x16``\x82\x01R`\x80\x81\x01\x89\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xE2\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83R\x81\x90R`\xFF\x88\x16\x91\x83\x01\x91\x90\x91R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12JW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x90a\x12\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x12\xAD\x82`\x01a)\x17V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`:` R`@\x90 Ua\x12\xD1\x89\x89\x89a\x153V[PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x13K\x84\x84\x84\x84a\x19\x97V[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14a\x0C\xC0W`=Ta\x0C\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x84a\x17\x87V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF6\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14a\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[PP`9\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0E\x10\x83\x83\x83`\0a\x1B\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`5` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x15\xDAW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[\x80Qa\x0CZ\x90`7\x90` \x84\x01\x90a#\xC0V[\x80Qa\x0CZ\x90`8\x90` \x84\x01\x90a#\xC0V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x16Ma\x1DyV[\x80Q` \x91\x82\x01 `@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B\x90\x84\x01R\x80Q\x92\x83\x01\x93\x90\x93R\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\x174W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07GV[P\x90V[a\x0E\x10\x83\x83\x83`\x01`\x01`\x80\x1B\x03\x16`\x01a\x1B\xC0V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x17\x7FWP`;T\x90V[a\x0B\xA4a\x16\"V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x90`\0\x80`D\x83\x82\x89Z\xF1a\x17\xC4W=`\0\x80>=`\0\xFD[Pa\x17\xCE\x84a\x1D\x83V[a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt#\xA8;\x19\x1D\x1030\xB4\xB62\xB2\x10:90\xB79\xB32\xB9`Y\x1B`D\x82\x01R`d\x01a\x07GV[`\0\x80a\x18\x1F\x84\x84a\x1E,V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x0C\x8D`\xF2\x1B` \x82\x01R\x90\x91P\x81a\x18YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\0a\x18e\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x18\x9D\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x18\xA7\x83\x87a\x15\xB8V[a\x18\xB1\x91\x90a(\xC1V[\x90Pa\x18\xBC\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua\x19\x01\x87a\x18\xFC\x85a\x16\xCBV[a\x1EkV[`\0a\x19\r\x82\x88a)\x17V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16`\0`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x19C\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x90\x8B\x16\x90`\0\x80Q` a)\xAD\x839\x81Q\x91R\x90``\x01`@Q\x80\x91\x03\x90\xA3PP\x15\x96\x95PPPPPPV[`\0a\x19\xA3\x83\x83a\x1E,V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra25`\xF0\x1B` \x82\x01R\x90\x91P\x81a\x19\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\0a\x19\xE9\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x1A!\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x1A+\x83\x86a\x15\xB8V[a\x1A5\x91\x90a(\xC1V[\x90Pa\x1A@\x84a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua\x1A\x85\x87a\x1A\x80\x85a\x16\xCBV[a\x1F\\V[\x84\x81\x11\x15a\x1B\x19W`\0a\x1A\x99\x86\x83a(\xC1V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16`\0`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x1A\xCF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x90`\0\x80Q` a)\xAD\x839\x81Q\x91R\x90``\x01`@Q\x80\x91\x03\x90\xA3Pa\x1B\xB7V[`\0a\x1B%\x82\x87a(\xC1V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x1B[\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x8A\x16\x90\x7FL\xF2[\xC1\xD9\x91\xC1u)\xC2R\x13\xD3\xCC\x0C\xDA)^\xEA\xAD_\x13\xF3a\x96\x9B\x12\xEAH\x01_\x90\x90``\x01[`@Q\x80\x91\x03\x90\xA3P[PPPPPPPV[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CU\x91\x90a'\xFBV[\x90P`\0a\x1Cf\x82a\re\x89a\x15\x94V[\x90P`\0a\x1Cw\x83a\re\x89a\x15\x94V[\x90Pa\x1C\x85\x88\x88\x88\x86a\x1F\xA1V[\x84\x15a\x1D,W`@Qc\xD5\xED93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD5\xED93\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D'W=`\0\x80>=`\0\xFD[PPPP[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x90\x89\x16\x7FK\xEC\xCB\x90\xF9\x94\xC3\x1A\xCE\xD7\xA2;V\x11\x02\x07(\xA2=\x8E\xC5\xCD\xDD\x1A>\x9D\x97\xB9o\xDA\x86fa\x1De\x89\x87a\x1E,V[`@\x80Q\x91\x82R` \x82\x01\x88\x90R\x01a\x1B\xADV[``a\x0B\xA4a\x05RV[`\0a\x1D\xA9V[bF\x1B\xCD`\xE5\x1B`\0R` `\x04R\x80`$RP\x80`DRP`d`\0\xFD[=\x80\x15a\x1D\xE8W` \x81\x14a\x1E\x19Wa\x1D\xE3\x7FGPv2: malformed transfer result\0`\x1Fa\x1D\x8AV[a\x1E&V[\x82;a\x1E\x10Wa\x1E\x10s\x11\xD4\x1D\x8C\x8E\x88\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B`\x14a\x1D\x8AV[`\x01\x91Pa\x1E&V[=`\0\x80>`\0Q\x15\x15\x91P[P\x91\x90PV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x1EPW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`6Ta\x1E\x81`\x01`\x01`\x80\x1B\x03\x83\x16\x82a)\x17V[`6U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\x1E\xB0\x83\x82a)/V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`9Ta\x01\0\x90\x04\x16\x80\x15a\x1FUW`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c1\x87>.\x90a\x1F'\x90\x88\x90\x87\x90\x87\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1FAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xD1W=`\0\x80>=`\0\xFD[PPPPPV[`6Ta\x1Fr`\x01`\x01`\x80\x1B\x03\x83\x16\x82a(\xC1V[`6U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\x1E\xB0\x83\x82a)\x84V[`\0a\x1F\xAC\x85a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x1F\xE4\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x1F\xEE\x83\x85a\x15\xB8V[a\x1F\xF8\x91\x90a(\xC1V[\x90P`\0a \x05\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a =\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a G\x83\x87a\x15\xB8V[a Q\x91\x90a(\xC1V[\x90Pa \\\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua \x98\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua \xE7\x88\x88a \xE2a \xDD\x8A\x8Aa\x1E,V[a\x16\xCBV[a\"\x1DV[\x82\x15a!XW`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90`\0\x90`\0\x80Q` a)\xCD\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x84\x81R` \x81\x01\x85\x90R\x80\x82\x01\x87\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x913\x91`\0\x80Q` a)\xAD\x839\x81Q\x91R\x91\x81\x90\x03``\x01\x90\xA3[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!zWP`\0\x81\x11[\x15a!\xEAW`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90`\0\x90`\0\x80Q` a)\xCD\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x913\x91`\0\x80Q` a)\xAD\x839\x81Q\x91R\x91\x81\x90\x03``\x01\x90\xA3[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x88`@Qa\x1B\xAD\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\"I\x82\x82a)\x84V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x80\x82 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x95\x86\x16\x17\x90U\x91\x86\x16\x81R T\x16a\"\x8F\x83\x82a)/V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`9Ta\x01\0\x90\x04\x16\x80\x15a#\xB8W`6T`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c1\x87>.\x90a#\t\x90\x8A\x90\x85\x90\x89\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a##W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#7W=`\0\x80>=`\0\xFD[PPPP\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xB7W`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c1\x87>.\x90a#\x84\x90\x89\x90\x85\x90\x88\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xB2W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x80Ta#\xCC\x90a'\xC6V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a#\xEEW`\0\x85Ua$4V[\x82`\x1F\x10a$\x07W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua$4V[\x82\x80\x01`\x01\x01\x85U\x82\x15a$4W\x91\x82\x01[\x82\x81\x11\x15a$4W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a$\x19V[Pa\x174\x92\x91P[\x80\x82\x11\x15a\x174W`\0\x81U`\x01\x01a$<V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$vW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$ZV[\x81\x81\x11\x15a$\x88W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a$\xB0` \x83\x01\x84a$PV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xCCW`\0\x80\xFD[PV[\x805a$\xDA\x81a$\xB7V[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xF2W`\0\x80\xFD[\x825a$\xFD\x81a$\xB7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a%\x1DW`\0\x80\xFD[\x815a$\xB0\x81a$\xB7V[\x805`\xFF\x81\x16\x81\x14a$\xDAW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a%KW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%cW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a%{W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\0\x8C\x8E\x03\x12\x15a%\xA4W`\0\x80\xFD[a%\xAD\x8Ca$\xCFV[\x9APa%\xBB` \x8D\x01a$\xCFV[\x99Pa%\xC9`@\x8D\x01a$\xCFV[\x98Pa%\xD7``\x8D\x01a$\xCFV[\x97Pa%\xE5`\x80\x8D\x01a%(V[\x96Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\xA0\x8E\x015\x11\x15a&\x01W`\0\x80\xFD[a&\x11\x8E`\xA0\x8F\x015\x8F\x01a%9V[\x90\x97P\x95P`\xC0\x8D\x015\x81\x10\x15a&'W`\0\x80\xFD[a&7\x8E`\xC0\x8F\x015\x8F\x01a%9V[\x90\x95P\x93P`\xE0\x8D\x015\x81\x10\x15a&MW`\0\x80\xFD[Pa&^\x8D`\xE0\x8E\x015\x8E\x01a%9V[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x8BW`\0\x80\xFD[\x835a&\x96\x81a$\xB7V[\x92P` \x84\x015a&\xA6\x81a$\xB7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a&\xCAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&\xEFW`\0\x80\xFD[\x845a&\xFA\x81a$\xB7V[\x93P` \x85\x015a'\n\x81a$\xB7V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a':W`\0\x80\xFD[\x875a'E\x81a$\xB7V[\x96P` \x88\x015a'U\x81a$\xB7V[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa'q`\x80\x89\x01a%(V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA0W`\0\x80\xFD[\x825a'\xAB\x81a$\xB7V[\x91P` \x83\x015a'\xBB\x81a$\xB7V[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\xDAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1E&WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a(\rW`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x82R\x89\x16` \x82\x01R`\xFF\x88\x16`@\x82\x01R`\xC0``\x82\x01\x81\x90R`\0\x90a(s\x90\x83\x01\x88\x8Aa(\x14V[\x82\x81\x03`\x80\x84\x01Ra(\x86\x81\x87\x89a(\x14V[\x90P\x82\x81\x03`\xA0\x84\x01Ra(\x9B\x81\x85\x87a(\x14V[\x9C\x9BPPPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(\xD3Wa(\xD3a(\xABV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a(\xEAW`\0\x80\xFD[\x81Qa$\xB0\x81a$\xB7V[`\0` \x82\x84\x03\x12\x15a)\x07W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a$\xB0W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a)*Wa)*a(\xABV[P\x01\x90V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a)QWa)Qa(\xABV[\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`@\x82\x01R``\x01\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)\xA4Wa)\xA4a(\xABV[\x03\x93\x92PPPV\xFEE\x8F_\xA4\x12\xD0\xF6\x9B\x08\xDD\x84\x87+\x02\x15g\\\xC6{\xC1\xD5\xB6\xFD\x930\n\x1C8x\xB8a\x96\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xD3\xFF-'=G\xAB\xEA\xBC\x06\x04b\xA2&zJNf^i\x16H\xE6\x9E\xE2\x9E%:)V!\xFBdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONAWAREATOKEN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cu\xD2d\x13\x11a\x01%W\x80c\xB1\xBF\x96-\x11a\0\xADW\x80c\xD7\x02\r\n\x11a\0|W\x80c\xD7\x02\r\n\x14a\x04\xA7W\x80c\xDDb\xED>\x14a\x04\xBAW\x80c\xE0u9\x86\x14a\x04\xF3W\x80c\xE6U\xDB\xD8\x14a\x05,W\x80c\xF8f\xC3\x19\x14a\x05?W`\0\x80\xFD[\x80c\xB1\xBF\x96-\x14a\x04fW\x80c\xB3\xF1\xC9=\x14a\x04nW\x80c\xCE\xA9\xD2o\x14a\x04\x81W\x80c\xD5\x05\xAC\xCF\x14a\x04\x94W`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xF4W\x80c\x95\xD8\x9BA\x14a\x04\x16W\x80c\xA4W\xC2\xD7\x14a\x04\x1EW\x80c\xA9\x05\x9C\xBB\x14a\x041W\x80c\xAE\x16s5\x14a\x04DW\x80c\xB1j\x19\xDE\x14a\x04UW`\0\x80\xFD[\x80cu\xD2d\x13\x14a\x03\xBAW\x80cx\x16\x03v\x14a\x03\xD0W\x80c}\xF5\xBD;\x14a\x03\xF0W\x80c~\xCE\xBE\0\x14a\x04\x03W`\0\x80\xFD[\x80c/\x11F\x18\x11a\x01\xA8W\x80c9P\x93Q\x11a\x01wW\x80c9P\x93Q\x14a\x03/W\x80cN\xFE\xCA\xA5\x14a\x03BW\x80co\xD9vv\x14a\x03UW\x80cp\xA0\x821\x14a\x03hW\x80cu5\xD2F\x14a\x03{W`\0\x80\xFD[\x80c/\x11F\x18\x14a\x02\xD8W\x80c0\xAD\xF8\x1F\x14a\x02\xEBW\x80c1<\xE5g\x14a\x03\x12W\x80c6D\xE5\x15\x14a\x03'W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x01\xE4W\x80c\x18\x16\r\xDD\x14a\x02\x95W\x80c\x18?\xB4\x13\x14a\x02\x9DW\x80c\x1D\xA2O>\x14a\x02\xB2W\x80c#\xB8r\xDD\x14a\x02\xC5W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x02\x16W\x80c\t^\xA7\xB3\x14a\x024W\x80c\n\xFB\xCD\xC9\x14a\x02WW\x80c\x0B\xD7\xAD;\x14a\x02\x7FW[`\0\x80\xFD[a\x02\x1Ea\x05RV[`@Qa\x02+\x91\x90a$\x9DV[`@Q\x80\x91\x03\x90\xF3[a\x02Ga\x02B6`\x04a$\xDFV[a\x05\xE4V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[a\x02ja\x02e6`\x04a%\x0BV[a\x05\xFAV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02+V[a\x02\x87`\x01\x81V[`@Q\x90\x81R` \x01a\x02+V[a\x02\x87a\x06\x12V[a\x02\xB0a\x02\xAB6`\x04a%\x82V[a\x06\xCBV[\0[a\x02\x87a\x02\xC06`\x04a%\x0BV[a\t`V[a\x02Ga\x02\xD36`\x04a&vV[a\tqV[a\x02\xB0a\x02\xE66`\x04a%\x0BV[a\t\xDBV[a\x02\x87\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81V[`9T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02+V[a\x02\x87a\x0B\x9AV[a\x02Ga\x03=6`\x04a$\xDFV[a\x0B\xA9V[a\x02\xB0a\x03P6`\x04a$\xDFV[a\x0B\xE0V[a\x02\xB0a\x03c6`\x04a&vV[a\x0C^V[a\x02\x87a\x03v6`\x04a%\x0BV[a\x0C\xC6V[a\x03\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[`9Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[a\x02\x1E`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x81V[a\x02\xB0a\x03\xFE6`\x04a&\xB7V[a\rkV[a\x02\x87a\x04\x116`\x04a%\x0BV[a\x0E\x15V[a\x02\x1Ea\x0E3V[a\x02Ga\x04,6`\x04a$\xDFV[a\x0EBV[a\x02Ga\x04?6`\x04a$\xDFV[a\x0EyV[`<T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[`=T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA2V[a\x02\x87a\x0E\x9CV[a\x02Ga\x04|6`\x04a&\xD9V[a\x0E\xA7V[a\x02\xB0a\x04\x8F6`\x04a&vV[a\x0F\"V[a\x02\xB0a\x04\xA26`\x04a'\x1FV[a\x10\xA8V[a\x02\xB0a\x04\xB56`\x04a&\xD9V[a\x12\xDCV[a\x02\x87a\x04\xC86`\x04a'\x8DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`5` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x02\x87a\x05\x016`\x04a%\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x02\xB0a\x05:6`\x04a%\x0BV[a\x13rV[a\x02\xB0a\x05M6`\x04a&vV[a\x14\xC3V[```7\x80Ta\x05a\x90a'\xC6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x8D\x90a'\xC6V[\x80\x15a\x05\xDAW\x80`\x1F\x10a\x05\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xDAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x05\xF13\x84\x84a\x153V[P`\x01\x92\x91PPV[`\0\x80a\x06\x06\x83a\x15\x94V[`6T\x91P\x91P\x91P\x91V[`\0\x80a\x06\x1E`6T\x90V[\x90P\x80a\x06-W`\0\x91PP\x90V[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x06\xC5\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xBE\x91\x90a'\xFBV[\x82\x90a\x15\xB8V[\x91PP\x90V[`\x01\x80T`\xFF\x16\x80a\x06\xDCWP0;\x15[\x80a\x06\xE8WP`\0T\x81\x11[a\x07PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x07oW`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a87`\xF0\x1B\x81RP\x90a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x08\x1D\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x15\xFC\x92PPPV[a\x08\\\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x16\x0F\x92PPPV[`9\x80T`\xFF\x19\x16`\xFF\x8B\x16\x17\x90U`<\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`=\x80T\x8E\x84\x16\x92\x16\x91\x90\x91\x17\x90U`9\x80T\x91\x8C\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08\xBFa\x16\"V[`;\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB1\x9E\x05\x1F\x8A\xF4\x11P\xCC\xCC\xB3\xFC,-\x8D\x15\xF4\xA4\xCFCO2\xA5Y\xBAu\xFEs\xD6\xEE\xA2\x0B\x8E\x8D\x8D\x8D\x8D\x8D\x8D\x8D\x8D`@Qa\t8\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a(=V[`@Q\x80\x91\x03\x90\xA3\x80\x15a\tQW`\x01\x80T`\xFF\x19\x16\x90U[PPPPPPPPPPPPPV[`\0a\tk\x82a\x15\x94V[\x92\x91PPV[`\0\x80a\t}\x83a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`5` \x90\x81R`@\x80\x83 3\x80\x85R\x92R\x90\x91 T\x91\x92Pa\t\xC5\x91\x87\x91\x90a\t\xC0\x90`\x01`\x01`\x80\x1B\x03\x86\x16\x90a(\xC1V[a\x153V[a\t\xD0\x85\x85\x83a\x178V[P`\x01\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n_\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCA\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=T`@Qc\x17\x06jW`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\\\x19\xA9\\\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0BJW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B^W=`\0\x80>=`\0\xFD[PP`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x92P\x7F\xC7\xA5R;\xFD\trO\xD5iP\xE7\x08\xE5#\xADja\xAB\x16[\x8E\x99z1\xD4#W\xA7\x7F\x0E\x0F\x91P`\0\x90\xA2PPV[`\0a\x0B\xA4a\x17NV[\x90P\x90V[3`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x81 T\x90\x91a\x05\xF1\x91\x85\x90a\t\xC0\x90\x86\x90a)\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=Ta\x0CZ\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a\x17\x87V[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[PPPPV[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x91a\tk\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\\\x91\x90a'\xFBV[a\re\x84a\x15\x94V[\x90a\x15\xB8V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P\x81a\r\xD7WPPV[`<Ta\x0E\x10\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x84a\x18\x12V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`:` R`@\x81 Ta\tkV[```8\x80Ta\x05a\x90a'\xC6V[3`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x81 T\x90\x91a\x05\xF1\x91\x85\x90a\t\xC0\x90\x86\x90a(\xC1V[`\0\x80a\x0E\x85\x83a\x16\xCBV[\x90Pa\x0E\x923\x85\x83a\x178V[P`\x01\x93\x92PPPV[`\0a\x0B\xA4`6T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R`\0\x903\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0F\x19\x85\x85\x85\x85a\x18\x12V[\x95\x94PPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xA6\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x11\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x10IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`=T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra85`\xF0\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14\x15a\x10\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0C\xC0`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84a\x17\x87V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra77`\xF0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16a\x10\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P\x83B\x11\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x06\xE7`\xF3\x1B\x81RP\x90a\x11&W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`:` R`@\x81 T\x90a\x11Ia\x0B\x9AV[`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16\x92\x82\x01\x92\x90\x92R\x90\x8A\x16``\x82\x01R`\x80\x81\x01\x89\x90R`\xA0\x81\x01\x84\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a\x11\xE2\x92\x91\x90a\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83R\x81\x90R`\xFF\x88\x16\x91\x83\x01\x91\x90\x91R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12JW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a79`\xF0\x1B\x81RP\x90a\x12\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x12\xAD\x82`\x01a)\x17V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`:` R`@\x90 Ua\x12\xD1\x89\x89\x89a\x153V[PPPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x13K\x84\x84\x84\x84a\x19\x97V[`\x01`\x01`\xA0\x1B\x03\x83\x160\x14a\x0C\xC0W`=Ta\x0C\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x84a\x17\x87V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF6\x91\x90a(\xD8V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14a\x91\x90a(\xF5V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[PP`9\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra23`\xF0\x1B` \x82\x01R3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[Pa\x0E\x10\x83\x83\x83`\0a\x1B\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`5` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x15\xDAW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[\x80Qa\x0CZ\x90`7\x90` \x84\x01\x90a#\xC0V[\x80Qa\x0CZ\x90`8\x90` \x84\x01\x90a#\xC0V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x16Ma\x1DyV[\x80Q` \x91\x82\x01 `@\x80Q\x80\x82\x01\x82R`\x01\x81R`1`\xF8\x1B\x90\x84\x01R\x80Q\x92\x83\x01\x93\x90\x93R\x91\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0`\x01`\x01`\x80\x1B\x03\x82\x11\x15a\x174W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07GV[P\x90V[a\x0E\x10\x83\x83\x83`\x01`\x01`\x80\x1B\x03\x16`\x01a\x1B\xC0V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x17\x7FWP`;T\x90V[a\x0B\xA4a\x16\"V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x90`\0\x80`D\x83\x82\x89Z\xF1a\x17\xC4W=`\0\x80>=`\0\xFD[Pa\x17\xCE\x84a\x1D\x83V[a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt#\xA8;\x19\x1D\x1030\xB4\xB62\xB2\x10:90\xB79\xB32\xB9`Y\x1B`D\x82\x01R`d\x01a\x07GV[`\0\x80a\x18\x1F\x84\x84a\x1E,V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x0C\x8D`\xF2\x1B` \x82\x01R\x90\x91P\x81a\x18YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\0a\x18e\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x18\x9D\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x18\xA7\x83\x87a\x15\xB8V[a\x18\xB1\x91\x90a(\xC1V[\x90Pa\x18\xBC\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua\x19\x01\x87a\x18\xFC\x85a\x16\xCBV[a\x1EkV[`\0a\x19\r\x82\x88a)\x17V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16`\0`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x19C\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x90\x8B\x16\x90`\0\x80Q` a)\xAD\x839\x81Q\x91R\x90``\x01`@Q\x80\x91\x03\x90\xA3PP\x15\x96\x95PPPPPPV[`\0a\x19\xA3\x83\x83a\x1E,V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra25`\xF0\x1B` \x82\x01R\x90\x91P\x81a\x19\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07G\x91\x90a$\x9DV[P`\0a\x19\xE9\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x1A!\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x1A+\x83\x86a\x15\xB8V[a\x1A5\x91\x90a(\xC1V[\x90Pa\x1A@\x84a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua\x1A\x85\x87a\x1A\x80\x85a\x16\xCBV[a\x1F\\V[\x84\x81\x11\x15a\x1B\x19W`\0a\x1A\x99\x86\x83a(\xC1V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16`\0`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x1A\xCF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90\x81\x90`\0\x80Q` a)\xAD\x839\x81Q\x91R\x90``\x01`@Q\x80\x91\x03\x90\xA3Pa\x1B\xB7V[`\0a\x1B%\x82\x87a(\xC1V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x83`@Qa\x1B[\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x90\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x8A\x16\x90\x7FL\xF2[\xC1\xD9\x91\xC1u)\xC2R\x13\xD3\xCC\x0C\xDA)^\xEA\xAD_\x13\xF3a\x96\x9B\x12\xEAH\x01_\x90\x90``\x01[`@Q\x80\x91\x03\x90\xA3P[PPPPPPPV[`=T`@Qc\xD1^\0S`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01\x81\x90R\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xD1^\0S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CU\x91\x90a'\xFBV[\x90P`\0a\x1Cf\x82a\re\x89a\x15\x94V[\x90P`\0a\x1Cw\x83a\re\x89a\x15\x94V[\x90Pa\x1C\x85\x88\x88\x88\x86a\x1F\xA1V[\x84\x15a\x1D,W`@Qc\xD5\xED93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x89\x81\x16`$\x83\x01R\x88\x81\x16`D\x83\x01R`d\x82\x01\x88\x90R`\x84\x82\x01\x84\x90R`\xA4\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD5\xED93\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D'W=`\0\x80>=`\0\xFD[PPPP[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x90\x89\x16\x7FK\xEC\xCB\x90\xF9\x94\xC3\x1A\xCE\xD7\xA2;V\x11\x02\x07(\xA2=\x8E\xC5\xCD\xDD\x1A>\x9D\x97\xB9o\xDA\x86fa\x1De\x89\x87a\x1E,V[`@\x80Q\x91\x82R` \x82\x01\x88\x90R\x01a\x1B\xADV[``a\x0B\xA4a\x05RV[`\0a\x1D\xA9V[bF\x1B\xCD`\xE5\x1B`\0R` `\x04R\x80`$RP\x80`DRP`d`\0\xFD[=\x80\x15a\x1D\xE8W` \x81\x14a\x1E\x19Wa\x1D\xE3\x7FGPv2: malformed transfer result\0`\x1Fa\x1D\x8AV[a\x1E&V[\x82;a\x1E\x10Wa\x1E\x10s\x11\xD4\x1D\x8C\x8E\x88\x1B\x9B\xDD\x08\x18H\x18\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B`\x14a\x1D\x8AV[`\x01\x91Pa\x1E&V[=`\0\x80>`\0Q\x15\x15\x91P[P\x91\x90PV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x1EPW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`6Ta\x1E\x81`\x01`\x01`\x80\x1B\x03\x83\x16\x82a)\x17V[`6U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\x1E\xB0\x83\x82a)/V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`9Ta\x01\0\x90\x04\x16\x80\x15a\x1FUW`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c1\x87>.\x90a\x1F'\x90\x88\x90\x87\x90\x87\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1FAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xD1W=`\0\x80>=`\0\xFD[PPPPPV[`6Ta\x1Fr`\x01`\x01`\x80\x1B\x03\x83\x16\x82a(\xC1V[`6U`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\x1E\xB0\x83\x82a)\x84V[`\0a\x1F\xAC\x85a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a\x1F\xE4\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a\x1F\xEE\x83\x85a\x15\xB8V[a\x1F\xF8\x91\x90a(\xC1V[\x90P`\0a \x05\x86a\x15\x94V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`4` R`@\x81 T\x91\x92P\x90a =\x90\x83\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`\x80\x1B\x03\x16a\x15\xB8V[a G\x83\x87a\x15\xB8V[a Q\x91\x90a(\xC1V[\x90Pa \\\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua \x98\x85a\x16\xCBV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90Ua \xE7\x88\x88a \xE2a \xDD\x8A\x8Aa\x1E,V[a\x16\xCBV[a\"\x1DV[\x82\x15a!XW`@Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16\x90`\0\x90`\0\x80Q` a)\xCD\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x84\x81R` \x81\x01\x85\x90R\x80\x82\x01\x87\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x913\x91`\0\x80Q` a)\xAD\x839\x81Q\x91R\x91\x81\x90\x03``\x01\x90\xA3[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!zWP`\0\x81\x11[\x15a!\xEAW`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x90`\0\x90`\0\x80Q` a)\xCD\x839\x81Q\x91R\x90` \x01`@Q\x80\x91\x03\x90\xA3`@\x80Q\x82\x81R` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x913\x91`\0\x80Q` a)\xAD\x839\x81Q\x91R\x91\x81\x90\x03``\x01\x90\xA3[\x86`\x01`\x01`\xA0\x1B\x03\x16\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x80Q` a)\xCD\x839\x81Q\x91R\x88`@Qa\x1B\xAD\x91\x81R` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`4` R`@\x90 T`\x01`\x01`\x80\x1B\x03\x16a\"I\x82\x82a)\x84V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x80\x82 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x95\x86\x16\x17\x90U\x91\x86\x16\x81R T\x16a\"\x8F\x83\x82a)/V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`4` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`9Ta\x01\0\x90\x04\x16\x80\x15a#\xB8W`6T`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c1\x87>.\x90a#\t\x90\x8A\x90\x85\x90\x89\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a##W`\0\x80\xFD[PZ\xF1\x15\x80\x15a#7W=`\0\x80>=`\0\xFD[PPPP\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1B\xB7W`@Qc\x18\xC3\x9F\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c1\x87>.\x90a#\x84\x90\x89\x90\x85\x90\x88\x90`\x04\x01a)ZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\xB2W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[\x82\x80Ta#\xCC\x90a'\xC6V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a#\xEEW`\0\x85Ua$4V[\x82`\x1F\x10a$\x07W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua$4V[\x82\x80\x01`\x01\x01\x85U\x82\x15a$4W\x91\x82\x01[\x82\x81\x11\x15a$4W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a$\x19V[Pa\x174\x92\x91P[\x80\x82\x11\x15a\x174W`\0\x81U`\x01\x01a$<V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$vW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$ZV[\x81\x81\x11\x15a$\x88W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a$\xB0` \x83\x01\x84a$PV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xCCW`\0\x80\xFD[PV[\x805a$\xDA\x81a$\xB7V[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a$\xF2W`\0\x80\xFD[\x825a$\xFD\x81a$\xB7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a%\x1DW`\0\x80\xFD[\x815a$\xB0\x81a$\xB7V[\x805`\xFF\x81\x16\x81\x14a$\xDAW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a%KW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%cW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a%{W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\0\x8C\x8E\x03\x12\x15a%\xA4W`\0\x80\xFD[a%\xAD\x8Ca$\xCFV[\x9APa%\xBB` \x8D\x01a$\xCFV[\x99Pa%\xC9`@\x8D\x01a$\xCFV[\x98Pa%\xD7``\x8D\x01a$\xCFV[\x97Pa%\xE5`\x80\x8D\x01a%(V[\x96Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80`\xA0\x8E\x015\x11\x15a&\x01W`\0\x80\xFD[a&\x11\x8E`\xA0\x8F\x015\x8F\x01a%9V[\x90\x97P\x95P`\xC0\x8D\x015\x81\x10\x15a&'W`\0\x80\xFD[a&7\x8E`\xC0\x8F\x015\x8F\x01a%9V[\x90\x95P\x93P`\xE0\x8D\x015\x81\x10\x15a&MW`\0\x80\xFD[Pa&^\x8D`\xE0\x8E\x015\x8E\x01a%9V[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x8BW`\0\x80\xFD[\x835a&\x96\x81a$\xB7V[\x92P` \x84\x015a&\xA6\x81a$\xB7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a&\xCAW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&\xEFW`\0\x80\xFD[\x845a&\xFA\x81a$\xB7V[\x93P` \x85\x015a'\n\x81a$\xB7V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a':W`\0\x80\xFD[\x875a'E\x81a$\xB7V[\x96P` \x88\x015a'U\x81a$\xB7V[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa'q`\x80\x89\x01a%(V[\x92P`\xA0\x88\x015\x91P`\xC0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x80`@\x83\x85\x03\x12\x15a'\xA0W`\0\x80\xFD[\x825a'\xAB\x81a$\xB7V[\x91P` \x83\x015a'\xBB\x81a$\xB7V[\x80\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a'\xDAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1E&WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a(\rW`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x82R\x89\x16` \x82\x01R`\xFF\x88\x16`@\x82\x01R`\xC0``\x82\x01\x81\x90R`\0\x90a(s\x90\x83\x01\x88\x8Aa(\x14V[\x82\x81\x03`\x80\x84\x01Ra(\x86\x81\x87\x89a(\x14V[\x90P\x82\x81\x03`\xA0\x84\x01Ra(\x9B\x81\x85\x87a(\x14V[\x9C\x9BPPPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a(\xD3Wa(\xD3a(\xABV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a(\xEAW`\0\x80\xFD[\x81Qa$\xB0\x81a$\xB7V[`\0` \x82\x84\x03\x12\x15a)\x07W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a$\xB0W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a)*Wa)*a(\xABV[P\x01\x90V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a)QWa)Qa(\xABV[\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`@\x82\x01R``\x01\x90V[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)\xA4Wa)\xA4a(\xABV[\x03\x93\x92PPPV\xFEE\x8F_\xA4\x12\xD0\xF6\x9B\x08\xDD\x84\x87+\x02\x15g\\\xC6{\xC1\xD5\xB6\xFD\x930\n\x1C8x\xB8a\x96\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xD3\xFF-'=G\xAB\xEA\xBC\x06\x04b\xA2&zJNf^i\x16H\xE6\x9E\xE2\x9E%:)V!\xFBdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static DELEGATIONAWAREATOKEN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DelegationAwareAToken<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DelegationAwareAToken<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DelegationAwareAToken<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DelegationAwareAToken<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DelegationAwareAToken<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DelegationAwareAToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DelegationAwareAToken<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DELEGATIONAWAREATOKEN_ABI.clone(),
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
                DELEGATIONAWAREATOKEN_ABI.clone(),
                DELEGATIONAWAREATOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ATOKEN_REVISION` (0x0bd7ad3b) function
        pub fn atoken_revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 215, 173, 59], ())
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
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
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
        ///Calls the contract's `RESERVE_TREASURY_ADDRESS` (0xae167335) function
        pub fn reserve_treasury_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 22, 115, 53], ())
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
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0xd7020d0a) function
        pub fn burn(
            &self,
            from: ::ethers::core::types::Address,
            receiver_of_underlying: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 2, 13, 10],
                    (from, receiver_of_underlying, amount, index),
                )
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
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateUnderlyingTo` (0x2f114618) function
        pub fn delegate_underlying_to(
            &self,
            delegatee: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 17, 70, 24], delegatee)
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
        ///Calls the contract's `getPreviousIndex` (0xe0753986) function
        pub fn get_previous_index(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([224, 117, 57, 134], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getScaledUserBalanceAndSupply` (0x0afbcdc9) function
        pub fn get_scaled_user_balance_and_supply(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 251, 205, 201], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleRepayment` (0x6fd97676) function
        pub fn handle_repayment(
            &self,
            user: ::ethers::core::types::Address,
            on_behalf_of: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 217, 118, 118], (user, on_behalf_of, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x183fb413) function
        pub fn initialize(
            &self,
            initializing_pool: ::ethers::core::types::Address,
            treasury: ::ethers::core::types::Address,
            underlying_asset: ::ethers::core::types::Address,
            incentives_controller: ::ethers::core::types::Address,
            a_token_decimals: u8,
            a_token_name: ::std::string::String,
            a_token_symbol: ::std::string::String,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [24, 63, 180, 19],
                    (
                        initializing_pool,
                        treasury,
                        underlying_asset,
                        incentives_controller,
                        a_token_decimals,
                        a_token_name,
                        a_token_symbol,
                        params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0xb3f1c93d) function
        pub fn mint(
            &self,
            caller: ::ethers::core::types::Address,
            on_behalf_of: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 241, 201, 61], (caller, on_behalf_of, amount, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintToTreasury` (0x7df5bd3b) function
        pub fn mint_to_treasury(
            &self,
            amount: ::ethers::core::types::U256,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 245, 189, 59], (amount, index))
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
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
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
        ///Calls the contract's `scaledBalanceOf` (0x1da24f3e) function
        pub fn scaled_balance_of(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([29, 162, 79, 62], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `scaledTotalSupply` (0xb1bf962d) function
        pub fn scaled_total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([177, 191, 150, 45], ())
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
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOnLiquidation` (0xf866c319) function
        pub fn transfer_on_liquidation(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 102, 195, 25], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferUnderlyingTo` (0x4efecaa5) function
        pub fn transfer_underlying_to(
            &self,
            target: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 254, 202, 165], (target, amount))
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
        ///Gets the contract's `BalanceTransfer` event
        pub fn balance_transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BalanceTransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `DelegateUnderlyingTo` event
        pub fn delegate_underlying_to_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegateUnderlyingToFilter,
        > {
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
            DelegationAwareATokenEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DelegationAwareAToken<M> {
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
        name = "BalanceTransfer",
        abi = "BalanceTransfer(address,address,uint256,uint256)"
    )]
    pub struct BalanceTransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,address,uint256,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub balance_increase: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    #[ethevent(name = "DelegateUnderlyingTo", abi = "DelegateUnderlyingTo(address)")]
    pub struct DelegateUnderlyingToFilter {
        #[ethevent(indexed)]
        pub delegatee: ::ethers::core::types::Address,
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
        abi = "Initialized(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializedFilter {
        #[ethevent(indexed)]
        pub underlying_asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
        pub treasury: ::ethers::core::types::Address,
        pub incentives_controller: ::ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: ::std::string::String,
        pub a_token_symbol: ::std::string::String,
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
    #[ethevent(name = "Mint", abi = "Mint(address,address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub balance_increase: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    pub enum DelegationAwareATokenEvents {
        ApprovalFilter(ApprovalFilter),
        BalanceTransferFilter(BalanceTransferFilter),
        BurnFilter(BurnFilter),
        DelegateUnderlyingToFilter(DelegateUnderlyingToFilter),
        InitializedFilter(InitializedFilter),
        MintFilter(MintFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for DelegationAwareATokenEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BalanceTransferFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::BalanceTransferFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = DelegateUnderlyingToFilter::decode_log(log) {
                return Ok(
                    DelegationAwareATokenEvents::DelegateUnderlyingToFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(DelegationAwareATokenEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DelegationAwareATokenEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceTransferFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateUnderlyingToFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for DelegationAwareATokenEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BalanceTransferFilter> for DelegationAwareATokenEvents {
        fn from(value: BalanceTransferFilter) -> Self {
            Self::BalanceTransferFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for DelegationAwareATokenEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<DelegateUnderlyingToFilter>
    for DelegationAwareATokenEvents {
        fn from(value: DelegateUnderlyingToFilter) -> Self {
            Self::DelegateUnderlyingToFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for DelegationAwareATokenEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for DelegationAwareATokenEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for DelegationAwareATokenEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `ATOKEN_REVISION` function with signature `ATOKEN_REVISION()` and selector `0x0bd7ad3b`
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
    #[ethcall(name = "ATOKEN_REVISION", abi = "ATOKEN_REVISION()")]
    pub struct AtokenRevisionCall;
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
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
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
    ///Container type for all input parameters for the `RESERVE_TREASURY_ADDRESS` function with signature `RESERVE_TREASURY_ADDRESS()` and selector `0xae167335`
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
    #[ethcall(name = "RESERVE_TREASURY_ADDRESS", abi = "RESERVE_TREASURY_ADDRESS()")]
    pub struct ReserveTreasuryAddressCall;
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
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
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
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
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
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(address,address,uint256,uint256)` and selector `0xd7020d0a`
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
    #[ethcall(name = "burn", abi = "burn(address,address,uint256,uint256)")]
    pub struct BurnCall {
        pub from: ::ethers::core::types::Address,
        pub receiver_of_underlying: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `delegateUnderlyingTo` function with signature `delegateUnderlyingTo(address)` and selector `0x2f114618`
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
    #[ethcall(name = "delegateUnderlyingTo", abi = "delegateUnderlyingTo(address)")]
    pub struct DelegateUnderlyingToCall {
        pub delegatee: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `getPreviousIndex` function with signature `getPreviousIndex(address)` and selector `0xe0753986`
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
    #[ethcall(name = "getPreviousIndex", abi = "getPreviousIndex(address)")]
    pub struct GetPreviousIndexCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getScaledUserBalanceAndSupply` function with signature `getScaledUserBalanceAndSupply(address)` and selector `0x0afbcdc9`
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
        name = "getScaledUserBalanceAndSupply",
        abi = "getScaledUserBalanceAndSupply(address)"
    )]
    pub struct GetScaledUserBalanceAndSupplyCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `handleRepayment` function with signature `handleRepayment(address,address,uint256)` and selector `0x6fd97676`
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
        name = "handleRepayment",
        abi = "handleRepayment(address,address,uint256)"
    )]
    pub struct HandleRepaymentCall {
        pub user: ::ethers::core::types::Address,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,uint8,string,string,bytes)` and selector `0x183fb413`
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
        abi = "initialize(address,address,address,address,uint8,string,string,bytes)"
    )]
    pub struct InitializeCall {
        pub initializing_pool: ::ethers::core::types::Address,
        pub treasury: ::ethers::core::types::Address,
        pub underlying_asset: ::ethers::core::types::Address,
        pub incentives_controller: ::ethers::core::types::Address,
        pub a_token_decimals: u8,
        pub a_token_name: ::std::string::String,
        pub a_token_symbol: ::std::string::String,
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
        pub caller: ::ethers::core::types::Address,
        pub on_behalf_of: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintToTreasury` function with signature `mintToTreasury(uint256,uint256)` and selector `0x7df5bd3b`
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
    #[ethcall(name = "mintToTreasury", abi = "mintToTreasury(uint256,uint256)")]
    pub struct MintToTreasuryCall {
        pub amount: ::ethers::core::types::U256,
        pub index: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
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
    ///Container type for all input parameters for the `scaledBalanceOf` function with signature `scaledBalanceOf(address)` and selector `0x1da24f3e`
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
    #[ethcall(name = "scaledBalanceOf", abi = "scaledBalanceOf(address)")]
    pub struct ScaledBalanceOfCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `scaledTotalSupply` function with signature `scaledTotalSupply()` and selector `0xb1bf962d`
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
    #[ethcall(name = "scaledTotalSupply", abi = "scaledTotalSupply()")]
    pub struct ScaledTotalSupplyCall;
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
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
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
    pub struct TransferFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOnLiquidation` function with signature `transferOnLiquidation(address,address,uint256)` and selector `0xf866c319`
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
        name = "transferOnLiquidation",
        abi = "transferOnLiquidation(address,address,uint256)"
    )]
    pub struct TransferOnLiquidationCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferUnderlyingTo` function with signature `transferUnderlyingTo(address,uint256)` and selector `0x4efecaa5`
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
        name = "transferUnderlyingTo",
        abi = "transferUnderlyingTo(address,uint256)"
    )]
    pub struct TransferUnderlyingToCall {
        pub target: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegationAwareATokenCalls {
        AtokenRevision(AtokenRevisionCall),
        DomainSeparator(DomainSeparatorCall),
        Eip712Revision(Eip712RevisionCall),
        PermitTypehash(PermitTypehashCall),
        Pool(PoolCall),
        ReserveTreasuryAddress(ReserveTreasuryAddressCall),
        UnderlyingAssetAddress(UnderlyingAssetAddressCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DelegateUnderlyingTo(DelegateUnderlyingToCall),
        GetIncentivesController(GetIncentivesControllerCall),
        GetPreviousIndex(GetPreviousIndexCall),
        GetScaledUserBalanceAndSupply(GetScaledUserBalanceAndSupplyCall),
        HandleRepayment(HandleRepaymentCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Mint(MintCall),
        MintToTreasury(MintToTreasuryCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        RescueTokens(RescueTokensCall),
        ScaledBalanceOf(ScaledBalanceOfCall),
        ScaledTotalSupply(ScaledTotalSupplyCall),
        SetIncentivesController(SetIncentivesControllerCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOnLiquidation(TransferOnLiquidationCall),
        TransferUnderlyingTo(TransferUnderlyingToCall),
    }
    impl ::ethers::core::abi::AbiDecode for DelegationAwareATokenCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AtokenRevisionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AtokenRevision(decoded));
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
                = <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PermitTypehash(decoded));
            }
            if let Ok(decoded)
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded)
                = <ReserveTreasuryAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ReserveTreasuryAddress(decoded));
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
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
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
                = <DelegateUnderlyingToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DelegateUnderlyingTo(decoded));
            }
            if let Ok(decoded)
                = <GetIncentivesControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetIncentivesController(decoded));
            }
            if let Ok(decoded)
                = <GetPreviousIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPreviousIndex(decoded));
            }
            if let Ok(decoded)
                = <GetScaledUserBalanceAndSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetScaledUserBalanceAndSupply(decoded));
            }
            if let Ok(decoded)
                = <HandleRepaymentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HandleRepayment(decoded));
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
                = <MintToTreasuryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintToTreasury(decoded));
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
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <RescueTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescueTokens(decoded));
            }
            if let Ok(decoded)
                = <ScaledBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ScaledBalanceOf(decoded));
            }
            if let Ok(decoded)
                = <ScaledTotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ScaledTotalSupply(decoded));
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
            if let Ok(decoded)
                = <TransferOnLiquidationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOnLiquidation(decoded));
            }
            if let Ok(decoded)
                = <TransferUnderlyingToCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferUnderlyingTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DelegationAwareATokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AtokenRevision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Revision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveTreasuryAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingAssetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateUnderlyingTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetIncentivesController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPreviousIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetScaledUserBalanceAndSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HandleRepayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintToTreasury(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RescueTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScaledBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScaledTotalSupply(element) => {
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
                Self::TransferOnLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferUnderlyingTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DelegationAwareATokenCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AtokenRevision(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Revision(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveTreasuryAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnderlyingAssetAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateUnderlyingTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetIncentivesController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPreviousIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetScaledUserBalanceAndSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HandleRepayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintToTreasury(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::ScaledBalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ScaledTotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetIncentivesController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOnLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferUnderlyingTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AtokenRevisionCall> for DelegationAwareATokenCalls {
        fn from(value: AtokenRevisionCall) -> Self {
            Self::AtokenRevision(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for DelegationAwareATokenCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<Eip712RevisionCall> for DelegationAwareATokenCalls {
        fn from(value: Eip712RevisionCall) -> Self {
            Self::Eip712Revision(value)
        }
    }
    impl ::core::convert::From<PermitTypehashCall> for DelegationAwareATokenCalls {
        fn from(value: PermitTypehashCall) -> Self {
            Self::PermitTypehash(value)
        }
    }
    impl ::core::convert::From<PoolCall> for DelegationAwareATokenCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ReserveTreasuryAddressCall>
    for DelegationAwareATokenCalls {
        fn from(value: ReserveTreasuryAddressCall) -> Self {
            Self::ReserveTreasuryAddress(value)
        }
    }
    impl ::core::convert::From<UnderlyingAssetAddressCall>
    for DelegationAwareATokenCalls {
        fn from(value: UnderlyingAssetAddressCall) -> Self {
            Self::UnderlyingAssetAddress(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for DelegationAwareATokenCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for DelegationAwareATokenCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DelegationAwareATokenCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for DelegationAwareATokenCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for DelegationAwareATokenCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for DelegationAwareATokenCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DelegateUnderlyingToCall> for DelegationAwareATokenCalls {
        fn from(value: DelegateUnderlyingToCall) -> Self {
            Self::DelegateUnderlyingTo(value)
        }
    }
    impl ::core::convert::From<GetIncentivesControllerCall>
    for DelegationAwareATokenCalls {
        fn from(value: GetIncentivesControllerCall) -> Self {
            Self::GetIncentivesController(value)
        }
    }
    impl ::core::convert::From<GetPreviousIndexCall> for DelegationAwareATokenCalls {
        fn from(value: GetPreviousIndexCall) -> Self {
            Self::GetPreviousIndex(value)
        }
    }
    impl ::core::convert::From<GetScaledUserBalanceAndSupplyCall>
    for DelegationAwareATokenCalls {
        fn from(value: GetScaledUserBalanceAndSupplyCall) -> Self {
            Self::GetScaledUserBalanceAndSupply(value)
        }
    }
    impl ::core::convert::From<HandleRepaymentCall> for DelegationAwareATokenCalls {
        fn from(value: HandleRepaymentCall) -> Self {
            Self::HandleRepayment(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for DelegationAwareATokenCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for DelegationAwareATokenCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MintCall> for DelegationAwareATokenCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintToTreasuryCall> for DelegationAwareATokenCalls {
        fn from(value: MintToTreasuryCall) -> Self {
            Self::MintToTreasury(value)
        }
    }
    impl ::core::convert::From<NameCall> for DelegationAwareATokenCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for DelegationAwareATokenCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for DelegationAwareATokenCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<RescueTokensCall> for DelegationAwareATokenCalls {
        fn from(value: RescueTokensCall) -> Self {
            Self::RescueTokens(value)
        }
    }
    impl ::core::convert::From<ScaledBalanceOfCall> for DelegationAwareATokenCalls {
        fn from(value: ScaledBalanceOfCall) -> Self {
            Self::ScaledBalanceOf(value)
        }
    }
    impl ::core::convert::From<ScaledTotalSupplyCall> for DelegationAwareATokenCalls {
        fn from(value: ScaledTotalSupplyCall) -> Self {
            Self::ScaledTotalSupply(value)
        }
    }
    impl ::core::convert::From<SetIncentivesControllerCall>
    for DelegationAwareATokenCalls {
        fn from(value: SetIncentivesControllerCall) -> Self {
            Self::SetIncentivesController(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for DelegationAwareATokenCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for DelegationAwareATokenCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for DelegationAwareATokenCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for DelegationAwareATokenCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOnLiquidationCall>
    for DelegationAwareATokenCalls {
        fn from(value: TransferOnLiquidationCall) -> Self {
            Self::TransferOnLiquidation(value)
        }
    }
    impl ::core::convert::From<TransferUnderlyingToCall> for DelegationAwareATokenCalls {
        fn from(value: TransferUnderlyingToCall) -> Self {
            Self::TransferUnderlyingTo(value)
        }
    }
    ///Container type for all return fields from the `ATOKEN_REVISION` function with signature `ATOKEN_REVISION()` and selector `0x0bd7ad3b`
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
    pub struct AtokenRevisionReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    pub struct PermitTypehashReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `RESERVE_TREASURY_ADDRESS` function with signature `RESERVE_TREASURY_ADDRESS()` and selector `0xae167335`
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
    pub struct ReserveTreasuryAddressReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getPreviousIndex` function with signature `getPreviousIndex(address)` and selector `0xe0753986`
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
    pub struct GetPreviousIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getScaledUserBalanceAndSupply` function with signature `getScaledUserBalanceAndSupply(address)` and selector `0x0afbcdc9`
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
    pub struct GetScaledUserBalanceAndSupplyReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    pub struct MintReturn(pub bool);
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
    ///Container type for all return fields from the `scaledBalanceOf` function with signature `scaledBalanceOf(address)` and selector `0x1da24f3e`
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
    pub struct ScaledBalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `scaledTotalSupply` function with signature `scaledTotalSupply()` and selector `0xb1bf962d`
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
    pub struct ScaledTotalSupplyReturn(pub ::ethers::core::types::U256);
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
