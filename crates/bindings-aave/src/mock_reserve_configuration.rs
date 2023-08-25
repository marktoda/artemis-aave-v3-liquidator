pub use mock_reserve_configuration::*;
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
pub mod mock_reserve_configuration {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("configuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configuration"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("getBorrowCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBorrowCap"),
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
                    ::std::borrow::ToOwned::to_owned("getBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBorrowingEnabled",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCaps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCaps"),
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
                    ::std::borrow::ToOwned::to_owned("getDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("getEModeCategory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEModeCategory"),
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
                    ::std::borrow::ToOwned::to_owned("getFlags"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFlags"),
                            inputs: ::std::vec![],
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFlashLoanEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFlashLoanEnabled",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFrozen"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidationBonus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationBonus",
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
                    ::std::borrow::ToOwned::to_owned("getLiquidationProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationProtocolFee",
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
                    ::std::borrow::ToOwned::to_owned("getLiquidationThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationThreshold",
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
                    ::std::borrow::ToOwned::to_owned("getLtv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLtv"),
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
                    ::std::borrow::ToOwned::to_owned("getParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getParams"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserveFactor"),
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
                    ::std::borrow::ToOwned::to_owned("getStableRateBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateBorrowingEnabled",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSupplyCap"),
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
                    ::std::borrow::ToOwned::to_owned("getUnbackedMintCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUnbackedMintCap"),
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
                    ::std::borrow::ToOwned::to_owned("setBorrowCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBorrowCap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowCap"),
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
                    ::std::borrow::ToOwned::to_owned("setBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setBorrowingEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
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
                    ::std::borrow::ToOwned::to_owned("setDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDecimals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("setEModeCategory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEModeCategory"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("categoryId"),
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
                    ::std::borrow::ToOwned::to_owned("setFlashLoanEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setFlashLoanEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
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
                    ::std::borrow::ToOwned::to_owned("setFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("frozen"),
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
                    ::std::borrow::ToOwned::to_owned("setLiquidationBonus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLiquidationBonus",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bonus"),
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
                    ::std::borrow::ToOwned::to_owned("setLiquidationProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLiquidationProtocolFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationProtocolFee",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setLiquidationThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLiquidationThreshold",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("threshold"),
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
                    ::std::borrow::ToOwned::to_owned("setLtv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLtv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
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
                    ::std::borrow::ToOwned::to_owned("setReserveFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReserveFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveFactor"),
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
                    ::std::borrow::ToOwned::to_owned("setStableRateBorrowingEnabled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStableRateBorrowingEnabled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
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
                    ::std::borrow::ToOwned::to_owned("setSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSupplyCap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("supplyCap"),
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
                    ::std::borrow::ToOwned::to_owned("setUnbackedMintCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUnbackedMintCap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("unbackedMintCap"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKRESERVECONFIGURATION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0C\x96\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xF0W`\x005`\xE0\x1C\x80c\x8C\x88\x85\xC8\x11a\x01\x0FW\x80c\xC3{\xDC\xEC\x11a\0\xA2W\x80c\xEA\xD8\xAA\x02\x11a\0qW\x80c\xEA\xD8\xAA\x02\x14a\x05\x18W\x80c\xF0\x14\x1D\x84\x14a\x059W\x80c\xF1QJ\x1A\x14a\x05VW\x80c\xFAW=\x07\x14a\x05iW`\0\x80\xFD[\x80c\xC3{\xDC\xEC\x14a\x04\xA5W\x80c\xD0\xB0\xC8\x16\x14a\x04\xC3W\x80c\xD1\xC1\x1F\x18\x14a\x04\xD6W\x80c\xE0\x8A(\xA3\x14a\x04\xF9W`\0\x80\xFD[\x80c\xA5Q\x02\xF7\x11a\0\xDEW\x80c\xA5Q\x02\xF7\x14a\x04KW\x80c\xA6 \x065\x14a\x04^W\x80c\xAE\xDE{v\x14a\x04qW\x80c\xB6\xA3\xF5\x9A\x14a\x04\x92W`\0\x80\xFD[\x80c\x8C\x88\x85\xC8\x14a\x03\xD9W\x80c\x92\xDF\xB2\xFB\x14a\x03\xECW\x80c\x9Dpm1\x14a\x03\xFFW\x80c\xA3~R\xE3\x14a\x048W`\0\x80\xFD[\x80clp\xBE\xE9\x11a\x01\x87W\x80ct\x95\xB3S\x11a\x01VW\x80ct\x95\xB3S\x14a\x03aW\x80cyu\x0B\xC4\x14a\x03\x8CW\x80c~\x93-2\x14a\x03\xABW\x80c\x81E\xBD.\x14a\x03\xBEW`\0\x80\xFD[\x80clp\xBE\xE9\x14a\x02\xF7W\x80cl\xC7\x14\x9D\x14a\x03\x01W\x80cqq\x86\xD1\x14a\x03;W\x80cq\xCB\x132\x14a\x03NW`\0\x80\xFD[\x80cJ\xE9\xB8\xBC\x11a\x01\xC3W\x80cJ\xE9\xB8\xBC\x14a\x02jW\x80cY\xAA\x9Er\x14a\x02\x88W\x80c^aZk\x14a\x02\xA6W\x80c_U\x8ES\x14a\x02\xDBW`\0\x80\xFD[\x80c\x1CDi\x83\x14a\x01\xF5W\x80c 6\x18\x14\x14a\x02\nW\x80c(\x84-O\x14a\x02:W\x80c5o#\\\x14a\x02MW[`\0\x80\xFD[a\x02\x08a\x02\x036`\x04a\x0B\xC9V[a\x05|V[\0[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`t\x1Cd\x0F\xFF\xFF\xFF\xFF\x16[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x08a\x02H6`\x04a\x0B\xC9V[a\x05\x9DV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\xA8\x1C`\xFF\x16a\x02'V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x10\x1Ca\xFF\xFF\x16a\x02'V[`@\x80Q` \x80\x82\x01\x90\x92R`\0T\x90\x81\x90R\x90\x1Ca\xFF\xFF\x16a\x02'V[a\x02\xAEa\x05\xB7V[`@\x80Q\x96\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x021V[`@\x80Q` \x81\x01\x82R`\0T\x90\x81\x90R\x90\x1Ca\xFF\xFF\x16a\x02'V[`\0Ta\x02'\x90\x81V[a\x03\ta\x06 V[`@\x80Q\x95\x15\x15\x86R\x93\x15\x15` \x86\x01R\x91\x15\x15\x92\x84\x01\x92\x90\x92R\x90\x15\x15``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x021V[a\x02\x08a\x03I6`\x04a\x0B\xC9V[a\x06\x8FV[a\x02\x08a\x03\\6`\x04a\x0B\xE2V[a\x06\xA9V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`9\x1B\x16\x15\x15[`@Q\x90\x15\x15\x81R` \x01a\x021V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`:\x1B\x16\x15\x15a\x03|V[a\x02\x08a\x03\xB96`\x04a\x0B\xE2V[a\x06\xC3V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Ra\xFF\xFF\x16a\x02'V[a\x02\x08a\x03\xE76`\x04a\x0B\xC9V[a\x06\xDDV[a\x02\x08a\x03\xFA6`\x04a\x0B\xC9V[a\x06\xF7V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Rd\x0F\xFF\xFF\xFF\xFF`P\x82\x90\x1C\x81\x16\x91`t\x1C\x16`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x021V[a\x02\x08a\x04F6`\x04a\x0B\xC9V[a\x07\x11V[a\x02\x08a\x04Y6`\x04a\x0B\xE2V[a\x07+V[a\x02\x08a\x04l6`\x04a\x0B\xC9V[a\x07EV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`P\x1Cd\x0F\xFF\xFF\xFF\xFF\x16a\x02'V[a\x02\x08a\x04\xA06`\x04a\x0B\xC9V[a\x07_V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x98\x1Ca\xFF\xFF\x16a\x02'V[a\x02\x08a\x04\xD16`\x04a\x0B\xC9V[a\x07yV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Rg\x80\0\0\0\0\0\0\0\x16\x15\x15a\x03|V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`;\x1B\x16\x15\x15a\x03|V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\xB0\x1Cd\x0F\xFF\xFF\xFF\xFF\x16a\x02'V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`0\x1C`\xFF\x16a\x02'V[a\x02\x08a\x05d6`\x04a\x0B\xE2V[a\x07\x93V[a\x02\x08a\x05w6`\x04a\x0B\xC9V[a\x07\xADV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x07\xC7V[Q`\0UPV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08'V[`\0\x80`\0\x80`\0\x80a\x06\r`\0`@Q\x80` \x01`@R\x90\x81`\0\x82\x01T\x81RPPQa\xFF\xFF\x80\x82\x16\x92`\x10\x83\x90\x1C\x82\x16\x92` \x81\x90\x1C\x83\x16\x92`0\x82\x90\x1C`\xFF\x90\x81\x16\x93`@\x84\x90\x1C\x90\x92\x16\x92`\xA8\x1C\x16\x90V[\x94\x9B\x93\x9AP\x91\x98P\x96P\x94P\x90\x92P\x90PV[`\0\x80`\0\x80`\0a\x06~`\0`@Q\x80` \x01`@R\x90\x81`\0\x82\x01T\x81RPPQg\x01\0\0\0\0\0\0\0\x81\x16\x15\x15\x91`\x01`9\x1B\x82\x16\x15\x15\x91`\x01`:\x1B\x81\x16\x15\x15\x91`\x01`;\x1B\x82\x16\x15\x15\x91g\x10\0\0\0\0\0\0\0\x16\x15\x15\x90V[\x94P\x94P\x94P\x94P\x94P\x90\x91\x92\x93\x94V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08zV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08\xD2V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\0V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t.V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\x81V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\xD9V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\"V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\nPV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\xA2V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\xFAV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x0BKV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x0ByV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra67`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x08\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[`@Q\x80\x91\x03\x90\xFD[P\x81Qi\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`@\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra65`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x08cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qe\xFF\xFF\0\0\0\0\x19\x16` \x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x06\xC7`\xF3\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`P\x1B\x19\x16`P\x91\x90\x91\x1B\x17\x90RV[`;\x81a\x08\xE0W`\0a\x08\xE3V[`\x01[\x83Qg\x08\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`9\x81a\t\x0EW`\0a\t\x11V[`\x01[\x83Qg\x02\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x1B\x1B`\xF1\x1B` \x82\x01R`\xFF\x82\x11\x15a\tiW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qf\xFF\0\0\0\0\0\0\x19\x16`0\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x1B\x99`\xF1\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\t\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra63`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qa\xFF\xFF\x19\x16\x17\x90RV[`?\x81a\n0W`\0a\n3V[`\x01[\x83Qg\x80\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x03s`\xF4\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\n\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qa\xFF\xFF`\x98\x1B\x19\x16`\x98\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra69`\xF0\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`t\x1B\x19\x16`t\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\r\x8D`\xF2\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x0B6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qc\xFF\xFF\0\0\x19\x16`\x10\x91\x90\x91\x1B\x17\x90RV[`:\x81a\x0BYW`\0a\x0B\\V[`\x01[\x83Qg\x04\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra71`\xF0\x1B` \x82\x01R`\xFF\x82\x11\x15a\x0B\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Q`\xFF`\xA8\x1B\x19\x16`\xA8\x91\x90\x91\x1B\x17\x90RV[`\0` \x82\x84\x03\x12\x15a\x0B\xDBW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xF4W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x0C\x04W`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0C8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0C\x1CV[\x81\x81\x11\x15a\x0CJW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x16\xD8\xCDj](8d+\xED\x92\xEF\xCF\x8F\xA8\xA3\x9A\xDD\xC7'A\xC12\xAFp{\xDB\xC7\xB43\x05\xB8dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static MOCKRESERVECONFIGURATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xF0W`\x005`\xE0\x1C\x80c\x8C\x88\x85\xC8\x11a\x01\x0FW\x80c\xC3{\xDC\xEC\x11a\0\xA2W\x80c\xEA\xD8\xAA\x02\x11a\0qW\x80c\xEA\xD8\xAA\x02\x14a\x05\x18W\x80c\xF0\x14\x1D\x84\x14a\x059W\x80c\xF1QJ\x1A\x14a\x05VW\x80c\xFAW=\x07\x14a\x05iW`\0\x80\xFD[\x80c\xC3{\xDC\xEC\x14a\x04\xA5W\x80c\xD0\xB0\xC8\x16\x14a\x04\xC3W\x80c\xD1\xC1\x1F\x18\x14a\x04\xD6W\x80c\xE0\x8A(\xA3\x14a\x04\xF9W`\0\x80\xFD[\x80c\xA5Q\x02\xF7\x11a\0\xDEW\x80c\xA5Q\x02\xF7\x14a\x04KW\x80c\xA6 \x065\x14a\x04^W\x80c\xAE\xDE{v\x14a\x04qW\x80c\xB6\xA3\xF5\x9A\x14a\x04\x92W`\0\x80\xFD[\x80c\x8C\x88\x85\xC8\x14a\x03\xD9W\x80c\x92\xDF\xB2\xFB\x14a\x03\xECW\x80c\x9Dpm1\x14a\x03\xFFW\x80c\xA3~R\xE3\x14a\x048W`\0\x80\xFD[\x80clp\xBE\xE9\x11a\x01\x87W\x80ct\x95\xB3S\x11a\x01VW\x80ct\x95\xB3S\x14a\x03aW\x80cyu\x0B\xC4\x14a\x03\x8CW\x80c~\x93-2\x14a\x03\xABW\x80c\x81E\xBD.\x14a\x03\xBEW`\0\x80\xFD[\x80clp\xBE\xE9\x14a\x02\xF7W\x80cl\xC7\x14\x9D\x14a\x03\x01W\x80cqq\x86\xD1\x14a\x03;W\x80cq\xCB\x132\x14a\x03NW`\0\x80\xFD[\x80cJ\xE9\xB8\xBC\x11a\x01\xC3W\x80cJ\xE9\xB8\xBC\x14a\x02jW\x80cY\xAA\x9Er\x14a\x02\x88W\x80c^aZk\x14a\x02\xA6W\x80c_U\x8ES\x14a\x02\xDBW`\0\x80\xFD[\x80c\x1CDi\x83\x14a\x01\xF5W\x80c 6\x18\x14\x14a\x02\nW\x80c(\x84-O\x14a\x02:W\x80c5o#\\\x14a\x02MW[`\0\x80\xFD[a\x02\x08a\x02\x036`\x04a\x0B\xC9V[a\x05|V[\0[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`t\x1Cd\x0F\xFF\xFF\xFF\xFF\x16[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x08a\x02H6`\x04a\x0B\xC9V[a\x05\x9DV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\xA8\x1C`\xFF\x16a\x02'V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x10\x1Ca\xFF\xFF\x16a\x02'V[`@\x80Q` \x80\x82\x01\x90\x92R`\0T\x90\x81\x90R\x90\x1Ca\xFF\xFF\x16a\x02'V[a\x02\xAEa\x05\xB7V[`@\x80Q\x96\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\x021V[`@\x80Q` \x81\x01\x82R`\0T\x90\x81\x90R\x90\x1Ca\xFF\xFF\x16a\x02'V[`\0Ta\x02'\x90\x81V[a\x03\ta\x06 V[`@\x80Q\x95\x15\x15\x86R\x93\x15\x15` \x86\x01R\x91\x15\x15\x92\x84\x01\x92\x90\x92R\x90\x15\x15``\x83\x01R\x15\x15`\x80\x82\x01R`\xA0\x01a\x021V[a\x02\x08a\x03I6`\x04a\x0B\xC9V[a\x06\x8FV[a\x02\x08a\x03\\6`\x04a\x0B\xE2V[a\x06\xA9V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`9\x1B\x16\x15\x15[`@Q\x90\x15\x15\x81R` \x01a\x021V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`:\x1B\x16\x15\x15a\x03|V[a\x02\x08a\x03\xB96`\x04a\x0B\xE2V[a\x06\xC3V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Ra\xFF\xFF\x16a\x02'V[a\x02\x08a\x03\xE76`\x04a\x0B\xC9V[a\x06\xDDV[a\x02\x08a\x03\xFA6`\x04a\x0B\xC9V[a\x06\xF7V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Rd\x0F\xFF\xFF\xFF\xFF`P\x82\x90\x1C\x81\x16\x91`t\x1C\x16`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x021V[a\x02\x08a\x04F6`\x04a\x0B\xC9V[a\x07\x11V[a\x02\x08a\x04Y6`\x04a\x0B\xE2V[a\x07+V[a\x02\x08a\x04l6`\x04a\x0B\xC9V[a\x07EV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`P\x1Cd\x0F\xFF\xFF\xFF\xFF\x16a\x02'V[a\x02\x08a\x04\xA06`\x04a\x0B\xC9V[a\x07_V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x98\x1Ca\xFF\xFF\x16a\x02'V[a\x02\x08a\x04\xD16`\x04a\x0B\xC9V[a\x07yV[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90Rg\x80\0\0\0\0\0\0\0\x16\x15\x15a\x03|V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\x01`;\x1B\x16\x15\x15a\x03|V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`\xB0\x1Cd\x0F\xFF\xFF\xFF\xFF\x16a\x02'V[`@\x80Q` \x81\x01\x90\x91R`\0T\x90\x81\x90R`0\x1C`\xFF\x16a\x02'V[a\x02\x08a\x05d6`\x04a\x0B\xE2V[a\x07\x93V[a\x02\x08a\x05w6`\x04a\x0B\xC9V[a\x07\xADV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x07\xC7V[Q`\0UPV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08'V[`\0\x80`\0\x80`\0\x80a\x06\r`\0`@Q\x80` \x01`@R\x90\x81`\0\x82\x01T\x81RPPQa\xFF\xFF\x80\x82\x16\x92`\x10\x83\x90\x1C\x82\x16\x92` \x81\x90\x1C\x83\x16\x92`0\x82\x90\x1C`\xFF\x90\x81\x16\x93`@\x84\x90\x1C\x90\x92\x16\x92`\xA8\x1C\x16\x90V[\x94\x9B\x93\x9AP\x91\x98P\x96P\x94P\x90\x92P\x90PV[`\0\x80`\0\x80`\0a\x06~`\0`@Q\x80` \x01`@R\x90\x81`\0\x82\x01T\x81RPPQg\x01\0\0\0\0\0\0\0\x81\x16\x15\x15\x91`\x01`9\x1B\x82\x16\x15\x15\x91`\x01`:\x1B\x81\x16\x15\x15\x91`\x01`;\x1B\x82\x16\x15\x15\x91g\x10\0\0\0\0\0\0\0\x16\x15\x15\x90V[\x94P\x94P\x94P\x94P\x94P\x90\x91\x92\x93\x94V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08zV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x08\xD2V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\0V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t.V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\x81V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\t\xD9V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\"V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\nPV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\xA2V[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\n\xFAV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x0BKV[`@\x80Q` \x81\x01\x90\x91R`\0T\x81Ra\x05\x96\x81\x83a\x0ByV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra67`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x08\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[`@Q\x80\x91\x03\x90\xFD[P\x81Qi\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`@\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra65`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x08cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qe\xFF\xFF\0\0\0\0\x19\x16` \x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x06\xC7`\xF3\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`P\x1B\x19\x16`P\x91\x90\x91\x1B\x17\x90RV[`;\x81a\x08\xE0W`\0a\x08\xE3V[`\x01[\x83Qg\x08\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`9\x81a\t\x0EW`\0a\t\x11V[`\x01[\x83Qg\x02\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x1B\x1B`\xF1\x1B` \x82\x01R`\xFF\x82\x11\x15a\tiW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qf\xFF\0\0\0\0\0\0\x19\x16`0\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x1B\x99`\xF1\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\t\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra63`\xF0\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qa\xFF\xFF\x19\x16\x17\x90RV[`?\x81a\n0W`\0a\n3V[`\x01[\x83Qg\x80\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x03s`\xF4\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\n\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qa\xFF\xFF`\x98\x1B\x19\x16`\x98\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra69`\xF0\x1B` \x82\x01Rd\x0F\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qd\x0F\xFF\xFF\xFF\xFF`t\x1B\x19\x16`t\x91\x90\x91\x1B\x17\x90RV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\r\x8D`\xF2\x1B` \x82\x01Ra\xFF\xFF\x82\x11\x15a\x0B6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Qc\xFF\xFF\0\0\x19\x16`\x10\x91\x90\x91\x1B\x17\x90RV[`:\x81a\x0BYW`\0a\x0B\\V[`\x01[\x83Qg\x04\0\0\0\0\0\0\0\x19\x16`\xFF\x91\x90\x91\x16\x90\x91\x1B\x17\x90\x91RPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra71`\xF0\x1B` \x82\x01R`\xFF\x82\x11\x15a\x0B\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x03\x91\x90a\x0C\x0BV[P\x81Q`\xFF`\xA8\x1B\x19\x16`\xA8\x91\x90\x91\x1B\x17\x90RV[`\0` \x82\x84\x03\x12\x15a\x0B\xDBW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\xF4W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x0C\x04W`\0\x80\xFD[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x0C8W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x0C\x1CV[\x81\x81\x11\x15a\x0CJW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x16\xD8\xCDj](8d+\xED\x92\xEF\xCF\x8F\xA8\xA3\x9A\xDD\xC7'A\xC12\xAFp{\xDB\xC7\xB43\x05\xB8dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKRESERVECONFIGURATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockReserveConfiguration<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockReserveConfiguration<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockReserveConfiguration<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockReserveConfiguration<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockReserveConfiguration<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockReserveConfiguration))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockReserveConfiguration<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKRESERVECONFIGURATION_ABI.clone(),
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
                MOCKRESERVECONFIGURATION_ABI.clone(),
                MOCKRESERVECONFIGURATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `configuration` (0x6c70bee9) function
        pub fn configuration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([108, 112, 190, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBorrowCap` (0xaede7b76) function
        pub fn get_borrow_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 222, 123, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBorrowingEnabled` (0x79750bc4) function
        pub fn get_borrowing_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([121, 117, 11, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCaps` (0x9d706d31) function
        pub fn get_caps(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 112, 109, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDecimals` (0xf0141d84) function
        pub fn get_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([240, 20, 29, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEModeCategory` (0x356f235c) function
        pub fn get_e_mode_category(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 111, 35, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFlags` (0x6cc7149d) function
        pub fn get_flags(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, bool, bool, bool, bool),
        > {
            self.0
                .method_hash([108, 199, 20, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFlashLoanEnabled` (0xd1c11f18) function
        pub fn get_flash_loan_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([209, 193, 31, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFrozen` (0x7495b353) function
        pub fn get_frozen(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([116, 149, 179, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationBonus` (0x59aa9e72) function
        pub fn get_liquidation_bonus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 170, 158, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationProtocolFee` (0xc37bdcec) function
        pub fn get_liquidation_protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 123, 220, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationThreshold` (0x4ae9b8bc) function
        pub fn get_liquidation_threshold(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 233, 184, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLtv` (0x8145bd2e) function
        pub fn get_ltv(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 69, 189, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParams` (0x5e615a6b) function
        pub fn get_params(
            &self,
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
                .method_hash([94, 97, 90, 107], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveFactor` (0x5f558e53) function
        pub fn get_reserve_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([95, 85, 142, 83], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateBorrowingEnabled` (0xe08a28a3) function
        pub fn get_stable_rate_borrowing_enabled(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 138, 40, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSupplyCap` (0x20361814) function
        pub fn get_supply_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([32, 54, 24, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnbackedMintCap` (0xead8aa02) function
        pub fn get_unbacked_mint_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([234, 216, 170, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBorrowCap` (0x717186d1) function
        pub fn set_borrow_cap(
            &self,
            borrow_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 113, 134, 209], borrow_cap)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBorrowingEnabled` (0xf1514a1a) function
        pub fn set_borrowing_enabled(
            &self,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 81, 74, 26], enabled)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDecimals` (0x8c8885c8) function
        pub fn set_decimals(
            &self,
            decimals: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 136, 133, 200], decimals)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEModeCategory` (0xfa573d07) function
        pub fn set_e_mode_category(
            &self,
            category_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 87, 61, 7], category_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFlashLoanEnabled` (0xa55102f7) function
        pub fn set_flash_loan_enabled(
            &self,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 81, 2, 247], enabled)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFrozen` (0x7e932d32) function
        pub fn set_frozen(
            &self,
            frozen: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 147, 45, 50], frozen)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidationBonus` (0x28842d4f) function
        pub fn set_liquidation_bonus(
            &self,
            bonus: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 132, 45, 79], bonus)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidationProtocolFee` (0xa6200635) function
        pub fn set_liquidation_protocol_fee(
            &self,
            liquidation_protocol_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 32, 6, 53], liquidation_protocol_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidationThreshold` (0xd0b0c816) function
        pub fn set_liquidation_threshold(
            &self,
            threshold: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 176, 200, 22], threshold)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLtv` (0xa37e52e3) function
        pub fn set_ltv(
            &self,
            ltv: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 126, 82, 227], ltv)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveFactor` (0x1c446983) function
        pub fn set_reserve_factor(
            &self,
            reserve_factor: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 68, 105, 131], reserve_factor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStableRateBorrowingEnabled` (0x71cb1332) function
        pub fn set_stable_rate_borrowing_enabled(
            &self,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 203, 19, 50], enabled)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSupplyCap` (0xb6a3f59a) function
        pub fn set_supply_cap(
            &self,
            supply_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 163, 245, 154], supply_cap)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUnbackedMintCap` (0x92dfb2fb) function
        pub fn set_unbacked_mint_cap(
            &self,
            unbacked_mint_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 223, 178, 251], unbacked_mint_cap)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockReserveConfiguration<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `configuration` function with signature `configuration()` and selector `0x6c70bee9`
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
    #[ethcall(name = "configuration", abi = "configuration()")]
    pub struct ConfigurationCall;
    ///Container type for all input parameters for the `getBorrowCap` function with signature `getBorrowCap()` and selector `0xaede7b76`
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
    #[ethcall(name = "getBorrowCap", abi = "getBorrowCap()")]
    pub struct GetBorrowCapCall;
    ///Container type for all input parameters for the `getBorrowingEnabled` function with signature `getBorrowingEnabled()` and selector `0x79750bc4`
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
    #[ethcall(name = "getBorrowingEnabled", abi = "getBorrowingEnabled()")]
    pub struct GetBorrowingEnabledCall;
    ///Container type for all input parameters for the `getCaps` function with signature `getCaps()` and selector `0x9d706d31`
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
    #[ethcall(name = "getCaps", abi = "getCaps()")]
    pub struct GetCapsCall;
    ///Container type for all input parameters for the `getDecimals` function with signature `getDecimals()` and selector `0xf0141d84`
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
    #[ethcall(name = "getDecimals", abi = "getDecimals()")]
    pub struct GetDecimalsCall;
    ///Container type for all input parameters for the `getEModeCategory` function with signature `getEModeCategory()` and selector `0x356f235c`
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
    #[ethcall(name = "getEModeCategory", abi = "getEModeCategory()")]
    pub struct GetEModeCategoryCall;
    ///Container type for all input parameters for the `getFlags` function with signature `getFlags()` and selector `0x6cc7149d`
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
    #[ethcall(name = "getFlags", abi = "getFlags()")]
    pub struct GetFlagsCall;
    ///Container type for all input parameters for the `getFlashLoanEnabled` function with signature `getFlashLoanEnabled()` and selector `0xd1c11f18`
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
    #[ethcall(name = "getFlashLoanEnabled", abi = "getFlashLoanEnabled()")]
    pub struct GetFlashLoanEnabledCall;
    ///Container type for all input parameters for the `getFrozen` function with signature `getFrozen()` and selector `0x7495b353`
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
    #[ethcall(name = "getFrozen", abi = "getFrozen()")]
    pub struct GetFrozenCall;
    ///Container type for all input parameters for the `getLiquidationBonus` function with signature `getLiquidationBonus()` and selector `0x59aa9e72`
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
    #[ethcall(name = "getLiquidationBonus", abi = "getLiquidationBonus()")]
    pub struct GetLiquidationBonusCall;
    ///Container type for all input parameters for the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee()` and selector `0xc37bdcec`
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
    #[ethcall(name = "getLiquidationProtocolFee", abi = "getLiquidationProtocolFee()")]
    pub struct GetLiquidationProtocolFeeCall;
    ///Container type for all input parameters for the `getLiquidationThreshold` function with signature `getLiquidationThreshold()` and selector `0x4ae9b8bc`
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
    #[ethcall(name = "getLiquidationThreshold", abi = "getLiquidationThreshold()")]
    pub struct GetLiquidationThresholdCall;
    ///Container type for all input parameters for the `getLtv` function with signature `getLtv()` and selector `0x8145bd2e`
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
    #[ethcall(name = "getLtv", abi = "getLtv()")]
    pub struct GetLtvCall;
    ///Container type for all input parameters for the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
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
    #[ethcall(name = "getParams", abi = "getParams()")]
    pub struct GetParamsCall;
    ///Container type for all input parameters for the `getReserveFactor` function with signature `getReserveFactor()` and selector `0x5f558e53`
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
    #[ethcall(name = "getReserveFactor", abi = "getReserveFactor()")]
    pub struct GetReserveFactorCall;
    ///Container type for all input parameters for the `getStableRateBorrowingEnabled` function with signature `getStableRateBorrowingEnabled()` and selector `0xe08a28a3`
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
        name = "getStableRateBorrowingEnabled",
        abi = "getStableRateBorrowingEnabled()"
    )]
    pub struct GetStableRateBorrowingEnabledCall;
    ///Container type for all input parameters for the `getSupplyCap` function with signature `getSupplyCap()` and selector `0x20361814`
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
    #[ethcall(name = "getSupplyCap", abi = "getSupplyCap()")]
    pub struct GetSupplyCapCall;
    ///Container type for all input parameters for the `getUnbackedMintCap` function with signature `getUnbackedMintCap()` and selector `0xead8aa02`
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
    #[ethcall(name = "getUnbackedMintCap", abi = "getUnbackedMintCap()")]
    pub struct GetUnbackedMintCapCall;
    ///Container type for all input parameters for the `setBorrowCap` function with signature `setBorrowCap(uint256)` and selector `0x717186d1`
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
    #[ethcall(name = "setBorrowCap", abi = "setBorrowCap(uint256)")]
    pub struct SetBorrowCapCall {
        pub borrow_cap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setBorrowingEnabled` function with signature `setBorrowingEnabled(bool)` and selector `0xf1514a1a`
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
    #[ethcall(name = "setBorrowingEnabled", abi = "setBorrowingEnabled(bool)")]
    pub struct SetBorrowingEnabledCall {
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setDecimals` function with signature `setDecimals(uint256)` and selector `0x8c8885c8`
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
    #[ethcall(name = "setDecimals", abi = "setDecimals(uint256)")]
    pub struct SetDecimalsCall {
        pub decimals: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEModeCategory` function with signature `setEModeCategory(uint256)` and selector `0xfa573d07`
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
    #[ethcall(name = "setEModeCategory", abi = "setEModeCategory(uint256)")]
    pub struct SetEModeCategoryCall {
        pub category_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFlashLoanEnabled` function with signature `setFlashLoanEnabled(bool)` and selector `0xa55102f7`
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
    #[ethcall(name = "setFlashLoanEnabled", abi = "setFlashLoanEnabled(bool)")]
    pub struct SetFlashLoanEnabledCall {
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setFrozen` function with signature `setFrozen(bool)` and selector `0x7e932d32`
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
    #[ethcall(name = "setFrozen", abi = "setFrozen(bool)")]
    pub struct SetFrozenCall {
        pub frozen: bool,
    }
    ///Container type for all input parameters for the `setLiquidationBonus` function with signature `setLiquidationBonus(uint256)` and selector `0x28842d4f`
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
    #[ethcall(name = "setLiquidationBonus", abi = "setLiquidationBonus(uint256)")]
    pub struct SetLiquidationBonusCall {
        pub bonus: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLiquidationProtocolFee` function with signature `setLiquidationProtocolFee(uint256)` and selector `0xa6200635`
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
        name = "setLiquidationProtocolFee",
        abi = "setLiquidationProtocolFee(uint256)"
    )]
    pub struct SetLiquidationProtocolFeeCall {
        pub liquidation_protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLiquidationThreshold` function with signature `setLiquidationThreshold(uint256)` and selector `0xd0b0c816`
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
        name = "setLiquidationThreshold",
        abi = "setLiquidationThreshold(uint256)"
    )]
    pub struct SetLiquidationThresholdCall {
        pub threshold: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setLtv` function with signature `setLtv(uint256)` and selector `0xa37e52e3`
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
    #[ethcall(name = "setLtv", abi = "setLtv(uint256)")]
    pub struct SetLtvCall {
        pub ltv: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setReserveFactor` function with signature `setReserveFactor(uint256)` and selector `0x1c446983`
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
    #[ethcall(name = "setReserveFactor", abi = "setReserveFactor(uint256)")]
    pub struct SetReserveFactorCall {
        pub reserve_factor: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setStableRateBorrowingEnabled` function with signature `setStableRateBorrowingEnabled(bool)` and selector `0x71cb1332`
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
        name = "setStableRateBorrowingEnabled",
        abi = "setStableRateBorrowingEnabled(bool)"
    )]
    pub struct SetStableRateBorrowingEnabledCall {
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setSupplyCap` function with signature `setSupplyCap(uint256)` and selector `0xb6a3f59a`
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
    #[ethcall(name = "setSupplyCap", abi = "setSupplyCap(uint256)")]
    pub struct SetSupplyCapCall {
        pub supply_cap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setUnbackedMintCap` function with signature `setUnbackedMintCap(uint256)` and selector `0x92dfb2fb`
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
    #[ethcall(name = "setUnbackedMintCap", abi = "setUnbackedMintCap(uint256)")]
    pub struct SetUnbackedMintCapCall {
        pub unbacked_mint_cap: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockReserveConfigurationCalls {
        Configuration(ConfigurationCall),
        GetBorrowCap(GetBorrowCapCall),
        GetBorrowingEnabled(GetBorrowingEnabledCall),
        GetCaps(GetCapsCall),
        GetDecimals(GetDecimalsCall),
        GetEModeCategory(GetEModeCategoryCall),
        GetFlags(GetFlagsCall),
        GetFlashLoanEnabled(GetFlashLoanEnabledCall),
        GetFrozen(GetFrozenCall),
        GetLiquidationBonus(GetLiquidationBonusCall),
        GetLiquidationProtocolFee(GetLiquidationProtocolFeeCall),
        GetLiquidationThreshold(GetLiquidationThresholdCall),
        GetLtv(GetLtvCall),
        GetParams(GetParamsCall),
        GetReserveFactor(GetReserveFactorCall),
        GetStableRateBorrowingEnabled(GetStableRateBorrowingEnabledCall),
        GetSupplyCap(GetSupplyCapCall),
        GetUnbackedMintCap(GetUnbackedMintCapCall),
        SetBorrowCap(SetBorrowCapCall),
        SetBorrowingEnabled(SetBorrowingEnabledCall),
        SetDecimals(SetDecimalsCall),
        SetEModeCategory(SetEModeCategoryCall),
        SetFlashLoanEnabled(SetFlashLoanEnabledCall),
        SetFrozen(SetFrozenCall),
        SetLiquidationBonus(SetLiquidationBonusCall),
        SetLiquidationProtocolFee(SetLiquidationProtocolFeeCall),
        SetLiquidationThreshold(SetLiquidationThresholdCall),
        SetLtv(SetLtvCall),
        SetReserveFactor(SetReserveFactorCall),
        SetStableRateBorrowingEnabled(SetStableRateBorrowingEnabledCall),
        SetSupplyCap(SetSupplyCapCall),
        SetUnbackedMintCap(SetUnbackedMintCapCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockReserveConfigurationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConfigurationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Configuration(decoded));
            }
            if let Ok(decoded)
                = <GetBorrowCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBorrowCap(decoded));
            }
            if let Ok(decoded)
                = <GetBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBorrowingEnabled(decoded));
            }
            if let Ok(decoded)
                = <GetCapsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCaps(decoded));
            }
            if let Ok(decoded)
                = <GetDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDecimals(decoded));
            }
            if let Ok(decoded)
                = <GetEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <GetFlagsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetFlags(decoded));
            }
            if let Ok(decoded)
                = <GetFlashLoanEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetFlashLoanEnabled(decoded));
            }
            if let Ok(decoded)
                = <GetFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetFrozen(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidationBonusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidationBonus(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidationProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidationThreshold(decoded));
            }
            if let Ok(decoded)
                = <GetLtvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLtv(decoded));
            }
            if let Ok(decoded)
                = <GetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetParams(decoded));
            }
            if let Ok(decoded)
                = <GetReserveFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveFactor(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateBorrowingEnabled(decoded));
            }
            if let Ok(decoded)
                = <GetSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSupplyCap(decoded));
            }
            if let Ok(decoded)
                = <GetUnbackedMintCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnbackedMintCap(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBorrowCap(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetBorrowingEnabled(decoded));
            }
            if let Ok(decoded)
                = <SetDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDecimals(decoded));
            }
            if let Ok(decoded)
                = <SetEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <SetFlashLoanEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFlashLoanEnabled(decoded));
            }
            if let Ok(decoded)
                = <SetFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFrozen(decoded));
            }
            if let Ok(decoded)
                = <SetLiquidationBonusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLiquidationBonus(decoded));
            }
            if let Ok(decoded)
                = <SetLiquidationProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <SetLiquidationThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLiquidationThreshold(decoded));
            }
            if let Ok(decoded)
                = <SetLtvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetLtv(decoded));
            }
            if let Ok(decoded)
                = <SetReserveFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveFactor(decoded));
            }
            if let Ok(decoded)
                = <SetStableRateBorrowingEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetStableRateBorrowingEnabled(decoded));
            }
            if let Ok(decoded)
                = <SetSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSupplyCap(decoded));
            }
            if let Ok(decoded)
                = <SetUnbackedMintCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetUnbackedMintCap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockReserveConfigurationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Configuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBorrowCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCaps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFlags(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFlashLoanEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationBonus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLtv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnbackedMintCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFlashLoanEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidationBonus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidationProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidationThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLtv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetReserveFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStableRateBorrowingEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnbackedMintCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockReserveConfigurationCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Configuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBorrowCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCaps(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEModeCategory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFlags(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFlashLoanEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidationBonus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidationProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLtv(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStableRateBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSupplyCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnbackedMintCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetBorrowCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEModeCategory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFlashLoanEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLiquidationBonus(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLiquidationProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLiquidationThreshold(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLtv(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStableRateBorrowingEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSupplyCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUnbackedMintCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ConfigurationCall> for MockReserveConfigurationCalls {
        fn from(value: ConfigurationCall) -> Self {
            Self::Configuration(value)
        }
    }
    impl ::core::convert::From<GetBorrowCapCall> for MockReserveConfigurationCalls {
        fn from(value: GetBorrowCapCall) -> Self {
            Self::GetBorrowCap(value)
        }
    }
    impl ::core::convert::From<GetBorrowingEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetBorrowingEnabledCall) -> Self {
            Self::GetBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<GetCapsCall> for MockReserveConfigurationCalls {
        fn from(value: GetCapsCall) -> Self {
            Self::GetCaps(value)
        }
    }
    impl ::core::convert::From<GetDecimalsCall> for MockReserveConfigurationCalls {
        fn from(value: GetDecimalsCall) -> Self {
            Self::GetDecimals(value)
        }
    }
    impl ::core::convert::From<GetEModeCategoryCall> for MockReserveConfigurationCalls {
        fn from(value: GetEModeCategoryCall) -> Self {
            Self::GetEModeCategory(value)
        }
    }
    impl ::core::convert::From<GetFlagsCall> for MockReserveConfigurationCalls {
        fn from(value: GetFlagsCall) -> Self {
            Self::GetFlags(value)
        }
    }
    impl ::core::convert::From<GetFlashLoanEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetFlashLoanEnabledCall) -> Self {
            Self::GetFlashLoanEnabled(value)
        }
    }
    impl ::core::convert::From<GetFrozenCall> for MockReserveConfigurationCalls {
        fn from(value: GetFrozenCall) -> Self {
            Self::GetFrozen(value)
        }
    }
    impl ::core::convert::From<GetLiquidationBonusCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetLiquidationBonusCall) -> Self {
            Self::GetLiquidationBonus(value)
        }
    }
    impl ::core::convert::From<GetLiquidationProtocolFeeCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetLiquidationProtocolFeeCall) -> Self {
            Self::GetLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<GetLiquidationThresholdCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetLiquidationThresholdCall) -> Self {
            Self::GetLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<GetLtvCall> for MockReserveConfigurationCalls {
        fn from(value: GetLtvCall) -> Self {
            Self::GetLtv(value)
        }
    }
    impl ::core::convert::From<GetParamsCall> for MockReserveConfigurationCalls {
        fn from(value: GetParamsCall) -> Self {
            Self::GetParams(value)
        }
    }
    impl ::core::convert::From<GetReserveFactorCall> for MockReserveConfigurationCalls {
        fn from(value: GetReserveFactorCall) -> Self {
            Self::GetReserveFactor(value)
        }
    }
    impl ::core::convert::From<GetStableRateBorrowingEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetStableRateBorrowingEnabledCall) -> Self {
            Self::GetStableRateBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<GetSupplyCapCall> for MockReserveConfigurationCalls {
        fn from(value: GetSupplyCapCall) -> Self {
            Self::GetSupplyCap(value)
        }
    }
    impl ::core::convert::From<GetUnbackedMintCapCall>
    for MockReserveConfigurationCalls {
        fn from(value: GetUnbackedMintCapCall) -> Self {
            Self::GetUnbackedMintCap(value)
        }
    }
    impl ::core::convert::From<SetBorrowCapCall> for MockReserveConfigurationCalls {
        fn from(value: SetBorrowCapCall) -> Self {
            Self::SetBorrowCap(value)
        }
    }
    impl ::core::convert::From<SetBorrowingEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetBorrowingEnabledCall) -> Self {
            Self::SetBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<SetDecimalsCall> for MockReserveConfigurationCalls {
        fn from(value: SetDecimalsCall) -> Self {
            Self::SetDecimals(value)
        }
    }
    impl ::core::convert::From<SetEModeCategoryCall> for MockReserveConfigurationCalls {
        fn from(value: SetEModeCategoryCall) -> Self {
            Self::SetEModeCategory(value)
        }
    }
    impl ::core::convert::From<SetFlashLoanEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetFlashLoanEnabledCall) -> Self {
            Self::SetFlashLoanEnabled(value)
        }
    }
    impl ::core::convert::From<SetFrozenCall> for MockReserveConfigurationCalls {
        fn from(value: SetFrozenCall) -> Self {
            Self::SetFrozen(value)
        }
    }
    impl ::core::convert::From<SetLiquidationBonusCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetLiquidationBonusCall) -> Self {
            Self::SetLiquidationBonus(value)
        }
    }
    impl ::core::convert::From<SetLiquidationProtocolFeeCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetLiquidationProtocolFeeCall) -> Self {
            Self::SetLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<SetLiquidationThresholdCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetLiquidationThresholdCall) -> Self {
            Self::SetLiquidationThreshold(value)
        }
    }
    impl ::core::convert::From<SetLtvCall> for MockReserveConfigurationCalls {
        fn from(value: SetLtvCall) -> Self {
            Self::SetLtv(value)
        }
    }
    impl ::core::convert::From<SetReserveFactorCall> for MockReserveConfigurationCalls {
        fn from(value: SetReserveFactorCall) -> Self {
            Self::SetReserveFactor(value)
        }
    }
    impl ::core::convert::From<SetStableRateBorrowingEnabledCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetStableRateBorrowingEnabledCall) -> Self {
            Self::SetStableRateBorrowingEnabled(value)
        }
    }
    impl ::core::convert::From<SetSupplyCapCall> for MockReserveConfigurationCalls {
        fn from(value: SetSupplyCapCall) -> Self {
            Self::SetSupplyCap(value)
        }
    }
    impl ::core::convert::From<SetUnbackedMintCapCall>
    for MockReserveConfigurationCalls {
        fn from(value: SetUnbackedMintCapCall) -> Self {
            Self::SetUnbackedMintCap(value)
        }
    }
    ///Container type for all return fields from the `configuration` function with signature `configuration()` and selector `0x6c70bee9`
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
    pub struct ConfigurationReturn {
        pub data: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getBorrowCap` function with signature `getBorrowCap()` and selector `0xaede7b76`
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
    pub struct GetBorrowCapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBorrowingEnabled` function with signature `getBorrowingEnabled()` and selector `0x79750bc4`
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
    pub struct GetBorrowingEnabledReturn(pub bool);
    ///Container type for all return fields from the `getCaps` function with signature `getCaps()` and selector `0x9d706d31`
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
    pub struct GetCapsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getDecimals` function with signature `getDecimals()` and selector `0xf0141d84`
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
    pub struct GetDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getEModeCategory` function with signature `getEModeCategory()` and selector `0x356f235c`
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
    pub struct GetEModeCategoryReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFlags` function with signature `getFlags()` and selector `0x6cc7149d`
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
    pub struct GetFlagsReturn(pub bool, pub bool, pub bool, pub bool, pub bool);
    ///Container type for all return fields from the `getFlashLoanEnabled` function with signature `getFlashLoanEnabled()` and selector `0xd1c11f18`
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
    pub struct GetFlashLoanEnabledReturn(pub bool);
    ///Container type for all return fields from the `getFrozen` function with signature `getFrozen()` and selector `0x7495b353`
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
    pub struct GetFrozenReturn(pub bool);
    ///Container type for all return fields from the `getLiquidationBonus` function with signature `getLiquidationBonus()` and selector `0x59aa9e72`
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
    pub struct GetLiquidationBonusReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee()` and selector `0xc37bdcec`
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
    pub struct GetLiquidationProtocolFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLiquidationThreshold` function with signature `getLiquidationThreshold()` and selector `0x4ae9b8bc`
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
    pub struct GetLiquidationThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLtv` function with signature `getLtv()` and selector `0x8145bd2e`
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
    pub struct GetLtvReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getParams` function with signature `getParams()` and selector `0x5e615a6b`
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
    pub struct GetParamsReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getReserveFactor` function with signature `getReserveFactor()` and selector `0x5f558e53`
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
    pub struct GetReserveFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateBorrowingEnabled` function with signature `getStableRateBorrowingEnabled()` and selector `0xe08a28a3`
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
    pub struct GetStableRateBorrowingEnabledReturn(pub bool);
    ///Container type for all return fields from the `getSupplyCap` function with signature `getSupplyCap()` and selector `0x20361814`
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
    pub struct GetSupplyCapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUnbackedMintCap` function with signature `getUnbackedMintCap()` and selector `0xead8aa02`
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
    pub struct GetUnbackedMintCapReturn(pub ::ethers::core::types::U256);
}
