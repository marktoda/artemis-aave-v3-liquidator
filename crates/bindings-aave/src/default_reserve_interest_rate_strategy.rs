pub use default_reserve_interest_rate_strategy::*;
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
pub mod default_reserve_interest_rate_strategy {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("optimalUsageRatio"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("baseVariableBorrowRate"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("variableRateSlope1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("variableRateSlope2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("stableRateSlope1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("stableRateSlope2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("baseStableRateOffset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("stableRateExcessOffset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "optimalStableToTotalDebtRatio",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
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
                    ::std::borrow::ToOwned::to_owned("MAX_EXCESS_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_EXCESS_USAGE_RATIO",
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
                    ::std::borrow::ToOwned::to_owned(
                        "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
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
                    ::std::borrow::ToOwned::to_owned("OPTIMAL_USAGE_RATIO"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPTIMAL_USAGE_RATIO",
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
                    ::std::borrow::ToOwned::to_owned("calculateInterestRates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateInterestRates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DataTypes.CalculateInterestRatesParams",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("getBaseStableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBaseStableBorrowRate",
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
                    ::std::borrow::ToOwned::to_owned("getBaseVariableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBaseVariableBorrowRate",
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
                    ::std::borrow::ToOwned::to_owned("getMaxVariableBorrowRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMaxVariableBorrowRate",
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
                    ::std::borrow::ToOwned::to_owned("getStableRateExcessOffset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateExcessOffset",
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
                    ::std::borrow::ToOwned::to_owned("getStableRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateSlope1",
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
                    ::std::borrow::ToOwned::to_owned("getStableRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStableRateSlope2",
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
                    ::std::borrow::ToOwned::to_owned("getVariableRateSlope1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVariableRateSlope1",
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
                    ::std::borrow::ToOwned::to_owned("getVariableRateSlope2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVariableRateSlope2",
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEFAULTRESERVEINTERESTRATESTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x02\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x0E\xD08\x03\x80a\x0E\xD0\x839\x81\x01`@\x81\x90Ra\x000\x91a\x01DV[\x88k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a83`\xF0\x1B\x81RP\x90a\0\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0y\x91\x90a\x01\xCFV[`@Q\x80\x91\x03\x90\xFD[P\x80k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x0E\r`\xF2\x1B\x81RP\x90a\0\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0y\x91\x90a\x01\xCFV[P`\x80\x89\x90Ra\0\xE8\x89k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x02$V[`\xC0R`\xA0\x81\x90Ra\x01\x06\x81k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\x02$V[`\xE0RP`\x01`\x01`\xA0\x1B\x03\x90\x98\x16a\x01\0Ra\x01 \x95\x90\x95Ra\x01@\x93\x90\x93Ra\x01`\x91\x90\x91Ra\x01\x80Ra\x01\xA0Ra\x01\xC0RPa\x01\xE0Ra\x02IV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\x01dW`\0\x80\xFD[\x8AQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01{W`\0\x80\xFD[\x80\x9APP` \x8B\x01Q\x98P`@\x8B\x01Q\x97P``\x8B\x01Q\x96P`\x80\x8B\x01Q\x95P`\xA0\x8B\x01Q\x94P`\xC0\x8B\x01Q\x93P`\xE0\x8B\x01Q\x92Pa\x01\0\x8B\x01Q\x91Pa\x01 \x8B\x01Q\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x01\xFCW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\xE0V[\x81\x81\x11\x15a\x02\x0EW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x82\x82\x10\x15a\x02DWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0Qa\x01\xE0Qa\x0Bka\x03e`\09`\0\x81\x81a\x02d\x01Ra\x07\xEE\x01R`\0a\x08\x93\x01R`\0\x81\x81a\x01e\x01Ra\x05\xB9\x01R`\0\x81\x81a\x02\x8A\x01R\x81\x81a\x05\xE4\x01Ra\x06\xB9\x01R`\0\x81\x81a\x02\xB0\x01R\x81\x81a\x02\xFF\x01Ra\x06!\x01R`\0\x81\x81a\x015\x01R\x81\x81a\x03#\x01R\x81\x81a\x06L\x01R\x81\x81a\x07+\x01Ra\x08\xB4\x01R`\0\x81\x81a\x01\x8B\x01R\x81\x81a\x03D\x01Ra\x03\xED\x01R`\0`\xF4\x01R`\0\x81\x81a\x02\xD9\x01Ra\x07\x98\x01R`\0\x81\x81a\x028\x01Ra\x05]\x01R`\0\x81\x81a\x01\xDB\x01R\x81\x81a\x07g\x01Ra\x07\xB9\x01R`\0\x81\x81a\x01\xB4\x01R\x81\x81a\x05,\x01R\x81\x81a\x05~\x01R\x81\x81a\x06\x90\x01Ra\x07\x05\x01Ra\x0Bk`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xA5\x89\x87\t\x11a\0\x8CW\x80c\xBCbi\x08\x11a\0fW\x80c\xBCbi\x08\x14a\x02bW\x80c\xD5\xCDs\x91\x14a\x02\x88W\x80c\xF4 $\t\x14a\x02\xAEW\x80c\xFE_\xD6\x98\x14a\x02\xD4W`\0\x80\xFD[\x80c\xA5\x89\x87\t\x14a\x02\x05W\x80c\xA9\xC6\"\xF8\x14a\x023W\x80c\xAC\xD7\x86\x86\x14a\x02ZW`\0\x80\xFD[\x80c4v,\xA5\x11a\0\xC8W\x80c4v,\xA5\x14a\x01\x89W\x80cT\xC3e\xC6\x14a\x01\xAFW\x80co\xB9%\x89\x14a\x01\xD6W\x80c\x80\x03\x1E7\x14a\x01\xFDW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\xEFW\x80c\x0B4)\xA2\x14a\x013W\x80c\x14\xE3-\xA4\x14a\x01cW[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01a\x01*V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ua\x02\xFBV[a\x02\x18a\x02\x136`\x04a\nRV[a\x03wV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01*V[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ua\x08\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[a\x03r\x91\x90a\n\xEDV[\x90P\x90V[`\0\x80`\0a\x03\xCB`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x84`\x80\x01Q\x85``\x01Qa\x03\xDF\x91\x90a\n\xEDV[` \x82\x01R`\0`\x80\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01Ra\x04\x19a\x08\x8CV[``\x82\x01R` \x81\x01Q\x15a\x05*W` \x81\x01Q``\x86\x01Qa\x04;\x91a\x08\xD8V[`\xE0\x80\x83\x01\x91\x90\x91R`@\x80\x87\x01Q` \x88\x01Q\x92\x88\x01Qa\x01\0\x89\x01Q\x92Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x93\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC4\x91\x90a\x0B\x05V[a\x04\xCE\x91\x90a\n\xEDV[a\x04\xD8\x91\x90a\x0B\x1EV[\x80\x82R` \x82\x01Qa\x04\xE9\x91a\n\xEDV[a\x01\0\x82\x01\x81\x90R` \x82\x01Qa\x04\xFF\x91a\x08\xD8V[`\xA0\x82\x01R\x84Qa\x01\0\x82\x01Qa\x05$\x91a\x05\x19\x91a\n\xEDV[` \x83\x01Q\x90a\x08\xD8V[`\xC0\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\xA0\x01Q\x11\x15a\x06\x8BW`\0a\x05\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xA0\x01Qa\x05\xAC\x91\x90a\x0B\x1EV[\x90a\x08\xD8V[\x90Pa\x05\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[a\x06\x08\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[\x82``\x01\x81\x81Qa\x06\x19\x91\x90a\n\xEDV[\x90RPa\x06F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[a\x06p\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[\x82`@\x01\x81\x81Qa\x06\x81\x91\x90a\n\xEDV[\x90RPa\x07e\x90PV[a\x06\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xAC\x83`\xA0\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x17\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81``\x01\x81\x81Qa\x06\xF7\x91\x90a\n\xEDV[\x90RP`\xA0\x81\x01Qa\x07P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x05\xAC\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\t\x17V[\x81`@\x01\x81\x81Qa\x07a\x91\x90a\n\xEDV[\x90RP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\xE0\x01Q\x11\x15a\x08)W`\0a\x07\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xE0\x01Qa\x05\xAC\x91\x90a\x0B\x1EV[\x90Pa\x08\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[\x82``\x01\x81\x81Qa\x08$\x91\x90a\n\xEDV[\x90RPP[a\x08n\x85`\xC0\x01Qa'\x10a\x08>\x91\x90a\x0B\x1EV[a\x08h\x83`\xC0\x01Qa\x08b\x89``\x01Q\x8A`\x80\x01Q\x87`@\x01Q\x8C`\xA0\x01Qa\t[V[\x90a\t\x17V[\x90a\t\xC2V[`\x80\x82\x01\x81\x90R``\x82\x01Q`@\x90\x92\x01Q\x90\x96\x91\x95P\x93P\x91PPV[`\0a\x03r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x08\xFCW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\t9W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x80a\th\x85\x87a\n\xEDV[\x90P\x80a\tyW`\0\x91PPa\t\xBAV[`\0a\t\x88\x85a\x08b\x88a\t\xE8V[\x90P`\0a\t\x99\x85a\x08b\x8Aa\t\xE8V[\x90P`\0a\t\xB3a\t\xA9\x85a\t\xE8V[a\x05\xAC\x84\x86a\n\xEDV[\x94PPPPP[\x94\x93PPPPV[`\0\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a\t\xDAW`\0\x80\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\t\xFEW`\0\x80\xFD[\x91\x90PV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFEW`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a\neW`\0\x80\xFD[a\nma\n\x03V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra\n\xB9`\xE0\x84\x01a\n;V[`\xE0\x82\x01Ra\x01\0a\n\xCC\x81\x85\x01a\n;V[\x90\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0B\0Wa\x0B\0a\n\xD7V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x0B0Wa\x0B0a\n\xD7V[P\x03\x90V\xFE\xA2dipfsX\"\x12 \xE6*%1\x916,\xA1\\Bh\xDA\x9A\xA0\xFE\xA3#\xF3\x95\xE7\xED\xF9\xA0\xA1\x96\xCD\x90\xE4\x8C\xB0\xB1\x8DdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static DEFAULTRESERVEINTERESTRATESTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xA5\x89\x87\t\x11a\0\x8CW\x80c\xBCbi\x08\x11a\0fW\x80c\xBCbi\x08\x14a\x02bW\x80c\xD5\xCDs\x91\x14a\x02\x88W\x80c\xF4 $\t\x14a\x02\xAEW\x80c\xFE_\xD6\x98\x14a\x02\xD4W`\0\x80\xFD[\x80c\xA5\x89\x87\t\x14a\x02\x05W\x80c\xA9\xC6\"\xF8\x14a\x023W\x80c\xAC\xD7\x86\x86\x14a\x02ZW`\0\x80\xFD[\x80c4v,\xA5\x11a\0\xC8W\x80c4v,\xA5\x14a\x01\x89W\x80cT\xC3e\xC6\x14a\x01\xAFW\x80co\xB9%\x89\x14a\x01\xD6W\x80c\x80\x03\x1E7\x14a\x01\xFDW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\xEFW\x80c\x0B4)\xA2\x14a\x013W\x80c\x14\xE3-\xA4\x14a\x01cW[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q\x90\x81R` \x01a\x01*V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ua\x02\xFBV[a\x02\x18a\x02\x136`\x04a\nRV[a\x03wV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01*V[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ua\x08\x8CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01UV[a\x01U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x03h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[a\x03r\x91\x90a\n\xEDV[\x90P\x90V[`\0\x80`\0a\x03\xCB`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x84`\x80\x01Q\x85``\x01Qa\x03\xDF\x91\x90a\n\xEDV[` \x82\x01R`\0`\x80\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01Ra\x04\x19a\x08\x8CV[``\x82\x01R` \x81\x01Q\x15a\x05*W` \x81\x01Q``\x86\x01Qa\x04;\x91a\x08\xD8V[`\xE0\x80\x83\x01\x91\x90\x91R`@\x80\x87\x01Q` \x88\x01Q\x92\x88\x01Qa\x01\0\x89\x01Q\x92Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x93\x92\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC4\x91\x90a\x0B\x05V[a\x04\xCE\x91\x90a\n\xEDV[a\x04\xD8\x91\x90a\x0B\x1EV[\x80\x82R` \x82\x01Qa\x04\xE9\x91a\n\xEDV[a\x01\0\x82\x01\x81\x90R` \x82\x01Qa\x04\xFF\x91a\x08\xD8V[`\xA0\x82\x01R\x84Qa\x01\0\x82\x01Qa\x05$\x91a\x05\x19\x91a\n\xEDV[` \x83\x01Q\x90a\x08\xD8V[`\xC0\x82\x01R[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\xA0\x01Q\x11\x15a\x06\x8BW`\0a\x05\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xA0\x01Qa\x05\xAC\x91\x90a\x0B\x1EV[\x90a\x08\xD8V[\x90Pa\x05\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[a\x06\x08\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[\x82``\x01\x81\x81Qa\x06\x19\x91\x90a\n\xEDV[\x90RPa\x06F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[a\x06p\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[\x82`@\x01\x81\x81Qa\x06\x81\x91\x90a\n\xEDV[\x90RPa\x07e\x90PV[a\x06\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05\xAC\x83`\xA0\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x17\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81``\x01\x81\x81Qa\x06\xF7\x91\x90a\n\xEDV[\x90RP`\xA0\x81\x01Qa\x07P\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x05\xAC\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\t\x17V[\x81`@\x01\x81\x81Qa\x07a\x91\x90a\n\xEDV[\x90RP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\xE0\x01Q\x11\x15a\x08)W`\0a\x07\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\xE0\x01Qa\x05\xAC\x91\x90a\x0B\x1EV[\x90Pa\x08\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\t\x17V[\x82``\x01\x81\x81Qa\x08$\x91\x90a\n\xEDV[\x90RPP[a\x08n\x85`\xC0\x01Qa'\x10a\x08>\x91\x90a\x0B\x1EV[a\x08h\x83`\xC0\x01Qa\x08b\x89``\x01Q\x8A`\x80\x01Q\x87`@\x01Q\x8C`\xA0\x01Qa\t[V[\x90a\t\x17V[\x90a\t\xC2V[`\x80\x82\x01\x81\x90R``\x82\x01Q`@\x90\x92\x01Q\x90\x96\x91\x95P\x93P\x91PPV[`\0a\x03r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xEDV[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x08\xFCW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\t9W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x80a\th\x85\x87a\n\xEDV[\x90P\x80a\tyW`\0\x91PPa\t\xBAV[`\0a\t\x88\x85a\x08b\x88a\t\xE8V[\x90P`\0a\t\x99\x85a\x08b\x8Aa\t\xE8V[\x90P`\0a\t\xB3a\t\xA9\x85a\t\xE8V[a\x05\xAC\x84\x86a\n\xEDV[\x94PPPPP[\x94\x93PPPPV[`\0\x81\x15a\x13\x88\x19\x83\x90\x04\x84\x11\x15\x17a\t\xDAW`\0\x80\xFD[Pa'\x10\x91\x02a\x13\x88\x01\x04\x90V[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\t\xFEW`\0\x80\xFD[\x91\x90PV[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFEW`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a\neW`\0\x80\xFD[a\nma\n\x03V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra\n\xB9`\xE0\x84\x01a\n;V[`\xE0\x82\x01Ra\x01\0a\n\xCC\x81\x85\x01a\n;V[\x90\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x0B\0Wa\x0B\0a\n\xD7V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x82\x10\x15a\x0B0Wa\x0B0a\n\xD7V[P\x03\x90V\xFE\xA2dipfsX\"\x12 \xE6*%1\x916,\xA1\\Bh\xDA\x9A\xA0\xFE\xA3#\xF3\x95\xE7\xED\xF9\xA0\xA1\x96\xCD\x90\xE4\x8C\xB0\xB1\x8DdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static DEFAULTRESERVEINTERESTRATESTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DefaultReserveInterestRateStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DefaultReserveInterestRateStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DefaultReserveInterestRateStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DefaultReserveInterestRateStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DefaultReserveInterestRateStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DefaultReserveInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DefaultReserveInterestRateStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEFAULTRESERVEINTERESTRATESTRATEGY_ABI.clone(),
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
                DEFAULTRESERVEINTERESTRATESTRATEGY_ABI.clone(),
                DEFAULTRESERVEINTERESTRATESTRATEGY_BYTECODE.clone().into(),
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
        ///Calls the contract's `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` (0xfe5fd698) function
        pub fn max_excess_stable_to_total_debt_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 95, 214, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_EXCESS_USAGE_RATIO` (0xa9c622f8) function
        pub fn max_excess_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 198, 34, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` (0x6fb92589) function
        pub fn optimal_stable_to_total_debt_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([111, 185, 37, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPTIMAL_USAGE_RATIO` (0x54c365c6) function
        pub fn optimal_usage_ratio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 195, 101, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateInterestRates` (0xa5898709) function
        pub fn calculate_interest_rates(
            &self,
            params: CalculateInterestRatesParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBaseStableBorrowRate` (0xacd78686) function
        pub fn get_base_stable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([172, 215, 134, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBaseVariableBorrowRate` (0x34762ca5) function
        pub fn get_base_variable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 118, 44, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxVariableBorrowRate` (0x80031e37) function
        pub fn get_max_variable_borrow_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 3, 30, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateExcessOffset` (0xbc626908) function
        pub fn get_stable_rate_excess_offset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 98, 105, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateSlope1` (0xd5cd7391) function
        pub fn get_stable_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([213, 205, 115, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStableRateSlope2` (0x14e32da4) function
        pub fn get_stable_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 227, 45, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVariableRateSlope1` (0x0b3429a2) function
        pub fn get_variable_rate_slope_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 52, 41, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVariableRateSlope2` (0xf4202409) function
        pub fn get_variable_rate_slope_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([244, 32, 36, 9], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DefaultReserveInterestRateStrategy<M> {
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
    ///Container type for all input parameters for the `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xfe5fd698`
    #[derive(
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
        name = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct MaxExcessStableToTotalDebtRatioCall;
    ///Container type for all input parameters for the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MAX_EXCESS_USAGE_RATIO", abi = "MAX_EXCESS_USAGE_RATIO()")]
    pub struct MaxExcessUsageRatioCall;
    ///Container type for all input parameters for the `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0x6fb92589`
    #[derive(
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
        name = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO",
        abi = "OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()"
    )]
    pub struct OptimalStableToTotalDebtRatioCall;
    ///Container type for all input parameters for the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "OPTIMAL_USAGE_RATIO", abi = "OPTIMAL_USAGE_RATIO()")]
    pub struct OptimalUsageRatioCall;
    ///Container type for all input parameters for the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `0xa5898709`
    #[derive(
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
        name = "calculateInterestRates",
        abi = "calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))"
    )]
    pub struct CalculateInterestRatesCall {
        pub params: CalculateInterestRatesParams,
    }
    ///Container type for all input parameters for the `getBaseStableBorrowRate` function with signature `getBaseStableBorrowRate()` and selector `0xacd78686`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getBaseStableBorrowRate", abi = "getBaseStableBorrowRate()")]
    pub struct GetBaseStableBorrowRateCall;
    ///Container type for all input parameters for the `getBaseVariableBorrowRate` function with signature `getBaseVariableBorrowRate()` and selector `0x34762ca5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getBaseVariableBorrowRate", abi = "getBaseVariableBorrowRate()")]
    pub struct GetBaseVariableBorrowRateCall;
    ///Container type for all input parameters for the `getMaxVariableBorrowRate` function with signature `getMaxVariableBorrowRate()` and selector `0x80031e37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMaxVariableBorrowRate", abi = "getMaxVariableBorrowRate()")]
    pub struct GetMaxVariableBorrowRateCall;
    ///Container type for all input parameters for the `getStableRateExcessOffset` function with signature `getStableRateExcessOffset()` and selector `0xbc626908`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getStableRateExcessOffset", abi = "getStableRateExcessOffset()")]
    pub struct GetStableRateExcessOffsetCall;
    ///Container type for all input parameters for the `getStableRateSlope1` function with signature `getStableRateSlope1()` and selector `0xd5cd7391`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getStableRateSlope1", abi = "getStableRateSlope1()")]
    pub struct GetStableRateSlope1Call;
    ///Container type for all input parameters for the `getStableRateSlope2` function with signature `getStableRateSlope2()` and selector `0x14e32da4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getStableRateSlope2", abi = "getStableRateSlope2()")]
    pub struct GetStableRateSlope2Call;
    ///Container type for all input parameters for the `getVariableRateSlope1` function with signature `getVariableRateSlope1()` and selector `0x0b3429a2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getVariableRateSlope1", abi = "getVariableRateSlope1()")]
    pub struct GetVariableRateSlope1Call;
    ///Container type for all input parameters for the `getVariableRateSlope2` function with signature `getVariableRateSlope2()` and selector `0xf4202409`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getVariableRateSlope2", abi = "getVariableRateSlope2()")]
    pub struct GetVariableRateSlope2Call;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DefaultReserveInterestRateStrategyCalls {
        AddressesProvider(AddressesProviderCall),
        MaxExcessStableToTotalDebtRatio(MaxExcessStableToTotalDebtRatioCall),
        MaxExcessUsageRatio(MaxExcessUsageRatioCall),
        OptimalStableToTotalDebtRatio(OptimalStableToTotalDebtRatioCall),
        OptimalUsageRatio(OptimalUsageRatioCall),
        CalculateInterestRates(CalculateInterestRatesCall),
        GetBaseStableBorrowRate(GetBaseStableBorrowRateCall),
        GetBaseVariableBorrowRate(GetBaseVariableBorrowRateCall),
        GetMaxVariableBorrowRate(GetMaxVariableBorrowRateCall),
        GetStableRateExcessOffset(GetStableRateExcessOffsetCall),
        GetStableRateSlope1(GetStableRateSlope1Call),
        GetStableRateSlope2(GetStableRateSlope2Call),
        GetVariableRateSlope1(GetVariableRateSlope1Call),
        GetVariableRateSlope2(GetVariableRateSlope2Call),
    }
    impl ::ethers::core::abi::AbiDecode for DefaultReserveInterestRateStrategyCalls {
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
                = <MaxExcessStableToTotalDebtRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxExcessStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded)
                = <MaxExcessUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxExcessUsageRatio(decoded));
            }
            if let Ok(decoded)
                = <OptimalStableToTotalDebtRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptimalStableToTotalDebtRatio(decoded));
            }
            if let Ok(decoded)
                = <OptimalUsageRatioCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OptimalUsageRatio(decoded));
            }
            if let Ok(decoded)
                = <CalculateInterestRatesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateInterestRates(decoded));
            }
            if let Ok(decoded)
                = <GetBaseStableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBaseStableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetBaseVariableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetBaseVariableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetMaxVariableBorrowRateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetMaxVariableBorrowRate(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateExcessOffsetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateExcessOffset(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateSlope1(decoded));
            }
            if let Ok(decoded)
                = <GetStableRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStableRateSlope2(decoded));
            }
            if let Ok(decoded)
                = <GetVariableRateSlope1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVariableRateSlope1(decoded));
            }
            if let Ok(decoded)
                = <GetVariableRateSlope2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVariableRateSlope2(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DefaultReserveInterestRateStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExcessStableToTotalDebtRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExcessUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimalStableToTotalDebtRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptimalUsageRatio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateInterestRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBaseStableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBaseVariableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxVariableBorrowRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateExcessOffset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStableRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVariableRateSlope1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVariableRateSlope2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DefaultReserveInterestRateStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxExcessStableToTotalDebtRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxExcessUsageRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimalStableToTotalDebtRatio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptimalUsageRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateInterestRates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBaseStableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetBaseVariableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMaxVariableBorrowRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateExcessOffset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateSlope1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStableRateSlope2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVariableRateSlope1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVariableRateSlope2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<MaxExcessStableToTotalDebtRatioCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: MaxExcessStableToTotalDebtRatioCall) -> Self {
            Self::MaxExcessStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<MaxExcessUsageRatioCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: MaxExcessUsageRatioCall) -> Self {
            Self::MaxExcessUsageRatio(value)
        }
    }
    impl ::core::convert::From<OptimalStableToTotalDebtRatioCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: OptimalStableToTotalDebtRatioCall) -> Self {
            Self::OptimalStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<OptimalUsageRatioCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: OptimalUsageRatioCall) -> Self {
            Self::OptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<CalculateInterestRatesCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: CalculateInterestRatesCall) -> Self {
            Self::CalculateInterestRates(value)
        }
    }
    impl ::core::convert::From<GetBaseStableBorrowRateCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetBaseStableBorrowRateCall) -> Self {
            Self::GetBaseStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetBaseVariableBorrowRateCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetBaseVariableBorrowRateCall) -> Self {
            Self::GetBaseVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetMaxVariableBorrowRateCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetMaxVariableBorrowRateCall) -> Self {
            Self::GetMaxVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetStableRateExcessOffsetCall>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateExcessOffsetCall) -> Self {
            Self::GetStableRateExcessOffset(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope1Call>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope1Call) -> Self {
            Self::GetStableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope2Call>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope2Call) -> Self {
            Self::GetStableRateSlope2(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope1Call>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetVariableRateSlope1Call) -> Self {
            Self::GetVariableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope2Call>
    for DefaultReserveInterestRateStrategyCalls {
        fn from(value: GetVariableRateSlope2Call) -> Self {
            Self::GetVariableRateSlope2(value)
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
    ///Container type for all return fields from the `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `MAX_EXCESS_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0xfe5fd698`
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
    pub struct MaxExcessStableToTotalDebtRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_EXCESS_USAGE_RATIO` function with signature `MAX_EXCESS_USAGE_RATIO()` and selector `0xa9c622f8`
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
    pub struct MaxExcessUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO` function with signature `OPTIMAL_STABLE_TO_TOTAL_DEBT_RATIO()` and selector `0x6fb92589`
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
    pub struct OptimalStableToTotalDebtRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `OPTIMAL_USAGE_RATIO` function with signature `OPTIMAL_USAGE_RATIO()` and selector `0x54c365c6`
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
    pub struct OptimalUsageRatioReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculateInterestRates` function with signature `calculateInterestRates((uint256,uint256,uint256,uint256,uint256,uint256,uint256,address,address))` and selector `0xa5898709`
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
    pub struct CalculateInterestRatesReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getBaseStableBorrowRate` function with signature `getBaseStableBorrowRate()` and selector `0xacd78686`
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
    pub struct GetBaseStableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getBaseVariableBorrowRate` function with signature `getBaseVariableBorrowRate()` and selector `0x34762ca5`
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
    pub struct GetBaseVariableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMaxVariableBorrowRate` function with signature `getMaxVariableBorrowRate()` and selector `0x80031e37`
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
    pub struct GetMaxVariableBorrowRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateExcessOffset` function with signature `getStableRateExcessOffset()` and selector `0xbc626908`
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
    pub struct GetStableRateExcessOffsetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateSlope1` function with signature `getStableRateSlope1()` and selector `0xd5cd7391`
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
    pub struct GetStableRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStableRateSlope2` function with signature `getStableRateSlope2()` and selector `0x14e32da4`
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
    pub struct GetStableRateSlope2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVariableRateSlope1` function with signature `getVariableRateSlope1()` and selector `0x0b3429a2`
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
    pub struct GetVariableRateSlope1Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVariableRateSlope2` function with signature `getVariableRateSlope2()` and selector `0xf4202409`
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
    pub struct GetVariableRateSlope2Return(pub ::ethers::core::types::U256);
}
