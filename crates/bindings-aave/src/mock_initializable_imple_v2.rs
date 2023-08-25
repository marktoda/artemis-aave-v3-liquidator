pub use mock_initializable_imple_v2::*;
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
pub mod mock_initializable_imple_v2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("REVISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("REVISION"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vals"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setValue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
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
                    ::std::borrow::ToOwned::to_owned("setValueViaProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setValueViaProxy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
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
                    ::std::borrow::ToOwned::to_owned("text"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("text"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("value"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("values"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("values"),
                            inputs: ::std::vec![
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
    pub static MOCKINITIALIZABLEIMPLEV2_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80U4\x80\x15a\0\x14W`\0\x80\xFD[Pa\x05\xCC\x80a\0$`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c]\xD2\x16\x10\x11a\0[W\x80c]\xD2\x16\x10\x14a\0\xB7W\x80c^8=!\x14a\0\xCCW\x80c\xD3\x1F\x8Bk\x14a\0\xDFW\x80c\xDD\xE4<\xBA\x14a\0\xF2W`\0\x80\xFD[\x80c\x1F\x1B\xD6\x92\x14a\0\x82W\x80c?\xA4\xF2E\x14a\0\xA0W\x80cU$\x10w\x14a\0\xB7W[`\0\x80\xFD[a\0\x8Aa\0\xFAV[`@Qa\0\x97\x91\x90a\x03fV[`@Q\x80\x91\x03\x90\xF3[a\0\xA9`4T\x81V[`@Q\x90\x81R` \x01a\0\x97V[a\0\xCAa\0\xC56`\x04a\x03\xBBV[`4UV[\0[a\0\xA9a\0\xDA6`\x04a\x03\xBBV[a\x01\x88V[a\0\xCAa\0\xED6`\x04a\x04\x9BV[a\x01\xA9V[a\0\xA9`\x02\x81V[`5\x80Ta\x01\x07\x90a\x05[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x013\x90a\x05[V[\x80\x15a\x01\x80W\x80`\x1F\x10a\x01UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`6\x81\x81T\x81\x10a\x01\x98W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x01T`\x02\x90`\xFF\x16\x80a\x01\xBCWP0;\x15[\x80a\x01\xC8WP`\0T\x81\x11[a\x02/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x02NW`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[`4\x85\x90U\x83Qa\x02f\x90`5\x90` \x87\x01\x90a\x02\x93V[P\x82Qa\x02z\x90`6\x90` \x86\x01\x90a\x03\x17V[P\x80\x15a\x02\x8CW`\x01\x80T`\xFF\x19\x16\x90U[PPPPPV[\x82\x80Ta\x02\x9F\x90a\x05[V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x02\xC1W`\0\x85Ua\x03\x07V[\x82`\x1F\x10a\x02\xDAW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x03\x07V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x03\x07W\x91\x82\x01[\x82\x81\x11\x15a\x03\x07W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x02\xECV[Pa\x03\x13\x92\x91Pa\x03QV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\x07W\x91` \x02\x82\x01\x82\x81\x11\x15a\x03\x07W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x02\xECV[[\x80\x82\x11\x15a\x03\x13W`\0\x81U`\x01\x01a\x03RV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x03\x93W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03wV[\x81\x81\x11\x15a\x03\xA5W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xCDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\x13Wa\x04\x13a\x03\xD4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04,W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04HWa\x04Ha\x03\xD4V[\x81`\x05\x1Ba\x04W\x82\x82\x01a\x03\xEAV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x04qW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x04\x90W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x04wV[\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\xB0W`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xD0W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x04\xE4W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xF6Wa\x04\xF6a\x03\xD4V[a\x05\x08`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x03\xEAV[\x81\x81R\x89\x85\x83\x86\x01\x01\x11\x15a\x05\x1CW`\0\x80\xFD[\x81\x85\x85\x01\x86\x83\x017`\0\x91\x81\x01\x90\x94\x01R\x91\x93P`@\x86\x015\x91\x80\x83\x11\x15a\x05CW`\0\x80\xFD[PPa\x05Q\x86\x82\x87\x01a\x04\x1BV[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x05oW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x05\x90WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0C\x84\xB0\x068\xC8\x95\x17\x7Fd\xBB$\t\x18\x9C\xB7\xED\xAD`d\x87*\xAC1H\t4O\x12\x82[ldsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static MOCKINITIALIZABLEIMPLEV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c]\xD2\x16\x10\x11a\0[W\x80c]\xD2\x16\x10\x14a\0\xB7W\x80c^8=!\x14a\0\xCCW\x80c\xD3\x1F\x8Bk\x14a\0\xDFW\x80c\xDD\xE4<\xBA\x14a\0\xF2W`\0\x80\xFD[\x80c\x1F\x1B\xD6\x92\x14a\0\x82W\x80c?\xA4\xF2E\x14a\0\xA0W\x80cU$\x10w\x14a\0\xB7W[`\0\x80\xFD[a\0\x8Aa\0\xFAV[`@Qa\0\x97\x91\x90a\x03fV[`@Q\x80\x91\x03\x90\xF3[a\0\xA9`4T\x81V[`@Q\x90\x81R` \x01a\0\x97V[a\0\xCAa\0\xC56`\x04a\x03\xBBV[`4UV[\0[a\0\xA9a\0\xDA6`\x04a\x03\xBBV[a\x01\x88V[a\0\xCAa\0\xED6`\x04a\x04\x9BV[a\x01\xA9V[a\0\xA9`\x02\x81V[`5\x80Ta\x01\x07\x90a\x05[V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x013\x90a\x05[V[\x80\x15a\x01\x80W\x80`\x1F\x10a\x01UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\x80V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01cW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`6\x81\x81T\x81\x10a\x01\x98W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T\x90P\x81V[`\x01T`\x02\x90`\xFF\x16\x80a\x01\xBCWP0;\x15[\x80a\x01\xC8WP`\0T\x81\x11[a\x02/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x02NW`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[`4\x85\x90U\x83Qa\x02f\x90`5\x90` \x87\x01\x90a\x02\x93V[P\x82Qa\x02z\x90`6\x90` \x86\x01\x90a\x03\x17V[P\x80\x15a\x02\x8CW`\x01\x80T`\xFF\x19\x16\x90U[PPPPPV[\x82\x80Ta\x02\x9F\x90a\x05[V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a\x02\xC1W`\0\x85Ua\x03\x07V[\x82`\x1F\x10a\x02\xDAW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua\x03\x07V[\x82\x80\x01`\x01\x01\x85U\x82\x15a\x03\x07W\x91\x82\x01[\x82\x81\x11\x15a\x03\x07W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x02\xECV[Pa\x03\x13\x92\x91Pa\x03QV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x03\x07W\x91` \x02\x82\x01\x82\x81\x11\x15a\x03\x07W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x02\xECV[[\x80\x82\x11\x15a\x03\x13W`\0\x81U`\x01\x01a\x03RV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x03\x93W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03wV[\x81\x81\x11\x15a\x03\xA5W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x03\xCDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\x13Wa\x04\x13a\x03\xD4V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04,W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04HWa\x04Ha\x03\xD4V[\x81`\x05\x1Ba\x04W\x82\x82\x01a\x03\xEAV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a\x04qW`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a\x04\x90W\x825\x82R\x91\x83\x01\x91\x90\x83\x01\x90a\x04wV[\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x04\xB0W`\0\x80\xFD[\x835\x92P` \x80\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xD0W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x04\xE4W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xF6Wa\x04\xF6a\x03\xD4V[a\x05\x08`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x03\xEAV[\x81\x81R\x89\x85\x83\x86\x01\x01\x11\x15a\x05\x1CW`\0\x80\xFD[\x81\x85\x85\x01\x86\x83\x017`\0\x91\x81\x01\x90\x94\x01R\x91\x93P`@\x86\x015\x91\x80\x83\x11\x15a\x05CW`\0\x80\xFD[PPa\x05Q\x86\x82\x87\x01a\x04\x1BV[\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x05oW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x05\x90WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0C\x84\xB0\x068\xC8\x95\x17\x7Fd\xBB$\t\x18\x9C\xB7\xED\xAD`d\x87*\xAC1H\t4O\x12\x82[ldsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKINITIALIZABLEIMPLEV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockInitializableImpleV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockInitializableImpleV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockInitializableImpleV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockInitializableImpleV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockInitializableImpleV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockInitializableImpleV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockInitializableImpleV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKINITIALIZABLEIMPLEV2_ABI.clone(),
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
                MOCKINITIALIZABLEIMPLEV2_ABI.clone(),
                MOCKINITIALIZABLEIMPLEV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `REVISION` (0xdde43cba) function
        pub fn revision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 228, 60, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xd31f8b6b) function
        pub fn initialize(
            &self,
            val: ::ethers::core::types::U256,
            txt: ::std::string::String,
            vals: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 31, 139, 107], (val, txt, vals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setValue` (0x55241077) function
        pub fn set_value(
            &self,
            new_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 36, 16, 119], new_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setValueViaProxy` (0x5dd21610) function
        pub fn set_value_via_proxy(
            &self,
            new_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 210, 22, 16], new_value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `text` (0x1f1bd692) function
        pub fn text(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([31, 27, 214, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `value` (0x3fa4f245) function
        pub fn value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([63, 164, 242, 69], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `values` (0x5e383d21) function
        pub fn values(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 56, 61, 33], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockInitializableImpleV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `REVISION` function with signature `REVISION()` and selector `0xdde43cba`
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
    #[ethcall(name = "REVISION", abi = "REVISION()")]
    pub struct RevisionCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,string,uint256[])` and selector `0xd31f8b6b`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256,string,uint256[])")]
    pub struct InitializeCall {
        pub val: ::ethers::core::types::U256,
        pub txt: ::std::string::String,
        pub vals: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `setValue` function with signature `setValue(uint256)` and selector `0x55241077`
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
    #[ethcall(name = "setValue", abi = "setValue(uint256)")]
    pub struct SetValueCall {
        pub new_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setValueViaProxy` function with signature `setValueViaProxy(uint256)` and selector `0x5dd21610`
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
    #[ethcall(name = "setValueViaProxy", abi = "setValueViaProxy(uint256)")]
    pub struct SetValueViaProxyCall {
        pub new_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `text` function with signature `text()` and selector `0x1f1bd692`
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
    #[ethcall(name = "text", abi = "text()")]
    pub struct TextCall;
    ///Container type for all input parameters for the `value` function with signature `value()` and selector `0x3fa4f245`
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
    #[ethcall(name = "value", abi = "value()")]
    pub struct ValueCall;
    ///Container type for all input parameters for the `values` function with signature `values(uint256)` and selector `0x5e383d21`
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
    #[ethcall(name = "values", abi = "values(uint256)")]
    pub struct ValuesCall(pub ::ethers::core::types::U256);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockInitializableImpleV2Calls {
        Revision(RevisionCall),
        Initialize(InitializeCall),
        SetValue(SetValueCall),
        SetValueViaProxy(SetValueViaProxyCall),
        Text(TextCall),
        Value(ValueCall),
        Values(ValuesCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockInitializableImpleV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RevisionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Revision(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <SetValueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetValue(decoded));
            }
            if let Ok(decoded)
                = <SetValueViaProxyCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetValueViaProxy(decoded));
            }
            if let Ok(decoded)
                = <TextCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Text(decoded));
            }
            if let Ok(decoded)
                = <ValueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Value(decoded));
            }
            if let Ok(decoded)
                = <ValuesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Values(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockInitializableImpleV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Revision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetValueViaProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Text(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Value(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Values(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockInitializableImpleV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Revision(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetValueViaProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Text(element) => ::core::fmt::Display::fmt(element, f),
                Self::Value(element) => ::core::fmt::Display::fmt(element, f),
                Self::Values(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RevisionCall> for MockInitializableImpleV2Calls {
        fn from(value: RevisionCall) -> Self {
            Self::Revision(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockInitializableImpleV2Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<SetValueCall> for MockInitializableImpleV2Calls {
        fn from(value: SetValueCall) -> Self {
            Self::SetValue(value)
        }
    }
    impl ::core::convert::From<SetValueViaProxyCall> for MockInitializableImpleV2Calls {
        fn from(value: SetValueViaProxyCall) -> Self {
            Self::SetValueViaProxy(value)
        }
    }
    impl ::core::convert::From<TextCall> for MockInitializableImpleV2Calls {
        fn from(value: TextCall) -> Self {
            Self::Text(value)
        }
    }
    impl ::core::convert::From<ValueCall> for MockInitializableImpleV2Calls {
        fn from(value: ValueCall) -> Self {
            Self::Value(value)
        }
    }
    impl ::core::convert::From<ValuesCall> for MockInitializableImpleV2Calls {
        fn from(value: ValuesCall) -> Self {
            Self::Values(value)
        }
    }
    ///Container type for all return fields from the `REVISION` function with signature `REVISION()` and selector `0xdde43cba`
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
    pub struct RevisionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `text` function with signature `text()` and selector `0x1f1bd692`
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
    pub struct TextReturn(pub ::std::string::String);
    ///Container type for all return fields from the `value` function with signature `value()` and selector `0x3fa4f245`
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
    pub struct ValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `values` function with signature `values(uint256)` and selector `0x5e383d21`
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
    pub struct ValuesReturn(pub ::ethers::core::types::U256);
}
