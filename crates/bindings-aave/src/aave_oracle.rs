pub use aave_oracle::*;
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
pub mod aave_oracle {
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
                        name: ::std::borrow::ToOwned::to_owned("sources"),
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
                        name: ::std::borrow::ToOwned::to_owned("fallbackOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("baseCurrency"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("baseCurrencyUnit"),
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
                    ::std::borrow::ToOwned::to_owned("BASE_CURRENCY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_CURRENCY"),
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
                    ::std::borrow::ToOwned::to_owned("BASE_CURRENCY_UNIT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE_CURRENCY_UNIT"),
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
                    ::std::borrow::ToOwned::to_owned("getAssetPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getAssetsPrices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetsPrices"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFallbackOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFallbackOracle"),
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
                    ::std::borrow::ToOwned::to_owned("getSourceOfAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSourceOfAsset"),
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
                    ::std::borrow::ToOwned::to_owned("setAssetSources"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAssetSources"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sources"),
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
                    ::std::borrow::ToOwned::to_owned("setFallbackOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFallbackOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackOracle"),
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
                    ::std::borrow::ToOwned::to_owned("AssetSourceUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssetSourceUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("source"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BaseCurrencySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BaseCurrencySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("baseCurrency"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("baseCurrencyUnit"),
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
                    ::std::borrow::ToOwned::to_owned("FallbackOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FallbackOracleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fallbackOracle"),
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
    pub static AAVEORACLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x0F\xB58\x03\x80b\0\x0F\xB5\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03NV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\x80Rb\0\0L\x83b\0\0\xABV[b\0\0X\x85\x85b\0\0\xF5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\xA0\x81\x90R`\xC0\x82\x90R`@Q\x82\x81R\x7F\xE2|L\x13r9j=\x15\xA9\x92/t\xF9\xDF\xC7\xC7+\x1A\xD6\xD68hG\x07\x87$\x9C5dT\xC1\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPPPPb\0\x04\x9AV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xCEzx\r3f[\x1E\xA0\x97\xAF_\x15^8!\xB8\t\xEC\xBA\xA89\xD3\xB3:\xA8;\xA2\x81h\xCE\xFB\x90`\0\x90\xA2PV[\x80Q\x82Q\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x9B`\xF1\x1B\x81RP\x90b\0\x01?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x016\x91\x90b\0\x04\x02V[`@Q\x80\x91\x03\x90\xFD[P`\0[\x82Q\x81\x10\x15b\0\x02[W\x81\x81\x81Q\x81\x10b\0\x01bWb\0\x01bb\0\x04ZV[` \x02` \x01\x01Q`\0\x80\x85\x84\x81Q\x81\x10b\0\x01\x82Wb\0\x01\x82b\0\x04ZV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x81\x81Q\x81\x10b\0\x01\xE3Wb\0\x01\xE3b\0\x04ZV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10b\0\x02\tWb\0\x02\tb\0\x04ZV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\"\xC5\xB7\xB2\xD8V\x1D9\xF7\xF2\x10\xB6\xB3&\xA1\xAAi\xF1S\x11\x160\x820\x8A\xC4\x87}\xB63\x9D\xC1`@Q`@Q\x80\x91\x03\x90\xA3\x80b\0\x02R\x81b\0\x04pV[\x91PPb\0\x01CV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02vW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qb\0\x02\x9C\x81b\0\x02`V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12b\0\x02\xB3W`\0\x80\xFD[\x81Q` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15b\0\x02\xD2Wb\0\x02\xD2b\0\x02yV[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15b\0\x02\xFAWb\0\x02\xFAb\0\x02yV[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15b\0\x03\x19W`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15b\0\x03CWb\0\x033\x82b\0\x02\x8FV[\x83R\x91\x83\x01\x91\x90\x83\x01\x90b\0\x03\x1FV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x03hW`\0\x80\xFD[\x86Qb\0\x03u\x81b\0\x02`V[` \x88\x01Q\x90\x96P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x03\x93W`\0\x80\xFD[b\0\x03\xA1\x8A\x83\x8B\x01b\0\x02\xA1V[\x96P`@\x89\x01Q\x91P\x80\x82\x11\x15b\0\x03\xB8W`\0\x80\xFD[Pb\0\x03\xC7\x89\x82\x8A\x01b\0\x02\xA1V[\x94PP``\x87\x01Qb\0\x03\xDA\x81b\0\x02`V[`\x80\x88\x01Q\x90\x93Pb\0\x03\xED\x81b\0\x02`V[\x80\x92PP`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x041W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x04\x13V[\x81\x81\x11\x15b\0\x04DW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15b\0\x04\x93WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\x80Q`\xA0Q`\xC0Qa\n\xD7b\0\x04\xDE`\09`\0\x81\x81a\x01\x07\x01Ra\x03a\x01R`\0\x81\x81a\x01\xAE\x01Ra\x036\x01R`\0\x81\x81`\x9D\x01Ra\x04\xF9\x01Ra\n\xD7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\x92\xBF+\xE0\x11a\0fW\x80c\x92\xBF+\xE0\x14a\x017W\x80c\x9D#\xD9\xF2\x14a\x01cW\x80c\xAB\xFDS\x10\x14a\x01\x83W\x80c\xB3Yo\x07\x14a\x01\x96W\x80c\xE1\x9FG\0\x14a\x01\xA9W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x98W\x80c\x17\n\xEEs\x14a\0\xDCW\x80cb\x100\x8C\x14a\0\xF1W\x80c\x8C\x89\xB6O\x14a\x01\x02W[`\0\x80\xFD[a\0\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEFa\0\xEA6`\x04a\x08DV[a\x01\xD0V[\0[`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\0\xBFV[a\x01)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\0\xD3V[a\0\xBFa\x01E6`\x04a\x08DV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x90V[a\x01va\x01q6`\x04a\x08\xADV[a\x01\xE4V[`@Qa\0\xD3\x91\x90a\x08\xEFV[a\0\xEFa\x01\x916`\x04a\t3V[a\x02\x99V[a\x01)a\x01\xA46`\x04a\x08DV[a\x03\x14V[a\0\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xD8a\x04\xF5V[a\x01\xE1\x81a\x06\x96V[PV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x01Wa\x02\x01a\t\x9FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02*W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x02\x91Wa\x02b\x85\x85\x83\x81\x81\x10a\x02MWa\x02Ma\t\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x01\xA4\x91\x90a\x08DV[\x82\x82\x81Q\x81\x10a\x02tWa\x02ta\t\xB5V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x02\x89\x81a\t\xCBV[\x91PPa\x020V[P\x93\x92PPPV[a\x02\xA1a\x04\xF5V[a\x03\x0E\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x88\x02\x82\x81\x01\x82\x01\x90\x93R\x87\x82R\x90\x93P\x87\x92P\x86\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x06\xE0\x92PPPV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R` \x81\x90R`@\x81 T\x90\x92\x90\x81\x16\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x14\x15a\x03\x86WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\tW`\x01T`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x02\x91\x90a\t\xF4V[\x93\x92PPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16cP\xD2[\xCD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a\t\xF4V[\x90P`\0\x81\x13\x15a\x04\x7FW\x93\x92PPPV[`\x01T`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xED\x91\x90a\t\xF4V[\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05y\x91\x90a\n\rV[`@Qb\x9Fq\x97`\xE5\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x13\xEE2\xE0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE3\x91\x90a\n*V[\x80a\x06QWP`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06Q\x91\x90a\n*V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`5`\xF8\x1B\x81RP\x90a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x89\x91\x90a\nLV[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xCEzx\r3f[\x1E\xA0\x97\xAF_\x15^8!\xB8\t\xEC\xBA\xA89\xD3\xB3:\xA8;\xA2\x81h\xCE\xFB\x90`\0\x90\xA2PV[\x80Q\x82Q\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x9B`\xF1\x1B\x81RP\x90a\x07\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x89\x91\x90a\nLV[P`\0[\x82Q\x81\x10\x15a\x08*W\x81\x81\x81Q\x81\x10a\x07=Wa\x07=a\t\xB5V[` \x02` \x01\x01Q`\0\x80\x85\x84\x81Q\x81\x10a\x07ZWa\x07Za\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x81\x81Q\x81\x10a\x07\xB8Wa\x07\xB8a\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x07\xDBWa\x07\xDBa\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\"\xC5\xB7\xB2\xD8V\x1D9\xF7\xF2\x10\xB6\xB3&\xA1\xAAi\xF1S\x11\x160\x820\x8A\xC4\x87}\xB63\x9D\xC1`@Q`@Q\x80\x91\x03\x90\xA3\x80a\x08\"\x81a\t\xCBV[\x91PPa\x07\"V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xE1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08VW`\0\x80\xFD[\x815a\x04\x02\x81a\x08/V[`\0\x80\x83`\x1F\x84\x01\x12a\x08sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x08\xA6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x08\xC0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD7W`\0\x80\xFD[a\x08\xE3\x85\x82\x86\x01a\x08aV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t'W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\x0BV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\tIW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\taW`\0\x80\xFD[a\tm\x88\x83\x89\x01a\x08aV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\t\x86W`\0\x80\xFD[Pa\t\x93\x87\x82\x88\x01a\x08aV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\t\xEDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\n\x06W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x1FW`\0\x80\xFD[\x81Qa\x04\x02\x81a\x08/V[`\0` \x82\x84\x03\x12\x15a\n<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\x02W`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\nyW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\n]V[\x81\x81\x11\x15a\n\x8BW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB6\x9A\x88\xE8p\x12\xFB\xF0\xB3S\x1F6]=S\xF4\xD8\xCFM\x8F}\xAA\x06\x1D\x83\x08^\xFC\x9A\xDC\xF5=dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static AAVEORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\x92\xBF+\xE0\x11a\0fW\x80c\x92\xBF+\xE0\x14a\x017W\x80c\x9D#\xD9\xF2\x14a\x01cW\x80c\xAB\xFDS\x10\x14a\x01\x83W\x80c\xB3Yo\x07\x14a\x01\x96W\x80c\xE1\x9FG\0\x14a\x01\xA9W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x98W\x80c\x17\n\xEEs\x14a\0\xDCW\x80cb\x100\x8C\x14a\0\xF1W\x80c\x8C\x89\xB6O\x14a\x01\x02W[`\0\x80\xFD[a\0\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEFa\0\xEA6`\x04a\x08DV[a\x01\xD0V[\0[`\x01T`\x01`\x01`\xA0\x1B\x03\x16a\0\xBFV[a\x01)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\0\xD3V[a\0\xBFa\x01E6`\x04a\x08DV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x16\x90V[a\x01va\x01q6`\x04a\x08\xADV[a\x01\xE4V[`@Qa\0\xD3\x91\x90a\x08\xEFV[a\0\xEFa\x01\x916`\x04a\t3V[a\x02\x99V[a\x01)a\x01\xA46`\x04a\x08DV[a\x03\x14V[a\0\xBF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xD8a\x04\xF5V[a\x01\xE1\x81a\x06\x96V[PV[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x01Wa\x02\x01a\t\x9FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02*W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x02\x91Wa\x02b\x85\x85\x83\x81\x81\x10a\x02MWa\x02Ma\t\xB5V[\x90P` \x02\x01` \x81\x01\x90a\x01\xA4\x91\x90a\x08DV[\x82\x82\x81Q\x81\x10a\x02tWa\x02ta\t\xB5V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x02\x89\x81a\t\xCBV[\x91PPa\x020V[P\x93\x92PPPV[a\x02\xA1a\x04\xF5V[a\x03\x0E\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` \x80\x88\x02\x82\x81\x01\x82\x01\x90\x93R\x87\x82R\x90\x93P\x87\x92P\x86\x91\x82\x91\x85\x01\x90\x84\x90\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x06\xE0\x92PPPV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R` \x81\x90R`@\x81 T\x90\x92\x90\x81\x16\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x14\x15a\x03\x86WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\tW`\x01T`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x02\x91\x90a\t\xF4V[\x93\x92PPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16cP\xD2[\xCD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04m\x91\x90a\t\xF4V[\x90P`\0\x81\x13\x15a\x04\x7FW\x93\x92PPPV[`\x01T`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xED\x91\x90a\t\xF4V[\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05y\x91\x90a\n\rV[`@Qb\x9Fq\x97`\xE5\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x13\xEE2\xE0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE3\x91\x90a\n*V[\x80a\x06QWP`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06Q\x91\x90a\n*V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`5`\xF8\x1B\x81RP\x90a\x06\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x89\x91\x90a\nLV[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xCEzx\r3f[\x1E\xA0\x97\xAF_\x15^8!\xB8\t\xEC\xBA\xA89\xD3\xB3:\xA8;\xA2\x81h\xCE\xFB\x90`\0\x90\xA2PV[\x80Q\x82Q\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x1B\x9B`\xF1\x1B\x81RP\x90a\x07\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x89\x91\x90a\nLV[P`\0[\x82Q\x81\x10\x15a\x08*W\x81\x81\x81Q\x81\x10a\x07=Wa\x07=a\t\xB5V[` \x02` \x01\x01Q`\0\x80\x85\x84\x81Q\x81\x10a\x07ZWa\x07Za\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x81\x81Q\x81\x10a\x07\xB8Wa\x07\xB8a\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x07\xDBWa\x07\xDBa\t\xB5V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\"\xC5\xB7\xB2\xD8V\x1D9\xF7\xF2\x10\xB6\xB3&\xA1\xAAi\xF1S\x11\x160\x820\x8A\xC4\x87}\xB63\x9D\xC1`@Q`@Q\x80\x91\x03\x90\xA3\x80a\x08\"\x81a\t\xCBV[\x91PPa\x07\"V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xE1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x08VW`\0\x80\xFD[\x815a\x04\x02\x81a\x08/V[`\0\x80\x83`\x1F\x84\x01\x12a\x08sW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x8BW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x08\xA6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x08\xC0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD7W`\0\x80\xFD[a\x08\xE3\x85\x82\x86\x01a\x08aV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\t'W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\x0BV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\tIW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\taW`\0\x80\xFD[a\tm\x88\x83\x89\x01a\x08aV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\t\x86W`\0\x80\xFD[Pa\t\x93\x87\x82\x88\x01a\x08aV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\t\xEDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\n\x06W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x1FW`\0\x80\xFD[\x81Qa\x04\x02\x81a\x08/V[`\0` \x82\x84\x03\x12\x15a\n<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\x02W`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\nyW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\n]V[\x81\x81\x11\x15a\n\x8BW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB6\x9A\x88\xE8p\x12\xFB\xF0\xB3S\x1F6]=S\xF4\xD8\xCFM\x8F}\xAA\x06\x1D\x83\x08^\xFC\x9A\xDC\xF5=dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveOracle)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEORACLE_ABI.clone(),
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
                AAVEORACLE_ABI.clone(),
                AAVEORACLE_BYTECODE.clone().into(),
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
        ///Calls the contract's `BASE_CURRENCY` (0xe19f4700) function
        pub fn base_currency(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([225, 159, 71, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BASE_CURRENCY_UNIT` (0x8c89b64f) function
        pub fn base_currency_unit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 137, 182, 79], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetPrice` (0xb3596f07) function
        pub fn get_asset_price(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 89, 111, 7], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetsPrices` (0x9d23d9f2) function
        pub fn get_assets_prices(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([157, 35, 217, 242], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFallbackOracle` (0x6210308c) function
        pub fn get_fallback_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([98, 16, 48, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSourceOfAsset` (0x92bf2be0) function
        pub fn get_source_of_asset(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([146, 191, 43, 224], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAssetSources` (0xabfd5310) function
        pub fn set_asset_sources(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            sources: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 253, 83, 16], (assets, sources))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFallbackOracle` (0x170aee73) function
        pub fn set_fallback_oracle(
            &self,
            fallback_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 10, 238, 115], fallback_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetSourceUpdated` event
        pub fn asset_source_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetSourceUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BaseCurrencySet` event
        pub fn base_currency_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BaseCurrencySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FallbackOracleUpdated` event
        pub fn fallback_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FallbackOracleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AaveOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AaveOracle<M> {
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
    #[ethevent(name = "AssetSourceUpdated", abi = "AssetSourceUpdated(address,address)")]
    pub struct AssetSourceUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub source: ::ethers::core::types::Address,
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
    #[ethevent(name = "BaseCurrencySet", abi = "BaseCurrencySet(address,uint256)")]
    pub struct BaseCurrencySetFilter {
        #[ethevent(indexed)]
        pub base_currency: ::ethers::core::types::Address,
        pub base_currency_unit: ::ethers::core::types::U256,
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
    #[ethevent(name = "FallbackOracleUpdated", abi = "FallbackOracleUpdated(address)")]
    pub struct FallbackOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub fallback_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AaveOracleEvents {
        AssetSourceUpdatedFilter(AssetSourceUpdatedFilter),
        BaseCurrencySetFilter(BaseCurrencySetFilter),
        FallbackOracleUpdatedFilter(FallbackOracleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AaveOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetSourceUpdatedFilter::decode_log(log) {
                return Ok(AaveOracleEvents::AssetSourceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BaseCurrencySetFilter::decode_log(log) {
                return Ok(AaveOracleEvents::BaseCurrencySetFilter(decoded));
            }
            if let Ok(decoded) = FallbackOracleUpdatedFilter::decode_log(log) {
                return Ok(AaveOracleEvents::FallbackOracleUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AaveOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetSourceUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseCurrencySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FallbackOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetSourceUpdatedFilter> for AaveOracleEvents {
        fn from(value: AssetSourceUpdatedFilter) -> Self {
            Self::AssetSourceUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BaseCurrencySetFilter> for AaveOracleEvents {
        fn from(value: BaseCurrencySetFilter) -> Self {
            Self::BaseCurrencySetFilter(value)
        }
    }
    impl ::core::convert::From<FallbackOracleUpdatedFilter> for AaveOracleEvents {
        fn from(value: FallbackOracleUpdatedFilter) -> Self {
            Self::FallbackOracleUpdatedFilter(value)
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
    ///Container type for all input parameters for the `BASE_CURRENCY` function with signature `BASE_CURRENCY()` and selector `0xe19f4700`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BASE_CURRENCY", abi = "BASE_CURRENCY()")]
    pub struct BaseCurrencyCall;
    ///Container type for all input parameters for the `BASE_CURRENCY_UNIT` function with signature `BASE_CURRENCY_UNIT()` and selector `0x8c89b64f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "BASE_CURRENCY_UNIT", abi = "BASE_CURRENCY_UNIT()")]
    pub struct BaseCurrencyUnitCall;
    ///Container type for all input parameters for the `getAssetPrice` function with signature `getAssetPrice(address)` and selector `0xb3596f07`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAssetPrice", abi = "getAssetPrice(address)")]
    pub struct GetAssetPriceCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAssetsPrices` function with signature `getAssetsPrices(address[])` and selector `0x9d23d9f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAssetsPrices", abi = "getAssetsPrices(address[])")]
    pub struct GetAssetsPricesCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getFallbackOracle` function with signature `getFallbackOracle()` and selector `0x6210308c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getFallbackOracle", abi = "getFallbackOracle()")]
    pub struct GetFallbackOracleCall;
    ///Container type for all input parameters for the `getSourceOfAsset` function with signature `getSourceOfAsset(address)` and selector `0x92bf2be0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSourceOfAsset", abi = "getSourceOfAsset(address)")]
    pub struct GetSourceOfAssetCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setAssetSources` function with signature `setAssetSources(address[],address[])` and selector `0xabfd5310`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAssetSources", abi = "setAssetSources(address[],address[])")]
    pub struct SetAssetSourcesCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub sources: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setFallbackOracle` function with signature `setFallbackOracle(address)` and selector `0x170aee73`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFallbackOracle", abi = "setFallbackOracle(address)")]
    pub struct SetFallbackOracleCall {
        pub fallback_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AaveOracleCalls {
        AddressesProvider(AddressesProviderCall),
        BaseCurrency(BaseCurrencyCall),
        BaseCurrencyUnit(BaseCurrencyUnitCall),
        GetAssetPrice(GetAssetPriceCall),
        GetAssetsPrices(GetAssetsPricesCall),
        GetFallbackOracle(GetFallbackOracleCall),
        GetSourceOfAsset(GetSourceOfAssetCall),
        SetAssetSources(SetAssetSourcesCall),
        SetFallbackOracle(SetFallbackOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for AaveOracleCalls {
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
                = <BaseCurrencyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseCurrency(decoded));
            }
            if let Ok(decoded)
                = <BaseCurrencyUnitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BaseCurrencyUnit(decoded));
            }
            if let Ok(decoded)
                = <GetAssetPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetPrice(decoded));
            }
            if let Ok(decoded)
                = <GetAssetsPricesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAssetsPrices(decoded));
            }
            if let Ok(decoded)
                = <GetFallbackOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetFallbackOracle(decoded));
            }
            if let Ok(decoded)
                = <GetSourceOfAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSourceOfAsset(decoded));
            }
            if let Ok(decoded)
                = <SetAssetSourcesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAssetSources(decoded));
            }
            if let Ok(decoded)
                = <SetFallbackOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFallbackOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AaveOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseCurrency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseCurrencyUnit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetsPrices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFallbackOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSourceOfAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAssetSources(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFallbackOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AaveOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseCurrency(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseCurrencyUnit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetsPrices(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFallbackOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSourceOfAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAssetSources(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFallbackOracle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for AaveOracleCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<BaseCurrencyCall> for AaveOracleCalls {
        fn from(value: BaseCurrencyCall) -> Self {
            Self::BaseCurrency(value)
        }
    }
    impl ::core::convert::From<BaseCurrencyUnitCall> for AaveOracleCalls {
        fn from(value: BaseCurrencyUnitCall) -> Self {
            Self::BaseCurrencyUnit(value)
        }
    }
    impl ::core::convert::From<GetAssetPriceCall> for AaveOracleCalls {
        fn from(value: GetAssetPriceCall) -> Self {
            Self::GetAssetPrice(value)
        }
    }
    impl ::core::convert::From<GetAssetsPricesCall> for AaveOracleCalls {
        fn from(value: GetAssetsPricesCall) -> Self {
            Self::GetAssetsPrices(value)
        }
    }
    impl ::core::convert::From<GetFallbackOracleCall> for AaveOracleCalls {
        fn from(value: GetFallbackOracleCall) -> Self {
            Self::GetFallbackOracle(value)
        }
    }
    impl ::core::convert::From<GetSourceOfAssetCall> for AaveOracleCalls {
        fn from(value: GetSourceOfAssetCall) -> Self {
            Self::GetSourceOfAsset(value)
        }
    }
    impl ::core::convert::From<SetAssetSourcesCall> for AaveOracleCalls {
        fn from(value: SetAssetSourcesCall) -> Self {
            Self::SetAssetSources(value)
        }
    }
    impl ::core::convert::From<SetFallbackOracleCall> for AaveOracleCalls {
        fn from(value: SetFallbackOracleCall) -> Self {
            Self::SetFallbackOracle(value)
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
    ///Container type for all return fields from the `BASE_CURRENCY` function with signature `BASE_CURRENCY()` and selector `0xe19f4700`
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
    pub struct BaseCurrencyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `BASE_CURRENCY_UNIT` function with signature `BASE_CURRENCY_UNIT()` and selector `0x8c89b64f`
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
    pub struct BaseCurrencyUnitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAssetPrice` function with signature `getAssetPrice(address)` and selector `0xb3596f07`
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
    pub struct GetAssetPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAssetsPrices` function with signature `getAssetsPrices(address[])` and selector `0x9d23d9f2`
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
    pub struct GetAssetsPricesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getFallbackOracle` function with signature `getFallbackOracle()` and selector `0x6210308c`
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
    pub struct GetFallbackOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSourceOfAsset` function with signature `getSourceOfAsset(address)` and selector `0x92bf2be0`
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
    pub struct GetSourceOfAssetReturn(pub ::ethers::core::types::Address);
}
