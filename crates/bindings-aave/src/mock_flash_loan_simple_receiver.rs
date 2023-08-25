pub use mock_flash_loan_simple_receiver::*;
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
pub mod mock_flash_loan_simple_receiver {
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                                    name: ::std::borrow::ToOwned::to_owned("premium"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("premium"),
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
                    ::std::borrow::ToOwned::to_owned("ExecutedWithSuccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExecutedWithSuccess",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("premium"),
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
    pub static MOCKFLASHLOANSIMPLERECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07:8\x03\x80a\x07:\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xD8V[\x80\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\0\xD8V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0RPa\0\xFC\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xD5W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\0\xEAW`\0\x80\xFD[\x81Qa\0\xF5\x81a\0\xC0V[\x93\x92PPPV[`\x80Q`\xA0Qa\x06\x13a\x01'`\09`\0\x81\x81a\x018\x01Ra\x03T\x01R`\0`\x92\x01Ra\x06\x13`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c^v\xBB\xA3\x11a\0[W\x80c^v\xBB\xA3\x14a\x01\"W\x80cu5\xD2F\x14a\x013W\x80c\xBFD?\x85\x14a\x01ZW\x80c\xE9\xA6\xA2[\x14a\x01mW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x8DW\x80c\x1B\x11\xD0\xFF\x14a\0\xD1W\x80c8\x8Fp\xF1\x14a\0\xF4W\x80cDD\xF31\x14a\x01\x17W[`\0\x80\xFD[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE4a\0\xDF6`\x04a\x04sV[a\x01\x8EV[`@Q\x90\x15\x15\x81R` \x01a\0\xC8V[a\x01\x15a\x01\x026`\x04a\x05jV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[`\x02T`\xFF\x16a\0\xE4V[`\x01T`@Q\x90\x81R` \x01a\0\xC8V[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x15a\x01h6`\x04a\x05\x8EV[`\x01UV[a\x01\x15a\x01{6`\x04a\x05jV[`\x02\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80T`\xFF\x16\x15a\x01\xF1W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R\x7F\x81ojk\xC0\x84\xE1\x99k\xE1\xA81\xAF\xA1\xAF0v=\x05\x01\xB6\xB4;\x9E\x19\"\xA1\x1F4sf\xD7\x90``\x01`@Q\x80\x91\x03\x90\xA1P`\x02T`\xFF\x16\x15a\x04\"V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x027W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02[\x91\x90a\x05\xA7V[\x86\x11\x15a\x02\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid balance for the contract`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01T`\0\x14\x15a\x02\xCAWa\x02\xC5\x87\x87a\x04+V[a\x02\xCEV[`\x01T[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03<\x91\x90a\x05\xC0V[P`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x89\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD0\x91\x90a\x05\xC0V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R\x7F}\x94\xE9\xD0\xC9\x06\xB8\xD7\xB2\xB5*X\x1B\x9E\x9B\xA7(\xAAo\x8C\xD8S+\xD8rC\xD1\x93\xF4t\x01\xBE\x90``\x01`@Q\x80\x91\x03\x90\xA1`\x01\x92PPP[\x95\x94PPPPPV[\x80\x82\x01\x82\x81\x10\x15a\x04;W`\0\x80\xFD[\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04XW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x04\x8BW`\0\x80\xFD[a\x04\x94\x86a\x04AV[\x94P` \x86\x015\x93P`@\x86\x015\x92Pa\x04\xB0``\x87\x01a\x04AV[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x04\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xF3Wa\x04\xF3a\x04]V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\x1BWa\x05\x1Ba\x04]V[\x81`@R\x82\x81R\x8B` \x84\x87\x01\x01\x11\x15a\x054W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95P\x92\x95\x90\x93PV[\x80\x15\x15\x81\x14a\x05gW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05|W`\0\x80\xFD[\x815a\x05\x87\x81a\x05YV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xA0W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xB9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xD2W`\0\x80\xFD[\x81Qa\x05\x87\x81a\x05YV\xFE\xA2dipfsX\"\x12 n\x8B\xF4\xE2\x0B\xC91\xCD\x0C\\T\x0C)bsg\x1C\xF9\xC3\x16k\xC4\xF2\x19\\\xB1\xDF;\xFD\x1F\xF6{dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static MOCKFLASHLOANSIMPLERECEIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c^v\xBB\xA3\x11a\0[W\x80c^v\xBB\xA3\x14a\x01\"W\x80cu5\xD2F\x14a\x013W\x80c\xBFD?\x85\x14a\x01ZW\x80c\xE9\xA6\xA2[\x14a\x01mW`\0\x80\xFD[\x80c\x05B\x97\\\x14a\0\x8DW\x80c\x1B\x11\xD0\xFF\x14a\0\xD1W\x80c8\x8Fp\xF1\x14a\0\xF4W\x80cDD\xF31\x14a\x01\x17W[`\0\x80\xFD[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xE4a\0\xDF6`\x04a\x04sV[a\x01\x8EV[`@Q\x90\x15\x15\x81R` \x01a\0\xC8V[a\x01\x15a\x01\x026`\x04a\x05jV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[`\x02T`\xFF\x16a\0\xE4V[`\x01T`@Q\x90\x81R` \x01a\0\xC8V[a\0\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x15a\x01h6`\x04a\x05\x8EV[`\x01UV[a\x01\x15a\x01{6`\x04a\x05jV[`\x02\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\0\x80T`\xFF\x16\x15a\x01\xF1W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R` \x81\x01\x87\x90R\x90\x81\x01\x85\x90R\x7F\x81ojk\xC0\x84\xE1\x99k\xE1\xA81\xAF\xA1\xAF0v=\x05\x01\xB6\xB4;\x9E\x19\"\xA1\x1F4sf\xD7\x90``\x01`@Q\x80\x91\x03\x90\xA1P`\x02T`\xFF\x16\x15a\x04\"V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x86\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x027W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02[\x91\x90a\x05\xA7V[\x86\x11\x15a\x02\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid balance for the contract`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01T`\0\x14\x15a\x02\xCAWa\x02\xC5\x87\x87a\x04+V[a\x02\xCEV[`\x01T[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x88\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03<\x91\x90a\x05\xC0V[P`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x89\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xD0\x91\x90a\x05\xC0V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R\x7F}\x94\xE9\xD0\xC9\x06\xB8\xD7\xB2\xB5*X\x1B\x9E\x9B\xA7(\xAAo\x8C\xD8S+\xD8rC\xD1\x93\xF4t\x01\xBE\x90``\x01`@Q\x80\x91\x03\x90\xA1`\x01\x92PPP[\x95\x94PPPPPV[\x80\x82\x01\x82\x81\x10\x15a\x04;W`\0\x80\xFD[\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04XW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x04\x8BW`\0\x80\xFD[a\x04\x94\x86a\x04AV[\x94P` \x86\x015\x93P`@\x86\x015\x92Pa\x04\xB0``\x87\x01a\x04AV[\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x04\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xF3Wa\x04\xF3a\x04]V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x05\x1BWa\x05\x1Ba\x04]V[\x81`@R\x82\x81R\x8B` \x84\x87\x01\x01\x11\x15a\x054W`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0` \x84\x83\x01\x01R\x80\x95PPPPPP\x92\x95P\x92\x95\x90\x93PV[\x80\x15\x15\x81\x14a\x05gW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x05|W`\0\x80\xFD[\x815a\x05\x87\x81a\x05YV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\xA0W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xB9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\xD2W`\0\x80\xFD[\x81Qa\x05\x87\x81a\x05YV\xFE\xA2dipfsX\"\x12 n\x8B\xF4\xE2\x0B\xC91\xCD\x0C\\T\x0C)bsg\x1C\xF9\xC3\x16k\xC4\xF2\x19\\\xB1\xDF;\xFD\x1F\xF6{dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFLASHLOANSIMPLERECEIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFlashLoanSimpleReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFlashLoanSimpleReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFlashLoanSimpleReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFlashLoanSimpleReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFlashLoanSimpleReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFlashLoanSimpleReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFlashLoanSimpleReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFLASHLOANSIMPLERECEIVER_ABI.clone(),
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
                MOCKFLASHLOANSIMPLERECEIVER_ABI.clone(),
                MOCKFLASHLOANSIMPLERECEIVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `executeOperation` (0x1b11d0ff) function
        pub fn execute_operation(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            premium: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Address,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 17, 208, 255], (asset, amount, premium, p3, p4))
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
            MockFlashLoanSimpleReceiverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFlashLoanSimpleReceiver<M> {
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
        abi = "ExecutedWithFail(address,uint256,uint256)"
    )]
    pub struct ExecutedWithFailFilter {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub premium: ::ethers::core::types::U256,
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
        abi = "ExecutedWithSuccess(address,uint256,uint256)"
    )]
    pub struct ExecutedWithSuccessFilter {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub premium: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockFlashLoanSimpleReceiverEvents {
        ExecutedWithFailFilter(ExecutedWithFailFilter),
        ExecutedWithSuccessFilter(ExecutedWithSuccessFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockFlashLoanSimpleReceiverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ExecutedWithFailFilter::decode_log(log) {
                return Ok(
                    MockFlashLoanSimpleReceiverEvents::ExecutedWithFailFilter(decoded),
                );
            }
            if let Ok(decoded) = ExecutedWithSuccessFilter::decode_log(log) {
                return Ok(
                    MockFlashLoanSimpleReceiverEvents::ExecutedWithSuccessFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockFlashLoanSimpleReceiverEvents {
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
    impl ::core::convert::From<ExecutedWithFailFilter>
    for MockFlashLoanSimpleReceiverEvents {
        fn from(value: ExecutedWithFailFilter) -> Self {
            Self::ExecutedWithFailFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedWithSuccessFilter>
    for MockFlashLoanSimpleReceiverEvents {
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
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `0x1b11d0ff`
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
        abi = "executeOperation(address,uint256,uint256,address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub premium: ::ethers::core::types::U256,
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
    pub enum MockFlashLoanSimpleReceiverCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        GetAmountToApprove(GetAmountToApproveCall),
        SetAmountToApprove(SetAmountToApproveCall),
        SetFailExecutionTransfer(SetFailExecutionTransferCall),
        SetSimulateEOA(SetSimulateEOACall),
        SimulateEOA(SimulateEOACall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFlashLoanSimpleReceiverCalls {
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
    impl ::ethers::core::abi::AbiEncode for MockFlashLoanSimpleReceiverCalls {
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
    impl ::core::fmt::Display for MockFlashLoanSimpleReceiverCalls {
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
    impl ::core::convert::From<AddressesProviderCall>
    for MockFlashLoanSimpleReceiverCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<PoolCall> for MockFlashLoanSimpleReceiverCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall>
    for MockFlashLoanSimpleReceiverCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<GetAmountToApproveCall>
    for MockFlashLoanSimpleReceiverCalls {
        fn from(value: GetAmountToApproveCall) -> Self {
            Self::GetAmountToApprove(value)
        }
    }
    impl ::core::convert::From<SetAmountToApproveCall>
    for MockFlashLoanSimpleReceiverCalls {
        fn from(value: SetAmountToApproveCall) -> Self {
            Self::SetAmountToApprove(value)
        }
    }
    impl ::core::convert::From<SetFailExecutionTransferCall>
    for MockFlashLoanSimpleReceiverCalls {
        fn from(value: SetFailExecutionTransferCall) -> Self {
            Self::SetFailExecutionTransfer(value)
        }
    }
    impl ::core::convert::From<SetSimulateEOACall> for MockFlashLoanSimpleReceiverCalls {
        fn from(value: SetSimulateEOACall) -> Self {
            Self::SetSimulateEOA(value)
        }
    }
    impl ::core::convert::From<SimulateEOACall> for MockFlashLoanSimpleReceiverCalls {
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
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `0x1b11d0ff`
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
