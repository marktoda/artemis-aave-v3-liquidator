pub use liquidator::*;
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
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("uniswapFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IL2Pool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa\x0C\xBE\x80a\0a`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xE2W\x80c\xB8E\xD3\xC1\x14a\0\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W\x80c\xFAF\x1E3\x14a\x01)W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x82W\x80cBL&[\x14a\0\xBAW\x80cW\x05\xAEC\x14a\0\xCFW[`\0\x80\xFD[a\0\x9Ds\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCDa\0\xC86`\x04a\tLV[a\x01<V[\0[a\0\xCDa\0\xDD6`\x04a\tpV[a\x01\xF9V[`\0Ta\0\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x08a\x01\x036`\x04a\t\xB4V[a\x02\xD4V[`@Q\x90\x81R` \x01a\0\xB1V[a\0\xCDa\x01$6`\x04a\tLV[a\x053V[a\0\xCDa\x0176`\x04a\n\x17V[a\x05\xA8V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF5\x91\x90a\n\xBDV[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02cW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02^W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02^\x91\x90a\n\xBDV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\0a\x03)s3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x03$\x8A\x8A\x8Aa\x07\x8BV[a\x07\xF6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x97\x91\x90a\n\xDFV[\x90P`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x8A\x82\x16\x10\x90\x83\x16c\x12\x8A\xCB\x080\x83a\x03\xBB\x8Ba\x0B\x0EV[\x85a\x03\xE4Wa\x03\xDF`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0B*V[a\x03\xF4V[a\x03\xF4d\x01\0\x02v\xA3`\x01a\x0BQV[\x8F\x8F\x8F\x8E\x8E`@Q` \x01a\x04>\x95\x94\x93\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x90\x94\x16` \x84\x01Rb\xFF\xFF\xFF\x91\x90\x91\x16`@\x83\x01R``\x82\x01R`\x80\x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04m\x95\x94\x93\x92\x91\x90a\x0BqV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xAF\x91\x90a\x0B\xEBV[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1B\x91\x90a\n\xDFV[a\x05%\x91\x90a\x0C\x0FV[\x9A\x99PPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0\x80\x80\x80\x80a\x05\xBA\x86\x88\x01\x88a\x0C/V[\x94P\x94P\x94P\x94P\x94Pa\x05\xECs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x05\xE7\x87\x87\x87a\x07\x8BV[a\x08\xDFV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90Rs\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06TW=`\0\x80>=`\0\xFD[PPPP`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x10a\x06{W\x85\x87a\x06~V[\x86\x86[\x91P\x91P`\0\x8B\x13\x15a\x07\x02W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFC\x91\x90a\n\xBDV[Pa\x07~V[`\0\x8A\x13\x15a\x07~W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07|\x91\x90a\n\xBDV[P[PPPPPPPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07\xC6W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x1EW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0a\x08\xEB\x83\x83a\x07\xF6V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tIW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\t^W`\0\x80\xFD[\x815a\ti\x81a\t4V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\t\x83W`\0\x80\xFD[\x825a\t\x8E\x81a\t4V[\x94` \x93\x90\x93\x015\x93PPPV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\t\xAFW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\xCDW`\0\x80\xFD[\x865a\t\xD8\x81a\t4V[\x95P` \x87\x015a\t\xE8\x81a\t4V[\x94Pa\t\xF6`@\x88\x01a\t\x9CV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n-W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nSW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\ngW`\0\x80\xFD[\x815\x81\x81\x11\x15a\nvW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\n\x88W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\n\xCFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\tiW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xF1W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0B#Wa\x0B#a\n\xF8V[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x0BJWa\x0BJa\n\xF8V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0BJWa\x0BJa\n\xF8V[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R` \x87\x15\x15\x81\x85\x01R\x86`@\x85\x01R\x81\x86\x16``\x85\x01R`\xA0`\x80\x85\x01R\x84Q\x91P\x81`\xA0\x85\x01R`\0[\x82\x81\x10\x15a\x0B\xC6W\x85\x81\x01\x82\x01Q\x85\x82\x01`\xC0\x01R\x81\x01a\x0B\xAAV[PP`\0`\xC0\x82\x85\x01\x01R`\xC0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xFEW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0BJWa\x0BJa\n\xF8V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0CGW`\0\x80\xFD[\x855a\x0CR\x81a\t4V[\x94P` \x86\x015a\x0Cb\x81a\t4V[\x93Pa\x0Cp`@\x87\x01a\t\x9CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xFD\xB07\x19.\x9C\x87\x03\x11\x80\x1A\x03\x82\x7F+\x0E[\xCF\x8C\xD0&\x8E\x82\x02\xFD\x91\x9Dj\\\xD4\xE8<dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xE2W\x80c\xB8E\xD3\xC1\x14a\0\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x16W\x80c\xFAF\x1E3\x14a\x01)W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x82W\x80cBL&[\x14a\0\xBAW\x80cW\x05\xAEC\x14a\0\xCFW[`\0\x80\xFD[a\0\x9Ds\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCDa\0\xC86`\x04a\tLV[a\x01<V[\0[a\0\xCDa\0\xDD6`\x04a\tpV[a\x01\xF9V[`\0Ta\0\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x08a\x01\x036`\x04a\t\xB4V[a\x02\xD4V[`@Q\x90\x81R` \x01a\0\xB1V[a\0\xCDa\x01$6`\x04a\tLV[a\x053V[a\0\xCDa\x0176`\x04a\n\x17V[a\x05\xA8V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`@Q\x80\x91\x03\x90\xFD[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF5\x91\x90a\n\xBDV[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02cW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x02^W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02^\x91\x90a\n\xBDV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\0a\x03)s3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x03$\x8A\x8A\x8Aa\x07\x8BV[a\x07\xF6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x97\x91\x90a\n\xDFV[\x90P`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x8A\x82\x16\x10\x90\x83\x16c\x12\x8A\xCB\x080\x83a\x03\xBB\x8Ba\x0B\x0EV[\x85a\x03\xE4Wa\x03\xDF`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\x0B*V[a\x03\xF4V[a\x03\xF4d\x01\0\x02v\xA3`\x01a\x0BQV[\x8F\x8F\x8F\x8E\x8E`@Q` \x01a\x04>\x95\x94\x93\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x90\x94\x16` \x84\x01Rb\xFF\xFF\xFF\x91\x90\x91\x16`@\x83\x01R``\x82\x01R`\x80\x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04m\x95\x94\x93\x92\x91\x90a\x0BqV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xAF\x91\x90a\x0B\xEBV[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1B\x91\x90a\n\xDFV[a\x05%\x91\x90a\x0C\x0FV[\x9A\x99PPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01f\x90a\n\x97V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0\x80\x80\x80\x80a\x05\xBA\x86\x88\x01\x88a\x0C/V[\x94P\x94P\x94P\x94P\x94Pa\x05\xECs3\x12\x8A\x8F\xC1xi\x89}\xCEh\xED\x02miF!\xF6\xFD\xFDa\x05\xE7\x87\x87\x87a\x07\x8BV[a\x08\xDFV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90Rs\xA28\xDD\x80\xC2Y\xA7.\x81\xD7\xE4fJ\x98\x01Y?\x98\xD1\xC5\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06TW=`\0\x80>=`\0\xFD[PPPP`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x10a\x06{W\x85\x87a\x06~V[\x86\x86[\x91P\x91P`\0\x8B\x13\x15a\x07\x02W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xFC\x91\x90a\n\xBDV[Pa\x07~V[`\0\x8A\x13\x15a\x07~W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07|\x91\x90a\n\xBDV[P[PPPPPPPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x07\xC6W\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[`\0\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x08\x1EW`\0\x80\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\xE3O\x19\x9B\x19\xB2\xB4\xF4\x7FhD&\x19\xD5UR}$Ox\xA3)~\xA8\x93%\xF8C\xF8{\x8BT`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0a\x08\xEB\x83\x83a\x07\xF6V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x01fV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\tIW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\t^W`\0\x80\xFD[\x815a\ti\x81a\t4V[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\t\x83W`\0\x80\xFD[\x825a\t\x8E\x81a\t4V[\x94` \x93\x90\x93\x015\x93PPPV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\t\xAFW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\t\xCDW`\0\x80\xFD[\x865a\t\xD8\x81a\t4V[\x95P` \x87\x015a\t\xE8\x81a\t4V[\x94Pa\t\xF6`@\x88\x01a\t\x9CV[\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n-W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nSW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\ngW`\0\x80\xFD[\x815\x81\x81\x11\x15a\nvW`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\n\x88W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\n\xCFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\tiW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xF1W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0B#Wa\x0B#a\n\xF8V[P`\0\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15a\x0BJWa\x0BJa\n\xF8V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15a\x0BJWa\x0BJa\n\xF8V[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R` \x87\x15\x15\x81\x85\x01R\x86`@\x85\x01R\x81\x86\x16``\x85\x01R`\xA0`\x80\x85\x01R\x84Q\x91P\x81`\xA0\x85\x01R`\0[\x82\x81\x10\x15a\x0B\xC6W\x85\x81\x01\x82\x01Q\x85\x82\x01`\xC0\x01R\x81\x01a\x0B\xAAV[PP`\0`\xC0\x82\x85\x01\x01R`\xC0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xFEW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x0BJWa\x0BJa\n\xF8V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0CGW`\0\x80\xFD[\x855a\x0CR\x81a\t4V[\x94P` \x86\x015a\x0Cb\x81a\t4V[\x93Pa\x0Cp`@\x87\x01a\t\x9CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xFD\xB07\x19.\x9C\x87\x03\x11\x80\x1A\x03\x82\x7F+\x0E[\xCF\x8C\xD0&\x8E\x82\x02\xFD\x91\x9Dj\\\xD4\xE8<dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
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
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0xb845d3c1) function
        pub fn liquidate(
            &self,
            collateral: ::ethers::core::types::Address,
            debt: ::ethers::core::types::Address,
            uniswap_fee: u32,
            debt_to_cover: ::ethers::core::types::U256,
            liquidation_arg_1: [u8; 32],
            liquidation_arg_2: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [184, 69, 211, 193],
                    (
                        collateral,
                        debt,
                        uniswap_fee,
                        debt_to_cover,
                        liquidation_arg_1,
                        liquidation_arg_2,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
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
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,uint24,uint256,bytes32,bytes32)` and selector `0xb845d3c1`
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
        name = "liquidate",
        abi = "liquidate(address,address,uint24,uint256,bytes32,bytes32)"
    )]
    pub struct LiquidateCall {
        pub collateral: ::ethers::core::types::Address,
        pub debt: ::ethers::core::types::Address,
        pub uniswap_fee: u32,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub liquidation_arg_1: [u8; 32],
        pub liquidation_arg_2: [u8; 32],
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
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
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
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
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorCalls {
        ApprovePool(ApprovePoolCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded)
                = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded)
                = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,uint24,uint256,bytes32,bytes32)` and selector `0xb845d3c1`
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
    pub struct LiquidateReturn {
        pub collateral_gain: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
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
}
