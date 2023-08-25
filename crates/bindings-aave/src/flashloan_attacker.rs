pub use flashloan_attacker::*;
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
pub mod flashloan_attacker {
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
                    ::std::borrow::ToOwned::to_owned("supplyAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supplyAsset"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static FLASHLOANATTACKER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\ne8\x03\x80a\ne\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01fV[\x80\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\x01fV[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP\x80`\x01`\x01`\xA0\x1B\x03\x16c\x02k\x1D_`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01(\x91\x90a\x01fV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\x01\x8AV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01cW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x01xW`\0\x80\xFD[\x81Qa\x01\x83\x81a\x01NV[\x93\x92PPPV[`\x80Q`\xA0Qa\x08\xB1a\x01\xB4`\09`\0\x81\x81`\xD2\x01Ra\x02\xEF\x01R`\0`V\x01Ra\x08\xB1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x05B\x97\\\x14a\0QW\x80c\x14\x16\xD7b\x14a\0\x95W\x80c\x1B\x11\xD0\xFF\x14a\0\xAAW\x80cu5\xD2F\x14a\0\xCDW[`\0\x80\xFD[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x05\x12V[a\0\xF4V[\0[a\0\xBDa\0\xB86`\x04a\x05\xAFV[a\x02SV[`@Q\x90\x15\x15\x81R` \x01a\0\x8CV[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x82\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01a\x91\x90a\x06}V[P`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x19`$\x82\x01R\x90\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDA\x91\x90a\x06}V[P`\x01T`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R0`D\x83\x01R`\0`d\x83\x01R\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x026W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02JW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x85\x81a\x02a\x87\x87a\x03zV[\x90Pa\x02l\x88a\x03\x90V[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD7\x91\x90a\x06}V[P`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x89\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03k\x91\x90a\x06}V[P`\x01\x98\x97PPPPPPPPV[\x80\x82\x01\x82\x81\x10\x15a\x03\x8AW`\0\x80\xFD[\x92\x91PPV[`\x01T`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\0\x91\x90a\x07?V[a\x01\0\x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P\x83\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04x\x91\x90a\x08bV[`\x01T`@Qc\xA4\x15\xBC\xAD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x02`D\x83\x01R`\0`d\x83\x01R0`\x84\x83\x01R\x92\x93P\x91\x16\x90c\xA4\x15\xBC\xAD\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xF0W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x0FW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05%W`\0\x80\xFD[\x825a\x050\x81a\x04\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05>V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xA7Wa\x05\xA7a\x05>V[`@R\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x05\xC7W`\0\x80\xFD[\x855a\x05\xD2\x81a\x04\xFAV[\x94P` \x86\x81\x015\x94P`@\x87\x015\x93P``\x87\x015a\x05\xF1\x81a\x04\xFAV[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x0EW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\"W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x064Wa\x064a\x05>V[a\x06F`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x05~V[\x91P\x80\x82R\x8A\x84\x82\x85\x01\x01\x11\x15a\x06\\W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x06\x8FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\x9FW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB8W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xDBWa\x06\xDBa\x05>V[`@R\x91Q\x82RP\x91\x90PV[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x91\x90PV[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x80Qa\x07\x08\x81a\x04\xFAV[`\0a\x01\xE0\x82\x84\x03\x12\x15a\x07RW`\0\x80\xFD[a\x07Za\x05TV[a\x07d\x84\x84a\x06\xA6V[\x81Ra\x07r` \x84\x01a\x06\xE8V[` \x82\x01Ra\x07\x83`@\x84\x01a\x06\xE8V[`@\x82\x01Ra\x07\x94``\x84\x01a\x06\xE8V[``\x82\x01Ra\x07\xA5`\x80\x84\x01a\x06\xE8V[`\x80\x82\x01Ra\x07\xB6`\xA0\x84\x01a\x06\xE8V[`\xA0\x82\x01Ra\x07\xC7`\xC0\x84\x01a\x07\rV[`\xC0\x82\x01Ra\x07\xD8`\xE0\x84\x01a\x07\"V[`\xE0\x82\x01Ra\x01\0a\x07\xEB\x81\x85\x01a\x074V[\x90\x82\x01Ra\x01 a\x07\xFD\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01@a\x08\x0F\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01`a\x08!\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01\x80a\x083\x84\x82\x01a\x06\xE8V[\x90\x82\x01Ra\x01\xA0a\x08E\x84\x82\x01a\x06\xE8V[\x90\x82\x01Ra\x01\xC0a\x08W\x84\x82\x01a\x06\xE8V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08tW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x1D_\xA0\xA9\x0FM\xDA\xD2{2n-5\xAE\xE66&\xFAb\x90\xFF\xF8\x0B\x88\xD8y\xD9\xB4C\x98\xE6_dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static FLASHLOANATTACKER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x05B\x97\\\x14a\0QW\x80c\x14\x16\xD7b\x14a\0\x95W\x80c\x1B\x11\xD0\xFF\x14a\0\xAAW\x80cu5\xD2F\x14a\0\xCDW[`\0\x80\xFD[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x05\x12V[a\0\xF4V[\0[a\0\xBDa\0\xB86`\x04a\x05\xAFV[a\x02SV[`@Q\x90\x15\x15\x81R` \x01a\0\x8CV[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x82\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01a\x91\x90a\x06}V[P`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\0\x19`$\x82\x01R\x90\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDA\x91\x90a\x06}V[P`\x01T`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R0`D\x83\x01R`\0`d\x83\x01R\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x026W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02JW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x85\x81a\x02a\x87\x87a\x03zV[\x90Pa\x02l\x88a\x03\x90V[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD7\x91\x90a\x06}V[P`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x89\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03k\x91\x90a\x06}V[P`\x01\x98\x97PPPPPPPPV[\x80\x82\x01\x82\x81\x10\x15a\x03\x8AW`\0\x80\xFD[\x92\x91PPV[`\x01T`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x92\x16\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\0\x91\x90a\x07?V[a\x01\0\x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P\x83\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04x\x91\x90a\x08bV[`\x01T`@Qc\xA4\x15\xBC\xAD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x02`D\x83\x01R`\0`d\x83\x01R0`\x84\x83\x01R\x92\x93P\x91\x16\x90c\xA4\x15\xBC\xAD\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xF0W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x0FW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\x05%W`\0\x80\xFD[\x825a\x050\x81a\x04\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05>V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xA7Wa\x05\xA7a\x05>V[`@R\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x05\xC7W`\0\x80\xFD[\x855a\x05\xD2\x81a\x04\xFAV[\x94P` \x86\x81\x015\x94P`@\x87\x015\x93P``\x87\x015a\x05\xF1\x81a\x04\xFAV[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x0EW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\"W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x064Wa\x064a\x05>V[a\x06F`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x05~V[\x91P\x80\x82R\x8A\x84\x82\x85\x01\x01\x11\x15a\x06\\W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[`\0` \x82\x84\x03\x12\x15a\x06\x8FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\x9FW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06\xB8W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\xDBWa\x06\xDBa\x05>V[`@R\x91Q\x82RP\x91\x90PV[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x91\x90PV[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14a\x07\x08W`\0\x80\xFD[\x80Qa\x07\x08\x81a\x04\xFAV[`\0a\x01\xE0\x82\x84\x03\x12\x15a\x07RW`\0\x80\xFD[a\x07Za\x05TV[a\x07d\x84\x84a\x06\xA6V[\x81Ra\x07r` \x84\x01a\x06\xE8V[` \x82\x01Ra\x07\x83`@\x84\x01a\x06\xE8V[`@\x82\x01Ra\x07\x94``\x84\x01a\x06\xE8V[``\x82\x01Ra\x07\xA5`\x80\x84\x01a\x06\xE8V[`\x80\x82\x01Ra\x07\xB6`\xA0\x84\x01a\x06\xE8V[`\xA0\x82\x01Ra\x07\xC7`\xC0\x84\x01a\x07\rV[`\xC0\x82\x01Ra\x07\xD8`\xE0\x84\x01a\x07\"V[`\xE0\x82\x01Ra\x01\0a\x07\xEB\x81\x85\x01a\x074V[\x90\x82\x01Ra\x01 a\x07\xFD\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01@a\x08\x0F\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01`a\x08!\x84\x82\x01a\x074V[\x90\x82\x01Ra\x01\x80a\x083\x84\x82\x01a\x06\xE8V[\x90\x82\x01Ra\x01\xA0a\x08E\x84\x82\x01a\x06\xE8V[\x90\x82\x01Ra\x01\xC0a\x08W\x84\x82\x01a\x06\xE8V[\x90\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x08tW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x1D_\xA0\xA9\x0FM\xDA\xD2{2n-5\xAE\xE66&\xFAb\x90\xFF\xF8\x0B\x88\xD8y\xD9\xB4C\x98\xE6_dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static FLASHLOANATTACKER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FlashloanAttacker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FlashloanAttacker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FlashloanAttacker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FlashloanAttacker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FlashloanAttacker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FlashloanAttacker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FlashloanAttacker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FLASHLOANATTACKER_ABI.clone(),
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
                FLASHLOANATTACKER_ABI.clone(),
                FLASHLOANATTACKER_BYTECODE.clone().into(),
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
        ///Calls the contract's `supplyAsset` (0x1416d762) function
        pub fn supply_asset(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 22, 215, 98], (asset, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FlashloanAttacker<M> {
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
    ///Container type for all input parameters for the `supplyAsset` function with signature `supplyAsset(address,uint256)` and selector `0x1416d762`
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
    #[ethcall(name = "supplyAsset", abi = "supplyAsset(address,uint256)")]
    pub struct SupplyAssetCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FlashloanAttackerCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        SupplyAsset(SupplyAssetCall),
    }
    impl ::ethers::core::abi::AbiDecode for FlashloanAttackerCalls {
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
                = <SupplyAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SupplyAsset(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FlashloanAttackerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupplyAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FlashloanAttackerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyAsset(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for FlashloanAttackerCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<PoolCall> for FlashloanAttackerCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for FlashloanAttackerCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<SupplyAssetCall> for FlashloanAttackerCalls {
        fn from(value: SupplyAssetCall) -> Self {
            Self::SupplyAsset(value)
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
}
