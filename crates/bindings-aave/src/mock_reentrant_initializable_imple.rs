pub use mock_reentrant_initializable_imple::*;
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
pub mod mock_reentrant_initializable_imple {
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKREENTRANTINITIALIZABLEIMPLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80U4\x80\x15a\0\x14W`\0\x80\xFD[Pa\x01\xCE\x80a\0$`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c?\xA4\xF2E\x14a\0FW\x80c\xDD\xE4<\xBA\x14a\0aW\x80c\xFEK\x84\xDF\x14a\0iW[`\0\x80\xFD[a\0O`4T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0O`\x02\x81V[a\0|a\0w6`\x04a\x01YV[a\0~V[\0[`\x01T`\x02\x90`\xFF\x16\x80a\0\x91WP0;\x15[\x80a\0\x9DWP`\0T\x81\x11[a\x01\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x01#W`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[`4\x83\x90U`\x02\x83\x10\x15a\x01CWa\x01C`4T`\x01a\0w\x91\x90a\x01rV[\x80\x15a\x01TW`\x01\x80T`\xFF\x19\x16\x90U[PPPV[`\0` \x82\x84\x03\x12\x15a\x01kW`\0\x80\xFD[P5\x91\x90PV[`\0\x82\x19\x82\x11\x15a\x01\x93WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V\xFE\xA2dipfsX\"\x12 \xAD\x0F0\xCF\xA8\xA8\xEAn\xBF\x11\xC9\xD7\xB8\x1E\xE9\x84B\xE21\x17\xC1D[g\x87\xA2\x16\xD7of0.dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static MOCKREENTRANTINITIALIZABLEIMPLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c?\xA4\xF2E\x14a\0FW\x80c\xDD\xE4<\xBA\x14a\0aW\x80c\xFEK\x84\xDF\x14a\0iW[`\0\x80\xFD[a\0O`4T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0O`\x02\x81V[a\0|a\0w6`\x04a\x01YV[a\0~V[\0[`\x01T`\x02\x90`\xFF\x16\x80a\0\x91WP0;\x15[\x80a\0\x9DWP`\0T\x81\x11[a\x01\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FContract instance has already be`D\x82\x01Rm\x19[\x88\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`\xFF\x16\x15\x80\x15a\x01#W`\x01\x80T`\xFF\x19\x16\x81\x17\x90U`\0\x82\x90U[`4\x83\x90U`\x02\x83\x10\x15a\x01CWa\x01C`4T`\x01a\0w\x91\x90a\x01rV[\x80\x15a\x01TW`\x01\x80T`\xFF\x19\x16\x90U[PPPV[`\0` \x82\x84\x03\x12\x15a\x01kW`\0\x80\xFD[P5\x91\x90PV[`\0\x82\x19\x82\x11\x15a\x01\x93WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V\xFE\xA2dipfsX\"\x12 \xAD\x0F0\xCF\xA8\xA8\xEAn\xBF\x11\xC9\xD7\xB8\x1E\xE9\x84B\xE21\x17\xC1D[g\x87\xA2\x16\xD7of0.dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKREENTRANTINITIALIZABLEIMPLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockReentrantInitializableImple<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockReentrantInitializableImple<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockReentrantInitializableImple<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockReentrantInitializableImple<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockReentrantInitializableImple<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockReentrantInitializableImple))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockReentrantInitializableImple<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKREENTRANTINITIALIZABLEIMPLE_ABI.clone(),
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
                MOCKREENTRANTINITIALIZABLEIMPLE_ABI.clone(),
                MOCKREENTRANTINITIALIZABLEIMPLE_BYTECODE.clone().into(),
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
        ///Calls the contract's `initialize` (0xfe4b84df) function
        pub fn initialize(
            &self,
            val: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 75, 132, 223], val)
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockReentrantInitializableImple<M> {
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256)` and selector `0xfe4b84df`
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
    #[ethcall(name = "initialize", abi = "initialize(uint256)")]
    pub struct InitializeCall {
        pub val: ::ethers::core::types::U256,
    }
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockReentrantInitializableImpleCalls {
        Revision(RevisionCall),
        Initialize(InitializeCall),
        Value(ValueCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockReentrantInitializableImpleCalls {
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
                = <ValueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Value(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockReentrantInitializableImpleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Revision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Value(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockReentrantInitializableImpleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Revision(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Value(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RevisionCall> for MockReentrantInitializableImpleCalls {
        fn from(value: RevisionCall) -> Self {
            Self::Revision(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockReentrantInitializableImpleCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ValueCall> for MockReentrantInitializableImpleCalls {
        fn from(value: ValueCall) -> Self {
            Self::Value(value)
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
}
