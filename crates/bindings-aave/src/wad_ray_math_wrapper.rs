pub use wad_ray_math_wrapper::*;
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
pub mod wad_ray_math_wrapper {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("halfRay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("halfRay"),
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
                    ::std::borrow::ToOwned::to_owned("halfWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("halfWad"),
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
                    ::std::borrow::ToOwned::to_owned("ray"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ray"),
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
                    ::std::borrow::ToOwned::to_owned("rayDiv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rayDiv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rayMul"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rayMul"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rayToWad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rayToWad"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wad"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wad"),
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
                    ::std::borrow::ToOwned::to_owned("wadDiv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wadDiv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wadMul"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wadMul"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("b"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wadToRay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wadToRay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
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
    pub static WADRAYMATHWRAPPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03[\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c}\xF3\x8C[\x11a\0fW\x80c}\xF3\x8C[\x14a\x01\x12W\x80c\x9C4\xD8\x80\x14a\x01 W\x80c\xD2\xE3\x05\x85\x14a\x013W\x80c\xE3\x04\xE1\xD3\x14a\x01FW\x80c\xE5{m;\x14a\x01TW`\0\x80\xFD[\x80c\x10\xDE'\xB9\x14a\0\xA3W\x80c\x1F\xA8\x9F\xC6\x14a\0\xC8W\x80c)\xCBZ\xA4\x14a\0\xDAW\x80cAj\x8B \x14a\0\xEDW\x80cv\x1F\xDA\xD6\x14a\0\xFFW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x02\xEAV[a\x01gV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0a\0\xB6V[a\0\xB6a\0\xE86`\x04a\x02\xEAV[a\x01xV[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\0\xB6V[a\0\xB6a\x01\r6`\x04a\x03\x03V[a\x01\x83V[g\r\xE0\xB6\xB3\xA7d\0\0a\0\xB6V[a\0\xB6a\x01.6`\x04a\x03\x03V[a\x01\x96V[a\0\xB6a\x01A6`\x04a\x03\x03V[a\x01\xA2V[g\x06\xF0[Y\xD3\xB2\0\0a\0\xB6V[a\0\xB6a\x01b6`\x04a\x03\x03V[a\x01\xAEV[`\0a\x01r\x82a\x01\xBAV[\x92\x91PPV[`\0a\x01r\x82a\x01\xD5V[`\0a\x01\x8F\x83\x83a\x01\xF8V[\x93\x92PPPV[`\0a\x01\x8F\x83\x83a\x020V[`\0a\x01\x8F\x83\x83a\x02oV[`\0a\x01\x8F\x83\x83a\x02\xB3V[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\x01\xD0W`\0\x80\xFD[\x91\x90PV[c;\x9A\xCA\0\x80\x82\x04\x90\x82\x06c\x1D\xCDe\0\x81\x10a\x01\xF2W`\x01\x82\x01\x91P[P\x91\x90PV[`\0\x81\x15g\x06\xF0[Y\xD3\xB2\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x02\x16W`\0\x80\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02g\x06\xF0[Y\xD3\xB2\0\0\x01\x04\x90V[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x02TW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x02\x91W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x81\x15g\r\xE0\xB6\xB3\xA7d\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x02\xD3W`\0\x80\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x02\xFCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\x16W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV\xFE\xA2dipfsX\"\x12 \x01q\x82\xB7\xC2M7j\xA7\x0E\\1\x8B;\x84\x9F:\xFC\xAB\xD1\xC9\xEBk,'\x87\xCF\x96\x9Eo\xA9jdsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static WADRAYMATHWRAPPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c}\xF3\x8C[\x11a\0fW\x80c}\xF3\x8C[\x14a\x01\x12W\x80c\x9C4\xD8\x80\x14a\x01 W\x80c\xD2\xE3\x05\x85\x14a\x013W\x80c\xE3\x04\xE1\xD3\x14a\x01FW\x80c\xE5{m;\x14a\x01TW`\0\x80\xFD[\x80c\x10\xDE'\xB9\x14a\0\xA3W\x80c\x1F\xA8\x9F\xC6\x14a\0\xC8W\x80c)\xCBZ\xA4\x14a\0\xDAW\x80cAj\x8B \x14a\0\xEDW\x80cv\x1F\xDA\xD6\x14a\0\xFFW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x02\xEAV[a\x01gV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0a\0\xB6V[a\0\xB6a\0\xE86`\x04a\x02\xEAV[a\x01xV[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a\0\xB6V[a\0\xB6a\x01\r6`\x04a\x03\x03V[a\x01\x83V[g\r\xE0\xB6\xB3\xA7d\0\0a\0\xB6V[a\0\xB6a\x01.6`\x04a\x03\x03V[a\x01\x96V[a\0\xB6a\x01A6`\x04a\x03\x03V[a\x01\xA2V[g\x06\xF0[Y\xD3\xB2\0\0a\0\xB6V[a\0\xB6a\x01b6`\x04a\x03\x03V[a\x01\xAEV[`\0a\x01r\x82a\x01\xBAV[\x92\x91PPV[`\0a\x01r\x82a\x01\xD5V[`\0a\x01\x8F\x83\x83a\x01\xF8V[\x93\x92PPPV[`\0a\x01\x8F\x83\x83a\x020V[`\0a\x01\x8F\x83\x83a\x02oV[`\0a\x01\x8F\x83\x83a\x02\xB3V[c;\x9A\xCA\0\x81\x81\x02\x90\x81\x04\x82\x14a\x01\xD0W`\0\x80\xFD[\x91\x90PV[c;\x9A\xCA\0\x80\x82\x04\x90\x82\x06c\x1D\xCDe\0\x81\x10a\x01\xF2W`\x01\x82\x01\x91P[P\x91\x90PV[`\0\x81\x15g\x06\xF0[Y\xD3\xB2\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x02\x16W`\0\x80\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02g\x06\xF0[Y\xD3\xB2\0\0\x01\x04\x90V[`\0\x81\x15k\x03;.<\x9F\xD0\x80<\xE8\0\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x02TW`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0\x81\x15k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x19\x83\x90\x04\x84\x11\x15\x17a\x02\x91W`\0\x80\xFD[Pk\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x91\x02k\x01\x9D\x97\x1EO\xE8@\x1Et\0\0\0\x01\x04\x90V[`\0\x81\x15g\r\xE0\xB6\xB3\xA7d\0\0`\x02\x84\x04\x19\x04\x84\x11\x17\x15a\x02\xD3W`\0\x80\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90\x91\x02`\x02\x82\x04\x01\x04\x90V[`\0` \x82\x84\x03\x12\x15a\x02\xFCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\x16W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV\xFE\xA2dipfsX\"\x12 \x01q\x82\xB7\xC2M7j\xA7\x0E\\1\x8B;\x84\x9F:\xFC\xAB\xD1\xC9\xEBk,'\x87\xCF\x96\x9Eo\xA9jdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static WADRAYMATHWRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct WadRayMathWrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WadRayMathWrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WadRayMathWrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WadRayMathWrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WadRayMathWrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WadRayMathWrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WadRayMathWrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WADRAYMATHWRAPPER_ABI.clone(),
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
                WADRAYMATHWRAPPER_ABI.clone(),
                WADRAYMATHWRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `halfRay` (0x1fa89fc6) function
        pub fn half_ray(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 168, 159, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `halfWad` (0xe304e1d3) function
        pub fn half_wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 4, 225, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ray` (0x416a8b20) function
        pub fn ray(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 106, 139, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rayDiv` (0x9c34d880) function
        pub fn ray_div(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 52, 216, 128], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rayMul` (0xd2e30585) function
        pub fn ray_mul(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([210, 227, 5, 133], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rayToWad` (0x29cb5aa4) function
        pub fn ray_to_wad(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 203, 90, 164], a)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wad` (0x7df38c5b) function
        pub fn wad(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 243, 140, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wadDiv` (0xe57b6d3b) function
        pub fn wad_div(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 123, 109, 59], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wadMul` (0x761fdad6) function
        pub fn wad_mul(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([118, 31, 218, 214], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wadToRay` (0x10de27b9) function
        pub fn wad_to_ray(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([16, 222, 39, 185], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for WadRayMathWrapper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `halfRay` function with signature `halfRay()` and selector `0x1fa89fc6`
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
    #[ethcall(name = "halfRay", abi = "halfRay()")]
    pub struct HalfRayCall;
    ///Container type for all input parameters for the `halfWad` function with signature `halfWad()` and selector `0xe304e1d3`
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
    #[ethcall(name = "halfWad", abi = "halfWad()")]
    pub struct HalfWadCall;
    ///Container type for all input parameters for the `ray` function with signature `ray()` and selector `0x416a8b20`
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
    #[ethcall(name = "ray", abi = "ray()")]
    pub struct RayCall;
    ///Container type for all input parameters for the `rayDiv` function with signature `rayDiv(uint256,uint256)` and selector `0x9c34d880`
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
    #[ethcall(name = "rayDiv", abi = "rayDiv(uint256,uint256)")]
    pub struct RayDivCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rayMul` function with signature `rayMul(uint256,uint256)` and selector `0xd2e30585`
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
    #[ethcall(name = "rayMul", abi = "rayMul(uint256,uint256)")]
    pub struct RayMulCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rayToWad` function with signature `rayToWad(uint256)` and selector `0x29cb5aa4`
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
    #[ethcall(name = "rayToWad", abi = "rayToWad(uint256)")]
    pub struct RayToWadCall {
        pub a: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wad` function with signature `wad()` and selector `0x7df38c5b`
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
    #[ethcall(name = "wad", abi = "wad()")]
    pub struct WadCall;
    ///Container type for all input parameters for the `wadDiv` function with signature `wadDiv(uint256,uint256)` and selector `0xe57b6d3b`
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
    #[ethcall(name = "wadDiv", abi = "wadDiv(uint256,uint256)")]
    pub struct WadDivCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wadMul` function with signature `wadMul(uint256,uint256)` and selector `0x761fdad6`
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
    #[ethcall(name = "wadMul", abi = "wadMul(uint256,uint256)")]
    pub struct WadMulCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wadToRay` function with signature `wadToRay(uint256)` and selector `0x10de27b9`
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
    #[ethcall(name = "wadToRay", abi = "wadToRay(uint256)")]
    pub struct WadToRayCall {
        pub a: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WadRayMathWrapperCalls {
        HalfRay(HalfRayCall),
        HalfWad(HalfWadCall),
        Ray(RayCall),
        RayDiv(RayDivCall),
        RayMul(RayMulCall),
        RayToWad(RayToWadCall),
        Wad(WadCall),
        WadDiv(WadDivCall),
        WadMul(WadMulCall),
        WadToRay(WadToRayCall),
    }
    impl ::ethers::core::abi::AbiDecode for WadRayMathWrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <HalfRayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HalfRay(decoded));
            }
            if let Ok(decoded)
                = <HalfWadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HalfWad(decoded));
            }
            if let Ok(decoded)
                = <RayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ray(decoded));
            }
            if let Ok(decoded)
                = <RayDivCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RayDiv(decoded));
            }
            if let Ok(decoded)
                = <RayMulCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RayMul(decoded));
            }
            if let Ok(decoded)
                = <RayToWadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RayToWad(decoded));
            }
            if let Ok(decoded)
                = <WadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Wad(decoded));
            }
            if let Ok(decoded)
                = <WadDivCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WadDiv(decoded));
            }
            if let Ok(decoded)
                = <WadMulCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WadMul(decoded));
            }
            if let Ok(decoded)
                = <WadToRayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WadToRay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WadRayMathWrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HalfRay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HalfWad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ray(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RayDiv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RayMul(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RayToWad(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Wad(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WadDiv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WadMul(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WadToRay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WadRayMathWrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HalfRay(element) => ::core::fmt::Display::fmt(element, f),
                Self::HalfWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ray(element) => ::core::fmt::Display::fmt(element, f),
                Self::RayDiv(element) => ::core::fmt::Display::fmt(element, f),
                Self::RayMul(element) => ::core::fmt::Display::fmt(element, f),
                Self::RayToWad(element) => ::core::fmt::Display::fmt(element, f),
                Self::Wad(element) => ::core::fmt::Display::fmt(element, f),
                Self::WadDiv(element) => ::core::fmt::Display::fmt(element, f),
                Self::WadMul(element) => ::core::fmt::Display::fmt(element, f),
                Self::WadToRay(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HalfRayCall> for WadRayMathWrapperCalls {
        fn from(value: HalfRayCall) -> Self {
            Self::HalfRay(value)
        }
    }
    impl ::core::convert::From<HalfWadCall> for WadRayMathWrapperCalls {
        fn from(value: HalfWadCall) -> Self {
            Self::HalfWad(value)
        }
    }
    impl ::core::convert::From<RayCall> for WadRayMathWrapperCalls {
        fn from(value: RayCall) -> Self {
            Self::Ray(value)
        }
    }
    impl ::core::convert::From<RayDivCall> for WadRayMathWrapperCalls {
        fn from(value: RayDivCall) -> Self {
            Self::RayDiv(value)
        }
    }
    impl ::core::convert::From<RayMulCall> for WadRayMathWrapperCalls {
        fn from(value: RayMulCall) -> Self {
            Self::RayMul(value)
        }
    }
    impl ::core::convert::From<RayToWadCall> for WadRayMathWrapperCalls {
        fn from(value: RayToWadCall) -> Self {
            Self::RayToWad(value)
        }
    }
    impl ::core::convert::From<WadCall> for WadRayMathWrapperCalls {
        fn from(value: WadCall) -> Self {
            Self::Wad(value)
        }
    }
    impl ::core::convert::From<WadDivCall> for WadRayMathWrapperCalls {
        fn from(value: WadDivCall) -> Self {
            Self::WadDiv(value)
        }
    }
    impl ::core::convert::From<WadMulCall> for WadRayMathWrapperCalls {
        fn from(value: WadMulCall) -> Self {
            Self::WadMul(value)
        }
    }
    impl ::core::convert::From<WadToRayCall> for WadRayMathWrapperCalls {
        fn from(value: WadToRayCall) -> Self {
            Self::WadToRay(value)
        }
    }
    ///Container type for all return fields from the `halfRay` function with signature `halfRay()` and selector `0x1fa89fc6`
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
    pub struct HalfRayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `halfWad` function with signature `halfWad()` and selector `0xe304e1d3`
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
    pub struct HalfWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ray` function with signature `ray()` and selector `0x416a8b20`
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
    pub struct RayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rayDiv` function with signature `rayDiv(uint256,uint256)` and selector `0x9c34d880`
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
    pub struct RayDivReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rayMul` function with signature `rayMul(uint256,uint256)` and selector `0xd2e30585`
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
    pub struct RayMulReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rayToWad` function with signature `rayToWad(uint256)` and selector `0x29cb5aa4`
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
    pub struct RayToWadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `wad` function with signature `wad()` and selector `0x7df38c5b`
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
    pub struct WadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `wadDiv` function with signature `wadDiv(uint256,uint256)` and selector `0xe57b6d3b`
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
    pub struct WadDivReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `wadMul` function with signature `wadMul(uint256,uint256)` and selector `0x761fdad6`
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
    pub struct WadMulReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `wadToRay` function with signature `wadToRay(uint256)` and selector `0x10de27b9`
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
    pub struct WadToRayReturn(pub ::ethers::core::types::U256);
}
