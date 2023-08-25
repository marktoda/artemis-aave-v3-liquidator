pub use iacl_manager::*;
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
pub mod iacl_manager {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IACLMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IACLManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IACLManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IACLManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IACLManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IACLManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IACLManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IACLManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IACLMANAGER_ABI.clone(),
                    client,
                ),
            )
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IACLManager<M> {
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IACLManagerCalls {
        AddressesProvider(AddressesProviderCall),
        AssetListingAdminRole(AssetListingAdminRoleCall),
        BridgeRole(BridgeRoleCall),
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
        SetRoleAdmin(SetRoleAdminCall),
    }
    impl ::ethers::core::abi::AbiDecode for IACLManagerCalls {
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
                = <SetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRoleAdmin(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IACLManagerCalls {
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
                Self::SetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IACLManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetListingAdminRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BridgeRole(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for IACLManagerCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<AssetListingAdminRoleCall> for IACLManagerCalls {
        fn from(value: AssetListingAdminRoleCall) -> Self {
            Self::AssetListingAdminRole(value)
        }
    }
    impl ::core::convert::From<BridgeRoleCall> for IACLManagerCalls {
        fn from(value: BridgeRoleCall) -> Self {
            Self::BridgeRole(value)
        }
    }
    impl ::core::convert::From<EmergencyAdminRoleCall> for IACLManagerCalls {
        fn from(value: EmergencyAdminRoleCall) -> Self {
            Self::EmergencyAdminRole(value)
        }
    }
    impl ::core::convert::From<FlashBorrowerRoleCall> for IACLManagerCalls {
        fn from(value: FlashBorrowerRoleCall) -> Self {
            Self::FlashBorrowerRole(value)
        }
    }
    impl ::core::convert::From<PoolAdminRoleCall> for IACLManagerCalls {
        fn from(value: PoolAdminRoleCall) -> Self {
            Self::PoolAdminRole(value)
        }
    }
    impl ::core::convert::From<RiskAdminRoleCall> for IACLManagerCalls {
        fn from(value: RiskAdminRoleCall) -> Self {
            Self::RiskAdminRole(value)
        }
    }
    impl ::core::convert::From<AddAssetListingAdminCall> for IACLManagerCalls {
        fn from(value: AddAssetListingAdminCall) -> Self {
            Self::AddAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<AddBridgeCall> for IACLManagerCalls {
        fn from(value: AddBridgeCall) -> Self {
            Self::AddBridge(value)
        }
    }
    impl ::core::convert::From<AddEmergencyAdminCall> for IACLManagerCalls {
        fn from(value: AddEmergencyAdminCall) -> Self {
            Self::AddEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<AddFlashBorrowerCall> for IACLManagerCalls {
        fn from(value: AddFlashBorrowerCall) -> Self {
            Self::AddFlashBorrower(value)
        }
    }
    impl ::core::convert::From<AddPoolAdminCall> for IACLManagerCalls {
        fn from(value: AddPoolAdminCall) -> Self {
            Self::AddPoolAdmin(value)
        }
    }
    impl ::core::convert::From<AddRiskAdminCall> for IACLManagerCalls {
        fn from(value: AddRiskAdminCall) -> Self {
            Self::AddRiskAdmin(value)
        }
    }
    impl ::core::convert::From<IsAssetListingAdminCall> for IACLManagerCalls {
        fn from(value: IsAssetListingAdminCall) -> Self {
            Self::IsAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<IsBridgeCall> for IACLManagerCalls {
        fn from(value: IsBridgeCall) -> Self {
            Self::IsBridge(value)
        }
    }
    impl ::core::convert::From<IsEmergencyAdminCall> for IACLManagerCalls {
        fn from(value: IsEmergencyAdminCall) -> Self {
            Self::IsEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<IsFlashBorrowerCall> for IACLManagerCalls {
        fn from(value: IsFlashBorrowerCall) -> Self {
            Self::IsFlashBorrower(value)
        }
    }
    impl ::core::convert::From<IsPoolAdminCall> for IACLManagerCalls {
        fn from(value: IsPoolAdminCall) -> Self {
            Self::IsPoolAdmin(value)
        }
    }
    impl ::core::convert::From<IsRiskAdminCall> for IACLManagerCalls {
        fn from(value: IsRiskAdminCall) -> Self {
            Self::IsRiskAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveAssetListingAdminCall> for IACLManagerCalls {
        fn from(value: RemoveAssetListingAdminCall) -> Self {
            Self::RemoveAssetListingAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveBridgeCall> for IACLManagerCalls {
        fn from(value: RemoveBridgeCall) -> Self {
            Self::RemoveBridge(value)
        }
    }
    impl ::core::convert::From<RemoveEmergencyAdminCall> for IACLManagerCalls {
        fn from(value: RemoveEmergencyAdminCall) -> Self {
            Self::RemoveEmergencyAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveFlashBorrowerCall> for IACLManagerCalls {
        fn from(value: RemoveFlashBorrowerCall) -> Self {
            Self::RemoveFlashBorrower(value)
        }
    }
    impl ::core::convert::From<RemovePoolAdminCall> for IACLManagerCalls {
        fn from(value: RemovePoolAdminCall) -> Self {
            Self::RemovePoolAdmin(value)
        }
    }
    impl ::core::convert::From<RemoveRiskAdminCall> for IACLManagerCalls {
        fn from(value: RemoveRiskAdminCall) -> Self {
            Self::RemoveRiskAdmin(value)
        }
    }
    impl ::core::convert::From<SetRoleAdminCall> for IACLManagerCalls {
        fn from(value: SetRoleAdminCall) -> Self {
            Self::SetRoleAdmin(value)
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
}
