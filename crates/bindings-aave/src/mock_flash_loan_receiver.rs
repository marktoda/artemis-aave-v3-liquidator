pub use mock_flash_loan_receiver::*;
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
pub mod mock_flash_loan_receiver {
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
                    ::std::borrow::ToOwned::to_owned("executeOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeOperation"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("premiums"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("getAmountToApprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAmountToApprove"),
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
                    ::std::borrow::ToOwned::to_owned("setAmountToApprove"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAmountToApprove"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToApprove"),
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
                    ::std::borrow::ToOwned::to_owned("setFailExecutionTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setFailExecutionTransfer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fail"),
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
                    ::std::borrow::ToOwned::to_owned("setSimulateEOA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSimulateEOA"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("flag"),
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
                    ::std::borrow::ToOwned::to_owned("simulateEOA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateEOA"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ExecutedWithFail"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExecutedWithFail"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_premiums"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutedWithSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutedWithSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_premiums"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
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
    pub static MOCKFLASHLOANRECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\nW8\x03\x80a\nW\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xD8V[\x80\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\0\xD8V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0RPa\0\xFC\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xD5W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\0\xEAW`\0\x80\xFD[\x81Qa\0\xF5\x81a\0\xC0V[\x93\x92PPPV[`\x80Q`\xA0Qa\t0a\x01'`\09`\0\x81\x81a\x01!\x01Ra\x04\x0F\x01R`\0`\x92\x01Ra\t0`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cu5\xD2F\x11a\0[W\x80cu5\xD2F\x14a\x01\x1CW\x80c\x92\x0F\\\x84\x14a\x01CW\x80c\xBFD?\x85\x14a\x01VW\x80c\xE9\xA6\xA2[\x14a\x01iW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x8DW\x80c8\x8Fp\xF1\x14a\0\xD1W\x80cDD\xF31\x14a\0\xF4W\x80c^v\xBB\xA3\x14a\x01\x0BW[`\0\x80\xFD[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF2a\0\xDF6`\x04a\x04\xFCV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[`\x02T`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01a\0\xC8V[`\x01T`@Q\x90\x81R` \x01a\0\xC8V[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFBa\x01Q6`\x04a\x06\x82V[a\x01\x8AV[a\0\xF2a\x01d6`\x04a\x07\x9CV[`\x01UV[a\0\xF2a\x01w6`\x04a\x04\xFCV[`\x02\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80T`\xFF\x16\x15a\x01\xDEW\x7F\x99r\xB2\x12\xE5)\x13x0r\xB9`\xDDARz\xE8\xB6\xE6\t\xD0\x17\xB6@9u\x8D\xDA\x0C\xE4\x12x\x86\x86\x86`@Qa\x01\xC9\x93\x92\x91\x90a\x07\xF0V[`@Q\x80\x91\x03\x90\xA1P`\x02T`\xFF\x16\x15a\x04\xE2V[`\0[\x86Q\x81\x10\x15a\x04\xA2W`\0\x87\x82\x81Q\x81\x10a\x01\xFEWa\x01\xFEa\x08eV[` \x02` \x01\x01Q\x90P\x87\x82\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x08eV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8E\x91\x90a\x08{V[\x87\x83\x81Q\x81\x10a\x02\xA0Wa\x02\xA0a\x08eV[` \x02` \x01\x01Q\x11\x15a\x02\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid balance for the contract`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01T`\0\x14\x15a\x03JW\x86\x83\x81Q\x81\x10a\x03\x19Wa\x03\x19a\x08eV[` \x02` \x01\x01Q\x88\x84\x81Q\x81\x10a\x033Wa\x033a\x08eV[` \x02` \x01\x01Qa\x03E\x91\x90a\x08\xAAV[a\x03NV[`\x01T[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-h\x88\x85\x81Q\x81\x10a\x03qWa\x03qa\x08eV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x97\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDA\x91\x90a\x08\xC2V[P\x88\x83\x81Q\x81\x10a\x03\xEDWa\x03\xEDa\x08eV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8C\x91\x90a\x08\xC2V[PPP\x80\x80a\x04\x9A\x90a\x08\xDFV[\x91PPa\x01\xE1V[P\x7F\xBDkk\xFA\xC5\x96\x12vZ\x81\xCCO\xDE\xE7J\xB4\x85\x96q\xFA\x14\xA5b\x05o\x9E\xEAC\x875\xA7\x8A\x86\x86\x86`@Qa\x04\xD6\x93\x92\x91\x90a\x07\xF0V[`@Q\x80\x91\x03\x90\xA1P`\x01[\x95\x94PPPPPV[\x80\x15\x15\x81\x14a\x04\xF9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05\x0EW`\0\x80\xFD[\x815a\x05\x19\x81a\x04\xEBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05_Wa\x05_a\x05 V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\x81Wa\x05\x81a\x05 V[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA2W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05\xB8W`\0\x80\xFD[\x815` a\x05\xCDa\x05\xC8\x83a\x05gV[a\x056V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x05\xECW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x06\x07W\x805\x83R\x91\x83\x01\x91\x83\x01a\x05\xF0V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x06#W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06=Wa\x06=a\x05 V[a\x06P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x056V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06\x9AW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB2W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x06\xC6W`\0\x80\xFD[\x815` a\x06\xD6a\x05\xC8\x83a\x05gV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8C\x84\x11\x15a\x06\xF5W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x07\x1AWa\x07\x0B\x86a\x05\x8BV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x06\xFAV[\x99PP\x89\x015\x92PP\x80\x82\x11\x15a\x070W`\0\x80\xFD[a\x07<\x89\x83\x8A\x01a\x05\xA7V[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x07RW`\0\x80\xFD[a\x07^\x89\x83\x8A\x01a\x05\xA7V[\x94Pa\x07l``\x89\x01a\x05\x8BV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x07\x82W`\0\x80\xFD[Pa\x07\x8F\x88\x82\x89\x01a\x06\x12V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x07\xAEW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07\xE5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xC9V[P\x94\x95\x94PPPPPV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\x082W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x08\rV[PPP\x83\x81\x03\x82\x85\x01Ra\x08F\x81\x87a\x07\xB5V[\x91PP\x82\x81\x03`@\x84\x01Ra\x08[\x81\x85a\x07\xB5V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\x8DW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x08\xBDWa\x08\xBDa\x08\x94V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x08\xD4W`\0\x80\xFD[\x81Qa\x05\x19\x81a\x04\xEBV[`\0`\0\x19\x82\x14\x15a\x08\xF3Wa\x08\xF3a\x08\x94V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xB6\x8B\xCA8eq[V\n\xC3\x11L\xECy\xB5\xF6}\xA4\xAE\xD5\x19\xF8\xCA\xD1M\xB1?C*f\x92\xE7dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static MOCKFLASHLOANRECEIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cu5\xD2F\x11a\0[W\x80cu5\xD2F\x14a\x01\x1CW\x80c\x92\x0F\\\x84\x14a\x01CW\x80c\xBFD?\x85\x14a\x01VW\x80c\xE9\xA6\xA2[\x14a\x01iW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x8DW\x80c8\x8Fp\xF1\x14a\0\xD1W\x80cDD\xF31\x14a\0\xF4W\x80c^v\xBB\xA3\x14a\x01\x0BW[`\0\x80\xFD[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF2a\0\xDF6`\x04a\x04\xFCV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[`\x02T`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01a\0\xC8V[`\x01T`@Q\x90\x81R` \x01a\0\xC8V[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFBa\x01Q6`\x04a\x06\x82V[a\x01\x8AV[a\0\xF2a\x01d6`\x04a\x07\x9CV[`\x01UV[a\0\xF2a\x01w6`\x04a\x04\xFCV[`\x02\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80T`\xFF\x16\x15a\x01\xDEW\x7F\x99r\xB2\x12\xE5)\x13x0r\xB9`\xDDARz\xE8\xB6\xE6\t\xD0\x17\xB6@9u\x8D\xDA\x0C\xE4\x12x\x86\x86\x86`@Qa\x01\xC9\x93\x92\x91\x90a\x07\xF0V[`@Q\x80\x91\x03\x90\xA1P`\x02T`\xFF\x16\x15a\x04\xE2V[`\0[\x86Q\x81\x10\x15a\x04\xA2W`\0\x87\x82\x81Q\x81\x10a\x01\xFEWa\x01\xFEa\x08eV[` \x02` \x01\x01Q\x90P\x87\x82\x81Q\x81\x10a\x02\x1AWa\x02\x1Aa\x08eV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8E\x91\x90a\x08{V[\x87\x83\x81Q\x81\x10a\x02\xA0Wa\x02\xA0a\x08eV[` \x02` \x01\x01Q\x11\x15a\x02\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid balance for the contract`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01T`\0\x14\x15a\x03JW\x86\x83\x81Q\x81\x10a\x03\x19Wa\x03\x19a\x08eV[` \x02` \x01\x01Q\x88\x84\x81Q\x81\x10a\x033Wa\x033a\x08eV[` \x02` \x01\x01Qa\x03E\x91\x90a\x08\xAAV[a\x03NV[`\x01T[\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16c\xA0q-h\x88\x85\x81Q\x81\x10a\x03qWa\x03qa\x08eV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x97\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDA\x91\x90a\x08\xC2V[P\x88\x83\x81Q\x81\x10a\x03\xEDWa\x03\xEDa\x08eV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8C\x91\x90a\x08\xC2V[PPP\x80\x80a\x04\x9A\x90a\x08\xDFV[\x91PPa\x01\xE1V[P\x7F\xBDkk\xFA\xC5\x96\x12vZ\x81\xCCO\xDE\xE7J\xB4\x85\x96q\xFA\x14\xA5b\x05o\x9E\xEAC\x875\xA7\x8A\x86\x86\x86`@Qa\x04\xD6\x93\x92\x91\x90a\x07\xF0V[`@Q\x80\x91\x03\x90\xA1P`\x01[\x95\x94PPPPPV[\x80\x15\x15\x81\x14a\x04\xF9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05\x0EW`\0\x80\xFD[\x815a\x05\x19\x81a\x04\xEBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05_Wa\x05_a\x05 V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\x81Wa\x05\x81a\x05 V[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xA2W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05\xB8W`\0\x80\xFD[\x815` a\x05\xCDa\x05\xC8\x83a\x05gV[a\x056V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x05\xECW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x06\x07W\x805\x83R\x91\x83\x01\x91\x83\x01a\x05\xF0V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x06#W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06=Wa\x06=a\x05 V[a\x06P`\x1F\x82\x01`\x1F\x19\x16` \x01a\x056V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06eW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x06\x9AW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB2W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x06\xC6W`\0\x80\xFD[\x815` a\x06\xD6a\x05\xC8\x83a\x05gV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8C\x84\x11\x15a\x06\xF5W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x07\x1AWa\x07\x0B\x86a\x05\x8BV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x06\xFAV[\x99PP\x89\x015\x92PP\x80\x82\x11\x15a\x070W`\0\x80\xFD[a\x07<\x89\x83\x8A\x01a\x05\xA7V[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x07RW`\0\x80\xFD[a\x07^\x89\x83\x8A\x01a\x05\xA7V[\x94Pa\x07l``\x89\x01a\x05\x8BV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x07\x82W`\0\x80\xFD[Pa\x07\x8F\x88\x82\x89\x01a\x06\x12V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x07\xAEW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07\xE5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\xC9V[P\x94\x95\x94PPPPPV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\x082W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x08\rV[PPP\x83\x81\x03\x82\x85\x01Ra\x08F\x81\x87a\x07\xB5V[\x91PP\x82\x81\x03`@\x84\x01Ra\x08[\x81\x85a\x07\xB5V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\x8DW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x08\xBDWa\x08\xBDa\x08\x94V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x08\xD4W`\0\x80\xFD[\x81Qa\x05\x19\x81a\x04\xEBV[`\0`\0\x19\x82\x14\x15a\x08\xF3Wa\x08\xF3a\x08\x94V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xB6\x8B\xCA8eq[V\n\xC3\x11L\xECy\xB5\xF6}\xA4\xAE\xD5\x19\xF8\xCA\xD1M\xB1?C*f\x92\xE7dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFLASHLOANRECEIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFlashLoanReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFlashLoanReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFlashLoanReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFlashLoanReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFlashLoanReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFlashLoanReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFlashLoanReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFLASHLOANRECEIVER_ABI.clone(),
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
                MOCKFLASHLOANRECEIVER_ABI.clone(),
                MOCKFLASHLOANRECEIVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `executeOperation` (0x920f5c84) function
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            premiums: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::ethers::core::types::Address,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 15, 92, 132], (assets, amounts, premiums, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountToApprove` (0x5e76bba3) function
        pub fn get_amount_to_approve(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 118, 187, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAmountToApprove` (0xbf443f85) function
        pub fn set_amount_to_approve(
            &self,
            amount_to_approve: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 68, 63, 133], amount_to_approve)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFailExecutionTransfer` (0x388f70f1) function
        pub fn set_fail_execution_transfer(
            &self,
            fail: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 143, 112, 241], fail)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSimulateEOA` (0xe9a6a25b) function
        pub fn set_simulate_eoa(
            &self,
            flag: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 166, 162, 91], flag)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateEOA` (0x4444f331) function
        pub fn simulate_eoa(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 68, 243, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ExecutedWithFail` event
        pub fn executed_with_fail_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedWithFailFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecutedWithSuccess` event
        pub fn executed_with_success_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedWithSuccessFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MockFlashLoanReceiverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFlashLoanReceiver<M> {
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
    #[ethevent(
        name = "ExecutedWithFail",
        abi = "ExecutedWithFail(address[],uint256[],uint256[])"
    )]
    pub struct ExecutedWithFailFilter {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<::ethers::core::types::U256>,
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
        name = "ExecutedWithSuccess",
        abi = "ExecutedWithSuccess(address[],uint256[],uint256[])"
    )]
    pub struct ExecutedWithSuccessFilter {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockFlashLoanReceiverEvents {
        ExecutedWithFailFilter(ExecutedWithFailFilter),
        ExecutedWithSuccessFilter(ExecutedWithSuccessFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockFlashLoanReceiverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ExecutedWithFailFilter::decode_log(log) {
                return Ok(MockFlashLoanReceiverEvents::ExecutedWithFailFilter(decoded));
            }
            if let Ok(decoded) = ExecutedWithSuccessFilter::decode_log(log) {
                return Ok(
                    MockFlashLoanReceiverEvents::ExecutedWithSuccessFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockFlashLoanReceiverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecutedWithFailFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutedWithSuccessFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ExecutedWithFailFilter> for MockFlashLoanReceiverEvents {
        fn from(value: ExecutedWithFailFilter) -> Self {
            Self::ExecutedWithFailFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedWithSuccessFilter>
    for MockFlashLoanReceiverEvents {
        fn from(value: ExecutedWithSuccessFilter) -> Self {
            Self::ExecutedWithSuccessFilter(value)
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
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
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
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<::ethers::core::types::U256>,
        pub p3: ::ethers::core::types::Address,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getAmountToApprove` function with signature `getAmountToApprove()` and selector `0x5e76bba3`
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
    #[ethcall(name = "getAmountToApprove", abi = "getAmountToApprove()")]
    pub struct GetAmountToApproveCall;
    ///Container type for all input parameters for the `setAmountToApprove` function with signature `setAmountToApprove(uint256)` and selector `0xbf443f85`
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
    #[ethcall(name = "setAmountToApprove", abi = "setAmountToApprove(uint256)")]
    pub struct SetAmountToApproveCall {
        pub amount_to_approve: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFailExecutionTransfer` function with signature `setFailExecutionTransfer(bool)` and selector `0x388f70f1`
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
    #[ethcall(name = "setFailExecutionTransfer", abi = "setFailExecutionTransfer(bool)")]
    pub struct SetFailExecutionTransferCall {
        pub fail: bool,
    }
    ///Container type for all input parameters for the `setSimulateEOA` function with signature `setSimulateEOA(bool)` and selector `0xe9a6a25b`
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
    #[ethcall(name = "setSimulateEOA", abi = "setSimulateEOA(bool)")]
    pub struct SetSimulateEOACall {
        pub flag: bool,
    }
    ///Container type for all input parameters for the `simulateEOA` function with signature `simulateEOA()` and selector `0x4444f331`
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
    #[ethcall(name = "simulateEOA", abi = "simulateEOA()")]
    pub struct SimulateEOACall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockFlashLoanReceiverCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        GetAmountToApprove(GetAmountToApproveCall),
        SetAmountToApprove(SetAmountToApproveCall),
        SetFailExecutionTransfer(SetFailExecutionTransferCall),
        SetSimulateEOA(SetSimulateEOACall),
        SimulateEOA(SimulateEOACall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFlashLoanReceiverCalls {
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
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded)
                = <ExecuteOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteOperation(decoded));
            }
            if let Ok(decoded)
                = <GetAmountToApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAmountToApprove(decoded));
            }
            if let Ok(decoded)
                = <SetAmountToApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetAmountToApprove(decoded));
            }
            if let Ok(decoded)
                = <SetFailExecutionTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetFailExecutionTransfer(decoded));
            }
            if let Ok(decoded)
                = <SetSimulateEOACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSimulateEOA(decoded));
            }
            if let Ok(decoded)
                = <SimulateEOACall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SimulateEOA(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFlashLoanReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountToApprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAmountToApprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFailExecutionTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSimulateEOA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateEOA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFlashLoanReceiverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountToApprove(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAmountToApprove(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetFailExecutionTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSimulateEOA(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateEOA(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for MockFlashLoanReceiverCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<PoolCall> for MockFlashLoanReceiverCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for MockFlashLoanReceiverCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<GetAmountToApproveCall> for MockFlashLoanReceiverCalls {
        fn from(value: GetAmountToApproveCall) -> Self {
            Self::GetAmountToApprove(value)
        }
    }
    impl ::core::convert::From<SetAmountToApproveCall> for MockFlashLoanReceiverCalls {
        fn from(value: SetAmountToApproveCall) -> Self {
            Self::SetAmountToApprove(value)
        }
    }
    impl ::core::convert::From<SetFailExecutionTransferCall>
    for MockFlashLoanReceiverCalls {
        fn from(value: SetFailExecutionTransferCall) -> Self {
            Self::SetFailExecutionTransfer(value)
        }
    }
    impl ::core::convert::From<SetSimulateEOACall> for MockFlashLoanReceiverCalls {
        fn from(value: SetSimulateEOACall) -> Self {
            Self::SetSimulateEOA(value)
        }
    }
    impl ::core::convert::From<SimulateEOACall> for MockFlashLoanReceiverCalls {
        fn from(value: SimulateEOACall) -> Self {
            Self::SimulateEOA(value)
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
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
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
    pub struct ExecuteOperationReturn(pub bool);
    ///Container type for all return fields from the `getAmountToApprove` function with signature `getAmountToApprove()` and selector `0x5e76bba3`
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
    pub struct GetAmountToApproveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateEOA` function with signature `simulateEOA()` and selector `0x4444f331`
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
    pub struct SimulateEOAReturn(pub bool);
}
