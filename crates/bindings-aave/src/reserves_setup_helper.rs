pub use reserves_setup_helper::*;
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
pub mod reserves_setup_helper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("configureReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configureReserves"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("configurator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract PoolConfigurator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ReservesSetupHelper.ConfigureReserveInput[]",
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
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
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
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
    pub static RESERVESSETUPHELPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa\t]\x80a\0a`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c#\xBB\x10\x93\x14a\0QW\x80cqP\x18\xA6\x14a\0fW\x80c\x8D\xA5\xCB[\x14a\0nW\x80c\xF2\xFD\xE3\x8B\x14a\0\x8DW[`\0\x80\xFD[a\0da\0_6`\x04a\x07\xE4V[a\0\xA0V[\0[a\0da\x06nV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0da\0\x9B6`\x04a\x08mV[a\x06\xE2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x06hW\x83`\x01`\x01`\xA0\x1B\x03\x16c|NV\x0B\x84\x84\x84\x81\x81\x10a\0\xFFWa\0\xFFa\x08\xC6V[a\x01\x16\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x01(Wa\x01(a\x08\xC6V[\x90Pa\x01@\x02\x01` \x015\x86\x86\x86\x81\x81\x10a\x01EWa\x01Ea\x08\xC6V[\x90Pa\x01@\x02\x01`@\x015\x87\x87\x87\x81\x81\x10a\x01bWa\x01ba\x08\xC6V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x89\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01\x94\x90\x94RP`D\x84\x01\x91\x90\x91R``a\x01@\x90\x92\x02\x01\x015`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xD8W=`\0\x80>=`\0\xFD[PPPP\x82\x82\x82\x81\x81\x10a\x01\xEEWa\x01\xEEa\x08\xC6V[\x90Pa\x01@\x02\x01a\x01\0\x01` \x81\x01\x90a\x02\x08\x91\x90a\x08\xDCV[\x15a\x04!W\x83`\x01`\x01`\xA0\x1B\x03\x16ch,\xF2d\x84\x84\x84\x81\x81\x10a\x02.Wa\x02.a\x08\xC6V[a\x02E\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\x01`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA1W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xD1J\t\x83\x84\x84\x84\x81\x81\x10a\x02\xC6Wa\x02\xC6a\x08\xC6V[a\x02\xDD\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x02\xEFWa\x02\xEFa\x08\xC6V[\x90Pa\x01@\x02\x01`\xA0\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03+\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03YW=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16c\x8Au\x1A`\x84\x84\x84\x81\x81\x10a\x03~Wa\x03~a\x08\xC6V[a\x03\x95\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x03\xA7Wa\x03\xA7a\x08\xC6V[\x90Pa\x01@\x02\x01`\xE0\x01` \x81\x01\x90a\x03\xC0\x91\x90a\x08\xDCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x1CW=`\0\x80>=`\0\xFD[PPPP[\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF2\x13\xEF\x0E\x84\x84\x84\x81\x81\x10a\x04BWa\x04Ba\x08\xC6V[a\x04Y\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x04kWa\x04ka\x08\xC6V[\x90Pa\x01@\x02\x01a\x01 \x01` \x81\x01\x90a\x04\x85\x91\x90a\x08\xDCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xCDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE1W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16cW\x1F\x03\xE5\x84\x84\x84\x81\x81\x10a\x05\x06Wa\x05\x06a\x08\xC6V[a\x05\x1D\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x05/Wa\x05/a\x08\xC6V[\x90Pa\x01@\x02\x01`\xC0\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05k\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x99W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16cKNgS\x84\x84\x84\x81\x81\x10a\x05\xBEWa\x05\xBEa\x08\xC6V[a\x05\xD5\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x05\xE7Wa\x05\xE7a\x08\xC6V[\x90Pa\x01@\x02\x01`\x80\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06#\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06=W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06QW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x06`\x90a\x08\xFEV[\x91PPa\0\xD6V[PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\0\xCAV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE1W`\0\x80\xFD[PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xF9W`\0\x80\xFD[\x835a\x08\x04\x81a\x07\xCCV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08!W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x085W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08DW`\0\x80\xFD[\x87` a\x01@\x83\x02\x85\x01\x01\x11\x15a\x08ZW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x08\x7FW`\0\x80\xFD[\x815a\x08\x8A\x81a\x07\xCCV[\x93\x92PPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xEEW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x08\x8AW`\0\x80\xFD[`\0`\0\x19\x82\x14\x15a\t WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xDD)X\xD7A\x89\xBC\xFF\xBDE\x1B\xCD\xD0\x7F\xFEP\xD4\xCCQ\xDE\xE7\xEB\xEFo*\xDA\xA6\xC7\xF5\x8B\xE9\xDDdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static RESERVESSETUPHELPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c#\xBB\x10\x93\x14a\0QW\x80cqP\x18\xA6\x14a\0fW\x80c\x8D\xA5\xCB[\x14a\0nW\x80c\xF2\xFD\xE3\x8B\x14a\0\x8DW[`\0\x80\xFD[a\0da\0_6`\x04a\x07\xE4V[a\0\xA0V[\0[a\0da\x06nV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0da\0\x9B6`\x04a\x08mV[a\x06\xE2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x06hW\x83`\x01`\x01`\xA0\x1B\x03\x16c|NV\x0B\x84\x84\x84\x81\x81\x10a\0\xFFWa\0\xFFa\x08\xC6V[a\x01\x16\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x01(Wa\x01(a\x08\xC6V[\x90Pa\x01@\x02\x01` \x015\x86\x86\x86\x81\x81\x10a\x01EWa\x01Ea\x08\xC6V[\x90Pa\x01@\x02\x01`@\x015\x87\x87\x87\x81\x81\x10a\x01bWa\x01ba\x08\xC6V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x89\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x96\x16`\x04\x87\x01R`$\x86\x01\x94\x90\x94RP`D\x84\x01\x91\x90\x91R``a\x01@\x90\x92\x02\x01\x015`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xD8W=`\0\x80>=`\0\xFD[PPPP\x82\x82\x82\x81\x81\x10a\x01\xEEWa\x01\xEEa\x08\xC6V[\x90Pa\x01@\x02\x01a\x01\0\x01` \x81\x01\x90a\x02\x08\x91\x90a\x08\xDCV[\x15a\x04!W\x83`\x01`\x01`\xA0\x1B\x03\x16ch,\xF2d\x84\x84\x84\x81\x81\x10a\x02.Wa\x02.a\x08\xC6V[a\x02E\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`\x01`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA1W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xD1J\t\x83\x84\x84\x84\x81\x81\x10a\x02\xC6Wa\x02\xC6a\x08\xC6V[a\x02\xDD\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x02\xEFWa\x02\xEFa\x08\xC6V[\x90Pa\x01@\x02\x01`\xA0\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03+\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03YW=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16c\x8Au\x1A`\x84\x84\x84\x81\x81\x10a\x03~Wa\x03~a\x08\xC6V[a\x03\x95\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x03\xA7Wa\x03\xA7a\x08\xC6V[\x90Pa\x01@\x02\x01`\xE0\x01` \x81\x01\x90a\x03\xC0\x91\x90a\x08\xDCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x08W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x1CW=`\0\x80>=`\0\xFD[PPPP[\x83`\x01`\x01`\xA0\x1B\x03\x16c\xF2\x13\xEF\x0E\x84\x84\x84\x81\x81\x10a\x04BWa\x04Ba\x08\xC6V[a\x04Y\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x04kWa\x04ka\x08\xC6V[\x90Pa\x01@\x02\x01a\x01 \x01` \x81\x01\x90a\x04\x85\x91\x90a\x08\xDCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x15\x15`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xCDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE1W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16cW\x1F\x03\xE5\x84\x84\x84\x81\x81\x10a\x05\x06Wa\x05\x06a\x08\xC6V[a\x05\x1D\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x05/Wa\x05/a\x08\xC6V[\x90Pa\x01@\x02\x01`\xC0\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05k\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x99W=`\0\x80>=`\0\xFD[PPPP\x83`\x01`\x01`\xA0\x1B\x03\x16cKNgS\x84\x84\x84\x81\x81\x10a\x05\xBEWa\x05\xBEa\x08\xC6V[a\x05\xD5\x92` a\x01@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08mV[\x85\x85\x85\x81\x81\x10a\x05\xE7Wa\x05\xE7a\x08\xC6V[\x90Pa\x01@\x02\x01`\x80\x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06#\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06=W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06QW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x06`\x90a\x08\xFEV[\x91PPa\0\xD6V[PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\0\xCA\x90a\x08\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\0\xCAV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE1W`\0\x80\xFD[PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x07\xF9W`\0\x80\xFD[\x835a\x08\x04\x81a\x07\xCCV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08!W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x085W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x08DW`\0\x80\xFD[\x87` a\x01@\x83\x02\x85\x01\x01\x11\x15a\x08ZW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x08\x7FW`\0\x80\xFD[\x815a\x08\x8A\x81a\x07\xCCV[\x93\x92PPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x08\xEEW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x08\x8AW`\0\x80\xFD[`\0`\0\x19\x82\x14\x15a\t WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xDD)X\xD7A\x89\xBC\xFF\xBDE\x1B\xCD\xD0\x7F\xFEP\xD4\xCCQ\xDE\xE7\xEB\xEFo*\xDA\xA6\xC7\xF5\x8B\xE9\xDDdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static RESERVESSETUPHELPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ReservesSetupHelper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReservesSetupHelper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReservesSetupHelper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReservesSetupHelper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReservesSetupHelper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReservesSetupHelper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReservesSetupHelper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESERVESSETUPHELPER_ABI.clone(),
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
                RESERVESSETUPHELPER_ABI.clone(),
                RESERVESSETUPHELPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `configureReserves` (0x23bb1093) function
        pub fn configure_reserves(
            &self,
            configurator: ::ethers::core::types::Address,
            input_params: ::std::vec::Vec<ConfigureReserveInput>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 187, 16, 147], (configurator, input_params))
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
    for ReservesSetupHelper<M> {
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
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `configureReserves` function with signature `configureReserves(address,(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool)[])` and selector `0x23bb1093`
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
        name = "configureReserves",
        abi = "configureReserves(address,(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool)[])"
    )]
    pub struct ConfigureReservesCall {
        pub configurator: ::ethers::core::types::Address,
        pub input_params: ::std::vec::Vec<ConfigureReserveInput>,
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ReservesSetupHelperCalls {
        ConfigureReserves(ConfigureReservesCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReservesSetupHelperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConfigureReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigureReserves(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReservesSetupHelperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConfigureReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReservesSetupHelperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfigureReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConfigureReservesCall> for ReservesSetupHelperCalls {
        fn from(value: ConfigureReservesCall) -> Self {
            Self::ConfigureReserves(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ReservesSetupHelperCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ReservesSetupHelperCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ReservesSetupHelperCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
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
    ///`ConfigureReserveInput(address,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool)`
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
    pub struct ConfigureReserveInput {
        pub asset: ::ethers::core::types::Address,
        pub base_ltv: ::ethers::core::types::U256,
        pub liquidation_threshold: ::ethers::core::types::U256,
        pub liquidation_bonus: ::ethers::core::types::U256,
        pub reserve_factor: ::ethers::core::types::U256,
        pub borrow_cap: ::ethers::core::types::U256,
        pub supply_cap: ::ethers::core::types::U256,
        pub stable_borrowing_enabled: bool,
        pub borrowing_enabled: bool,
        pub flash_loan_enabled: bool,
    }
}
