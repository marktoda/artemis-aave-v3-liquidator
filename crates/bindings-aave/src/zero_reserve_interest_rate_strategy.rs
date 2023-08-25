pub use zero_reserve_interest_rate_strategy::*;
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
pub mod zero_reserve_interest_rate_strategy {
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static ZERORESERVEINTERESTRATESTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03v8\x03\x80a\x03v\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x02\xECa\0\x8A`\09`\0`\xF4\x01Ra\x02\xEC`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xA5\x89\x87\t\x11a\0\x8CW\x80c\xBCbi\x08\x11a\0fW\x80c\xBCbi\x08\x14a\x013W\x80c\xD5\xCDs\x91\x14a\x013W\x80c\xF4 $\t\x14a\x013W\x80c\xFE_\xD6\x98\x14a\x01DW`\0\x80\xFD[\x80c\xA5\x89\x87\t\x14a\x01TW\x80c\xA9\xC6\"\xF8\x14a\x01DW\x80c\xAC\xD7\x86\x86\x14a\x01\x87W`\0\x80\xFD[\x80c4v,\xA5\x11a\0\xC8W\x80c4v,\xA5\x14a\x013W\x80cT\xC3e\xC6\x14a\x01DW\x80co\xB9%\x89\x14a\x01DW\x80c\x80\x03\x1E7\x14a\x01LW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\xEFW\x80c\x0B4)\xA2\x14a\x013W\x80c\x14\xE3-\xA4\x14a\x013W[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0[`@Q\x90\x81R` \x01a\x01*V[a\x016`\0\x81V[a\x016a\x01\x8FV[a\x01la\x01b6`\x04a\x02\x0BV[P`\0\x90\x81\x90\x81\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01*V[a\x016a\x01\xABV[`\0\x80a\x01\x9C\x81\x80a\x02\x90V[a\x01\xA6\x91\x90a\x02\x90V[\x90P\x90V[`\0a\x01\xA6\x81\x80a\x02\x90V[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xE9WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x06W`\0\x80\xFD[\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a\x02\x1EW`\0\x80\xFD[a\x02&a\x01\xB7V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra\x02r`\xE0\x84\x01a\x01\xEFV[`\xE0\x82\x01Ra\x01\0a\x02\x85\x81\x85\x01a\x01\xEFV[\x90\x82\x01R\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x02\xB1WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V\xFE\xA2dipfsX\"\x12 \x89\x90\x10|\xAB\xD0\x88\xEE\xF1nUO\xD0\xE3P9S*\xF0#]\xAA[\xDDy\xC5\xB7lw\xF7\x949dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static ZERORESERVEINTERESTRATESTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xA5\x89\x87\t\x11a\0\x8CW\x80c\xBCbi\x08\x11a\0fW\x80c\xBCbi\x08\x14a\x013W\x80c\xD5\xCDs\x91\x14a\x013W\x80c\xF4 $\t\x14a\x013W\x80c\xFE_\xD6\x98\x14a\x01DW`\0\x80\xFD[\x80c\xA5\x89\x87\t\x14a\x01TW\x80c\xA9\xC6\"\xF8\x14a\x01DW\x80c\xAC\xD7\x86\x86\x14a\x01\x87W`\0\x80\xFD[\x80c4v,\xA5\x11a\0\xC8W\x80c4v,\xA5\x14a\x013W\x80cT\xC3e\xC6\x14a\x01DW\x80co\xB9%\x89\x14a\x01DW\x80c\x80\x03\x1E7\x14a\x01LW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\xEFW\x80c\x0B4)\xA2\x14a\x013W\x80c\x14\xE3-\xA4\x14a\x013W[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0[`@Q\x90\x81R` \x01a\x01*V[a\x016`\0\x81V[a\x016a\x01\x8FV[a\x01la\x01b6`\x04a\x02\x0BV[P`\0\x90\x81\x90\x81\x90V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01*V[a\x016a\x01\xABV[`\0\x80a\x01\x9C\x81\x80a\x02\x90V[a\x01\xA6\x91\x90a\x02\x90V[\x90P\x90V[`\0a\x01\xA6\x81\x80a\x02\x90V[`@Qa\x01 \x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xE9WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x06W`\0\x80\xFD[\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a\x02\x1EW`\0\x80\xFD[a\x02&a\x01\xB7V[\x825\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R`\x80\x83\x015`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01Ra\x02r`\xE0\x84\x01a\x01\xEFV[`\xE0\x82\x01Ra\x01\0a\x02\x85\x81\x85\x01a\x01\xEFV[\x90\x82\x01R\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a\x02\xB1WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V\xFE\xA2dipfsX\"\x12 \x89\x90\x10|\xAB\xD0\x88\xEE\xF1nUO\xD0\xE3P9S*\xF0#]\xAA[\xDDy\xC5\xB7lw\xF7\x949dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static ZERORESERVEINTERESTRATESTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ZeroReserveInterestRateStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ZeroReserveInterestRateStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ZeroReserveInterestRateStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ZeroReserveInterestRateStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ZeroReserveInterestRateStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ZeroReserveInterestRateStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ZeroReserveInterestRateStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ZERORESERVEINTERESTRATESTRATEGY_ABI.clone(),
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
                ZERORESERVEINTERESTRATESTRATEGY_ABI.clone(),
                ZERORESERVEINTERESTRATESTRATEGY_BYTECODE.clone().into(),
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
            p0: CalculateInterestRatesParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 137, 135, 9], (p0,))
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
    for ZeroReserveInterestRateStrategy<M> {
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
    pub struct CalculateInterestRatesCall(pub CalculateInterestRatesParams);
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
    pub enum ZeroReserveInterestRateStrategyCalls {
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
    impl ::ethers::core::abi::AbiDecode for ZeroReserveInterestRateStrategyCalls {
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
    impl ::ethers::core::abi::AbiEncode for ZeroReserveInterestRateStrategyCalls {
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
    impl ::core::fmt::Display for ZeroReserveInterestRateStrategyCalls {
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
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<MaxExcessStableToTotalDebtRatioCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: MaxExcessStableToTotalDebtRatioCall) -> Self {
            Self::MaxExcessStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<MaxExcessUsageRatioCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: MaxExcessUsageRatioCall) -> Self {
            Self::MaxExcessUsageRatio(value)
        }
    }
    impl ::core::convert::From<OptimalStableToTotalDebtRatioCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: OptimalStableToTotalDebtRatioCall) -> Self {
            Self::OptimalStableToTotalDebtRatio(value)
        }
    }
    impl ::core::convert::From<OptimalUsageRatioCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: OptimalUsageRatioCall) -> Self {
            Self::OptimalUsageRatio(value)
        }
    }
    impl ::core::convert::From<CalculateInterestRatesCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: CalculateInterestRatesCall) -> Self {
            Self::CalculateInterestRates(value)
        }
    }
    impl ::core::convert::From<GetBaseStableBorrowRateCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetBaseStableBorrowRateCall) -> Self {
            Self::GetBaseStableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetBaseVariableBorrowRateCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetBaseVariableBorrowRateCall) -> Self {
            Self::GetBaseVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetMaxVariableBorrowRateCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetMaxVariableBorrowRateCall) -> Self {
            Self::GetMaxVariableBorrowRate(value)
        }
    }
    impl ::core::convert::From<GetStableRateExcessOffsetCall>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateExcessOffsetCall) -> Self {
            Self::GetStableRateExcessOffset(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope1Call>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope1Call) -> Self {
            Self::GetStableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetStableRateSlope2Call>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetStableRateSlope2Call) -> Self {
            Self::GetStableRateSlope2(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope1Call>
    for ZeroReserveInterestRateStrategyCalls {
        fn from(value: GetVariableRateSlope1Call) -> Self {
            Self::GetVariableRateSlope1(value)
        }
    }
    impl ::core::convert::From<GetVariableRateSlope2Call>
    for ZeroReserveInterestRateStrategyCalls {
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
