pub use aave_protocol_data_provider::*;
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
pub mod aave_protocol_data_provider {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("addressesProvider"),
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
    pub static AAVEPROTOCOLDATAPROVIDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)G8\x03\x80b\0)G\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0xV[`\0` \x82\x84\x03\x12\x15b\0\0YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa(5b\0\x01\x12`\09`\0\x81\x81a\x01+\x01R\x81\x81a\x04\x0E\x01R\x81\x81a\x05\x1E\x01R\x81\x81a\x06\x11\x01R\x81\x81a\n\x86\x01R\x81\x81a\r\xCD\x01R\x81\x81a\x0E\xD4\x01R\x81\x81a\x0F\xDE\x01R\x81\x81a\x11,\x01R\x81\x81a\x12@\x01R\x81\x81a\x14\x0C\x01R\x81\x81a\x15g\x01R\x81\x81a\x16g\x01R\x81\x81a\x17g\x01R\x81\x81a\x1A\xF8\x01R\x81\x81a\x1B\xFB\x01R\x81\x81a\x1D\r\x01R\x81\x81a\x1E\x15\x01Ra \xD1\x01Ra(5`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80cQF\x0E%\x11a\0\xADW\x80c\xB5]\x99\x04\x11a\0qW\x80c\xB5]\x99\x04\x14a\x03{W\x80c\xD2I;l\x14a\x03\x9EW\x80c\xD7\xED>\xF4\x14a\x03\xDBW\x80c\xF5a\xAEA\x14a\x03\xEEW\x80c\xFC\xF4\nb\x14a\x03\xF6W`\0\x80\xFD[\x80cQF\x0E%\x14a\x03&W\x80cgD6*\x14a\x039W\x80ci\xB1i\xE1\x14a\x03LW\x80c{\xA1\xAE6\x14a\x03SW\x80c\xB3\x16\xFF\x89\x14a\x03fW`\0\x80\xFD[\x80c<y\x81\t\x11a\0\xF4W\x80c<y\x81\t\x14a\x02^W\x80c<\xB8\xA6\"\x14a\x02qW\x80c>\x15\x01A\x14a\x02\x84W\x80cF\xFB\xE5X\x14a\x02\xEBW\x80cMD\xACO\x14a\x03\x13W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\x01&W\x80c\x16:\x0F \x14a\x01jW\x80c(\xDD-\x01\x14a\x01\x8BW\x80c5\xEAju\x14a\x01\xEBW[`\0\x80\xFD[a\x01M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01}a\x01x6`\x04a\"\x85V[a\x04\tV[`@Q\x90\x81R` \x01a\x01aV[a\x01\x9Ea\x01\x996`\x04a\"\xA2V[a\x05\rV[`@\x80Q\x99\x8AR` \x8A\x01\x98\x90\x98R\x96\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01Rd\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x83\x01R\x15\x15a\x01\0\x82\x01Ra\x01 \x01a\x01aV[a\x01\xFEa\x01\xF96`\x04a\"\x85V[a\npV[`@\x80Q\x9C\x8DR` \x8D\x01\x9B\x90\x9BR\x99\x8B\x01\x98\x90\x98R``\x8A\x01\x96\x90\x96R`\x80\x89\x01\x94\x90\x94R`\xA0\x88\x01\x92\x90\x92R`\xC0\x87\x01R`\xE0\x86\x01Ra\x01\0\x85\x01Ra\x01 \x84\x01Ra\x01@\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x82\x01Ra\x01\x80\x01a\x01aV[a\x01}a\x02l6`\x04a\"\x85V[a\r\xC6V[a\x01}a\x02\x7F6`\x04a\"\x85V[a\x0E\xCDV[a\x02\x97a\x02\x926`\x04a\"\x85V[a\x0F\xCBV[`@\x80Q\x9A\x8BR` \x8B\x01\x99\x90\x99R\x97\x89\x01\x96\x90\x96R``\x88\x01\x94\x90\x94R`\x80\x87\x01\x92\x90\x92R\x15\x15`\xA0\x86\x01R\x15\x15`\xC0\x85\x01R\x15\x15`\xE0\x84\x01R\x15\x15a\x01\0\x83\x01R\x15\x15a\x01 \x82\x01Ra\x01@\x01a\x01aV[a\x02\xFEa\x02\xF96`\x04a\"\x85V[a\x11$V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01aV[a\x01}a\x03!6`\x04a\"\x85V[a\x12;V[a\x01}a\x0346`\x04a\"\x85V[a\x14\x07V[a\x01Ma\x03G6`\x04a\"\x85V[a\x15bV[`\x02a\x01}V[a\x01}a\x03a6`\x04a\"\x85V[a\x16`V[a\x03na\x17aV[`@Qa\x01a\x91\x90a#7V[a\x03\x8Ea\x03\x896`\x04a\"\x85V[a\x1A\xF1V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[a\x03\xB1a\x03\xAC6`\x04a\"\x85V[a\x1B\xF3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R\x92\x16\x91\x81\x01\x91\x90\x91R``\x01a\x01aV[a\x03\x8Ea\x03\xE96`\x04a\"\x85V[a\x1D\x08V[a\x03na\x1E\x0FV[a\x03\x8Ea\x04\x046`\x04a\"\x85V[a \xCAV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8E\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a$\x96V[\x80Q\x90\x91P`\xA8\x1C`\xFF\x16[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9E\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0B\x91\x90a$\xF0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x91\x91\x90a#\xC6V[`@QcD\x17\xA5\x83`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cD\x17\xA5\x83\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFD\x91\x90a$\x96V[a\x01\0\x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07p\x91\x90a&\x13V[a\x01@\x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9DP\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE3\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9BP\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x082W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08V\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qcc\x1Ao\xD5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9CP\x91\x16\x90c\xC64\xDF\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a&\x13V[a\x01@\x83\x01Q`@Qc\x0E\xD1'\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9AP\x91\x16\x90c\x1D\xA2O>\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t<\x91\x90a&\x13V[\x96P\x81`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x94P\x81a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xE7\x8C\x9B;\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x8E\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCF\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qc\x1Es\x9A\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x98P\x91\x16\x90cy\xCEk\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a&,V[\x93Pa\n_\x82`\xE0\x01Qa\xFF\xFF\x16\x82a!\xCD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x06\x91\x90a#\xC6V[`\x01`\x01`\xA0\x1B\x03\x16c5\xEAju\x8F`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B@\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x82\x91\x90a$\xF0V[\x90P\x80a\x01\xA0\x01Q\x81a\x01\x80\x01Q\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF7\x91\x90a&\x13V[\x83a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C^\x91\x90a&\x13V[\x84a\x01@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC5\x91\x90a&\x13V[\x85`@\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x88a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x90\xF6\xFC\xF2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r;\x91\x90a&\x13V[\x89` \x01Q\x8A``\x01Q\x8B`\xC0\x01Q\x8B`\x01`\x01`\x80\x1B\x03\x16\x9BP\x8A`\x01`\x01`\x80\x1B\x03\x16\x9AP\x86`\x01`\x01`\x80\x1B\x03\x16\x96P\x85`\x01`\x01`\x80\x1B\x03\x16\x95P\x84`\x01`\x01`\x80\x1B\x03\x16\x94P\x82`\x01`\x01`\x80\x1B\x03\x16\x92P\x81`\x01`\x01`\x80\x1B\x03\x16\x91P\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CPP\x91\x93\x95\x97\x99\x9BP\x91\x93\x95\x97\x99\x9BV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EM\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xB9\x91\x90a$\x96V[Q`\xD4\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x92\x91PPV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FT\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC0\x91\x90a$\x96V[Q`\x98\x1Ca\xFF\xFF\x16\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10^\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCA\x91\x90a$\x96V[\x80Q`0\x81\x90\x1C`\xFF\x16\x9CPa\xFF\xFF\x80\x82\x16\x9CP`\x10\x82\x90\x1C\x81\x16\x9BP` \x82\x90\x1C\x81\x16\x9AP`@\x91\x90\x91\x1C\x16\x97P\x90Pa\x11\x04\x81a\"%V[P\x9D\x9F\x9C\x9EP\x9A\x9C\x99\x9B\x98\x9A\x8D\x15\x15\x9A\x90\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x80a\x121\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xAC\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x18\x91\x90a$\x96V[Qd\x0F\xFF\xFF\xFF\xFF`P\x82\x90\x1C\x81\x16\x92`t\x92\x90\x92\x1C\x16\x90V[\x90\x94\x90\x93P\x91PPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC0\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13-\x91\x90a$\xF0V[\x90P\x80a\x01@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x96\x91\x90a&\x13V[\x81a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFD\x91\x90a&\x13V[a\x05\x06\x91\x90a&]V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x8C\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xF9\x91\x90a$\xF0V[\x90P\x80a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x06\x91\x90a&\x13V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE7\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x160W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16T\x91\x90a$\xF0V[a\x01`\x01Q\x93\x92PPPV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE7\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17S\x91\x90a$\x96V[Q`\xB0\x1Cd\x0F\xFF\xFF\xFF\xFF\x16\x90V[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE7\x91\x90a#\xC6V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xD1\x94m\xBC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18Q\x91\x90\x81\x01\x90a&uV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18oWa\x18oa#\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xB5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x8DW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1A\xE9Ws\x9F\x8Fr\xAA\x93\x04\xC8\xB5\x93\xD5U\xF1.\xF6X\x9C\xC3\xA5y\xA2`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x18\xF4Wa\x18\xF4a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x19\x7FW`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b&\xA5\xA9`\xE9\x1B\x81RP\x81R` \x01\x84\x83\x81Q\x81\x10a\x19IWa\x19Ia''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x19oWa\x19oa''V[` \x02` \x01\x01\x81\x90RPa\x1A\xD7V[s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x19\xAFWa\x19\xAFa''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x1A\x04W`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x08\xAA\x89`\xEB\x1B\x81RP\x81R` \x01\x84\x83\x81Q\x81\x10a\x19IWa\x19Ia''V[`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1A!Wa\x1A!a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\x8E\x91\x90\x81\x01\x90a'=V[\x81R` \x01\x84\x83\x81Q\x81\x10a\x1A\xA5Wa\x1A\xA5a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1A\xCBWa\x1A\xCBa''V[` \x02` \x01\x01\x81\x90RP[\x80a\x1A\xE1\x81a'\xD1V[\x91PPa\x18\xBBV[P\x93\x92PPPV[`\0a\x1B\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bx\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE4\x91\x90a$\x96V[a\"%V[\x96\x95PPPPPPV[`\0\x80`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C{\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE8\x91\x90a$\xF0V[a\x01\0\x81\x01Qa\x01 \x82\x01Qa\x01@\x90\x92\x01Q\x90\x97\x91\x96P\x94P\x92PPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x8D\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xF9\x91\x90a$\x96V[\x90Pa\x05\x06\x81Qg\x80\0\0\0\0\0\0\0\x16\x15\x15\x90V[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x95\x91\x90a#\xC6V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xD1\x94m\xBC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xFF\x91\x90\x81\x01\x90a&uV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x1DWa\x1F\x1Da#\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FcW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1F;W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1A\xE9W`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c5\xEAju\x85\x84\x81Q\x81\x10a\x1F\x95Wa\x1F\x95a''V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xC8\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \n\x91\x90a$\xF0V[\x90P`@Q\x80`@\x01`@R\x80\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x82\x91\x90\x81\x01\x90a'=V[\x81R` \x01\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x83\x83\x81Q\x81\x10a \xABWa \xABa''V[` \x02` \x01\x01\x81\x90RPP\x80\x80a \xC2\x90a'\xD1V[\x91PPa\x1FiV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Q\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBD\x91\x90a$\x96V[Qg@\0\0\0\0\0\0\0\x16\x15\x15\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\r\xCD`\xF2\x1B` \x82\x01R`\0\x90`\x80\x83\x10a\"\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\"\n\x91\x90a'\xECV[`@Q\x80\x91\x03\x90\xFD[PP\x90Q`\x01\x91\x82\x1B\x82\x01\x1C\x16\x15\x15\x90V[Qg\x01\0\0\0\0\0\0\0\x81\x16\x15\x15\x91g\x02\0\0\0\0\0\0\0\x82\x16\x15\x15\x91g\x04\0\0\0\0\0\0\0\x81\x16\x15\x15\x91g\x08\0\0\0\0\0\0\0\x82\x16\x15\x15\x91g\x10\0\0\0\0\0\0\0\x16\x15\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x82W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\"\x97W`\0\x80\xFD[\x815a\x05\x06\x81a\"mV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xB5W`\0\x80\xFD[\x825a\"\xC0\x81a\"mV[\x91P` \x83\x015a\"\xD0\x81a\"mV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\"\xF6W\x81\x81\x01Q\x83\x82\x01R` \x01a\"\xDEV[\x83\x81\x11\x15a#\x05W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra##\x81` \x86\x01` \x86\x01a\"\xDBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a#\xA8W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x87\x85Ra#\x82\x88\x86\x01\x82a#\x0BV[\x91\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x94\x89\x01\x94\x90\x94R\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a#^V[P\x90\x98\x97PPPPPPPPV[\x80Qa#\xC1\x81a\"mV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\xD8W`\0\x80\xFD[\x81Qa\x05\x06\x81a\"mV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\x1DWa$\x1Da#\xE3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$LWa$La#\xE3V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$fW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a$\x89Wa$\x89a#\xE3V[`@R\x91Q\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xA8W`\0\x80\xFD[a\x05\x06\x83\x83a$TV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[`\0a\x01\xE0\x82\x84\x03\x12\x15a%\x03W`\0\x80\xFD[a%\x0Ba#\xF9V[a%\x15\x84\x84a$TV[\x81Ra%#` \x84\x01a$\xB2V[` \x82\x01Ra%4`@\x84\x01a$\xB2V[`@\x82\x01Ra%E``\x84\x01a$\xB2V[``\x82\x01Ra%V`\x80\x84\x01a$\xB2V[`\x80\x82\x01Ra%g`\xA0\x84\x01a$\xB2V[`\xA0\x82\x01Ra%x`\xC0\x84\x01a$\xC9V[`\xC0\x82\x01Ra%\x89`\xE0\x84\x01a$\xDEV[`\xE0\x82\x01Ra\x01\0a%\x9C\x81\x85\x01a#\xB6V[\x90\x82\x01Ra\x01 a%\xAE\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01@a%\xC0\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01`a%\xD2\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01\x80a%\xE4\x84\x82\x01a$\xB2V[\x90\x82\x01Ra\x01\xA0a%\xF6\x84\x82\x01a$\xB2V[\x90\x82\x01Ra\x01\xC0a&\x08\x84\x82\x01a$\xB2V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a&%W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a&>W`\0\x80\xFD[a\x05\x06\x82a$\xC9V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a&pWa&pa&GV[P\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a&\x88W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xA0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a&\xB4W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a&\xC6Wa&\xC6a#\xE3V[\x80`\x05\x1B\x91Pa&\xD7\x84\x83\x01a$#V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a&\xF1W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a'\x1BW\x84Q\x92Pa'\x0B\x83a\"mV[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a&\xF6V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a'OW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'gW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a'{W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a'\x8DWa'\x8Da#\xE3V[a'\xA0`\x1F\x82\x01`\x1F\x19\x16` \x01a$#V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a'\xB7W`\0\x80\xFD[a'\xC8\x81` \x84\x01` \x86\x01a\"\xDBV[P\x94\x93PPPPV[`\0`\0\x19\x82\x14\x15a'\xE5Wa'\xE5a&GV[P`\x01\x01\x90V[` \x81R`\0a\x05\x06` \x83\x01\x84a#\x0BV\xFE\xA2dipfsX\"\x12 \xF1ze\x8C~\xC5\xB0\xF9\t\xE0\xF6H\x1F\x8D\xE4\x16\xF88(vhU6Kl[\xBD\xDC\xE7\xC1\xE8\xDCdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static AAVEPROTOCOLDATAPROVIDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80cQF\x0E%\x11a\0\xADW\x80c\xB5]\x99\x04\x11a\0qW\x80c\xB5]\x99\x04\x14a\x03{W\x80c\xD2I;l\x14a\x03\x9EW\x80c\xD7\xED>\xF4\x14a\x03\xDBW\x80c\xF5a\xAEA\x14a\x03\xEEW\x80c\xFC\xF4\nb\x14a\x03\xF6W`\0\x80\xFD[\x80cQF\x0E%\x14a\x03&W\x80cgD6*\x14a\x039W\x80ci\xB1i\xE1\x14a\x03LW\x80c{\xA1\xAE6\x14a\x03SW\x80c\xB3\x16\xFF\x89\x14a\x03fW`\0\x80\xFD[\x80c<y\x81\t\x11a\0\xF4W\x80c<y\x81\t\x14a\x02^W\x80c<\xB8\xA6\"\x14a\x02qW\x80c>\x15\x01A\x14a\x02\x84W\x80cF\xFB\xE5X\x14a\x02\xEBW\x80cMD\xACO\x14a\x03\x13W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\x01&W\x80c\x16:\x0F \x14a\x01jW\x80c(\xDD-\x01\x14a\x01\x8BW\x80c5\xEAju\x14a\x01\xEBW[`\0\x80\xFD[a\x01M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01}a\x01x6`\x04a\"\x85V[a\x04\tV[`@Q\x90\x81R` \x01a\x01aV[a\x01\x9Ea\x01\x996`\x04a\"\xA2V[a\x05\rV[`@\x80Q\x99\x8AR` \x8A\x01\x98\x90\x98R\x96\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01Rd\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x83\x01R\x15\x15a\x01\0\x82\x01Ra\x01 \x01a\x01aV[a\x01\xFEa\x01\xF96`\x04a\"\x85V[a\npV[`@\x80Q\x9C\x8DR` \x8D\x01\x9B\x90\x9BR\x99\x8B\x01\x98\x90\x98R``\x8A\x01\x96\x90\x96R`\x80\x89\x01\x94\x90\x94R`\xA0\x88\x01\x92\x90\x92R`\xC0\x87\x01R`\xE0\x86\x01Ra\x01\0\x85\x01Ra\x01 \x84\x01Ra\x01@\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x16a\x01`\x82\x01Ra\x01\x80\x01a\x01aV[a\x01}a\x02l6`\x04a\"\x85V[a\r\xC6V[a\x01}a\x02\x7F6`\x04a\"\x85V[a\x0E\xCDV[a\x02\x97a\x02\x926`\x04a\"\x85V[a\x0F\xCBV[`@\x80Q\x9A\x8BR` \x8B\x01\x99\x90\x99R\x97\x89\x01\x96\x90\x96R``\x88\x01\x94\x90\x94R`\x80\x87\x01\x92\x90\x92R\x15\x15`\xA0\x86\x01R\x15\x15`\xC0\x85\x01R\x15\x15`\xE0\x84\x01R\x15\x15a\x01\0\x83\x01R\x15\x15a\x01 \x82\x01Ra\x01@\x01a\x01aV[a\x02\xFEa\x02\xF96`\x04a\"\x85V[a\x11$V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01aV[a\x01}a\x03!6`\x04a\"\x85V[a\x12;V[a\x01}a\x0346`\x04a\"\x85V[a\x14\x07V[a\x01Ma\x03G6`\x04a\"\x85V[a\x15bV[`\x02a\x01}V[a\x01}a\x03a6`\x04a\"\x85V[a\x16`V[a\x03na\x17aV[`@Qa\x01a\x91\x90a#7V[a\x03\x8Ea\x03\x896`\x04a\"\x85V[a\x1A\xF1V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[a\x03\xB1a\x03\xAC6`\x04a\"\x85V[a\x1B\xF3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R\x92\x16\x91\x81\x01\x91\x90\x91R``\x01a\x01aV[a\x03\x8Ea\x03\xE96`\x04a\"\x85V[a\x1D\x08V[a\x03na\x1E\x0FV[a\x03\x8Ea\x04\x046`\x04a\"\x85V[a \xCAV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8E\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a$\x96V[\x80Q\x90\x91P`\xA8\x1C`\xFF\x16[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9E\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0B\x91\x90a$\xF0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x91\x91\x90a#\xC6V[`@QcD\x17\xA5\x83`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cD\x17\xA5\x83\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFD\x91\x90a$\x96V[a\x01\0\x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07p\x91\x90a&\x13V[a\x01@\x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9DP\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE3\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9BP\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x082W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08V\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qcc\x1Ao\xD5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9CP\x91\x16\x90c\xC64\xDF\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a&\x13V[a\x01@\x83\x01Q`@Qc\x0E\xD1'\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x9AP\x91\x16\x90c\x1D\xA2O>\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t<\x91\x90a&\x13V[\x96P\x81`@\x01Q`\x01`\x01`\x80\x1B\x03\x16\x94P\x81a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xE7\x8C\x9B;\x8D`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x8E\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCF\x91\x90a&\x13V[a\x01 \x83\x01Q`@Qc\x1Es\x9A\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x81\x16`\x04\x83\x01R\x92\x98P\x91\x16\x90cy\xCEk\x8C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a&,V[\x93Pa\n_\x82`\xE0\x01Qa\xFF\xFF\x16\x82a!\xCD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x92PPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x06\x91\x90a#\xC6V[`\x01`\x01`\xA0\x1B\x03\x16c5\xEAju\x8F`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B@\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x82\x91\x90a$\xF0V[\x90P\x80a\x01\xA0\x01Q\x81a\x01\x80\x01Q\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF7\x91\x90a&\x13V[\x83a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C^\x91\x90a&\x13V[\x84a\x01@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC5\x91\x90a&\x13V[\x85`@\x01Q\x86`\x80\x01Q\x87`\xA0\x01Q\x88a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x90\xF6\xFC\xF2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r;\x91\x90a&\x13V[\x89` \x01Q\x8A``\x01Q\x8B`\xC0\x01Q\x8B`\x01`\x01`\x80\x1B\x03\x16\x9BP\x8A`\x01`\x01`\x80\x1B\x03\x16\x9AP\x86`\x01`\x01`\x80\x1B\x03\x16\x96P\x85`\x01`\x01`\x80\x1B\x03\x16\x95P\x84`\x01`\x01`\x80\x1B\x03\x16\x94P\x82`\x01`\x01`\x80\x1B\x03\x16\x92P\x81`\x01`\x01`\x80\x1B\x03\x16\x91P\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CP\x9CPP\x91\x93\x95\x97\x99\x9BP\x91\x93\x95\x97\x99\x9BV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EM\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xB9\x91\x90a$\x96V[Q`\xD4\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x92\x91PPV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FT\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC0\x91\x90a$\x96V[Q`\x98\x1Ca\xFF\xFF\x16\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10^\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8E\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xCA\x91\x90a$\x96V[\x80Q`0\x81\x90\x1C`\xFF\x16\x9CPa\xFF\xFF\x80\x82\x16\x9CP`\x10\x82\x90\x1C\x81\x16\x9BP` \x82\x90\x1C\x81\x16\x9AP`@\x91\x90\x91\x1C\x16\x97P\x90Pa\x11\x04\x81a\"%V[P\x9D\x9F\x9C\x9EP\x9A\x9C\x99\x9B\x98\x9A\x8D\x15\x15\x9A\x90\x99\x90\x98P\x91\x96P\x94P\x92PPPV[`\0\x80a\x121\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xAC\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x18\x91\x90a$\x96V[Qd\x0F\xFF\xFF\xFF\xFF`P\x82\x90\x1C\x81\x16\x92`t\x92\x90\x92\x1C\x16\x90V[\x90\x94\x90\x93P\x91PPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC0\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13-\x91\x90a$\xF0V[\x90P\x80a\x01@\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x96\x91\x90a&\x13V[\x81a\x01 \x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xFD\x91\x90a&\x13V[a\x05\x06\x91\x90a&]V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x8C\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xF9\x91\x90a$\xF0V[\x90P\x80a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x06\x91\x90a&\x13V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE7\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x160W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16T\x91\x90a$\xF0V[a\x01`\x01Q\x93\x92PPPV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE7\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17S\x91\x90a$\x96V[Q`\xB0\x1Cd\x0F\xFF\xFF\xFF\xFF\x16\x90V[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE7\x91\x90a#\xC6V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xD1\x94m\xBC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18Q\x91\x90\x81\x01\x90a&uV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18oWa\x18oa#\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xB5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x8DW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1A\xE9Ws\x9F\x8Fr\xAA\x93\x04\xC8\xB5\x93\xD5U\xF1.\xF6X\x9C\xC3\xA5y\xA2`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x18\xF4Wa\x18\xF4a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x19\x7FW`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b&\xA5\xA9`\xE9\x1B\x81RP\x81R` \x01\x84\x83\x81Q\x81\x10a\x19IWa\x19Ia''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x19oWa\x19oa''V[` \x02` \x01\x01\x81\x90RPa\x1A\xD7V[s\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x19\xAFWa\x19\xAFa''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x1A\x04W`@Q\x80`@\x01`@R\x80`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x08\xAA\x89`\xEB\x1B\x81RP\x81R` \x01\x84\x83\x81Q\x81\x10a\x19IWa\x19Ia''V[`@Q\x80`@\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1A!Wa\x1A!a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\x8E\x91\x90\x81\x01\x90a'=V[\x81R` \x01\x84\x83\x81Q\x81\x10a\x1A\xA5Wa\x1A\xA5a''V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1A\xCBWa\x1A\xCBa''V[` \x02` \x01\x01\x81\x90RP[\x80a\x1A\xE1\x81a'\xD1V[\x91PPa\x18\xBBV[P\x93\x92PPPV[`\0a\x1B\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bx\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xE4\x91\x90a$\x96V[a\"%V[\x96\x95PPPPPPV[`\0\x80`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C{\x91\x90a#\xC6V[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE8\x91\x90a$\xF0V[a\x01\0\x81\x01Qa\x01 \x82\x01Qa\x01@\x90\x92\x01Q\x90\x97\x91\x96P\x94P\x92PPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x8D\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xF9\x91\x90a$\x96V[\x90Pa\x05\x06\x81Qg\x80\0\0\0\0\0\0\0\x16\x15\x15\x90V[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x95\x91\x90a#\xC6V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xD1\x94m\xBC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xFF\x91\x90\x81\x01\x90a&uV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F\x1DWa\x1F\x1Da#\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FcW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1F;W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1A\xE9W`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c5\xEAju\x85\x84\x81Q\x81\x10a\x1F\x95Wa\x1F\x95a''V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xC8\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \n\x91\x90a$\xF0V[\x90P`@Q\x80`@\x01`@R\x80\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x82\x91\x90\x81\x01\x90a'=V[\x81R` \x01\x82a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x83\x83\x81Q\x81\x10a \xABWa \xABa''V[` \x02` \x01\x01\x81\x90RPP\x80\x80a \xC2\x90a'\xD1V[\x91PPa\x1FiV[`\0a\x0E\xC7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!Q\x91\x90a#\xC6V[`@Qc\xC4K\x11\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\xC4K\x11\xF7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBD\x91\x90a$\x96V[Qg@\0\0\0\0\0\0\0\x16\x15\x15\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\r\xCD`\xF2\x1B` \x82\x01R`\0\x90`\x80\x83\x10a\"\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\"\n\x91\x90a'\xECV[`@Q\x80\x91\x03\x90\xFD[PP\x90Q`\x01\x91\x82\x1B\x82\x01\x1C\x16\x15\x15\x90V[Qg\x01\0\0\0\0\0\0\0\x81\x16\x15\x15\x91g\x02\0\0\0\0\0\0\0\x82\x16\x15\x15\x91g\x04\0\0\0\0\0\0\0\x81\x16\x15\x15\x91g\x08\0\0\0\0\0\0\0\x82\x16\x15\x15\x91g\x10\0\0\0\0\0\0\0\x16\x15\x15\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x82W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\"\x97W`\0\x80\xFD[\x815a\x05\x06\x81a\"mV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xB5W`\0\x80\xFD[\x825a\"\xC0\x81a\"mV[\x91P` \x83\x015a\"\xD0\x81a\"mV[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\"\xF6W\x81\x81\x01Q\x83\x82\x01R` \x01a\"\xDEV[\x83\x81\x11\x15a#\x05W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra##\x81` \x86\x01` \x86\x01a\"\xDBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a#\xA8W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x87\x85Ra#\x82\x88\x86\x01\x82a#\x0BV[\x91\x89\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x94\x89\x01\x94\x90\x94R\x94\x87\x01\x94\x92P\x90\x86\x01\x90`\x01\x01a#^V[P\x90\x98\x97PPPPPPPPV[\x80Qa#\xC1\x81a\"mV[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a#\xD8W`\0\x80\xFD[\x81Qa\x05\x06\x81a\"mV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\x1DWa$\x1Da#\xE3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$LWa$La#\xE3V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$fW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a$\x89Wa$\x89a#\xE3V[`@R\x91Q\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a$\xA8W`\0\x80\xFD[a\x05\x06\x83\x83a$TV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a#\xC1W`\0\x80\xFD[`\0a\x01\xE0\x82\x84\x03\x12\x15a%\x03W`\0\x80\xFD[a%\x0Ba#\xF9V[a%\x15\x84\x84a$TV[\x81Ra%#` \x84\x01a$\xB2V[` \x82\x01Ra%4`@\x84\x01a$\xB2V[`@\x82\x01Ra%E``\x84\x01a$\xB2V[``\x82\x01Ra%V`\x80\x84\x01a$\xB2V[`\x80\x82\x01Ra%g`\xA0\x84\x01a$\xB2V[`\xA0\x82\x01Ra%x`\xC0\x84\x01a$\xC9V[`\xC0\x82\x01Ra%\x89`\xE0\x84\x01a$\xDEV[`\xE0\x82\x01Ra\x01\0a%\x9C\x81\x85\x01a#\xB6V[\x90\x82\x01Ra\x01 a%\xAE\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01@a%\xC0\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01`a%\xD2\x84\x82\x01a#\xB6V[\x90\x82\x01Ra\x01\x80a%\xE4\x84\x82\x01a$\xB2V[\x90\x82\x01Ra\x01\xA0a%\xF6\x84\x82\x01a$\xB2V[\x90\x82\x01Ra\x01\xC0a&\x08\x84\x82\x01a$\xB2V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a&%W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a&>W`\0\x80\xFD[a\x05\x06\x82a$\xC9V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a&pWa&pa&GV[P\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a&\x88W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xA0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a&\xB4W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a&\xC6Wa&\xC6a#\xE3V[\x80`\x05\x1B\x91Pa&\xD7\x84\x83\x01a$#V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15a&\xF1W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a'\x1BW\x84Q\x92Pa'\x0B\x83a\"mV[\x82\x82R\x93\x85\x01\x93\x90\x85\x01\x90a&\xF6V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a'OW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'gW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a'{W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a'\x8DWa'\x8Da#\xE3V[a'\xA0`\x1F\x82\x01`\x1F\x19\x16` \x01a$#V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a'\xB7W`\0\x80\xFD[a'\xC8\x81` \x84\x01` \x86\x01a\"\xDBV[P\x94\x93PPPPV[`\0`\0\x19\x82\x14\x15a'\xE5Wa'\xE5a&GV[P`\x01\x01\x90V[` \x81R`\0a\x05\x06` \x83\x01\x84a#\x0BV\xFE\xA2dipfsX\"\x12 \xF1ze\x8C~\xC5\xB0\xF9\t\xE0\xF6H\x1F\x8D\xE4\x16\xF88(vhU6Kl[\xBD\xDC\xE7\xC1\xE8\xDCdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEPROTOCOLDATAPROVIDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveProtocolDataProvider<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveProtocolDataProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveProtocolDataProvider<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveProtocolDataProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveProtocolDataProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveProtocolDataProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveProtocolDataProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEPROTOCOLDATAPROVIDER_ABI.clone(),
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
                AAVEPROTOCOLDATAPROVIDER_ABI.clone(),
                AAVEPROTOCOLDATAPROVIDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    for AaveProtocolDataProvider<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    pub enum AaveProtocolDataProviderCalls {
        AddressesProvider(AddressesProviderCall),
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
    impl ::ethers::core::abi::AbiDecode for AaveProtocolDataProviderCalls {
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
    impl ::ethers::core::abi::AbiEncode for AaveProtocolDataProviderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for AaveProtocolDataProviderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AddressesProviderCall> for AaveProtocolDataProviderCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<GetATokenTotalSupplyCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetATokenTotalSupplyCall) -> Self {
            Self::GetATokenTotalSupply(value)
        }
    }
    impl ::core::convert::From<GetAllATokensCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetAllATokensCall) -> Self {
            Self::GetAllATokens(value)
        }
    }
    impl ::core::convert::From<GetAllReservesTokensCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetAllReservesTokensCall) -> Self {
            Self::GetAllReservesTokens(value)
        }
    }
    impl ::core::convert::From<GetDebtCeilingCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetDebtCeilingCall) -> Self {
            Self::GetDebtCeiling(value)
        }
    }
    impl ::core::convert::From<GetDebtCeilingDecimalsCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetDebtCeilingDecimalsCall) -> Self {
            Self::GetDebtCeilingDecimals(value)
        }
    }
    impl ::core::convert::From<GetFlashLoanEnabledCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetFlashLoanEnabledCall) -> Self {
            Self::GetFlashLoanEnabled(value)
        }
    }
    impl ::core::convert::From<GetInterestRateStrategyAddressCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetInterestRateStrategyAddressCall) -> Self {
            Self::GetInterestRateStrategyAddress(value)
        }
    }
    impl ::core::convert::From<GetLiquidationProtocolFeeCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetLiquidationProtocolFeeCall) -> Self {
            Self::GetLiquidationProtocolFee(value)
        }
    }
    impl ::core::convert::From<GetPausedCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetPausedCall) -> Self {
            Self::GetPaused(value)
        }
    }
    impl ::core::convert::From<GetReserveCapsCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetReserveCapsCall) -> Self {
            Self::GetReserveCaps(value)
        }
    }
    impl ::core::convert::From<GetReserveConfigurationDataCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetReserveConfigurationDataCall) -> Self {
            Self::GetReserveConfigurationData(value)
        }
    }
    impl ::core::convert::From<GetReserveDataCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetReserveDataCall) -> Self {
            Self::GetReserveData(value)
        }
    }
    impl ::core::convert::From<GetReserveEModeCategoryCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetReserveEModeCategoryCall) -> Self {
            Self::GetReserveEModeCategory(value)
        }
    }
    impl ::core::convert::From<GetReserveTokensAddressesCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetReserveTokensAddressesCall) -> Self {
            Self::GetReserveTokensAddresses(value)
        }
    }
    impl ::core::convert::From<GetSiloedBorrowingCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetSiloedBorrowingCall) -> Self {
            Self::GetSiloedBorrowing(value)
        }
    }
    impl ::core::convert::From<GetTotalDebtCall> for AaveProtocolDataProviderCalls {
        fn from(value: GetTotalDebtCall) -> Self {
            Self::GetTotalDebt(value)
        }
    }
    impl ::core::convert::From<GetUnbackedMintCapCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetUnbackedMintCapCall) -> Self {
            Self::GetUnbackedMintCap(value)
        }
    }
    impl ::core::convert::From<GetUserReserveDataCall>
    for AaveProtocolDataProviderCalls {
        fn from(value: GetUserReserveDataCall) -> Self {
            Self::GetUserReserveData(value)
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
}
