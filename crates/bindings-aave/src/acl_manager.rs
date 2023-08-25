pub use acl_manager::*;
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
pub mod acl_manager {
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
                    ::std::borrow::ToOwned::to_owned("ASSET_LISTING_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ASSET_LISTING_ADMIN_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BRIDGE_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BRIDGE_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EMERGENCY_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EMERGENCY_ADMIN_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FLASH_BORROWER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FLASH_BORROWER_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("POOL_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RISK_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("RISK_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addAssetListingAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addAssetListingAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("addBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addBridge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bridge"),
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
                    ::std::borrow::ToOwned::to_owned("addEmergencyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addEmergencyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("addFlashBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addFlashBorrower"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
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
                    ::std::borrow::ToOwned::to_owned("addPoolAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPoolAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("addRiskAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addRiskAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isAssetListingAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isAssetListingAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isBridge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bridge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isEmergencyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isEmergencyAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isFlashBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isFlashBorrower"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPoolAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPoolAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isRiskAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isRiskAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeAssetListingAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAssetListingAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("removeBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeBridge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bridge"),
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
                    ::std::borrow::ToOwned::to_owned("removeEmergencyAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeEmergencyAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("removeFlashBorrower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeFlashBorrower",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("borrower"),
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
                    ::std::borrow::ToOwned::to_owned("removePoolAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePoolAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("removeRiskAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeRiskAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("setRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
    pub static ACLMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x10\xB58\x03\x80b\0\x10\xB5\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xE3V[\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x0Eg\x17\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xB5\x91\x90b\0\x01\xE3V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra75`\xF0\x1B` \x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16b\0\x01\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\0\xFB\x91\x90b\0\x02\nV[`@Q\x80\x91\x03\x90\xFD[Pb\0\x01\x12`\0\x82b\0\x01\x1AV[PPb\0\x02bV[b\0\x01&\x82\x82b\0\x01*V[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01&W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01\x863\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xE0W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15b\0\x01\xF6W`\0\x80\xFD[\x81Qb\0\x02\x03\x81b\0\x01\xCAV[\x93\x92PPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15b\0\x029W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01b\0\x02\x1BV[\x81\x81\x11\x15b\0\x02LW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\x80Qa\x0E7b\0\x02~`\09`\0a\x02B\x01Ra\x0E7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80cgK^M\x11a\x01\x1AW\x80c\x9A+\x96\xF7\x11a\0\xADW\x80c\xB5\xBF\xDD\xEA\x11a\0|W\x80c\xB5\xBF\xDD\xEA\x14a\x04rW\x80c\xB8\xF6\xDB\xA7\x14a\x04\x87W\x80c\xD5Gt\x1F\x14a\x04\x9CW\x80c\xF86\x95\xCB\x14a\x04\xAFW\x80c\xFAP\xF2\x97\x14a\x04\xC2W`\0\x80\xFD[\x80c\x9A+\x96\xF7\x14a\x041W\x80c\x9A\xC9\xD8\x0B\x14a\x04DW\x80c\xA2\x17\xFD\xDF\x14a\x04WW\x80c\xA2\x1B\xCE\x15\x14a\x04_W`\0\x80\xFD[\x80cz\x9A\x93\xF4\x11a\0\xE9W\x80cz\x9A\x93\xF4\x14a\x03\xE5W\x80c{\xE5<\xA1\x14a\x03\xF8W\x80c\x91\xD1HT\x14a\x04\x0BW\x80c\x97\x12\xFD\xF8\x14a\x04\x1EW`\0\x80\xFD[\x80cgK^M\x14a\x03\x95W\x80cnv\xFC\x8F\x14a\x03\xA8W\x80crf\0\xCE\x14a\x03\xBDW\x80cx\xBB\nC\x14a\x03\xD0W`\0\x80\xFD[\x80c%\0\xF2\xB6\x11a\x01\x92W\x80c<Z\x08\xE5\x11a\x01aW\x80c<Z\x08\xE5\x14a\x03EW\x80cO\x16\xB4%\x14a\x03XW\x80cUw\xB7\xA9\x14a\x03mW\x80c[\x9A\x94\xE4\x14a\x03\x82W`\0\x80\xFD[\x80c%\0\xF2\xB6\x14a\x02\xF9W\x80c%<\xF9\x80\x14a\x03\x0CW\x80c//\xF1]\x14a\x03\x1FW\x80c6V\x8A\xBE\x14a\x032W`\0\x80\xFD[\x80c\x17\x9E\xFB\t\x11a\x01\xCEW\x80c\x17\x9E\xFB\t\x14a\x02\x8FW\x80c\x1EN\0\x91\x14a\x02\xA2W\x80c\"e\x0C\xAF\x14a\x02\xB5W\x80c$\x8A\x9C\xA3\x14a\x02\xC8W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\0W\x80c\x04\xDF\x01}\x14a\x02(W\x80c\x05B\x97\\\x14a\x02=W\x80c\x13\xEE2\xE0\x14a\x02|W[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a\x0B\x11V[a\x04\xD5V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02;a\x0266`\x04a\x0BWV[a\x05\x0CV[\0[a\x02d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1FV[a\x02\x13a\x02\x8A6`\x04a\x0BWV[a\x05'V[a\x02;a\x02\x9D6`\x04a\x0BWV[a\x05AV[a\x02;a\x02\xB06`\x04a\x0BrV[a\x05YV[a\x02;a\x02\xC36`\x04a\x0BWV[a\x05tV[a\x02\xEBa\x02\xD66`\x04a\x0B\x94V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x1FV[a\x02\x13a\x03\x076`\x04a\x0BWV[a\x05\x8CV[a\x02;a\x03\x1A6`\x04a\x0BWV[a\x05\xA6V[a\x02;a\x03-6`\x04a\x0B\xADV[a\x05\xBEV[a\x02;a\x03@6`\x04a\x0B\xADV[a\x05\xE4V[a\x02;a\x03S6`\x04a\x0BWV[a\x06gV[a\x02\xEB`\0\x80Q` a\rb\x839\x81Q\x91R\x81V[a\x02\xEB`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x81V[a\x02;a\x03\x906`\x04a\x0BWV[a\x06\x7FV[a\x02\x13a\x03\xA36`\x04a\x0BWV[a\x06\x97V[a\x02\xEB`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x81V[a\x02\x13a\x03\xCB6`\x04a\x0BWV[a\x06\xB1V[a\x02\xEB`\0\x80Q` a\r\x82\x839\x81Q\x91R\x81V[a\x02;a\x03\xF36`\x04a\x0BWV[a\x06\xCBV[a\x02\x13a\x04\x066`\x04a\x0BWV[a\x06\xE3V[a\x02\x13a\x04\x196`\x04a\x0B\xADV[a\x06\xF9V[a\x02;a\x04,6`\x04a\x0BWV[a\x07\"V[a\x02;a\x04?6`\x04a\x0BWV[a\x07:V[a\x02;a\x04R6`\x04a\x0BWV[a\x07RV[a\x02\xEB`\0\x81V[a\x02;a\x04m6`\x04a\x0BWV[a\x07jV[a\x02\xEB`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x81V[a\x02\xEB`\0\x80Q` a\rB\x839\x81Q\x91R\x81V[a\x02;a\x04\xAA6`\x04a\x0B\xADV[a\x07~V[a\x02;a\x04\xBD6`\x04a\x0BWV[a\x07\xA4V[a\x02\x13a\x04\xD06`\x04a\x0BWV[a\x07\xBCV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\x06WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[a\x05$`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x82a\x07~V[PV[`\0a\x05\x06`\0\x80Q` a\r\x82\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05e\x813a\x07\xD6V[a\x05o\x83\x83a\x08:V[PPPV[a\x05$`\0\x80Q` a\rB\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05\x06`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x82a\x07~V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xDA\x813a\x07\xD6V[a\x05o\x83\x83a\x08\x85V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06c\x82\x82a\t\tV[PPV[a\x05$`\0\x80Q` a\rb\x839\x81Q\x91R\x82a\x07~V[a\x05$`\0\x80Q` a\rb\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05\x06`\0\x80Q` a\rb\x839\x81Q\x91R\x83a\x06\xF9V[`\0a\x05\x06`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x82a\x07~V[`\0a\x05\x06`\0\x80Q` a\rB\x839\x81Q\x91R\x83[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x05$`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\x82\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\x82\x839\x81Q\x91R\x82[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07\x9A\x813a\x07\xD6V[a\x05o\x83\x83a\t\tV[a\x05$`\0\x80Q` a\rB\x839\x81Q\x91R\x82a\x07~V[`\0a\x05\x06`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x83a\x06\xF9V[a\x07\xE0\x82\x82a\x06\xF9V[a\x06cWa\x07\xF8\x81`\x01`\x01`\xA0\x1B\x03\x16`\x14a\tnV[a\x08\x03\x83` a\tnV[`@Q` \x01a\x08\x14\x92\x91\x90a\x0C\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x06P\x91`\x04\x01a\x0C~V[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[a\x08\x8F\x82\x82a\x06\xF9V[a\x06cW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08\xC53\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\t\x13\x82\x82a\x06\xF9V[\x15a\x06cW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[```\0a\t}\x83`\x02a\x0C\xC7V[a\t\x88\x90`\x02a\x0C\xE6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xA0Wa\t\xA0a\x0C\xFEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\t\xE5Wa\t\xE5a\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\n\x14Wa\n\x14a\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\n8\x84`\x02a\x0C\xC7V[a\nC\x90`\x01a\x0C\xE6V[\x90P[`\x01\x81\x11\x15a\n\xBBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\nwWa\nwa\r\x14V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\x8DWa\n\x8Da\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n\xB4\x81a\r*V[\x90Pa\nFV[P\x83\x15a\x0B\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x06PV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B#W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\nW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BRW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0BiW`\0\x80\xFD[a\x0B\n\x82a\x0B;V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x85W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x0B\xA6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC0W`\0\x80\xFD[\x825\x91Pa\x0B\xD0` \x84\x01a\x0B;V[\x90P\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x0B\xF4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xDCV[\x83\x81\x11\x15a\x0C\x03W`\0\x84\x84\x01R[PPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x0CA\x81`\x17\x85\x01` \x88\x01a\x0B\xD9V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0Cr\x81`(\x84\x01` \x88\x01a\x0B\xD9V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x9D\x81`@\x85\x01` \x87\x01a\x0B\xD9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0C\xE1Wa\x0C\xE1a\x0C\xB1V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x0C\xF9Wa\x0C\xF9a\x0C\xB1V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\r9Wa\r9a\x0C\xB1V[P`\0\x19\x01\x90V\xFE\x12\xAD\x05\xBD\xE7\x8CZ\xB7R8\xCE\x88S\x07\xF9n\xCDH+\xB4\x02\xEF\x83\x1F\x99\xE7\x01\x8A\x0F\x16\x9B{\x8A\xA8U\xA9\x11Q\x8E\xCF\xBE[\xC3\x08\x8C\x8F=\xDA{\xAD\xF10\xFA\xAF\x8A\xCE3\xFD\xC38(\xE1\x81g\x19\xC8`\xA62X\xEF\xBD\x0E\xCB}U\xC6&#{\xF5\xC2\x04L&\xC0s9\x0Bt\xF0\xC1<\x85t3\x08\xFB1\xC3\xE8\x16$5l3\x14\x08\x8A\xA9q\xB7;\xCC\x82\xD2+\xC3\xE3\xB1\x84\xB4Y0w\xAE2x\\\x91Q@\x91\xAF1\xF6/Yj1J\xF7\xD5\xBE@\x14k/#U\x96\x93\x92\xF0U\xE1.\t\x82\xFB\x93\x9B\x8D\xFBW\xEC\xEF*\xEAT\xA9:\x15\xE8gh\xB9\xD4\x08\x9F\x1B\xA6\x1C$^n\xC9\x80i_L\xA4\xA2dipfsX\"\x12 \x1C\xC5\xCC\xAE\xF3\xA7q#\x129\x8F\x1C\x98z\x97\xD3|h\xDFi\x9ELn\xC4$\xA0\xB7\xBDO\xFEfudsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static ACLMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80cgK^M\x11a\x01\x1AW\x80c\x9A+\x96\xF7\x11a\0\xADW\x80c\xB5\xBF\xDD\xEA\x11a\0|W\x80c\xB5\xBF\xDD\xEA\x14a\x04rW\x80c\xB8\xF6\xDB\xA7\x14a\x04\x87W\x80c\xD5Gt\x1F\x14a\x04\x9CW\x80c\xF86\x95\xCB\x14a\x04\xAFW\x80c\xFAP\xF2\x97\x14a\x04\xC2W`\0\x80\xFD[\x80c\x9A+\x96\xF7\x14a\x041W\x80c\x9A\xC9\xD8\x0B\x14a\x04DW\x80c\xA2\x17\xFD\xDF\x14a\x04WW\x80c\xA2\x1B\xCE\x15\x14a\x04_W`\0\x80\xFD[\x80cz\x9A\x93\xF4\x11a\0\xE9W\x80cz\x9A\x93\xF4\x14a\x03\xE5W\x80c{\xE5<\xA1\x14a\x03\xF8W\x80c\x91\xD1HT\x14a\x04\x0BW\x80c\x97\x12\xFD\xF8\x14a\x04\x1EW`\0\x80\xFD[\x80cgK^M\x14a\x03\x95W\x80cnv\xFC\x8F\x14a\x03\xA8W\x80crf\0\xCE\x14a\x03\xBDW\x80cx\xBB\nC\x14a\x03\xD0W`\0\x80\xFD[\x80c%\0\xF2\xB6\x11a\x01\x92W\x80c<Z\x08\xE5\x11a\x01aW\x80c<Z\x08\xE5\x14a\x03EW\x80cO\x16\xB4%\x14a\x03XW\x80cUw\xB7\xA9\x14a\x03mW\x80c[\x9A\x94\xE4\x14a\x03\x82W`\0\x80\xFD[\x80c%\0\xF2\xB6\x14a\x02\xF9W\x80c%<\xF9\x80\x14a\x03\x0CW\x80c//\xF1]\x14a\x03\x1FW\x80c6V\x8A\xBE\x14a\x032W`\0\x80\xFD[\x80c\x17\x9E\xFB\t\x11a\x01\xCEW\x80c\x17\x9E\xFB\t\x14a\x02\x8FW\x80c\x1EN\0\x91\x14a\x02\xA2W\x80c\"e\x0C\xAF\x14a\x02\xB5W\x80c$\x8A\x9C\xA3\x14a\x02\xC8W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x02\0W\x80c\x04\xDF\x01}\x14a\x02(W\x80c\x05B\x97\\\x14a\x02=W\x80c\x13\xEE2\xE0\x14a\x02|W[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a\x0B\x11V[a\x04\xD5V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02;a\x0266`\x04a\x0BWV[a\x05\x0CV[\0[a\x02d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x1FV[a\x02\x13a\x02\x8A6`\x04a\x0BWV[a\x05'V[a\x02;a\x02\x9D6`\x04a\x0BWV[a\x05AV[a\x02;a\x02\xB06`\x04a\x0BrV[a\x05YV[a\x02;a\x02\xC36`\x04a\x0BWV[a\x05tV[a\x02\xEBa\x02\xD66`\x04a\x0B\x94V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x02\x1FV[a\x02\x13a\x03\x076`\x04a\x0BWV[a\x05\x8CV[a\x02;a\x03\x1A6`\x04a\x0BWV[a\x05\xA6V[a\x02;a\x03-6`\x04a\x0B\xADV[a\x05\xBEV[a\x02;a\x03@6`\x04a\x0B\xADV[a\x05\xE4V[a\x02;a\x03S6`\x04a\x0BWV[a\x06gV[a\x02\xEB`\0\x80Q` a\rb\x839\x81Q\x91R\x81V[a\x02\xEB`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x81V[a\x02;a\x03\x906`\x04a\x0BWV[a\x06\x7FV[a\x02\x13a\x03\xA36`\x04a\x0BWV[a\x06\x97V[a\x02\xEB`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x81V[a\x02\x13a\x03\xCB6`\x04a\x0BWV[a\x06\xB1V[a\x02\xEB`\0\x80Q` a\r\x82\x839\x81Q\x91R\x81V[a\x02;a\x03\xF36`\x04a\x0BWV[a\x06\xCBV[a\x02\x13a\x04\x066`\x04a\x0BWV[a\x06\xE3V[a\x02\x13a\x04\x196`\x04a\x0B\xADV[a\x06\xF9V[a\x02;a\x04,6`\x04a\x0BWV[a\x07\"V[a\x02;a\x04?6`\x04a\x0BWV[a\x07:V[a\x02;a\x04R6`\x04a\x0BWV[a\x07RV[a\x02\xEB`\0\x81V[a\x02;a\x04m6`\x04a\x0BWV[a\x07jV[a\x02\xEB`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x81V[a\x02\xEB`\0\x80Q` a\rB\x839\x81Q\x91R\x81V[a\x02;a\x04\xAA6`\x04a\x0B\xADV[a\x07~V[a\x02;a\x04\xBD6`\x04a\x0BWV[a\x07\xA4V[a\x02\x13a\x04\xD06`\x04a\x0BWV[a\x07\xBCV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x05\x06WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[a\x05$`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x82a\x07~V[PV[`\0a\x05\x06`\0\x80Q` a\r\x82\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05e\x813a\x07\xD6V[a\x05o\x83\x83a\x08:V[PPPV[a\x05$`\0\x80Q` a\rB\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05\x06`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x82a\x07~V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\xDA\x813a\x07\xD6V[a\x05o\x83\x83a\x08\x85V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x06YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x06c\x82\x82a\t\tV[PPV[a\x05$`\0\x80Q` a\rb\x839\x81Q\x91R\x82a\x07~V[a\x05$`\0\x80Q` a\rb\x839\x81Q\x91R\x82a\x05\xBEV[`\0a\x05\x06`\0\x80Q` a\rb\x839\x81Q\x91R\x83a\x06\xF9V[`\0a\x05\x06`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x83a\x06\xF9V[a\x05$`\0\x80Q` a\r\xC2\x839\x81Q\x91R\x82a\x07~V[`\0a\x05\x06`\0\x80Q` a\rB\x839\x81Q\x91R\x83[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x05$`\0\x80Q` a\r\xA2\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\x82\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x82a\x05\xBEV[a\x05$`\0\x80Q` a\r\x82\x839\x81Q\x91R\x82[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x07\x9A\x813a\x07\xD6V[a\x05o\x83\x83a\t\tV[a\x05$`\0\x80Q` a\rB\x839\x81Q\x91R\x82a\x07~V[`\0a\x05\x06`\0\x80Q` a\r\xE2\x839\x81Q\x91R\x83a\x06\xF9V[a\x07\xE0\x82\x82a\x06\xF9V[a\x06cWa\x07\xF8\x81`\x01`\x01`\xA0\x1B\x03\x16`\x14a\tnV[a\x08\x03\x83` a\tnV[`@Q` \x01a\x08\x14\x92\x91\x90a\x0C\tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x06P\x91`\x04\x01a\x0C~V[`\0\x82\x81R` \x81\x90R`@\x80\x82 `\x01\x01\x80T\x90\x84\x90U\x90Q\x90\x91\x83\x91\x83\x91\x86\x91\x7F\xBDy\xB8o\xFE\n\xB8\xE8waQQB\x17\xCD|\xAC\xD5,\x90\x9FfG\\:\xF4N\x12\x9F\x0B\0\xFF\x91\x90\xA4PPPV[a\x08\x8F\x82\x82a\x06\xF9V[a\x06cW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08\xC53\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\t\x13\x82\x82a\x06\xF9V[\x15a\x06cW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[```\0a\t}\x83`\x02a\x0C\xC7V[a\t\x88\x90`\x02a\x0C\xE6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xA0Wa\t\xA0a\x0C\xFEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\t\xCAW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\t\xE5Wa\t\xE5a\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\n\x14Wa\n\x14a\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\n8\x84`\x02a\x0C\xC7V[a\nC\x90`\x01a\x0C\xE6V[\x90P[`\x01\x81\x11\x15a\n\xBBWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\nwWa\nwa\r\x14V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\x8DWa\n\x8Da\r\x14V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n\xB4\x81a\r*V[\x90Pa\nFV[P\x83\x15a\x0B\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x06PV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B#W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\nW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BRW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0BiW`\0\x80\xFD[a\x0B\n\x82a\x0B;V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x85W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\x0B\xA6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xC0W`\0\x80\xFD[\x825\x91Pa\x0B\xD0` \x84\x01a\x0B;V[\x90P\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x0B\xF4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xDCV[\x83\x81\x11\x15a\x0C\x03W`\0\x84\x84\x01R[PPPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x0CA\x81`\x17\x85\x01` \x88\x01a\x0B\xD9V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x0Cr\x81`(\x84\x01` \x88\x01a\x0B\xD9V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x9D\x81`@\x85\x01` \x87\x01a\x0B\xD9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0C\xE1Wa\x0C\xE1a\x0C\xB1V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x0C\xF9Wa\x0C\xF9a\x0C\xB1V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\r9Wa\r9a\x0C\xB1V[P`\0\x19\x01\x90V\xFE\x12\xAD\x05\xBD\xE7\x8CZ\xB7R8\xCE\x88S\x07\xF9n\xCDH+\xB4\x02\xEF\x83\x1F\x99\xE7\x01\x8A\x0F\x16\x9B{\x8A\xA8U\xA9\x11Q\x8E\xCF\xBE[\xC3\x08\x8C\x8F=\xDA{\xAD\xF10\xFA\xAF\x8A\xCE3\xFD\xC38(\xE1\x81g\x19\xC8`\xA62X\xEF\xBD\x0E\xCB}U\xC6&#{\xF5\xC2\x04L&\xC0s9\x0Bt\xF0\xC1<\x85t3\x08\xFB1\xC3\xE8\x16$5l3\x14\x08\x8A\xA9q\xB7;\xCC\x82\xD2+\xC3\xE3\xB1\x84\xB4Y0w\xAE2x\\\x91Q@\x91\xAF1\xF6/Yj1J\xF7\xD5\xBE@\x14k/#U\x96\x93\x92\xF0U\xE1.\t\x82\xFB\x93\x9B\x8D\xFBW\xEC\xEF*\xEAT\xA9:\x15\xE8gh\xB9\xD4\x08\x9F\x1B\xA6\x1C$^n\xC9\x80i_L\xA4\xA2dipfsX\"\x12 \x1C\xC5\xCC\xAE\xF3\xA7q#\x129\x8F\x1C\x98z\x97\xD3|h\xDFi\x9ELn\xC4$\xA0\xB7\xBDO\xFEfudsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static ACLMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ACLManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ACLManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ACLManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ACLManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ACLManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ACLManager)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ACLManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACLMANAGER_ABI.clone(),
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
                ACLMANAGER_ABI.clone(),
                ACLMANAGER_BYTECODE.clone().into(),
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
        ///Calls the contract's `ASSET_LISTING_ADMIN_ROLE` (0x78bb0a43) function
        pub fn asset_listing_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([120, 187, 10, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BRIDGE_ROLE` (0xb5bfddea) function
        pub fn bridge_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([181, 191, 221, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EMERGENCY_ADMIN_ROLE` (0x6e76fc8f) function
        pub fn emergency_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([110, 118, 252, 143], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLASH_BORROWER_ROLE` (0x5577b7a9) function
        pub fn flash_borrower_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([85, 119, 183, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL_ADMIN_ROLE` (0xb8f6dba7) function
        pub fn pool_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([184, 246, 219, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RISK_ADMIN_ROLE` (0x4f16b425) function
        pub fn risk_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([79, 22, 180, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAssetListingAdmin` (0x9a2b96f7) function
        pub fn add_asset_listing_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 43, 150, 247], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBridge` (0x9712fdf8) function
        pub fn add_bridge(
            &self,
            bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 18, 253, 248], bridge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addEmergencyAdmin` (0x179efb09) function
        pub fn add_emergency_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 158, 251, 9], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addFlashBorrower` (0x9ac9d80b) function
        pub fn add_flash_borrower(
            &self,
            borrower: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 201, 216, 11], borrower)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPoolAdmin` (0x22650caf) function
        pub fn add_pool_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 101, 12, 175], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addRiskAdmin` (0x5b9a94e4) function
        pub fn add_risk_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 154, 148, 228], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAssetListingAdmin` (0x13ee32e0) function
        pub fn is_asset_listing_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 238, 50, 224], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isBridge` (0x726600ce) function
        pub fn is_bridge(
            &self,
            bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([114, 102, 0, 206], bridge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isEmergencyAdmin` (0x2500f2b6) function
        pub fn is_emergency_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 0, 242, 182], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFlashBorrower` (0xfa50f297) function
        pub fn is_flash_borrower(
            &self,
            borrower: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 80, 242, 151], borrower)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPoolAdmin` (0x7be53ca1) function
        pub fn is_pool_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 229, 60, 161], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isRiskAdmin` (0x674b5e4d) function
        pub fn is_risk_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([103, 75, 94, 77], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAssetListingAdmin` (0xa21bce15) function
        pub fn remove_asset_listing_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 27, 206, 21], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeBridge` (0x04df017d) function
        pub fn remove_bridge(
            &self,
            bridge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 223, 1, 125], bridge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeEmergencyAdmin` (0x7a9a93f4) function
        pub fn remove_emergency_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 154, 147, 244], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFlashBorrower` (0x253cf980) function
        pub fn remove_flash_borrower(
            &self,
            borrower: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 60, 249, 128], borrower)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePoolAdmin` (0xf83695cb) function
        pub fn remove_pool_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 54, 149, 203], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeRiskAdmin` (0x3c5a08e5) function
        pub fn remove_risk_admin(
            &self,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 90, 8, 229], admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRoleAdmin` (0x1e4e0091) function
        pub fn set_role_admin(
            &self,
            role: [u8; 32],
            admin_role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 78, 0, 145], (role, admin_role))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ACLManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ACLManager<M> {
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ACLManagerEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ACLManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(ACLManagerEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ACLManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for ACLManagerEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for ACLManagerEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for ACLManagerEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
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
    ///Container type for all input parameters for the `ASSET_LISTING_ADMIN_ROLE` function with signature `ASSET_LISTING_ADMIN_ROLE()` and selector `0x78bb0a43`
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
    #[ethcall(name = "ASSET_LISTING_ADMIN_ROLE", abi = "ASSET_LISTING_ADMIN_ROLE()")]
    pub struct AssetListingAdminRoleCall;
    ///Container type for all input parameters for the `BRIDGE_ROLE` function with signature `BRIDGE_ROLE()` and selector `0xb5bfddea`
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
    #[ethcall(name = "BRIDGE_ROLE", abi = "BRIDGE_ROLE()")]
    pub struct BridgeRoleCall;
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `EMERGENCY_ADMIN_ROLE` function with signature `EMERGENCY_ADMIN_ROLE()` and selector `0x6e76fc8f`
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
    #[ethcall(name = "EMERGENCY_ADMIN_ROLE", abi = "EMERGENCY_ADMIN_ROLE()")]
    pub struct EmergencyAdminRoleCall;
    ///Container type for all input parameters for the `FLASH_BORROWER_ROLE` function with signature `FLASH_BORROWER_ROLE()` and selector `0x5577b7a9`
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
    #[ethcall(name = "FLASH_BORROWER_ROLE", abi = "FLASH_BORROWER_ROLE()")]
    pub struct FlashBorrowerRoleCall;
    ///Container type for all input parameters for the `POOL_ADMIN_ROLE` function with signature `POOL_ADMIN_ROLE()` and selector `0xb8f6dba7`
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
    #[ethcall(name = "POOL_ADMIN_ROLE", abi = "POOL_ADMIN_ROLE()")]
    pub struct PoolAdminRoleCall;
    ///Container type for all input parameters for the `RISK_ADMIN_ROLE` function with signature `RISK_ADMIN_ROLE()` and selector `0x4f16b425`
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
    #[ethcall(name = "RISK_ADMIN_ROLE", abi = "RISK_ADMIN_ROLE()")]
    pub struct RiskAdminRoleCall;
    ///Container type for all input parameters for the `addAssetListingAdmin` function with signature `addAssetListingAdmin(address)` and selector `0x9a2b96f7`
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
    #[ethcall(name = "addAssetListingAdmin", abi = "addAssetListingAdmin(address)")]
    pub struct AddAssetListingAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addBridge` function with signature `addBridge(address)` and selector `0x9712fdf8`
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
    #[ethcall(name = "addBridge", abi = "addBridge(address)")]
    pub struct AddBridgeCall {
        pub bridge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addEmergencyAdmin` function with signature `addEmergencyAdmin(address)` and selector `0x179efb09`
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
    #[ethcall(name = "addEmergencyAdmin", abi = "addEmergencyAdmin(address)")]
    pub struct AddEmergencyAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addFlashBorrower` function with signature `addFlashBorrower(address)` and selector `0x9ac9d80b`
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
    #[ethcall(name = "addFlashBorrower", abi = "addFlashBorrower(address)")]
    pub struct AddFlashBorrowerCall {
        pub borrower: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addPoolAdmin` function with signature `addPoolAdmin(address)` and selector `0x22650caf`
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
    #[ethcall(name = "addPoolAdmin", abi = "addPoolAdmin(address)")]
    pub struct AddPoolAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addRiskAdmin` function with signature `addRiskAdmin(address)` and selector `0x5b9a94e4`
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
    #[ethcall(name = "addRiskAdmin", abi = "addRiskAdmin(address)")]
    pub struct AddRiskAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAssetListingAdmin` function with signature `isAssetListingAdmin(address)` and selector `0x13ee32e0`
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
    #[ethcall(name = "isAssetListingAdmin", abi = "isAssetListingAdmin(address)")]
    pub struct IsAssetListingAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isBridge` function with signature `isBridge(address)` and selector `0x726600ce`
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
    #[ethcall(name = "isBridge", abi = "isBridge(address)")]
    pub struct IsBridgeCall {
        pub bridge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isEmergencyAdmin` function with signature `isEmergencyAdmin(address)` and selector `0x2500f2b6`
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
    #[ethcall(name = "isEmergencyAdmin", abi = "isEmergencyAdmin(address)")]
    pub struct IsEmergencyAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isFlashBorrower` function with signature `isFlashBorrower(address)` and selector `0xfa50f297`
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
    #[ethcall(name = "isFlashBorrower", abi = "isFlashBorrower(address)")]
    pub struct IsFlashBorrowerCall {
        pub borrower: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPoolAdmin` function with signature `isPoolAdmin(address)` and selector `0x7be53ca1`
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
    #[ethcall(name = "isPoolAdmin", abi = "isPoolAdmin(address)")]
    pub struct IsPoolAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isRiskAdmin` function with signature `isRiskAdmin(address)` and selector `0x674b5e4d`
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
    #[ethcall(name = "isRiskAdmin", abi = "isRiskAdmin(address)")]
    pub struct IsRiskAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeAssetListingAdmin` function with signature `removeAssetListingAdmin(address)` and selector `0xa21bce15`
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
        name = "removeAssetListingAdmin",
        abi = "removeAssetListingAdmin(address)"
    )]
    pub struct RemoveAssetListingAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeBridge` function with signature `removeBridge(address)` and selector `0x04df017d`
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
    #[ethcall(name = "removeBridge", abi = "removeBridge(address)")]
    pub struct RemoveBridgeCall {
        pub bridge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeEmergencyAdmin` function with signature `removeEmergencyAdmin(address)` and selector `0x7a9a93f4`
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
    #[ethcall(name = "removeEmergencyAdmin", abi = "removeEmergencyAdmin(address)")]
    pub struct RemoveEmergencyAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeFlashBorrower` function with signature `removeFlashBorrower(address)` and selector `0x253cf980`
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
    #[ethcall(name = "removeFlashBorrower", abi = "removeFlashBorrower(address)")]
    pub struct RemoveFlashBorrowerCall {
        pub borrower: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removePoolAdmin` function with signature `removePoolAdmin(address)` and selector `0xf83695cb`
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
    #[ethcall(name = "removePoolAdmin", abi = "removePoolAdmin(address)")]
    pub struct RemovePoolAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeRiskAdmin` function with signature `removeRiskAdmin(address)` and selector `0x3c5a08e5`
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
    #[ethcall(name = "removeRiskAdmin", abi = "removeRiskAdmin(address)")]
    pub struct RemoveRiskAdminCall {
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRoleAdmin` function with signature `setRoleAdmin(bytes32,bytes32)` and selector `0x1e4e0091`
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
    #[ethcall(name = "setRoleAdmin", abi = "setRoleAdmin(bytes32,bytes32)")]
    pub struct SetRoleAdminCall {
        pub role: [u8; 32],
        pub admin_role: [u8; 32],
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ACLManagerCalls {
        AddressesProvider(AddressesProviderCall),
        AssetListingAdminRole(AssetListingAdminRoleCall),
        BridgeRole(BridgeRoleCall),
        DefaultAdminRole(DefaultAdminRoleCall),
        EmergencyAdminRole(EmergencyAdminRoleCall),
        FlashBorrowerRole(FlashBorrowerRoleCall),
        PoolAdminRole(PoolAdminRoleCall),
        RiskAdminRole(RiskAdminRoleCall),
        AddAssetListingAdmin(AddAssetListingAdminCall),
        AddBridge(AddBridgeCall),
        AddEmergencyAdmin(AddEmergencyAdminCall),
        AddFlashBorrower(AddFlashBorrowerCall),
        AddPoolAdmin(AddPoolAdminCall),
        AddRiskAdmin(AddRiskAdminCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsAssetListingAdmin(IsAssetListingAdminCall),
        IsBridge(IsBridgeCall),
        IsEmergencyAdmin(IsEmergencyAdminCall),
        IsFlashBorrower(IsFlashBorrowerCall),
        IsPoolAdmin(IsPoolAdminCall),
        IsRiskAdmin(IsRiskAdminCall),
        RemoveAssetListingAdmin(RemoveAssetListingAdminCall),
        RemoveBridge(RemoveBridgeCall),
        RemoveEmergencyAdmin(RemoveEmergencyAdminCall),
        RemoveFlashBorrower(RemoveFlashBorrowerCall),
        RemovePoolAdmin(RemovePoolAdminCall),
        RemoveRiskAdmin(RemoveRiskAdminCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetRoleAdmin(SetRoleAdminCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ACLManagerCalls {
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
                = <AssetListingAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AssetListingAdminRole(decoded));
            }
            if let Ok(decoded)
                = <BridgeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BridgeRole(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <EmergencyAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EmergencyAdminRole(decoded));
            }
            if let Ok(decoded)
                = <FlashBorrowerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FlashBorrowerRole(decoded));
            }
            if let Ok(decoded)
                = <PoolAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolAdminRole(decoded));
            }
            if let Ok(decoded)
                = <RiskAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RiskAdminRole(decoded));
            }
            if let Ok(decoded)
                = <AddAssetListingAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddAssetListingAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddBridgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddBridge(decoded));
            }
            if let Ok(decoded)
                = <AddEmergencyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddEmergencyAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddFlashBorrowerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AddFlashBorrower(decoded));
            }
            if let Ok(decoded)
                = <AddPoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <AddRiskAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddRiskAdmin(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <IsAssetListingAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAssetListingAdmin(decoded));
            }
            if let Ok(decoded)
                = <IsBridgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsBridge(decoded));
            }
            if let Ok(decoded)
                = <IsEmergencyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsEmergencyAdmin(decoded));
            }
            if let Ok(decoded)
                = <IsFlashBorrowerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsFlashBorrower(decoded));
            }
            if let Ok(decoded)
                = <IsPoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <IsRiskAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsRiskAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveAssetListingAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveAssetListingAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveBridgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveBridge(decoded));
            }
            if let Ok(decoded)
                = <RemoveEmergencyAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveEmergencyAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveFlashBorrowerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemoveFlashBorrower(decoded));
            }
            if let Ok(decoded)
                = <RemovePoolAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemovePoolAdmin(decoded));
            }
            if let Ok(decoded)
                = <RemoveRiskAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemoveRiskAdmin(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <SetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ACLManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetListingAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BridgeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmergencyAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashBorrowerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RiskAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAssetListingAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddEmergencyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddFlashBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddRiskAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsAssetListingAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsEmergencyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsFlashBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsRiskAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAssetListingAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveEmergencyAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFlashBorrower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePoolAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveRiskAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ACLManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetListingAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmergencyAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashBorrowerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RiskAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAssetListingAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEmergencyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddFlashBorrower(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPoolAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddRiskAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAssetListingAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsEmergencyAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsFlashBorrower(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPoolAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRiskAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAssetListingAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveEmergencyAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveFlashBorrower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovePoolAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveRiskAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for ACLManagerCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<AssetListingAdminRoleCall> for ACLManagerCalls {
        fn from(value: AssetListingAdminRoleCall) -> Self {
            Self::AssetListingAdminRole(value)
        }
    }
    impl ::core::convert::From<BridgeRoleCall> for ACLManagerCalls {
        fn from(value: BridgeRoleCall) -> Self {
            Self::BridgeRole(value)
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for ACLManagerCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<EmergencyAdminRoleCall> for ACLManagerCalls {
        fn from(value: EmergencyAdminRoleCall) -> Self {
            Self::EmergencyAdminRole(value)
        }
    }
    impl ::core::convert::From<FlashBorrowerRoleCall> for ACLManagerCalls {
        fn from(value: FlashBorrowerRoleCall) -> Self {
            Self::FlashBorrowerRole(value)
        }
    }
    impl ::core::convert::From<PoolAdminRoleCall> for ACLManagerCalls {
        fn from(value: PoolAdminRoleCall) -> Self {
            Self::PoolAdminRole(value)
        }
    }
    impl ::core::convert::From<RiskAdminRoleCall> for ACLManagerCalls {
        fn from(value: RiskAdminRoleCall) -> Self {
            Self::RiskAdminRole(value)
        }
    }
    impl ::core::convert::From<AddAssetListingAdminCall> for ACLManagerCalls {
        fn from(value: AddAssetListingAdminCall) -> Self {
            Self::AddAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<AddBridgeCall> for ACLManagerCalls {
        fn from(value: AddBridgeCall) -> Self {
            Self::AddBridge(value)
        }
    }
    impl ::core::convert::From<AddEmergencyAdminCall> for ACLManagerCalls {
        fn from(value: AddEmergencyAdminCall) -> Self {
            Self::AddEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<AddFlashBorrowerCall> for ACLManagerCalls {
        fn from(value: AddFlashBorrowerCall) -> Self {
            Self::AddFlashBorrower(value)
        }
    }
    impl ::core::convert::From<AddPoolAdminCall> for ACLManagerCalls {
        fn from(value: AddPoolAdminCall) -> Self {
            Self::AddPoolAdmin(value)
        }
    }
    impl ::core::convert::From<AddRiskAdminCall> for ACLManagerCalls {
        fn from(value: AddRiskAdminCall) -> Self {
            Self::AddRiskAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for ACLManagerCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for ACLManagerCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for ACLManagerCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsAssetListingAdminCall> for ACLManagerCalls {
        fn from(value: IsAssetListingAdminCall) -> Self {
            Self::IsAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<IsBridgeCall> for ACLManagerCalls {
        fn from(value: IsBridgeCall) -> Self {
            Self::IsBridge(value)
        }
    }
    impl ::core::convert::From<IsEmergencyAdminCall> for ACLManagerCalls {
        fn from(value: IsEmergencyAdminCall) -> Self {
            Self::IsEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<IsFlashBorrowerCall> for ACLManagerCalls {
        fn from(value: IsFlashBorrowerCall) -> Self {
            Self::IsFlashBorrower(value)
        }
    }
    impl ::core::convert::From<IsPoolAdminCall> for ACLManagerCalls {
        fn from(value: IsPoolAdminCall) -> Self {
            Self::IsPoolAdmin(value)
        }
    }
    impl ::core::convert::From<IsRiskAdminCall> for ACLManagerCalls {
        fn from(value: IsRiskAdminCall) -> Self {
            Self::IsRiskAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveAssetListingAdminCall> for ACLManagerCalls {
        fn from(value: RemoveAssetListingAdminCall) -> Self {
            Self::RemoveAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveBridgeCall> for ACLManagerCalls {
        fn from(value: RemoveBridgeCall) -> Self {
            Self::RemoveBridge(value)
        }
    }
    impl ::core::convert::From<RemoveEmergencyAdminCall> for ACLManagerCalls {
        fn from(value: RemoveEmergencyAdminCall) -> Self {
            Self::RemoveEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveFlashBorrowerCall> for ACLManagerCalls {
        fn from(value: RemoveFlashBorrowerCall) -> Self {
            Self::RemoveFlashBorrower(value)
        }
    }
    impl ::core::convert::From<RemovePoolAdminCall> for ACLManagerCalls {
        fn from(value: RemovePoolAdminCall) -> Self {
            Self::RemovePoolAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveRiskAdminCall> for ACLManagerCalls {
        fn from(value: RemoveRiskAdminCall) -> Self {
            Self::RemoveRiskAdmin(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for ACLManagerCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for ACLManagerCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetRoleAdminCall> for ACLManagerCalls {
        fn from(value: SetRoleAdminCall) -> Self {
            Self::SetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ACLManagerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
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
    ///Container type for all return fields from the `ASSET_LISTING_ADMIN_ROLE` function with signature `ASSET_LISTING_ADMIN_ROLE()` and selector `0x78bb0a43`
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
    pub struct AssetListingAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `BRIDGE_ROLE` function with signature `BRIDGE_ROLE()` and selector `0xb5bfddea`
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
    pub struct BridgeRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `EMERGENCY_ADMIN_ROLE` function with signature `EMERGENCY_ADMIN_ROLE()` and selector `0x6e76fc8f`
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
    pub struct EmergencyAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `FLASH_BORROWER_ROLE` function with signature `FLASH_BORROWER_ROLE()` and selector `0x5577b7a9`
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
    pub struct FlashBorrowerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `POOL_ADMIN_ROLE` function with signature `POOL_ADMIN_ROLE()` and selector `0xb8f6dba7`
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
    pub struct PoolAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `RISK_ADMIN_ROLE` function with signature `RISK_ADMIN_ROLE()` and selector `0x4f16b425`
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
    pub struct RiskAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `isAssetListingAdmin` function with signature `isAssetListingAdmin(address)` and selector `0x13ee32e0`
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
    pub struct IsAssetListingAdminReturn(pub bool);
    ///Container type for all return fields from the `isBridge` function with signature `isBridge(address)` and selector `0x726600ce`
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
    pub struct IsBridgeReturn(pub bool);
    ///Container type for all return fields from the `isEmergencyAdmin` function with signature `isEmergencyAdmin(address)` and selector `0x2500f2b6`
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
    pub struct IsEmergencyAdminReturn(pub bool);
    ///Container type for all return fields from the `isFlashBorrower` function with signature `isFlashBorrower(address)` and selector `0xfa50f297`
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
    pub struct IsFlashBorrowerReturn(pub bool);
    ///Container type for all return fields from the `isPoolAdmin` function with signature `isPoolAdmin(address)` and selector `0x7be53ca1`
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
    pub struct IsPoolAdminReturn(pub bool);
    ///Container type for all return fields from the `isRiskAdmin` function with signature `isRiskAdmin(address)` and selector `0x674b5e4d`
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
    pub struct IsRiskAdminReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
