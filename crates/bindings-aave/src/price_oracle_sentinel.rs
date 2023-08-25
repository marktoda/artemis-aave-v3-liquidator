pub use price_oracle_sentinel::*;
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
pub mod price_oracle_sentinel {
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
                        name: ::std::borrow::ToOwned::to_owned("oracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISequencerOracle"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("gracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("getGracePeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("getSequencerOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSequencerOracle"),
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
                    ::std::borrow::ToOwned::to_owned("isBorrowAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isBorrowAllowed"),
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
                    ::std::borrow::ToOwned::to_owned("isLiquidationAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isLiquidationAllowed",
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
                    ::std::borrow::ToOwned::to_owned("setGracePeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGracePeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newGracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("setSequencerOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSequencerOracle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSequencerOracle",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GracePeriodUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GracePeriodUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGracePeriod"),
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
                    ::std::borrow::ToOwned::to_owned("SequencerOracleUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SequencerOracleUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newSequencerOracle",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static PRICEORACLESENTINEL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xC08\x03\x80a\x07\xC0\x839\x81\x01`@\x81\x90Ra\0/\x91a\0vV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`\x01Ua\0\xB9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0sW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\x8BW`\0\x80\xFD[\x83Qa\0\x96\x81a\0^V[` \x85\x01Q\x90\x93Pa\0\xA7\x81a\0^V[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x80Qa\x06\xDFa\0\xE1`\09`\0\x81\x81`\x87\x01R\x81\x81a\x01;\x01Ra\x02\xC2\x01Ra\x06\xDF`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cz] \xEA\x11a\0[W\x80cz] \xEA\x14a\0\xD7W\x80c\xDB\xD1\x83\x88\x14a\0\xEFW\x80c\xF0\xAE\xF3\x1C\x14a\x01\0W\x80c\xF2\xF6Y`\x14a\x01\x15W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x82W\x80c\x12\x16\x8D\xC2\x14a\0\xC6W\x80cI\xAA.\x81\x14a\0\xD7W[`\0\x80\xFD[a\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xA9V[a\0\xDFa\x01(V[`@Q\x90\x15\x15\x81R` \x01a\0\xBDV[`\x01T`@Q\x90\x81R` \x01a\0\xBDV[a\x01\x13a\x01\x0E6`\x04a\x05DV[a\x017V[\0[a\x01\x13a\x01#6`\x04a\x05hV[a\x02\xBEV[`\0a\x012a\x04\x89V[\x90P\x90V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBB\x91\x90a\x05\x81V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02&\x91\x90a\x05\x9EV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x02gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02^\x91\x90a\x05\xC0V[`@Q\x80\x91\x03\x90\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x95\xCB\xF1\xD8\xF4N\xC8\x1F\xF3E\xED\x9C\xF2\xFES\xB6\xA6G>\x07+\xF0F\xEEA/\x19\x8CT\xDB\xA4I\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03B\x91\x90a\x05\x81V[`@QcgK^M`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cgK^M\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAD\x91\x90a\x05\x9EV[\x80a\x04\x1BWP`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1B\x91\x90a\x05\x9EV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\r`\xFA\x1B\x81RP\x90a\x04SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02^\x91\x90a\x05\xC0V[P`\x01\x82\x90U`@Q\x82\x81R\x7F3\xD1\x19\x1FZ:\xBF\xE1\x9DF\x8DQ\xBB^\xCE\x97H\x9F\x12w\xA9\x12\xA5\xB5\xC6Y\x92\xFC'\x9A\xD3\xD4\x90` \x01a\x02\xB2V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x04\x91\x90a\x064V[P\x93PP\x92PP\x81`\0\x14\x80\x15a\x05%WP`\x01Ta\x05#\x82Ba\x06\x84V[\x11[\x92PPP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05VW`\0\x80\xFD[\x815a\x05a\x81a\x05,V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05zW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[\x81Qa\x05a\x81a\x05,V[`\0` \x82\x84\x03\x12\x15a\x05\xB0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05aW`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x05\xEDW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x05\xD1V[\x81\x81\x11\x15a\x05\xFFW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06/W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06LW`\0\x80\xFD[a\x06U\x86a\x06\x15V[\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa\x06x`\x80\x87\x01a\x06\x15V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82\x82\x10\x15a\x06\xA4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V\xFE\xA2dipfsX\"\x12 \xB3\xD6\x9C\x13<\xF51\xCB\xD6\xC6\xC3P\x01*\xD57#d\xDE~E\xD2\x8C\x08%mC\x9Ca\xA0\xA3\x0BdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static PRICEORACLESENTINEL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cz] \xEA\x11a\0[W\x80cz] \xEA\x14a\0\xD7W\x80c\xDB\xD1\x83\x88\x14a\0\xEFW\x80c\xF0\xAE\xF3\x1C\x14a\x01\0W\x80c\xF2\xF6Y`\x14a\x01\x15W`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x82W\x80c\x12\x16\x8D\xC2\x14a\0\xC6W\x80cI\xAA.\x81\x14a\0\xD7W[`\0\x80\xFD[a\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xA9V[a\0\xDFa\x01(V[`@Q\x90\x15\x15\x81R` \x01a\0\xBDV[`\x01T`@Q\x90\x81R` \x01a\0\xBDV[a\x01\x13a\x01\x0E6`\x04a\x05DV[a\x017V[\0[a\x01\x13a\x01#6`\x04a\x05hV[a\x02\xBEV[`\0a\x012a\x04\x89V[\x90P\x90V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBB\x91\x90a\x05\x81V[`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02&\x91\x90a\x05\x9EV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x90a\x02gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02^\x91\x90a\x05\xC0V[`@Q\x80\x91\x03\x90\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x95\xCB\xF1\xD8\xF4N\xC8\x1F\xF3E\xED\x9C\xF2\xFES\xB6\xA6G>\x07+\xF0F\xEEA/\x19\x8CT\xDB\xA4I\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cp|\xD7\x16`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03B\x91\x90a\x05\x81V[`@QcgK^M`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cgK^M\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAD\x91\x90a\x05\x9EV[\x80a\x04\x1BWP`@Qc{\xE5<\xA1`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c{\xE5<\xA1\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1B\x91\x90a\x05\x9EV[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`\r`\xFA\x1B\x81RP\x90a\x04SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02^\x91\x90a\x05\xC0V[P`\x01\x82\x90U`@Q\x82\x81R\x7F3\xD1\x19\x1FZ:\xBF\xE1\x9DF\x8DQ\xBB^\xCE\x97H\x9F\x12w\xA9\x12\xA5\xB5\xC6Y\x92\xFC'\x9A\xD3\xD4\x90` \x01a\x02\xB2V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x04\x91\x90a\x064V[P\x93PP\x92PP\x81`\0\x14\x80\x15a\x05%WP`\x01Ta\x05#\x82Ba\x06\x84V[\x11[\x92PPP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05AW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05VW`\0\x80\xFD[\x815a\x05a\x81a\x05,V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05zW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x93W`\0\x80\xFD[\x81Qa\x05a\x81a\x05,V[`\0` \x82\x84\x03\x12\x15a\x05\xB0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05aW`\0\x80\xFD[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x05\xEDW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x05\xD1V[\x81\x81\x11\x15a\x05\xFFW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06/W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06LW`\0\x80\xFD[a\x06U\x86a\x06\x15V[\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa\x06x`\x80\x87\x01a\x06\x15V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82\x82\x10\x15a\x06\xA4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V\xFE\xA2dipfsX\"\x12 \xB3\xD6\x9C\x13<\xF51\xCB\xD6\xC6\xC3P\x01*\xD57#d\xDE~E\xD2\x8C\x08%mC\x9Ca\xA0\xA3\x0BdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static PRICEORACLESENTINEL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PriceOracleSentinel<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PriceOracleSentinel<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PriceOracleSentinel<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PriceOracleSentinel<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PriceOracleSentinel<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PriceOracleSentinel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PriceOracleSentinel<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PRICEORACLESENTINEL_ABI.clone(),
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
                PRICEORACLESENTINEL_ABI.clone(),
                PRICEORACLESENTINEL_BYTECODE.clone().into(),
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
        ///Calls the contract's `getGracePeriod` (0xdbd18388) function
        pub fn get_grace_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([219, 209, 131, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencerOracle` (0x12168dc2) function
        pub fn get_sequencer_oracle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([18, 22, 141, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBorrowAllowed` (0x49aa2e81) function
        pub fn is_borrow_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 170, 46, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLiquidationAllowed` (0x7a5d20ea) function
        pub fn is_liquidation_allowed(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 93, 32, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGracePeriod` (0xf2f65960) function
        pub fn set_grace_period(
            &self,
            new_grace_period: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 246, 89, 96], new_grace_period)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSequencerOracle` (0xf0aef31c) function
        pub fn set_sequencer_oracle(
            &self,
            new_sequencer_oracle: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 174, 243, 28], new_sequencer_oracle)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GracePeriodUpdated` event
        pub fn grace_period_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GracePeriodUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SequencerOracleUpdated` event
        pub fn sequencer_oracle_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SequencerOracleUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceOracleSentinelEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PriceOracleSentinel<M> {
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
    #[ethevent(name = "GracePeriodUpdated", abi = "GracePeriodUpdated(uint256)")]
    pub struct GracePeriodUpdatedFilter {
        pub new_grace_period: ::ethers::core::types::U256,
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
    #[ethevent(name = "SequencerOracleUpdated", abi = "SequencerOracleUpdated(address)")]
    pub struct SequencerOracleUpdatedFilter {
        pub new_sequencer_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PriceOracleSentinelEvents {
        GracePeriodUpdatedFilter(GracePeriodUpdatedFilter),
        SequencerOracleUpdatedFilter(SequencerOracleUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PriceOracleSentinelEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GracePeriodUpdatedFilter::decode_log(log) {
                return Ok(PriceOracleSentinelEvents::GracePeriodUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SequencerOracleUpdatedFilter::decode_log(log) {
                return Ok(
                    PriceOracleSentinelEvents::SequencerOracleUpdatedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PriceOracleSentinelEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GracePeriodUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerOracleUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GracePeriodUpdatedFilter> for PriceOracleSentinelEvents {
        fn from(value: GracePeriodUpdatedFilter) -> Self {
            Self::GracePeriodUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SequencerOracleUpdatedFilter>
    for PriceOracleSentinelEvents {
        fn from(value: SequencerOracleUpdatedFilter) -> Self {
            Self::SequencerOracleUpdatedFilter(value)
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
    ///Container type for all input parameters for the `getGracePeriod` function with signature `getGracePeriod()` and selector `0xdbd18388`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getGracePeriod", abi = "getGracePeriod()")]
    pub struct GetGracePeriodCall;
    ///Container type for all input parameters for the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `0x12168dc2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSequencerOracle", abi = "getSequencerOracle()")]
    pub struct GetSequencerOracleCall;
    ///Container type for all input parameters for the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `0x49aa2e81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isBorrowAllowed", abi = "isBorrowAllowed()")]
    pub struct IsBorrowAllowedCall;
    ///Container type for all input parameters for the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `0x7a5d20ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isLiquidationAllowed", abi = "isLiquidationAllowed()")]
    pub struct IsLiquidationAllowedCall;
    ///Container type for all input parameters for the `setGracePeriod` function with signature `setGracePeriod(uint256)` and selector `0xf2f65960`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setGracePeriod", abi = "setGracePeriod(uint256)")]
    pub struct SetGracePeriodCall {
        pub new_grace_period: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setSequencerOracle` function with signature `setSequencerOracle(address)` and selector `0xf0aef31c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setSequencerOracle", abi = "setSequencerOracle(address)")]
    pub struct SetSequencerOracleCall {
        pub new_sequencer_oracle: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PriceOracleSentinelCalls {
        AddressesProvider(AddressesProviderCall),
        GetGracePeriod(GetGracePeriodCall),
        GetSequencerOracle(GetSequencerOracleCall),
        IsBorrowAllowed(IsBorrowAllowedCall),
        IsLiquidationAllowed(IsLiquidationAllowedCall),
        SetGracePeriod(SetGracePeriodCall),
        SetSequencerOracle(SetSequencerOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for PriceOracleSentinelCalls {
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
                = <GetGracePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetGracePeriod(decoded));
            }
            if let Ok(decoded)
                = <GetSequencerOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetSequencerOracle(decoded));
            }
            if let Ok(decoded)
                = <IsBorrowAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBorrowAllowed(decoded));
            }
            if let Ok(decoded)
                = <IsLiquidationAllowedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsLiquidationAllowed(decoded));
            }
            if let Ok(decoded)
                = <SetGracePeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGracePeriod(decoded));
            }
            if let Ok(decoded)
                = <SetSequencerOracleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetSequencerOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PriceOracleSentinelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGracePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSequencerOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBorrowAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsLiquidationAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGracePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSequencerOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PriceOracleSentinelCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGracePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencerOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsBorrowAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsLiquidationAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetGracePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSequencerOracle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for PriceOracleSentinelCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<GetGracePeriodCall> for PriceOracleSentinelCalls {
        fn from(value: GetGracePeriodCall) -> Self {
            Self::GetGracePeriod(value)
        }
    }
    impl ::core::convert::From<GetSequencerOracleCall> for PriceOracleSentinelCalls {
        fn from(value: GetSequencerOracleCall) -> Self {
            Self::GetSequencerOracle(value)
        }
    }
    impl ::core::convert::From<IsBorrowAllowedCall> for PriceOracleSentinelCalls {
        fn from(value: IsBorrowAllowedCall) -> Self {
            Self::IsBorrowAllowed(value)
        }
    }
    impl ::core::convert::From<IsLiquidationAllowedCall> for PriceOracleSentinelCalls {
        fn from(value: IsLiquidationAllowedCall) -> Self {
            Self::IsLiquidationAllowed(value)
        }
    }
    impl ::core::convert::From<SetGracePeriodCall> for PriceOracleSentinelCalls {
        fn from(value: SetGracePeriodCall) -> Self {
            Self::SetGracePeriod(value)
        }
    }
    impl ::core::convert::From<SetSequencerOracleCall> for PriceOracleSentinelCalls {
        fn from(value: SetSequencerOracleCall) -> Self {
            Self::SetSequencerOracle(value)
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
    ///Container type for all return fields from the `getGracePeriod` function with signature `getGracePeriod()` and selector `0xdbd18388`
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
    pub struct GetGracePeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSequencerOracle` function with signature `getSequencerOracle()` and selector `0x12168dc2`
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
    pub struct GetSequencerOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isBorrowAllowed` function with signature `isBorrowAllowed()` and selector `0x49aa2e81`
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
    pub struct IsBorrowAllowedReturn(pub bool);
    ///Container type for all return fields from the `isLiquidationAllowed` function with signature `isLiquidationAllowed()` and selector `0x7a5d20ea`
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
    pub struct IsLiquidationAllowedReturn(pub bool);
}
