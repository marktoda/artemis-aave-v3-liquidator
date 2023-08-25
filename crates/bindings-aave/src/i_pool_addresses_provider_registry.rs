pub use i_pool_addresses_provider_registry::*;
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
pub mod i_pool_addresses_provider_registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IPOOLADDRESSESPROVIDERREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IPoolAddressesProviderRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPoolAddressesProviderRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPoolAddressesProviderRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPoolAddressesProviderRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPoolAddressesProviderRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPoolAddressesProviderRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPoolAddressesProviderRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPOOLADDRESSESPROVIDERREGISTRY_ABI.clone(),
                    client,
                ),
            )
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPoolAddressesProviderRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPoolAddressesProviderRegistry<M> {
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
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPoolAddressesProviderRegistryEvents {
        AddressesProviderRegisteredFilter(AddressesProviderRegisteredFilter),
        AddressesProviderUnregisteredFilter(AddressesProviderUnregisteredFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPoolAddressesProviderRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddressesProviderRegisteredFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderRegistryEvents::AddressesProviderRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = AddressesProviderUnregisteredFilter::decode_log(log) {
                return Ok(
                    IPoolAddressesProviderRegistryEvents::AddressesProviderUnregisteredFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPoolAddressesProviderRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProviderRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddressesProviderUnregisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddressesProviderRegisteredFilter>
    for IPoolAddressesProviderRegistryEvents {
        fn from(value: AddressesProviderRegisteredFilter) -> Self {
            Self::AddressesProviderRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<AddressesProviderUnregisteredFilter>
    for IPoolAddressesProviderRegistryEvents {
        fn from(value: AddressesProviderUnregisteredFilter) -> Self {
            Self::AddressesProviderUnregisteredFilter(value)
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
    pub enum IPoolAddressesProviderRegistryCalls {
        GetAddressesProviderAddressById(GetAddressesProviderAddressByIdCall),
        GetAddressesProviderIdByAddress(GetAddressesProviderIdByAddressCall),
        GetAddressesProvidersList(GetAddressesProvidersListCall),
        RegisterAddressesProvider(RegisterAddressesProviderCall),
        UnregisterAddressesProvider(UnregisterAddressesProviderCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPoolAddressesProviderRegistryCalls {
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
                = <RegisterAddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RegisterAddressesProvider(decoded));
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
    impl ::ethers::core::abi::AbiEncode for IPoolAddressesProviderRegistryCalls {
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
                Self::RegisterAddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnregisterAddressesProvider(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPoolAddressesProviderRegistryCalls {
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
                Self::RegisterAddressesProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnregisterAddressesProvider(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetAddressesProviderAddressByIdCall>
    for IPoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProviderAddressByIdCall) -> Self {
            Self::GetAddressesProviderAddressById(value)
        }
    }
    impl ::core::convert::From<GetAddressesProviderIdByAddressCall>
    for IPoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProviderIdByAddressCall) -> Self {
            Self::GetAddressesProviderIdByAddress(value)
        }
    }
    impl ::core::convert::From<GetAddressesProvidersListCall>
    for IPoolAddressesProviderRegistryCalls {
        fn from(value: GetAddressesProvidersListCall) -> Self {
            Self::GetAddressesProvidersList(value)
        }
    }
    impl ::core::convert::From<RegisterAddressesProviderCall>
    for IPoolAddressesProviderRegistryCalls {
        fn from(value: RegisterAddressesProviderCall) -> Self {
            Self::RegisterAddressesProvider(value)
        }
    }
    impl ::core::convert::From<UnregisterAddressesProviderCall>
    for IPoolAddressesProviderRegistryCalls {
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
}
