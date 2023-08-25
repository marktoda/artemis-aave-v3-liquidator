pub use pool_configurator::*;
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
pub mod pool_configurator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CONFIGURATOR_REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CONFIGURATOR_REVISION",
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
                    ::std::borrow::ToOwned::to_owned("configureReserveAsCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "configureReserveAsCollateral",
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
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationBonus"),
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
                    ::std::borrow::ToOwned::to_owned("initReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initReserves"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConfiguratorInputTypes.InitReserveInput[]",
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
                    ::std::borrow::ToOwned::to_owned("setAssetEModeCategory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setAssetEModeCategory",
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
                                    name: ::std::borrow::ToOwned::to_owned("newCategoryId"),
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
                    ::std::borrow::ToOwned::to_owned("setBorrowCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBorrowCap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newBorrowCap"),
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
                    ::std::borrow::ToOwned::to_owned("setBorrowableInIsolation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setBorrowableInIsolation",
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
                                    name: ::std::borrow::ToOwned::to_owned("borrowable"),
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
                    ::std::borrow::ToOwned::to_owned("setDebtCeiling"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDebtCeiling"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDebtCeiling"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationBonus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("label"),
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
                    ::std::borrow::ToOwned::to_owned("setLiquidationProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setLiquidationProtocolFee",
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
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
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
                    ::std::borrow::ToOwned::to_owned("setPoolPause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPoolPause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("setReserveActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReserveActive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("active"),
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
                    ::std::borrow::ToOwned::to_owned("setReserveBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setReserveBorrowing",
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
                    ::std::borrow::ToOwned::to_owned("setReserveFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReserveFactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newReserveFactor"),
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
                    ::std::borrow::ToOwned::to_owned("setReserveFlashLoaning"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setReserveFlashLoaning",
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
                    ::std::borrow::ToOwned::to_owned("setReserveFreeze"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReserveFreeze"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("freeze"),
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
                                        "newRateStrategyAddress",
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
                    ::std::borrow::ToOwned::to_owned("setReservePause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setReservePause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("setReserveStableRateBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setReserveStableRateBorrowing",
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
                    ::std::borrow::ToOwned::to_owned("setSiloedBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSiloedBorrowing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSiloed"),
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newSupplyCap"),
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newUnbackedMintCap",
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
                    ::std::borrow::ToOwned::to_owned("updateAToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateAToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConfiguratorInputTypes.UpdateATokenInput",
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
                    ::std::borrow::ToOwned::to_owned("updateBridgeProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateBridgeProtocolFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newBridgeProtocolFee",
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
                    ::std::borrow::ToOwned::to_owned("updateFlashloanPremiumToProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateFlashloanPremiumToProtocol",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashloanPremiumToProtocol",
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
                    ::std::borrow::ToOwned::to_owned("updateFlashloanPremiumTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateFlashloanPremiumTotal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashloanPremiumTotal",
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
                    ::std::borrow::ToOwned::to_owned("updateStableDebtToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateStableDebtToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConfiguratorInputTypes.UpdateDebtTokenInput",
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
                    ::std::borrow::ToOwned::to_owned("updateVariableDebtToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateVariableDebtToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ConfiguratorInputTypes.UpdateDebtTokenInput",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ATokenUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ATokenUpgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BorrowCapChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BorrowCapChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldBorrowCap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newBorrowCap"),
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
                    ::std::borrow::ToOwned::to_owned("BorrowableInIsolationChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BorrowableInIsolationChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("borrowable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BridgeProtocolFeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BridgeProtocolFeeUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldBridgeProtocolFee",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newBridgeProtocolFee",
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
                    ::std::borrow::ToOwned::to_owned("CollateralConfigurationChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralConfigurationChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationBonus"),
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
                    ::std::borrow::ToOwned::to_owned("DebtCeilingChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DebtCeilingChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldDebtCeiling"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDebtCeiling"),
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
                    ::std::borrow::ToOwned::to_owned("EModeAssetCategoryChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EModeAssetCategoryChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldCategoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newCategoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EModeCategoryAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EModeCategoryAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("categoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ltv"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidationThreshold",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationBonus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("label"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "FlashloanPremiumToProtocolUpdated",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FlashloanPremiumToProtocolUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldFlashloanPremiumToProtocol",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashloanPremiumToProtocol",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FlashloanPremiumTotalUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FlashloanPremiumTotalUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldFlashloanPremiumTotal",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFlashloanPremiumTotal",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidationProtocolFeeChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidationProtocolFeeChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newFee"),
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
                    ::std::borrow::ToOwned::to_owned("ReserveActive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveActive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("active"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveBorrowing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveDropped"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveDropped"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveFactorChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveFactorChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldReserveFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newReserveFactor"),
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
                    ::std::borrow::ToOwned::to_owned("ReserveFlashLoaning"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveFlashLoaning",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("frozen"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReserveInitialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("stableDebtToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("variableDebtToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "interestRateStrategyAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "ReserveInterestRateStrategyChanged",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveInterestRateStrategyChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldStrategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newStrategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReservePaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ReservePaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paused"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReserveStableRateBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ReserveStableRateBorrowing",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("enabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SiloedBorrowingChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SiloedBorrowingChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newState"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StableDebtTokenUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StableDebtTokenUpgraded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SupplyCapChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SupplyCapChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldSupplyCap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newSupplyCap"),
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
                    ::std::borrow::ToOwned::to_owned("UnbackedMintCapChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnbackedMintCapChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldUnbackedMintCap",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newUnbackedMintCap",
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
                    ::std::borrow::ToOwned::to_owned("VariableDebtTokenUpgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VariableDebtTokenUpgraded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proxy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
    pub static POOLCONFIGURATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct PoolConfigurator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoolConfigurator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoolConfigurator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoolConfigurator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoolConfigurator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PoolConfigurator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoolConfigurator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POOLCONFIGURATOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `CONFIGURATOR_REVISION` (0x7af635a6) function
        pub fn configurator_revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 246, 53, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureReserveAsCollateral` (0x7c4e560b) function
        pub fn configure_reserve_as_collateral(
            &self,
            asset: ::ethers::core::types::Address,
            ltv: ::ethers::core::types::U256,
            liquidation_threshold: ::ethers::core::types::U256,
            liquidation_bonus: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [124, 78, 86, 11],
                    (asset, ltv, liquidation_threshold, liquidation_bonus),
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
        ///Calls the contract's `initReserves` (0x02fb45e6) function
        pub fn init_reserves(
            &self,
            input: ::std::vec::Vec<InitReserveInput>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 251, 69, 230], input)
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
        ///Calls the contract's `setAssetEModeCategory` (0xd4fe3f99) function
        pub fn set_asset_e_mode_category(
            &self,
            asset: ::ethers::core::types::Address,
            new_category_id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 254, 63, 153], (asset, new_category_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBorrowCap` (0xd14a0983) function
        pub fn set_borrow_cap(
            &self,
            asset: ::ethers::core::types::Address,
            new_borrow_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 74, 9, 131], (asset, new_borrow_cap))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBorrowableInIsolation` (0x38ae0cc3) function
        pub fn set_borrowable_in_isolation(
            &self,
            asset: ::ethers::core::types::Address,
            borrowable: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 174, 12, 195], (asset, borrowable))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDebtCeiling` (0xaeb4fcc1) function
        pub fn set_debt_ceiling(
            &self,
            asset: ::ethers::core::types::Address,
            new_debt_ceiling: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 180, 252, 193], (asset, new_debt_ceiling))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEModeCategory` (0xc19d61e4) function
        pub fn set_e_mode_category(
            &self,
            category_id: u8,
            ltv: u16,
            liquidation_threshold: u16,
            liquidation_bonus: u16,
            oracle: ::ethers::core::types::Address,
            label: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 157, 97, 228],
                    (
                        category_id,
                        ltv,
                        liquidation_threshold,
                        liquidation_bonus,
                        oracle,
                        label,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidationProtocolFee` (0x26d2cec2) function
        pub fn set_liquidation_protocol_fee(
            &self,
            asset: ::ethers::core::types::Address,
            new_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 210, 206, 194], (asset, new_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPoolPause` (0x7641f3d9) function
        pub fn set_pool_pause(
            &self,
            paused: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 65, 243, 217], paused)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveActive` (0xb736aaeb) function
        pub fn set_reserve_active(
            &self,
            asset: ::ethers::core::types::Address,
            active: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 54, 170, 235], (asset, active))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveBorrowing` (0x682cf264) function
        pub fn set_reserve_borrowing(
            &self,
            asset: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 44, 242, 100], (asset, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveFactor` (0x4b4e6753) function
        pub fn set_reserve_factor(
            &self,
            asset: ::ethers::core::types::Address,
            new_reserve_factor: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 78, 103, 83], (asset, new_reserve_factor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveFlashLoaning` (0xf213ef0e) function
        pub fn set_reserve_flash_loaning(
            &self,
            asset: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 19, 239, 14], (asset, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveFreeze` (0x96e957c4) function
        pub fn set_reserve_freeze(
            &self,
            asset: ::ethers::core::types::Address,
            freeze: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 233, 87, 196], (asset, freeze))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            asset: ::ethers::core::types::Address,
            new_rate_strategy_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (asset, new_rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReservePause` (0x48d9fba9) function
        pub fn set_reserve_pause(
            &self,
            asset: ::ethers::core::types::Address,
            paused: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 217, 251, 169], (asset, paused))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReserveStableRateBorrowing` (0x8a751a60) function
        pub fn set_reserve_stable_rate_borrowing(
            &self,
            asset: ::ethers::core::types::Address,
            enabled: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 117, 26, 96], (asset, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSiloedBorrowing` (0xa7fa83b7) function
        pub fn set_siloed_borrowing(
            &self,
            asset: ::ethers::core::types::Address,
            new_siloed: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 250, 131, 183], (asset, new_siloed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSupplyCap` (0x571f03e5) function
        pub fn set_supply_cap(
            &self,
            asset: ::ethers::core::types::Address,
            new_supply_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 31, 3, 229], (asset, new_supply_cap))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUnbackedMintCap` (0x145f5892) function
        pub fn set_unbacked_mint_cap(
            &self,
            asset: ::ethers::core::types::Address,
            new_unbacked_mint_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 95, 88, 146], (asset, new_unbacked_mint_cap))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAToken` (0xbb01c37c) function
        pub fn update_a_token(
            &self,
            input: UpdateATokenInput,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 1, 195, 124], (input,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBridgeProtocolFee` (0x3036b439) function
        pub fn update_bridge_protocol_fee(
            &self,
            new_bridge_protocol_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 54, 180, 57], new_bridge_protocol_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFlashloanPremiumToProtocol` (0x1df970bd) function
        pub fn update_flashloan_premium_to_protocol(
            &self,
            new_flashloan_premium_to_protocol: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 249, 112, 189], new_flashloan_premium_to_protocol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFlashloanPremiumTotal` (0x8a493676) function
        pub fn update_flashloan_premium_total(
            &self,
            new_flashloan_premium_total: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 73, 54, 118], new_flashloan_premium_total)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateStableDebtToken` (0x7626cde3) function
        pub fn update_stable_debt_token(
            &self,
            input: UpdateDebtTokenInput,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 38, 205, 227], (input,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateVariableDebtToken` (0xad4e6432) function
        pub fn update_variable_debt_token(
            &self,
            input: UpdateDebtTokenInput,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 78, 100, 50], (input,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ATokenUpgraded` event
        pub fn a_token_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AtokenUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BorrowCapChanged` event
        pub fn borrow_cap_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BorrowCapChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BorrowableInIsolationChanged` event
        pub fn borrowable_in_isolation_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BorrowableInIsolationChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BridgeProtocolFeeUpdated` event
        pub fn bridge_protocol_fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BridgeProtocolFeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CollateralConfigurationChanged` event
        pub fn collateral_configuration_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CollateralConfigurationChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DebtCeilingChanged` event
        pub fn debt_ceiling_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DebtCeilingChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EModeAssetCategoryChanged` event
        pub fn e_mode_asset_category_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EmodeAssetCategoryChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EModeCategoryAdded` event
        pub fn e_mode_category_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EmodeCategoryAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FlashloanPremiumToProtocolUpdated` event
        pub fn flashloan_premium_to_protocol_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FlashloanPremiumToProtocolUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FlashloanPremiumTotalUpdated` event
        pub fn flashloan_premium_total_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FlashloanPremiumTotalUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidationProtocolFeeChanged` event
        pub fn liquidation_protocol_fee_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidationProtocolFeeChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveActive` event
        pub fn reserve_active_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveActiveFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveBorrowing` event
        pub fn reserve_borrowing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveBorrowingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveDropped` event
        pub fn reserve_dropped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveDroppedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveFactorChanged` event
        pub fn reserve_factor_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveFactorChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveFlashLoaning` event
        pub fn reserve_flash_loaning_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveFlashLoaningFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveFrozen` event
        pub fn reserve_frozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveFrozenFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveInitialized` event
        pub fn reserve_initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveInitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveInterestRateStrategyChanged` event
        pub fn reserve_interest_rate_strategy_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveInterestRateStrategyChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReservePaused` event
        pub fn reserve_paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReservePausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ReserveStableRateBorrowing` event
        pub fn reserve_stable_rate_borrowing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReserveStableRateBorrowingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SiloedBorrowingChanged` event
        pub fn siloed_borrowing_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SiloedBorrowingChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StableDebtTokenUpgraded` event
        pub fn stable_debt_token_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StableDebtTokenUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SupplyCapChanged` event
        pub fn supply_cap_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SupplyCapChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UnbackedMintCapChanged` event
        pub fn unbacked_mint_cap_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnbackedMintCapChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VariableDebtTokenUpgraded` event
        pub fn variable_debt_token_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VariableDebtTokenUpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PoolConfiguratorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PoolConfigurator<M> {
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
    #[ethevent(name = "ATokenUpgraded", abi = "ATokenUpgraded(address,address,address)")]
    pub struct AtokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
        name = "BorrowCapChanged",
        abi = "BorrowCapChanged(address,uint256,uint256)"
    )]
    pub struct BorrowCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_borrow_cap: ::ethers::core::types::U256,
        pub new_borrow_cap: ::ethers::core::types::U256,
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
        name = "BorrowableInIsolationChanged",
        abi = "BorrowableInIsolationChanged(address,bool)"
    )]
    pub struct BorrowableInIsolationChangedFilter {
        pub asset: ::ethers::core::types::Address,
        pub borrowable: bool,
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
        name = "BridgeProtocolFeeUpdated",
        abi = "BridgeProtocolFeeUpdated(uint256,uint256)"
    )]
    pub struct BridgeProtocolFeeUpdatedFilter {
        pub old_bridge_protocol_fee: ::ethers::core::types::U256,
        pub new_bridge_protocol_fee: ::ethers::core::types::U256,
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
        name = "CollateralConfigurationChanged",
        abi = "CollateralConfigurationChanged(address,uint256,uint256,uint256)"
    )]
    pub struct CollateralConfigurationChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub ltv: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub liquidation_bonus: ::ethers::core::types::U256,
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
        name = "DebtCeilingChanged",
        abi = "DebtCeilingChanged(address,uint256,uint256)"
    )]
    pub struct DebtCeilingChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_debt_ceiling: ::ethers::core::types::U256,
        pub new_debt_ceiling: ::ethers::core::types::U256,
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
        name = "EModeAssetCategoryChanged",
        abi = "EModeAssetCategoryChanged(address,uint8,uint8)"
    )]
    pub struct EmodeAssetCategoryChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_category_id: u8,
        pub new_category_id: u8,
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
        name = "EModeCategoryAdded",
        abi = "EModeCategoryAdded(uint8,uint256,uint256,uint256,address,string)"
    )]
    pub struct EmodeCategoryAddedFilter {
        #[ethevent(indexed)]
        pub category_id: u8,
        pub ltv: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub liquidation_bonus: ::ethers::core::types::U256,
        pub oracle: ::ethers::core::types::Address,
        pub label: ::std::string::String,
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
        name = "FlashloanPremiumToProtocolUpdated",
        abi = "FlashloanPremiumToProtocolUpdated(uint128,uint128)"
    )]
    pub struct FlashloanPremiumToProtocolUpdatedFilter {
        pub old_flashloan_premium_to_protocol: u128,
        pub new_flashloan_premium_to_protocol: u128,
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
        name = "FlashloanPremiumTotalUpdated",
        abi = "FlashloanPremiumTotalUpdated(uint128,uint128)"
    )]
    pub struct FlashloanPremiumTotalUpdatedFilter {
        pub old_flashloan_premium_total: u128,
        pub new_flashloan_premium_total: u128,
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
        name = "LiquidationProtocolFeeChanged",
        abi = "LiquidationProtocolFeeChanged(address,uint256,uint256)"
    )]
    pub struct LiquidationProtocolFeeChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_fee: ::ethers::core::types::U256,
        pub new_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "ReserveActive", abi = "ReserveActive(address,bool)")]
    pub struct ReserveActiveFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub active: bool,
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
    #[ethevent(name = "ReserveBorrowing", abi = "ReserveBorrowing(address,bool)")]
    pub struct ReserveBorrowingFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "ReserveDropped", abi = "ReserveDropped(address)")]
    pub struct ReserveDroppedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
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
        name = "ReserveFactorChanged",
        abi = "ReserveFactorChanged(address,uint256,uint256)"
    )]
    pub struct ReserveFactorChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_reserve_factor: ::ethers::core::types::U256,
        pub new_reserve_factor: ::ethers::core::types::U256,
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
    #[ethevent(name = "ReserveFlashLoaning", abi = "ReserveFlashLoaning(address,bool)")]
    pub struct ReserveFlashLoaningFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "ReserveFrozen", abi = "ReserveFrozen(address,bool)")]
    pub struct ReserveFrozenFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub frozen: bool,
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
        name = "ReserveInitialized",
        abi = "ReserveInitialized(address,address,address,address,address)"
    )]
    pub struct ReserveInitializedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub a_token: ::ethers::core::types::Address,
        pub stable_debt_token: ::ethers::core::types::Address,
        pub variable_debt_token: ::ethers::core::types::Address,
        pub interest_rate_strategy_address: ::ethers::core::types::Address,
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
        name = "ReserveInterestRateStrategyChanged",
        abi = "ReserveInterestRateStrategyChanged(address,address,address)"
    )]
    pub struct ReserveInterestRateStrategyChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_strategy: ::ethers::core::types::Address,
        pub new_strategy: ::ethers::core::types::Address,
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
    #[ethevent(name = "ReservePaused", abi = "ReservePaused(address,bool)")]
    pub struct ReservePausedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub paused: bool,
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
        name = "ReserveStableRateBorrowing",
        abi = "ReserveStableRateBorrowing(address,bool)"
    )]
    pub struct ReserveStableRateBorrowingFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
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
        name = "SiloedBorrowingChanged",
        abi = "SiloedBorrowingChanged(address,bool,bool)"
    )]
    pub struct SiloedBorrowingChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_state: bool,
        pub new_state: bool,
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
        name = "StableDebtTokenUpgraded",
        abi = "StableDebtTokenUpgraded(address,address,address)"
    )]
    pub struct StableDebtTokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
        name = "SupplyCapChanged",
        abi = "SupplyCapChanged(address,uint256,uint256)"
    )]
    pub struct SupplyCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_supply_cap: ::ethers::core::types::U256,
        pub new_supply_cap: ::ethers::core::types::U256,
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
        name = "UnbackedMintCapChanged",
        abi = "UnbackedMintCapChanged(address,uint256,uint256)"
    )]
    pub struct UnbackedMintCapChangedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub old_unbacked_mint_cap: ::ethers::core::types::U256,
        pub new_unbacked_mint_cap: ::ethers::core::types::U256,
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
        name = "VariableDebtTokenUpgraded",
        abi = "VariableDebtTokenUpgraded(address,address,address)"
    )]
    pub struct VariableDebtTokenUpgradedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proxy: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolConfiguratorEvents {
        AtokenUpgradedFilter(AtokenUpgradedFilter),
        BorrowCapChangedFilter(BorrowCapChangedFilter),
        BorrowableInIsolationChangedFilter(BorrowableInIsolationChangedFilter),
        BridgeProtocolFeeUpdatedFilter(BridgeProtocolFeeUpdatedFilter),
        CollateralConfigurationChangedFilter(CollateralConfigurationChangedFilter),
        DebtCeilingChangedFilter(DebtCeilingChangedFilter),
        EmodeAssetCategoryChangedFilter(EmodeAssetCategoryChangedFilter),
        EmodeCategoryAddedFilter(EmodeCategoryAddedFilter),
        FlashloanPremiumToProtocolUpdatedFilter(FlashloanPremiumToProtocolUpdatedFilter),
        FlashloanPremiumTotalUpdatedFilter(FlashloanPremiumTotalUpdatedFilter),
        LiquidationProtocolFeeChangedFilter(LiquidationProtocolFeeChangedFilter),
        ReserveActiveFilter(ReserveActiveFilter),
        ReserveBorrowingFilter(ReserveBorrowingFilter),
        ReserveDroppedFilter(ReserveDroppedFilter),
        ReserveFactorChangedFilter(ReserveFactorChangedFilter),
        ReserveFlashLoaningFilter(ReserveFlashLoaningFilter),
        ReserveFrozenFilter(ReserveFrozenFilter),
        ReserveInitializedFilter(ReserveInitializedFilter),
        ReserveInterestRateStrategyChangedFilter(
            ReserveInterestRateStrategyChangedFilter,
        ),
        ReservePausedFilter(ReservePausedFilter),
        ReserveStableRateBorrowingFilter(ReserveStableRateBorrowingFilter),
        SiloedBorrowingChangedFilter(SiloedBorrowingChangedFilter),
        StableDebtTokenUpgradedFilter(StableDebtTokenUpgradedFilter),
        SupplyCapChangedFilter(SupplyCapChangedFilter),
        UnbackedMintCapChangedFilter(UnbackedMintCapChangedFilter),
        VariableDebtTokenUpgradedFilter(VariableDebtTokenUpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PoolConfiguratorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AtokenUpgradedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::AtokenUpgradedFilter(decoded));
            }
            if let Ok(decoded) = BorrowCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::BorrowCapChangedFilter(decoded));
            }
            if let Ok(decoded) = BorrowableInIsolationChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::BorrowableInIsolationChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = BridgeProtocolFeeUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::BridgeProtocolFeeUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = CollateralConfigurationChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::CollateralConfigurationChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = DebtCeilingChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::DebtCeilingChangedFilter(decoded));
            }
            if let Ok(decoded) = EmodeAssetCategoryChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::EmodeAssetCategoryChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = EmodeCategoryAddedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::EmodeCategoryAddedFilter(decoded));
            }
            if let Ok(decoded)
                = FlashloanPremiumToProtocolUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::FlashloanPremiumToProtocolUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = FlashloanPremiumTotalUpdatedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::FlashloanPremiumTotalUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = LiquidationProtocolFeeChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::LiquidationProtocolFeeChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ReserveActiveFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveActiveFilter(decoded));
            }
            if let Ok(decoded) = ReserveBorrowingFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveBorrowingFilter(decoded));
            }
            if let Ok(decoded) = ReserveDroppedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveDroppedFilter(decoded));
            }
            if let Ok(decoded) = ReserveFactorChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveFactorChangedFilter(decoded));
            }
            if let Ok(decoded) = ReserveFlashLoaningFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveFlashLoaningFilter(decoded));
            }
            if let Ok(decoded) = ReserveFrozenFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveFrozenFilter(decoded));
            }
            if let Ok(decoded) = ReserveInitializedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReserveInitializedFilter(decoded));
            }
            if let Ok(decoded)
                = ReserveInterestRateStrategyChangedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::ReserveInterestRateStrategyChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ReservePausedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::ReservePausedFilter(decoded));
            }
            if let Ok(decoded) = ReserveStableRateBorrowingFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::ReserveStableRateBorrowingFilter(decoded),
                );
            }
            if let Ok(decoded) = SiloedBorrowingChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::SiloedBorrowingChangedFilter(decoded));
            }
            if let Ok(decoded) = StableDebtTokenUpgradedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::StableDebtTokenUpgradedFilter(decoded),
                );
            }
            if let Ok(decoded) = SupplyCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::SupplyCapChangedFilter(decoded));
            }
            if let Ok(decoded) = UnbackedMintCapChangedFilter::decode_log(log) {
                return Ok(PoolConfiguratorEvents::UnbackedMintCapChangedFilter(decoded));
            }
            if let Ok(decoded) = VariableDebtTokenUpgradedFilter::decode_log(log) {
                return Ok(
                    PoolConfiguratorEvents::VariableDebtTokenUpgradedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PoolConfiguratorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AtokenUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowCapChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BorrowableInIsolationChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeProtocolFeeUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollateralConfigurationChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtCeilingChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmodeAssetCategoryChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EmodeCategoryAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashloanPremiumToProtocolUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashloanPremiumTotalUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidationProtocolFeeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveActiveFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveBorrowingFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveDroppedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveFactorChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveFlashLoaningFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveFrozenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveInitializedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveInterestRateStrategyChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReservePausedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ReserveStableRateBorrowingFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SiloedBorrowingChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StableDebtTokenUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupplyCapChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnbackedMintCapChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VariableDebtTokenUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AtokenUpgradedFilter> for PoolConfiguratorEvents {
        fn from(value: AtokenUpgradedFilter) -> Self {
            Self::AtokenUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<BorrowCapChangedFilter> for PoolConfiguratorEvents {
        fn from(value: BorrowCapChangedFilter) -> Self {
            Self::BorrowCapChangedFilter(value)
        }
    }
    impl ::core::convert::From<BorrowableInIsolationChangedFilter>
    for PoolConfiguratorEvents {
        fn from(value: BorrowableInIsolationChangedFilter) -> Self {
            Self::BorrowableInIsolationChangedFilter(value)
        }
    }
    impl ::core::convert::From<BridgeProtocolFeeUpdatedFilter>
    for PoolConfiguratorEvents {
        fn from(value: BridgeProtocolFeeUpdatedFilter) -> Self {
            Self::BridgeProtocolFeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<CollateralConfigurationChangedFilter>
    for PoolConfiguratorEvents {
        fn from(value: CollateralConfigurationChangedFilter) -> Self {
            Self::CollateralConfigurationChangedFilter(value)
        }
    }
    impl ::core::convert::From<DebtCeilingChangedFilter> for PoolConfiguratorEvents {
        fn from(value: DebtCeilingChangedFilter) -> Self {
            Self::DebtCeilingChangedFilter(value)
        }
    }
    impl ::core::convert::From<EmodeAssetCategoryChangedFilter>
    for PoolConfiguratorEvents {
        fn from(value: EmodeAssetCategoryChangedFilter) -> Self {
            Self::EmodeAssetCategoryChangedFilter(value)
        }
    }
    impl ::core::convert::From<EmodeCategoryAddedFilter> for PoolConfiguratorEvents {
        fn from(value: EmodeCategoryAddedFilter) -> Self {
            Self::EmodeCategoryAddedFilter(value)
        }
    }
    impl ::core::convert::From<FlashloanPremiumToProtocolUpdatedFilter>
    for PoolConfiguratorEvents {
        fn from(value: FlashloanPremiumToProtocolUpdatedFilter) -> Self {
            Self::FlashloanPremiumToProtocolUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FlashloanPremiumTotalUpdatedFilter>
    for PoolConfiguratorEvents {
        fn from(value: FlashloanPremiumTotalUpdatedFilter) -> Self {
            Self::FlashloanPremiumTotalUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidationProtocolFeeChangedFilter>
    for PoolConfiguratorEvents {
        fn from(value: LiquidationProtocolFeeChangedFilter) -> Self {
            Self::LiquidationProtocolFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveActiveFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveActiveFilter) -> Self {
            Self::ReserveActiveFilter(value)
        }
    }
    impl ::core::convert::From<ReserveBorrowingFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveBorrowingFilter) -> Self {
            Self::ReserveBorrowingFilter(value)
        }
    }
    impl ::core::convert::From<ReserveDroppedFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveDroppedFilter) -> Self {
            Self::ReserveDroppedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveFactorChangedFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveFactorChangedFilter) -> Self {
            Self::ReserveFactorChangedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveFlashLoaningFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveFlashLoaningFilter) -> Self {
            Self::ReserveFlashLoaningFilter(value)
        }
    }
    impl ::core::convert::From<ReserveFrozenFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveFrozenFilter) -> Self {
            Self::ReserveFrozenFilter(value)
        }
    }
    impl ::core::convert::From<ReserveInitializedFilter> for PoolConfiguratorEvents {
        fn from(value: ReserveInitializedFilter) -> Self {
            Self::ReserveInitializedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveInterestRateStrategyChangedFilter>
    for PoolConfiguratorEvents {
        fn from(value: ReserveInterestRateStrategyChangedFilter) -> Self {
            Self::ReserveInterestRateStrategyChangedFilter(value)
        }
    }
    impl ::core::convert::From<ReservePausedFilter> for PoolConfiguratorEvents {
        fn from(value: ReservePausedFilter) -> Self {
            Self::ReservePausedFilter(value)
        }
    }
    impl ::core::convert::From<ReserveStableRateBorrowingFilter>
    for PoolConfiguratorEvents {
        fn from(value: ReserveStableRateBorrowingFilter) -> Self {
            Self::ReserveStableRateBorrowingFilter(value)
        }
    }
    impl ::core::convert::From<SiloedBorrowingChangedFilter> for PoolConfiguratorEvents {
        fn from(value: SiloedBorrowingChangedFilter) -> Self {
            Self::SiloedBorrowingChangedFilter(value)
        }
    }
    impl ::core::convert::From<StableDebtTokenUpgradedFilter>
    for PoolConfiguratorEvents {
        fn from(value: StableDebtTokenUpgradedFilter) -> Self {
            Self::StableDebtTokenUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<SupplyCapChangedFilter> for PoolConfiguratorEvents {
        fn from(value: SupplyCapChangedFilter) -> Self {
            Self::SupplyCapChangedFilter(value)
        }
    }
    impl ::core::convert::From<UnbackedMintCapChangedFilter> for PoolConfiguratorEvents {
        fn from(value: UnbackedMintCapChangedFilter) -> Self {
            Self::UnbackedMintCapChangedFilter(value)
        }
    }
    impl ::core::convert::From<VariableDebtTokenUpgradedFilter>
    for PoolConfiguratorEvents {
        fn from(value: VariableDebtTokenUpgradedFilter) -> Self {
            Self::VariableDebtTokenUpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `CONFIGURATOR_REVISION` function with signature `CONFIGURATOR_REVISION()` and selector `0x7af635a6`
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
    #[ethcall(name = "CONFIGURATOR_REVISION", abi = "CONFIGURATOR_REVISION()")]
    pub struct ConfiguratorRevisionCall;
    ///Container type for all input parameters for the `configureReserveAsCollateral` function with signature `configureReserveAsCollateral(address,uint256,uint256,uint256)` and selector `0x7c4e560b`
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
        name = "configureReserveAsCollateral",
        abi = "configureReserveAsCollateral(address,uint256,uint256,uint256)"
    )]
    pub struct ConfigureReserveAsCollateralCall {
        pub asset: ::ethers::core::types::Address,
        pub ltv: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub liquidation_bonus: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `initReserves` function with signature `initReserves((address,address,address,uint8,address,address,address,address,string,string,string,string,string,string,bytes)[])` and selector `0x02fb45e6`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "initReserves",
        abi = "initReserves((address,address,address,uint8,address,address,address,address,string,string,string,string,string,string,bytes)[])"
    )]
    pub struct InitReservesCall {
        pub input: ::std::vec::Vec<InitReserveInput>,
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
    ///Container type for all input parameters for the `setAssetEModeCategory` function with signature `setAssetEModeCategory(address,uint8)` and selector `0xd4fe3f99`
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
        name = "setAssetEModeCategory",
        abi = "setAssetEModeCategory(address,uint8)"
    )]
    pub struct SetAssetEModeCategoryCall {
        pub asset: ::ethers::core::types::Address,
        pub new_category_id: u8,
    }
    ///Container type for all input parameters for the `setBorrowCap` function with signature `setBorrowCap(address,uint256)` and selector `0xd14a0983`
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
    #[ethcall(name = "setBorrowCap", abi = "setBorrowCap(address,uint256)")]
    pub struct SetBorrowCapCall {
        pub asset: ::ethers::core::types::Address,
        pub new_borrow_cap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setBorrowableInIsolation` function with signature `setBorrowableInIsolation(address,bool)` and selector `0x38ae0cc3`
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
        name = "setBorrowableInIsolation",
        abi = "setBorrowableInIsolation(address,bool)"
    )]
    pub struct SetBorrowableInIsolationCall {
        pub asset: ::ethers::core::types::Address,
        pub borrowable: bool,
    }
    ///Container type for all input parameters for the `setDebtCeiling` function with signature `setDebtCeiling(address,uint256)` and selector `0xaeb4fcc1`
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
    #[ethcall(name = "setDebtCeiling", abi = "setDebtCeiling(address,uint256)")]
    pub struct SetDebtCeilingCall {
        pub asset: ::ethers::core::types::Address,
        pub new_debt_ceiling: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEModeCategory` function with signature `setEModeCategory(uint8,uint16,uint16,uint16,address,string)` and selector `0xc19d61e4`
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
        name = "setEModeCategory",
        abi = "setEModeCategory(uint8,uint16,uint16,uint16,address,string)"
    )]
    pub struct SetEModeCategoryCall {
        pub category_id: u8,
        pub ltv: u16,
        pub liquidation_threshold: u16,
        pub liquidation_bonus: u16,
        pub oracle: ::ethers::core::types::Address,
        pub label: ::std::string::String,
    }
    ///Container type for all input parameters for the `setLiquidationProtocolFee` function with signature `setLiquidationProtocolFee(address,uint256)` and selector `0x26d2cec2`
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
        abi = "setLiquidationProtocolFee(address,uint256)"
    )]
    pub struct SetLiquidationProtocolFeeCall {
        pub asset: ::ethers::core::types::Address,
        pub new_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPoolPause` function with signature `setPoolPause(bool)` and selector `0x7641f3d9`
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
    #[ethcall(name = "setPoolPause", abi = "setPoolPause(bool)")]
    pub struct SetPoolPauseCall {
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setReserveActive` function with signature `setReserveActive(address,bool)` and selector `0xb736aaeb`
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
    #[ethcall(name = "setReserveActive", abi = "setReserveActive(address,bool)")]
    pub struct SetReserveActiveCall {
        pub asset: ::ethers::core::types::Address,
        pub active: bool,
    }
    ///Container type for all input parameters for the `setReserveBorrowing` function with signature `setReserveBorrowing(address,bool)` and selector `0x682cf264`
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
    #[ethcall(name = "setReserveBorrowing", abi = "setReserveBorrowing(address,bool)")]
    pub struct SetReserveBorrowingCall {
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setReserveFactor` function with signature `setReserveFactor(address,uint256)` and selector `0x4b4e6753`
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
    #[ethcall(name = "setReserveFactor", abi = "setReserveFactor(address,uint256)")]
    pub struct SetReserveFactorCall {
        pub asset: ::ethers::core::types::Address,
        pub new_reserve_factor: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setReserveFlashLoaning` function with signature `setReserveFlashLoaning(address,bool)` and selector `0xf213ef0e`
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
        name = "setReserveFlashLoaning",
        abi = "setReserveFlashLoaning(address,bool)"
    )]
    pub struct SetReserveFlashLoaningCall {
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setReserveFreeze` function with signature `setReserveFreeze(address,bool)` and selector `0x96e957c4`
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
    #[ethcall(name = "setReserveFreeze", abi = "setReserveFreeze(address,bool)")]
    pub struct SetReserveFreezeCall {
        pub asset: ::ethers::core::types::Address,
        pub freeze: bool,
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
        pub new_rate_strategy_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setReservePause` function with signature `setReservePause(address,bool)` and selector `0x48d9fba9`
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
    #[ethcall(name = "setReservePause", abi = "setReservePause(address,bool)")]
    pub struct SetReservePauseCall {
        pub asset: ::ethers::core::types::Address,
        pub paused: bool,
    }
    ///Container type for all input parameters for the `setReserveStableRateBorrowing` function with signature `setReserveStableRateBorrowing(address,bool)` and selector `0x8a751a60`
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
        name = "setReserveStableRateBorrowing",
        abi = "setReserveStableRateBorrowing(address,bool)"
    )]
    pub struct SetReserveStableRateBorrowingCall {
        pub asset: ::ethers::core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setSiloedBorrowing` function with signature `setSiloedBorrowing(address,bool)` and selector `0xa7fa83b7`
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
    #[ethcall(name = "setSiloedBorrowing", abi = "setSiloedBorrowing(address,bool)")]
    pub struct SetSiloedBorrowingCall {
        pub asset: ::ethers::core::types::Address,
        pub new_siloed: bool,
    }
    ///Container type for all input parameters for the `setSupplyCap` function with signature `setSupplyCap(address,uint256)` and selector `0x571f03e5`
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
    #[ethcall(name = "setSupplyCap", abi = "setSupplyCap(address,uint256)")]
    pub struct SetSupplyCapCall {
        pub asset: ::ethers::core::types::Address,
        pub new_supply_cap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setUnbackedMintCap` function with signature `setUnbackedMintCap(address,uint256)` and selector `0x145f5892`
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
    #[ethcall(name = "setUnbackedMintCap", abi = "setUnbackedMintCap(address,uint256)")]
    pub struct SetUnbackedMintCapCall {
        pub asset: ::ethers::core::types::Address,
        pub new_unbacked_mint_cap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateAToken` function with signature `updateAToken((address,address,address,string,string,address,bytes))` and selector `0xbb01c37c`
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
        name = "updateAToken",
        abi = "updateAToken((address,address,address,string,string,address,bytes))"
    )]
    pub struct UpdateATokenCall {
        pub input: UpdateATokenInput,
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
        pub new_bridge_protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateFlashloanPremiumToProtocol` function with signature `updateFlashloanPremiumToProtocol(uint128)` and selector `0x1df970bd`
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
        name = "updateFlashloanPremiumToProtocol",
        abi = "updateFlashloanPremiumToProtocol(uint128)"
    )]
    pub struct UpdateFlashloanPremiumToProtocolCall {
        pub new_flashloan_premium_to_protocol: u128,
    }
    ///Container type for all input parameters for the `updateFlashloanPremiumTotal` function with signature `updateFlashloanPremiumTotal(uint128)` and selector `0x8a493676`
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
        name = "updateFlashloanPremiumTotal",
        abi = "updateFlashloanPremiumTotal(uint128)"
    )]
    pub struct UpdateFlashloanPremiumTotalCall {
        pub new_flashloan_premium_total: u128,
    }
    ///Container type for all input parameters for the `updateStableDebtToken` function with signature `updateStableDebtToken((address,address,string,string,address,bytes))` and selector `0x7626cde3`
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
        name = "updateStableDebtToken",
        abi = "updateStableDebtToken((address,address,string,string,address,bytes))"
    )]
    pub struct UpdateStableDebtTokenCall {
        pub input: UpdateDebtTokenInput,
    }
    ///Container type for all input parameters for the `updateVariableDebtToken` function with signature `updateVariableDebtToken((address,address,string,string,address,bytes))` and selector `0xad4e6432`
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
        name = "updateVariableDebtToken",
        abi = "updateVariableDebtToken((address,address,string,string,address,bytes))"
    )]
    pub struct UpdateVariableDebtTokenCall {
        pub input: UpdateDebtTokenInput,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum PoolConfiguratorCalls {
        ConfiguratorRevision(ConfiguratorRevisionCall),
        ConfigureReserveAsCollateral(ConfigureReserveAsCollateralCall),
        DropReserve(DropReserveCall),
        InitReserves(InitReservesCall),
        Initialize(InitializeCall),
        SetAssetEModeCategory(SetAssetEModeCategoryCall),
        SetBorrowCap(SetBorrowCapCall),
        SetBorrowableInIsolation(SetBorrowableInIsolationCall),
        SetDebtCeiling(SetDebtCeilingCall),
        SetEModeCategory(SetEModeCategoryCall),
        SetLiquidationProtocolFee(SetLiquidationProtocolFeeCall),
        SetPoolPause(SetPoolPauseCall),
        SetReserveActive(SetReserveActiveCall),
        SetReserveBorrowing(SetReserveBorrowingCall),
        SetReserveFactor(SetReserveFactorCall),
        SetReserveFlashLoaning(SetReserveFlashLoaningCall),
        SetReserveFreeze(SetReserveFreezeCall),
        SetReserveInterestRateStrategyAddress(SetReserveInterestRateStrategyAddressCall),
        SetReservePause(SetReservePauseCall),
        SetReserveStableRateBorrowing(SetReserveStableRateBorrowingCall),
        SetSiloedBorrowing(SetSiloedBorrowingCall),
        SetSupplyCap(SetSupplyCapCall),
        SetUnbackedMintCap(SetUnbackedMintCapCall),
        UpdateAToken(UpdateATokenCall),
        UpdateBridgeProtocolFee(UpdateBridgeProtocolFeeCall),
        UpdateFlashloanPremiumToProtocol(UpdateFlashloanPremiumToProtocolCall),
        UpdateFlashloanPremiumTotal(UpdateFlashloanPremiumTotalCall),
        UpdateStableDebtToken(UpdateStableDebtTokenCall),
        UpdateVariableDebtToken(UpdateVariableDebtTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolConfiguratorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConfiguratorRevisionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfiguratorRevision(decoded));
            }
            if let Ok(decoded)
                = <ConfigureReserveAsCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigureReserveAsCollateral(decoded));
            }
            if let Ok(decoded)
                = <DropReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropReserve(decoded));
            }
            if let Ok(decoded)
                = <InitReservesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitReserves(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <SetAssetEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetAssetEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBorrowCap(decoded));
            }
            if let Ok(decoded)
                = <SetBorrowableInIsolationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetBorrowableInIsolation(decoded));
            }
            if let Ok(decoded)
                = <SetDebtCeilingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDebtCeiling(decoded));
            }
            if let Ok(decoded)
                = <SetEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <SetLiquidationProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <SetPoolPauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPoolPause(decoded));
            }
            if let Ok(decoded)
                = <SetReserveActiveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveActive(decoded));
            }
            if let Ok(decoded)
                = <SetReserveBorrowingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveBorrowing(decoded));
            }
            if let Ok(decoded)
                = <SetReserveFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveFactor(decoded));
            }
            if let Ok(decoded)
                = <SetReserveFlashLoaningCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveFlashLoaning(decoded));
            }
            if let Ok(decoded)
                = <SetReserveFreezeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveFreeze(decoded));
            }
            if let Ok(decoded)
                = <SetReserveInterestRateStrategyAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded)
                = <SetReservePauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetReservePause(decoded));
            }
            if let Ok(decoded)
                = <SetReserveStableRateBorrowingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetReserveStableRateBorrowing(decoded));
            }
            if let Ok(decoded)
                = <SetSiloedBorrowingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetSiloedBorrowing(decoded));
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
            if let Ok(decoded)
                = <UpdateATokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateAToken(decoded));
            }
            if let Ok(decoded)
                = <UpdateBridgeProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateBridgeProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <UpdateFlashloanPremiumToProtocolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateFlashloanPremiumToProtocol(decoded));
            }
            if let Ok(decoded)
                = <UpdateFlashloanPremiumTotalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateFlashloanPremiumTotal(decoded));
            }
            if let Ok(decoded)
                = <UpdateStableDebtTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateStableDebtToken(decoded));
            }
            if let Ok(decoded)
                = <UpdateVariableDebtTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateVariableDebtToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolConfiguratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConfiguratorRevision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigureReserveAsCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DropReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAssetEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBorrowableInIsolation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDebtCeiling(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidationProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPoolPause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveBorrowing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveFlashLoaning(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveFreeze(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveInterestRateStrategyAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReservePause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetReserveStableRateBorrowing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSiloedBorrowing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnbackedMintCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateBridgeProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFlashloanPremiumToProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFlashloanPremiumTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateStableDebtToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateVariableDebtToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoolConfiguratorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfiguratorRevision(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigureReserveAsCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DropReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssetEModeCategory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetBorrowCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBorrowableInIsolation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDebtCeiling(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEModeCategory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLiquidationProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPoolPause(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetReserveFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveFlashLoaning(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetReserveFreeze(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveInterestRateStrategyAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetReservePause(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReserveStableRateBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSiloedBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSupplyCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUnbackedMintCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateAToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBridgeProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateFlashloanPremiumToProtocol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateFlashloanPremiumTotal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateStableDebtToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateVariableDebtToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ConfiguratorRevisionCall> for PoolConfiguratorCalls {
        fn from(value: ConfiguratorRevisionCall) -> Self {
            Self::ConfiguratorRevision(value)
        }
    }
    impl ::core::convert::From<ConfigureReserveAsCollateralCall>
    for PoolConfiguratorCalls {
        fn from(value: ConfigureReserveAsCollateralCall) -> Self {
            Self::ConfigureReserveAsCollateral(value)
        }
    }
    impl ::core::convert::From<DropReserveCall> for PoolConfiguratorCalls {
        fn from(value: DropReserveCall) -> Self {
            Self::DropReserve(value)
        }
    }
    impl ::core::convert::From<InitReservesCall> for PoolConfiguratorCalls {
        fn from(value: InitReservesCall) -> Self {
            Self::InitReserves(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for PoolConfiguratorCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<SetAssetEModeCategoryCall> for PoolConfiguratorCalls {
        fn from(value: SetAssetEModeCategoryCall) -> Self {
            Self::SetAssetEModeCategory(value)
        }
    }
    impl ::core::convert::From<SetBorrowCapCall> for PoolConfiguratorCalls {
        fn from(value: SetBorrowCapCall) -> Self {
            Self::SetBorrowCap(value)
        }
    }
    impl ::core::convert::From<SetBorrowableInIsolationCall> for PoolConfiguratorCalls {
        fn from(value: SetBorrowableInIsolationCall) -> Self {
            Self::SetBorrowableInIsolation(value)
        }
    }
    impl ::core::convert::From<SetDebtCeilingCall> for PoolConfiguratorCalls {
        fn from(value: SetDebtCeilingCall) -> Self {
            Self::SetDebtCeiling(value)
        }
    }
    impl ::core::convert::From<SetEModeCategoryCall> for PoolConfiguratorCalls {
        fn from(value: SetEModeCategoryCall) -> Self {
            Self::SetEModeCategory(value)
        }
    }
    impl ::core::convert::From<SetLiquidationProtocolFeeCall> for PoolConfiguratorCalls {
        fn from(value: SetLiquidationProtocolFeeCall) -> Self {
            Self::SetLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<SetPoolPauseCall> for PoolConfiguratorCalls {
        fn from(value: SetPoolPauseCall) -> Self {
            Self::SetPoolPause(value)
        }
    }
    impl ::core::convert::From<SetReserveActiveCall> for PoolConfiguratorCalls {
        fn from(value: SetReserveActiveCall) -> Self {
            Self::SetReserveActive(value)
        }
    }
    impl ::core::convert::From<SetReserveBorrowingCall> for PoolConfiguratorCalls {
        fn from(value: SetReserveBorrowingCall) -> Self {
            Self::SetReserveBorrowing(value)
        }
    }
    impl ::core::convert::From<SetReserveFactorCall> for PoolConfiguratorCalls {
        fn from(value: SetReserveFactorCall) -> Self {
            Self::SetReserveFactor(value)
        }
    }
    impl ::core::convert::From<SetReserveFlashLoaningCall> for PoolConfiguratorCalls {
        fn from(value: SetReserveFlashLoaningCall) -> Self {
            Self::SetReserveFlashLoaning(value)
        }
    }
    impl ::core::convert::From<SetReserveFreezeCall> for PoolConfiguratorCalls {
        fn from(value: SetReserveFreezeCall) -> Self {
            Self::SetReserveFreeze(value)
        }
    }
    impl ::core::convert::From<SetReserveInterestRateStrategyAddressCall>
    for PoolConfiguratorCalls {
        fn from(value: SetReserveInterestRateStrategyAddressCall) -> Self {
            Self::SetReserveInterestRateStrategyAddress(value)
        }
    }
    impl ::core::convert::From<SetReservePauseCall> for PoolConfiguratorCalls {
        fn from(value: SetReservePauseCall) -> Self {
            Self::SetReservePause(value)
        }
    }
    impl ::core::convert::From<SetReserveStableRateBorrowingCall>
    for PoolConfiguratorCalls {
        fn from(value: SetReserveStableRateBorrowingCall) -> Self {
            Self::SetReserveStableRateBorrowing(value)
        }
    }
    impl ::core::convert::From<SetSiloedBorrowingCall> for PoolConfiguratorCalls {
        fn from(value: SetSiloedBorrowingCall) -> Self {
            Self::SetSiloedBorrowing(value)
        }
    }
    impl ::core::convert::From<SetSupplyCapCall> for PoolConfiguratorCalls {
        fn from(value: SetSupplyCapCall) -> Self {
            Self::SetSupplyCap(value)
        }
    }
    impl ::core::convert::From<SetUnbackedMintCapCall> for PoolConfiguratorCalls {
        fn from(value: SetUnbackedMintCapCall) -> Self {
            Self::SetUnbackedMintCap(value)
        }
    }
    impl ::core::convert::From<UpdateATokenCall> for PoolConfiguratorCalls {
        fn from(value: UpdateATokenCall) -> Self {
            Self::UpdateAToken(value)
        }
    }
    impl ::core::convert::From<UpdateBridgeProtocolFeeCall> for PoolConfiguratorCalls {
        fn from(value: UpdateBridgeProtocolFeeCall) -> Self {
            Self::UpdateBridgeProtocolFee(value)
        }
    }
    impl ::core::convert::From<UpdateFlashloanPremiumToProtocolCall>
    for PoolConfiguratorCalls {
        fn from(value: UpdateFlashloanPremiumToProtocolCall) -> Self {
            Self::UpdateFlashloanPremiumToProtocol(value)
        }
    }
    impl ::core::convert::From<UpdateFlashloanPremiumTotalCall>
    for PoolConfiguratorCalls {
        fn from(value: UpdateFlashloanPremiumTotalCall) -> Self {
            Self::UpdateFlashloanPremiumTotal(value)
        }
    }
    impl ::core::convert::From<UpdateStableDebtTokenCall> for PoolConfiguratorCalls {
        fn from(value: UpdateStableDebtTokenCall) -> Self {
            Self::UpdateStableDebtToken(value)
        }
    }
    impl ::core::convert::From<UpdateVariableDebtTokenCall> for PoolConfiguratorCalls {
        fn from(value: UpdateVariableDebtTokenCall) -> Self {
            Self::UpdateVariableDebtToken(value)
        }
    }
    ///Container type for all return fields from the `CONFIGURATOR_REVISION` function with signature `CONFIGURATOR_REVISION()` and selector `0x7af635a6`
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
    pub struct ConfiguratorRevisionReturn(pub ::ethers::core::types::U256);
}
