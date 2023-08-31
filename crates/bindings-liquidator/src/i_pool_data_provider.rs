pub use i_pool_data_provider::*;
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
pub mod i_pool_data_provider {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getATokenTotalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getATokenTotalSupply",
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
                    ::std::borrow::ToOwned::to_owned("getAllATokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllATokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolDataProvider.TokenData[]",
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
                    ::std::borrow::ToOwned::to_owned("getAllReservesTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAllReservesTokens",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolDataProvider.TokenData[]",
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
                    ::std::borrow::ToOwned::to_owned("getDebtCeiling"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDebtCeiling"),
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
                    ::std::borrow::ToOwned::to_owned("getDebtCeilingDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDebtCeilingDecimals",
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("getInterestRateStrategyAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getInterestRateStrategyAddress",
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
                                    name: ::std::borrow::ToOwned::to_owned("irStrategyAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getLiquidationProtocolFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidationProtocolFee",
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
                    ::std::borrow::ToOwned::to_owned("getPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPaused"),
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
                                    name: ::std::borrow::ToOwned::to_owned("isPaused"),
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
                    ::std::borrow::ToOwned::to_owned("getReserveCaps"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReserveCaps"),
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
                                    name: ::std::borrow::ToOwned::to_owned("borrowCap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserveConfigurationData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveConfigurationData",
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
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveFactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "usageAsCollateralEnabled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrowingEnabled"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "stableBorrowRateEnabled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isActive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isFrozen"),
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
                                    name: ::std::borrow::ToOwned::to_owned("unbacked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "accruedToTreasuryScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalAToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalStableDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalVariableDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableBorrowRate",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stableBorrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "averageStableBorrowRate",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableBorrowIndex",
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
                                        "lastUpdateTimestamp",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("getReserveEModeCategory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveEModeCategory",
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
                    ::std::borrow::ToOwned::to_owned("getReserveTokensAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReserveTokensAddresses",
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
                                    name: ::std::borrow::ToOwned::to_owned("aTokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "stableDebtTokenAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "variableDebtTokenAddress",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("getSiloedBorrowing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSiloedBorrowing"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTotalDebt"),
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
                    ::std::borrow::ToOwned::to_owned("getUnbackedMintCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUnbackedMintCap"),
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
                    ::std::borrow::ToOwned::to_owned("getUserReserveData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUserReserveData"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentATokenBalance",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentStableDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "currentVariableDebt",
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
                                        "principalStableDebt",
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
                                        "scaledVariableDebt",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stableBorrowRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "stableRateLastUpdated",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint40"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "usageAsCollateralEnabled",
                                    ),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IPOOLDATAPROVIDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPoolDataProvider<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPoolDataProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPoolDataProvider<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPoolDataProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPoolDataProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPoolDataProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPoolDataProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOOLDATAPROVIDER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getATokenTotalSupply` (0x51460e25) function
        pub fn get_a_token_total_supply(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 70, 14, 37], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllATokens` (0xf561ae41) function
        pub fn get_all_a_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenData>> {
            self.0
                .method_hash([245, 97, 174, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllReservesTokens` (0xb316ff89) function
        pub fn get_all_reserves_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenData>> {
            self.0
                .method_hash([179, 22, 255, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDebtCeiling` (0x3c798109) function
        pub fn get_debt_ceiling(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 121, 129, 9], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDebtCeilingDecimals` (0x69b169e1) function
        pub fn get_debt_ceiling_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([105, 177, 105, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFlashLoanEnabled` (0xd7ed3ef4) function
        pub fn get_flash_loan_enabled(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([215, 237, 62, 244], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInterestRateStrategyAddress` (0x6744362a) function
        pub fn get_interest_rate_strategy_address(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([103, 68, 54, 42], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationProtocolFee` (0x3cb8a622) function
        pub fn get_liquidation_protocol_fee(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 184, 166, 34], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPaused` (0xb55d9904) function
        pub fn get_paused(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 93, 153, 4], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveCaps` (0x46fbe558) function
        pub fn get_reserve_caps(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([70, 251, 229, 88], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveConfigurationData` (0x3e150141) function
        pub fn get_reserve_configuration_data(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                bool,
                bool,
                bool,
                bool,
            ),
        > {
            self.0
                .method_hash([62, 21, 1, 65], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveData` (0x35ea6a75) function
        pub fn get_reserve_data(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u64,
            ),
        > {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveEModeCategory` (0x163a0f20) function
        pub fn get_reserve_e_mode_category(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 58, 15, 32], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserveTokensAddresses` (0xd2493b6c) function
        pub fn get_reserve_tokens_addresses(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([210, 73, 59, 108], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSiloedBorrowing` (0xfcf40a62) function
        pub fn get_siloed_borrowing(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 244, 10, 98], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalDebt` (0x4d44ac4f) function
        pub fn get_total_debt(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 68, 172, 79], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnbackedMintCap` (0x7ba1ae36) function
        pub fn get_unbacked_mint_cap(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([123, 161, 174, 54], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUserReserveData` (0x28dd2d01) function
        pub fn get_user_reserve_data(
            &self,
            asset: ::ethers::core::types::Address,
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
                ::ethers::core::types::U256,
                u64,
                bool,
            ),
        > {
            self.0
                .method_hash([40, 221, 45, 1], (asset, user))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPoolDataProvider<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getATokenTotalSupply` function with signature `getATokenTotalSupply(address)` and selector `0x51460e25`
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
    #[ethcall(name = "getATokenTotalSupply", abi = "getATokenTotalSupply(address)")]
    pub struct GetATokenTotalSupplyCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAllATokens` function with signature `getAllATokens()` and selector `0xf561ae41`
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
    #[ethcall(name = "getAllATokens", abi = "getAllATokens()")]
    pub struct GetAllATokensCall;
    ///Container type for all input parameters for the `getAllReservesTokens` function with signature `getAllReservesTokens()` and selector `0xb316ff89`
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
    #[ethcall(name = "getAllReservesTokens", abi = "getAllReservesTokens()")]
    pub struct GetAllReservesTokensCall;
    ///Container type for all input parameters for the `getDebtCeiling` function with signature `getDebtCeiling(address)` and selector `0x3c798109`
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
    #[ethcall(name = "getDebtCeiling", abi = "getDebtCeiling(address)")]
    pub struct GetDebtCeilingCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDebtCeilingDecimals` function with signature `getDebtCeilingDecimals()` and selector `0x69b169e1`
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
    #[ethcall(name = "getDebtCeilingDecimals", abi = "getDebtCeilingDecimals()")]
    pub struct GetDebtCeilingDecimalsCall;
    ///Container type for all input parameters for the `getFlashLoanEnabled` function with signature `getFlashLoanEnabled(address)` and selector `0xd7ed3ef4`
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
    #[ethcall(name = "getFlashLoanEnabled", abi = "getFlashLoanEnabled(address)")]
    pub struct GetFlashLoanEnabledCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInterestRateStrategyAddress` function with signature `getInterestRateStrategyAddress(address)` and selector `0x6744362a`
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
        name = "getInterestRateStrategyAddress",
        abi = "getInterestRateStrategyAddress(address)"
    )]
    pub struct GetInterestRateStrategyAddressCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee(address)` and selector `0x3cb8a622`
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
        name = "getLiquidationProtocolFee",
        abi = "getLiquidationProtocolFee(address)"
    )]
    pub struct GetLiquidationProtocolFeeCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPaused` function with signature `getPaused(address)` and selector `0xb55d9904`
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
    #[ethcall(name = "getPaused", abi = "getPaused(address)")]
    pub struct GetPausedCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReserveCaps` function with signature `getReserveCaps(address)` and selector `0x46fbe558`
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
    #[ethcall(name = "getReserveCaps", abi = "getReserveCaps(address)")]
    pub struct GetReserveCapsCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReserveConfigurationData` function with signature `getReserveConfigurationData(address)` and selector `0x3e150141`
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
        name = "getReserveConfigurationData",
        abi = "getReserveConfigurationData(address)"
    )]
    pub struct GetReserveConfigurationDataCall {
        pub asset: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getReserveEModeCategory` function with signature `getReserveEModeCategory(address)` and selector `0x163a0f20`
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
        name = "getReserveEModeCategory",
        abi = "getReserveEModeCategory(address)"
    )]
    pub struct GetReserveEModeCategoryCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReserveTokensAddresses` function with signature `getReserveTokensAddresses(address)` and selector `0xd2493b6c`
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
        name = "getReserveTokensAddresses",
        abi = "getReserveTokensAddresses(address)"
    )]
    pub struct GetReserveTokensAddressesCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSiloedBorrowing` function with signature `getSiloedBorrowing(address)` and selector `0xfcf40a62`
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
    #[ethcall(name = "getSiloedBorrowing", abi = "getSiloedBorrowing(address)")]
    pub struct GetSiloedBorrowingCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTotalDebt` function with signature `getTotalDebt(address)` and selector `0x4d44ac4f`
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
    #[ethcall(name = "getTotalDebt", abi = "getTotalDebt(address)")]
    pub struct GetTotalDebtCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUnbackedMintCap` function with signature `getUnbackedMintCap(address)` and selector `0x7ba1ae36`
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
    #[ethcall(name = "getUnbackedMintCap", abi = "getUnbackedMintCap(address)")]
    pub struct GetUnbackedMintCapCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getUserReserveData` function with signature `getUserReserveData(address,address)` and selector `0x28dd2d01`
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
    #[ethcall(name = "getUserReserveData", abi = "getUserReserveData(address,address)")]
    pub struct GetUserReserveDataCall {
        pub asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPoolDataProviderCalls {
        GetATokenTotalSupply(GetATokenTotalSupplyCall),
        GetAllATokens(GetAllATokensCall),
        GetAllReservesTokens(GetAllReservesTokensCall),
        GetDebtCeiling(GetDebtCeilingCall),
        GetDebtCeilingDecimals(GetDebtCeilingDecimalsCall),
        GetFlashLoanEnabled(GetFlashLoanEnabledCall),
        GetInterestRateStrategyAddress(GetInterestRateStrategyAddressCall),
        GetLiquidationProtocolFee(GetLiquidationProtocolFeeCall),
        GetPaused(GetPausedCall),
        GetReserveCaps(GetReserveCapsCall),
        GetReserveConfigurationData(GetReserveConfigurationDataCall),
        GetReserveData(GetReserveDataCall),
        GetReserveEModeCategory(GetReserveEModeCategoryCall),
        GetReserveTokensAddresses(GetReserveTokensAddressesCall),
        GetSiloedBorrowing(GetSiloedBorrowingCall),
        GetTotalDebt(GetTotalDebtCall),
        GetUnbackedMintCap(GetUnbackedMintCapCall),
        GetUserReserveData(GetUserReserveDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolDataProviderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetATokenTotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetATokenTotalSupply(decoded));
            }
            if let Ok(decoded)
                = <GetAllATokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllATokens(decoded));
            }
            if let Ok(decoded)
                = <GetAllReservesTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAllReservesTokens(decoded));
            }
            if let Ok(decoded)
                = <GetDebtCeilingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDebtCeiling(decoded));
            }
            if let Ok(decoded)
                = <GetDebtCeilingDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetDebtCeilingDecimals(decoded));
            }
            if let Ok(decoded)
                = <GetFlashLoanEnabledCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetFlashLoanEnabled(decoded));
            }
            if let Ok(decoded)
                = <GetInterestRateStrategyAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded)
                = <GetLiquidationProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLiquidationProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <GetPausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPaused(decoded));
            }
            if let Ok(decoded)
                = <GetReserveCapsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserveCaps(decoded));
            }
            if let Ok(decoded)
                = <GetReserveConfigurationDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveConfigurationData(decoded));
            }
            if let Ok(decoded)
                = <GetReserveDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserveData(decoded));
            }
            if let Ok(decoded)
                = <GetReserveEModeCategoryCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveEModeCategory(decoded));
            }
            if let Ok(decoded)
                = <GetReserveTokensAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetReserveTokensAddresses(decoded));
            }
            if let Ok(decoded)
                = <GetSiloedBorrowingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSiloedBorrowing(decoded));
            }
            if let Ok(decoded)
                = <GetTotalDebtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTotalDebt(decoded));
            }
            if let Ok(decoded)
                = <GetUnbackedMintCapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnbackedMintCap(decoded));
            }
            if let Ok(decoded)
                = <GetUserReserveDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUserReserveData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPoolDataProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetATokenTotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllATokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllReservesTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDebtCeiling(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDebtCeilingDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFlashLoanEnabled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInterestRateStrategyAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidationProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveCaps(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveConfigurationData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveEModeCategory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReserveTokensAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSiloedBorrowing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnbackedMintCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUserReserveData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPoolDataProviderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetATokenTotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAllATokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllReservesTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDebtCeiling(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDebtCeilingDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFlashLoanEnabled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInterestRateStrategyAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLiquidationProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveCaps(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveConfigurationData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReserveData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserveEModeCategory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetReserveTokensAddresses(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSiloedBorrowing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnbackedMintCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUserReserveData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetATokenTotalSupplyCall> for IPoolDataProviderCalls {
        fn from(value: GetATokenTotalSupplyCall) -> Self {
            Self::GetATokenTotalSupply(value)
        }
    }
    impl ::core::convert::From<GetAllATokensCall> for IPoolDataProviderCalls {
        fn from(value: GetAllATokensCall) -> Self {
            Self::GetAllATokens(value)
        }
    }
    impl ::core::convert::From<GetAllReservesTokensCall> for IPoolDataProviderCalls {
        fn from(value: GetAllReservesTokensCall) -> Self {
            Self::GetAllReservesTokens(value)
        }
    }
    impl ::core::convert::From<GetDebtCeilingCall> for IPoolDataProviderCalls {
        fn from(value: GetDebtCeilingCall) -> Self {
            Self::GetDebtCeiling(value)
        }
    }
    impl ::core::convert::From<GetDebtCeilingDecimalsCall> for IPoolDataProviderCalls {
        fn from(value: GetDebtCeilingDecimalsCall) -> Self {
            Self::GetDebtCeilingDecimals(value)
        }
    }
    impl ::core::convert::From<GetFlashLoanEnabledCall> for IPoolDataProviderCalls {
        fn from(value: GetFlashLoanEnabledCall) -> Self {
            Self::GetFlashLoanEnabled(value)
        }
    }
    impl ::core::convert::From<GetInterestRateStrategyAddressCall>
    for IPoolDataProviderCalls {
        fn from(value: GetInterestRateStrategyAddressCall) -> Self {
            Self::GetInterestRateStrategyAddress(value)
        }
    }
    impl ::core::convert::From<GetLiquidationProtocolFeeCall>
    for IPoolDataProviderCalls {
        fn from(value: GetLiquidationProtocolFeeCall) -> Self {
            Self::GetLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<GetPausedCall> for IPoolDataProviderCalls {
        fn from(value: GetPausedCall) -> Self {
            Self::GetPaused(value)
        }
    }
    impl ::core::convert::From<GetReserveCapsCall> for IPoolDataProviderCalls {
        fn from(value: GetReserveCapsCall) -> Self {
            Self::GetReserveCaps(value)
        }
    }
    impl ::core::convert::From<GetReserveConfigurationDataCall>
    for IPoolDataProviderCalls {
        fn from(value: GetReserveConfigurationDataCall) -> Self {
            Self::GetReserveConfigurationData(value)
        }
    }
    impl ::core::convert::From<GetReserveDataCall> for IPoolDataProviderCalls {
        fn from(value: GetReserveDataCall) -> Self {
            Self::GetReserveData(value)
        }
    }
    impl ::core::convert::From<GetReserveEModeCategoryCall> for IPoolDataProviderCalls {
        fn from(value: GetReserveEModeCategoryCall) -> Self {
            Self::GetReserveEModeCategory(value)
        }
    }
    impl ::core::convert::From<GetReserveTokensAddressesCall>
    for IPoolDataProviderCalls {
        fn from(value: GetReserveTokensAddressesCall) -> Self {
            Self::GetReserveTokensAddresses(value)
        }
    }
    impl ::core::convert::From<GetSiloedBorrowingCall> for IPoolDataProviderCalls {
        fn from(value: GetSiloedBorrowingCall) -> Self {
            Self::GetSiloedBorrowing(value)
        }
    }
    impl ::core::convert::From<GetTotalDebtCall> for IPoolDataProviderCalls {
        fn from(value: GetTotalDebtCall) -> Self {
            Self::GetTotalDebt(value)
        }
    }
    impl ::core::convert::From<GetUnbackedMintCapCall> for IPoolDataProviderCalls {
        fn from(value: GetUnbackedMintCapCall) -> Self {
            Self::GetUnbackedMintCap(value)
        }
    }
    impl ::core::convert::From<GetUserReserveDataCall> for IPoolDataProviderCalls {
        fn from(value: GetUserReserveDataCall) -> Self {
            Self::GetUserReserveData(value)
        }
    }
    ///Container type for all return fields from the `getATokenTotalSupply` function with signature `getATokenTotalSupply(address)` and selector `0x51460e25`
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
    pub struct GetATokenTotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAllATokens` function with signature `getAllATokens()` and selector `0xf561ae41`
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
    pub struct GetAllATokensReturn(pub ::std::vec::Vec<TokenData>);
    ///Container type for all return fields from the `getAllReservesTokens` function with signature `getAllReservesTokens()` and selector `0xb316ff89`
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
    pub struct GetAllReservesTokensReturn(pub ::std::vec::Vec<TokenData>);
    ///Container type for all return fields from the `getDebtCeiling` function with signature `getDebtCeiling(address)` and selector `0x3c798109`
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
    pub struct GetDebtCeilingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getDebtCeilingDecimals` function with signature `getDebtCeilingDecimals()` and selector `0x69b169e1`
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
    pub struct GetDebtCeilingDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFlashLoanEnabled` function with signature `getFlashLoanEnabled(address)` and selector `0xd7ed3ef4`
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
    ///Container type for all return fields from the `getInterestRateStrategyAddress` function with signature `getInterestRateStrategyAddress(address)` and selector `0x6744362a`
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
    pub struct GetInterestRateStrategyAddressReturn {
        pub ir_strategy_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getLiquidationProtocolFee` function with signature `getLiquidationProtocolFee(address)` and selector `0x3cb8a622`
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
    ///Container type for all return fields from the `getPaused` function with signature `getPaused(address)` and selector `0xb55d9904`
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
    pub struct GetPausedReturn {
        pub is_paused: bool,
    }
    ///Container type for all return fields from the `getReserveCaps` function with signature `getReserveCaps(address)` and selector `0x46fbe558`
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
    pub struct GetReserveCapsReturn {
        pub borrow_cap: ::ethers::core::types::U256,
        pub supply_cap: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReserveConfigurationData` function with signature `getReserveConfigurationData(address)` and selector `0x3e150141`
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
    pub struct GetReserveConfigurationDataReturn {
        pub decimals: ::ethers::core::types::U256,
        pub ltv: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub liquidation_bonus: ::ethers::core::types::U256,
        pub reserve_factor: ::ethers::core::types::U256,
        pub usage_as_collateral_enabled: bool,
        pub borrowing_enabled: bool,
        pub stable_borrow_rate_enabled: bool,
        pub is_active: bool,
        pub is_frozen: bool,
    }
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
    pub struct GetReserveDataReturn {
        pub unbacked: ::ethers::core::types::U256,
        pub accrued_to_treasury_scaled: ::ethers::core::types::U256,
        pub total_a_token: ::ethers::core::types::U256,
        pub total_stable_debt: ::ethers::core::types::U256,
        pub total_variable_debt: ::ethers::core::types::U256,
        pub liquidity_rate: ::ethers::core::types::U256,
        pub variable_borrow_rate: ::ethers::core::types::U256,
        pub stable_borrow_rate: ::ethers::core::types::U256,
        pub average_stable_borrow_rate: ::ethers::core::types::U256,
        pub liquidity_index: ::ethers::core::types::U256,
        pub variable_borrow_index: ::ethers::core::types::U256,
        pub last_update_timestamp: u64,
    }
    ///Container type for all return fields from the `getReserveEModeCategory` function with signature `getReserveEModeCategory(address)` and selector `0x163a0f20`
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
    pub struct GetReserveEModeCategoryReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReserveTokensAddresses` function with signature `getReserveTokensAddresses(address)` and selector `0xd2493b6c`
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
    pub struct GetReserveTokensAddressesReturn {
        pub a_token_address: ::ethers::core::types::Address,
        pub stable_debt_token_address: ::ethers::core::types::Address,
        pub variable_debt_token_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getSiloedBorrowing` function with signature `getSiloedBorrowing(address)` and selector `0xfcf40a62`
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
    pub struct GetSiloedBorrowingReturn(pub bool);
    ///Container type for all return fields from the `getTotalDebt` function with signature `getTotalDebt(address)` and selector `0x4d44ac4f`
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
    pub struct GetTotalDebtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUnbackedMintCap` function with signature `getUnbackedMintCap(address)` and selector `0x7ba1ae36`
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
    ///Container type for all return fields from the `getUserReserveData` function with signature `getUserReserveData(address,address)` and selector `0x28dd2d01`
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
    pub struct GetUserReserveDataReturn {
        pub current_a_token_balance: ::ethers::core::types::U256,
        pub current_stable_debt: ::ethers::core::types::U256,
        pub current_variable_debt: ::ethers::core::types::U256,
        pub principal_stable_debt: ::ethers::core::types::U256,
        pub scaled_variable_debt: ::ethers::core::types::U256,
        pub stable_borrow_rate: ::ethers::core::types::U256,
        pub liquidity_rate: ::ethers::core::types::U256,
        pub stable_rate_last_updated: u64,
        pub usage_as_collateral_enabled: bool,
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
}
