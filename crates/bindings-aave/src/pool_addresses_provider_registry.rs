pub use pool_addresses_provider_registry::*;
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
pub mod pool_addresses_provider_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAddressesProviderAddressById"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAddressesProviderAddressById",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("getAddressesProviderIdByAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAddressesProviderIdByAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addressesProvider"),
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
                    ::std::borrow::ToOwned::to_owned("getAddressesProvidersList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAddressesProvidersList",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("registerAddressesProvider"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerAddressesProvider",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                (
                    ::std::borrow::ToOwned::to_owned("unregisterAddressesProvider"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "unregisterAddressesProvider",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
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
                    ::std::borrow::ToOwned::to_owned("AddressesProviderRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressesProviderRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addressesProvider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressesProviderUnregistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressesProviderUnregistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("addressesProvider"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
    pub static POOLADDRESSESPROVIDERREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\n\x998\x03\x80a\n\x99\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01zV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91`\0\x80Q` a\ny\x839\x81Q\x91R\x90\x82\x90\xA3Pa\0g\x81a\0mV[Pa\x01\xAAV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x011W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\0\xC3V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91`\0\x80Q` a\ny\x839\x81Q\x91R\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0` \x82\x84\x03\x12\x15a\x01\x8CW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xA3W`\0\x80\xFD[\x93\x92PPPV[a\x08\xC0\x80a\x01\xB9`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\x01\tW\x80c\xD0&{\xE7\x14a\x01\x1AW\x80c\xD2X\x19\x1E\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01dW`\0\x80\xFD[\x80c\r\xE2g\x07\x14a\0\x8DW\x80c6\\\xCB\xBF\x14a\0\xA2W\x80cW\xDC\x05f\x14a\0\xC0W\x80cqP\x18\xA6\x14a\x01\x01W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x06\xFDV[a\x01wV[\0[a\0\xAAa\x02yV[`@Qa\0\xB7\x91\x90a\x07\x1FV[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xCE6`\x04a\x07lV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB7V[a\0\xA0a\x02\xDBV[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xE9V[a\x01Ca\x01(6`\x04a\x06\xFDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\0\xB7V[a\0\xA0a\x01_6`\x04a\x07\x85V[a\x03OV[a\0\xA0a\x01r6`\x04a\x06\xFDV[a\x05\x06V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` \x81\x81R`@\x92\x83\x90 T\x83Q\x80\x85\x01\x90\x94R\x91\x83R`7`\xF8\x1B\x90\x83\x01Ra\x01\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x80\x86R`\x02\x84R\x91\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x94\x84R\x91\x90R\x91Ua\x02?\x82a\x05\xF0V[`@Q\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F%G#\x08\x07\x01\xBD\xE7\x1DV,\xAD\x0E\x96|\xEF#\xD8k\xB2~\xE8B\xC1\x90\xA2Yh \xF3\xB2A\x90`\0\x90\xA3PPV[```\x03\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\xD1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02\xB3W[PPPPP\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x07`\xFB\x1B` \x82\x01R\x81a\x03\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\0\x81\x81R`\x02` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`\x07`\xFB\x1B\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x1C\x1B`\xF1\x1B\x91\x83\x01\x91\x90\x91R\x15a\x04WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x86\x90U\x85\x84R`\x02\x82R\x80\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x03\x80T\x87\x87R`\x04\x90\x94R\x82\x86 \x84\x90U\x93\x83\x01\x84U\x92\x84R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x90\x91\x01\x80T\x90\x92\x16\x84\x17\x90\x91UQ\x83\x92\x91\x7F\xC2\xE7\xCC\x815P\xEF\x0Eq&\xCC\x05q(\x18P\xCE]\xF2\xE9\xC4\0\xAC\xF3X\x9C8\xE4b\x7F\x85\xF1\x91\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x050W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x01\xA1V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x82\x90U`\x03T\x90\x91\x90a\x06 \x90`\x01\x90a\x089V[\x90P\x80\x82\x10\x15a\x06\xA9W`\0`\x03\x82\x81T\x81\x10a\x06?Wa\x06?a\x08^V[`\0\x91\x82R` \x90\x91 \x01T`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92P\x82\x91\x85\x90\x81\x10a\x06nWa\x06na\x08^V[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x92\x90\x91\x16\x81R`\x04\x90\x91R`@\x90 \x82\x90U[`\x03\x80T\x80a\x06\xBAWa\x06\xBAa\x08tV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xF8W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07\x0FW`\0\x80\xFD[a\x07\x18\x82a\x06\xE1V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x07`W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x07;V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07~W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x98W`\0\x80\xFD[a\x07\xA1\x83a\x06\xE1V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x08\x11W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x07\xF5V[\x81\x81\x11\x15a\x08#W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x82\x82\x10\x15a\x08YWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF9\xFD\xFB\xE0\x9A\x9D\xFA\x96\xD4!\xA2\x96 <\xE6\xC4,\xBB\xB9\x90\xABYk=\xA9\xC4*\xF1<T\xFC\xECdsolcC\0\x08\n\x003\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0";
    /// The bytecode of the contract.
    pub static POOLADDRESSESPROVIDERREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\x01\tW\x80c\xD0&{\xE7\x14a\x01\x1AW\x80c\xD2X\x19\x1E\x14a\x01QW\x80c\xF2\xFD\xE3\x8B\x14a\x01dW`\0\x80\xFD[\x80c\r\xE2g\x07\x14a\0\x8DW\x80c6\\\xCB\xBF\x14a\0\xA2W\x80cW\xDC\x05f\x14a\0\xC0W\x80cqP\x18\xA6\x14a\x01\x01W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x06\xFDV[a\x01wV[\0[a\0\xAAa\x02yV[`@Qa\0\xB7\x91\x90a\x07\x1FV[`@Q\x80\x91\x03\x90\xF3[a\0\xE9a\0\xCE6`\x04a\x07lV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB7V[a\0\xA0a\x02\xDBV[`\0T`\x01`\x01`\xA0\x1B\x03\x16a\0\xE9V[a\x01Ca\x01(6`\x04a\x06\xFDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\0\xB7V[a\0\xA0a\x01_6`\x04a\x07\x85V[a\x03OV[a\0\xA0a\x01r6`\x04a\x06\xFDV[a\x05\x06V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x01` \x81\x81R`@\x92\x83\x90 T\x83Q\x80\x85\x01\x90\x94R\x91\x83R`7`\xF8\x1B\x90\x83\x01Ra\x01\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x80\x86R`\x02\x84R\x91\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x94\x84R\x91\x90R\x91Ua\x02?\x82a\x05\xF0V[`@Q\x81\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F%G#\x08\x07\x01\xBD\xE7\x1DV,\xAD\x0E\x96|\xEF#\xD8k\xB2~\xE8B\xC1\x90\xA2Yh \xF3\xB2A\x90`\0\x90\xA3PPV[```\x03\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\xD1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02\xB3W[PPPPP\x90P\x90V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x83\x90\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x07`\xFB\x1B` \x82\x01R\x81a\x03\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\0\x81\x81R`\x02` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`\x07`\xFB\x1B\x91\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x1C\x1B`\xF1\x1B\x91\x83\x01\x91\x90\x91R\x15a\x04WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x91\x90a\x07\xE4V[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x86\x90U\x85\x84R`\x02\x82R\x80\x84 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x03\x80T\x87\x87R`\x04\x90\x94R\x82\x86 \x84\x90U\x93\x83\x01\x84U\x92\x84R\x7F\xC2WZ\x0E\x9EY<\0\xF9Y\xF8\xC9/\x12\xDB(i\xC39Z;\x05\x02\xD0^%\x16Doq\xF8[\x90\x91\x01\x80T\x90\x92\x16\x84\x17\x90\x91UQ\x83\x92\x91\x7F\xC2\xE7\xCC\x815P\xEF\x0Eq&\xCC\x05q(\x18P\xCE]\xF2\xE9\xC4\0\xAC\xF3X\x9C8\xE4b\x7F\x85\xF1\x91\xA3PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x050W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xA1\x90a\x07\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x01\xA1V[`\0\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x93\x92\x16\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\xA3`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x82\x90U`\x03T\x90\x91\x90a\x06 \x90`\x01\x90a\x089V[\x90P\x80\x82\x10\x15a\x06\xA9W`\0`\x03\x82\x81T\x81\x10a\x06?Wa\x06?a\x08^V[`\0\x91\x82R` \x90\x91 \x01T`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92P\x82\x91\x85\x90\x81\x10a\x06nWa\x06na\x08^V[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x92\x90\x91\x16\x81R`\x04\x90\x91R`@\x90 \x82\x90U[`\x03\x80T\x80a\x06\xBAWa\x06\xBAa\x08tV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xF8W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x07\x0FW`\0\x80\xFD[a\x07\x18\x82a\x06\xE1V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x07`W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x07;V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07~W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\x98W`\0\x80\xFD[a\x07\xA1\x83a\x06\xE1V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x08\x11W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x07\xF5V[\x81\x81\x11\x15a\x08#W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x82\x82\x10\x15a\x08YWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xF9\xFD\xFB\xE0\x9A\x9D\xFA\x96\xD4!\xA2\x96 <\xE6\xC4,\xBB\xB9\x90\xABYk=\xA9\xC4*\xF1<T\xFC\xECdsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static POOLADDRESSESPROVIDERREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PoolAddressesProviderRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoolAddressesProviderRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoolAddressesProviderRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoolAddressesProviderRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoolAddressesProviderRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PoolAddressesProviderRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoolAddressesProviderRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
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
                POOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
                POOLADDRESSESPROVIDERREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getAddressesProviderAddressById` (0x57dc0566) function
        pub fn get_addresses_provider_address_by_id(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([87, 220, 5, 102], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressesProviderIdByAddress` (0xd0267be7) function
        pub fn get_addresses_provider_id_by_address(
            &self,
            addresses_provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([208, 38, 123, 231], addresses_provider)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAddressesProvidersList` (0x365ccbbf) function
        pub fn get_addresses_providers_list(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([54, 92, 203, 191], ())
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
        ///Calls the contract's `registerAddressesProvider` (0xd258191e) function
        pub fn register_addresses_provider(
            &self,
            provider: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 88, 25, 30], (provider, id))
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
        ///Calls the contract's `unregisterAddressesProvider` (0x0de26707) function
        pub fn unregister_addresses_provider(
            &self,
            provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 226, 103, 7], provider)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddressesProviderRegistered` event
        pub fn addresses_provider_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddressesProviderRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AddressesProviderUnregistered` event
        pub fn addresses_provider_unregistered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AddressesProviderUnregisteredFilter,
        > {
            self.0.event()
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
            PoolAddressesProviderRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PoolAddressesProviderRegistry<M> {
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
        name = "AddressesProviderRegistered",
        abi = "AddressesProviderRegistered(address,uint256)"
    )]
    pub struct AddressesProviderRegisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
        name = "AddressesProviderUnregistered",
        abi = "AddressesProviderUnregistered(address,uint256)"
    )]
    pub struct AddressesProviderUnregisteredFilter {
        #[ethevent(indexed)]
        pub addresses_provider: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolAddressesProviderRegistryEvents {
        AddressesProviderRegisteredFilter(AddressesProviderRegisteredFilter),
        AddressesProviderUnregisteredFilter(AddressesProviderUnregisteredFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for PoolAddressesProviderRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddressesProviderRegisteredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AddressesProviderUnregisteredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    PoolAddressesProviderRegistryEvents::OwnershipTransferredFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PoolAddressesProviderRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProviderRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressesProviderUnregisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderRegisteredFilter>
    for PoolAddressesProviderRegistryEvents {
        fn from(value: AddressesProviderRegisteredFilter) -> Self {
            Self::AddressesProviderRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<AddressesProviderUnregisteredFilter>
    for PoolAddressesProviderRegistryEvents {
        fn from(value: AddressesProviderUnregisteredFilter) -> Self {
            Self::AddressesProviderUnregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for PoolAddressesProviderRegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `getAddressesProviderAddressById` function with signature `getAddressesProviderAddressById(uint256)` and selector `0x57dc0566`
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
        name = "getAddressesProviderAddressById",
        abi = "getAddressesProviderAddressById(uint256)"
    )]
    pub struct GetAddressesProviderAddressByIdCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAddressesProviderIdByAddress` function with signature `getAddressesProviderIdByAddress(address)` and selector `0xd0267be7`
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
        name = "getAddressesProviderIdByAddress",
        abi = "getAddressesProviderIdByAddress(address)"
    )]
    pub struct GetAddressesProviderIdByAddressCall {
        pub addresses_provider: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAddressesProvidersList` function with signature `getAddressesProvidersList()` and selector `0x365ccbbf`
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
    #[ethcall(name = "getAddressesProvidersList", abi = "getAddressesProvidersList()")]
    pub struct GetAddressesProvidersListCall;
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
    ///Container type for all input parameters for the `registerAddressesProvider` function with signature `registerAddressesProvider(address,uint256)` and selector `0xd258191e`
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
        name = "registerAddressesProvider",
        abi = "registerAddressesProvider(address,uint256)"
    )]
    pub struct RegisterAddressesProviderCall {
        pub provider: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
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
    ///Container type for all input parameters for the `unregisterAddressesProvider` function with signature `unregisterAddressesProvider(address)` and selector `0x0de26707`
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
        name = "unregisterAddressesProvider",
        abi = "unregisterAddressesProvider(address)"
    )]
    pub struct UnregisterAddressesProviderCall {
        pub provider: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolAddressesProviderRegistryCalls {
        GetAddressesProviderAddressById(GetAddressesProviderAddressByIdCall),
        GetAddressesProviderIdByAddress(GetAddressesProviderIdByAddressCall),
        GetAddressesProvidersList(GetAddressesProvidersListCall),
        Owner(OwnerCall),
        RegisterAddressesProvider(RegisterAddressesProviderCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        UnregisterAddressesProvider(UnregisterAddressesProviderCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolAddressesProviderRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetAddressesProviderAddressByIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAddressesProviderAddressById(decoded));
            }
            if let Ok(decoded)
                = <GetAddressesProviderIdByAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAddressesProviderIdByAddress(decoded));
            }
            if let Ok(decoded)
                = <GetAddressesProvidersListCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAddressesProvidersList(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RegisterAddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RegisterAddressesProvider(decoded));
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
            if let Ok(decoded)
                = <UnregisterAddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnregisterAddressesProvider(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolAddressesProviderRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAddressesProviderAddressById(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressesProviderIdByAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAddressesProvidersList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterAddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnregisterAddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoolAddressesProviderRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAddressesProviderAddressById(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAddressesProviderIdByAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAddressesProvidersList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterAddressesProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnregisterAddressesProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetAddressesProviderAddressByIdCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProviderAddressByIdCall) -> Self {
            Self::GetAddressesProviderAddressById(value)
        }
    }
    impl ::core::convert::From<GetAddressesProviderIdByAddressCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProviderIdByAddressCall) -> Self {
            Self::GetAddressesProviderIdByAddress(value)
        }
    }
    impl ::core::convert::From<GetAddressesProvidersListCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProvidersListCall) -> Self {
            Self::GetAddressesProvidersList(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PoolAddressesProviderRegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterAddressesProviderCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: RegisterAddressesProviderCall) -> Self {
            Self::RegisterAddressesProvider(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnregisterAddressesProviderCall>
    for PoolAddressesProviderRegistryCalls {
        fn from(value: UnregisterAddressesProviderCall) -> Self {
            Self::UnregisterAddressesProvider(value)
        }
    }
    ///Container type for all return fields from the `getAddressesProviderAddressById` function with signature `getAddressesProviderAddressById(uint256)` and selector `0x57dc0566`
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
    pub struct GetAddressesProviderAddressByIdReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAddressesProviderIdByAddress` function with signature `getAddressesProviderIdByAddress(address)` and selector `0xd0267be7`
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
    pub struct GetAddressesProviderIdByAddressReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAddressesProvidersList` function with signature `getAddressesProvidersList()` and selector `0x365ccbbf`
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
    pub struct GetAddressesProvidersListReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
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
}
