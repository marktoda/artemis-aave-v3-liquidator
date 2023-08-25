pub use reserve_configuration::*;
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
pub mod reserve_configuration {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEBT_CEILING_DECIMALS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEBT_CEILING_DECIMALS",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("MAX_RESERVES_COUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_RESERVES_COUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
    pub static RESERVECONFIGURATION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xABa\08`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14`+WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`=W`\x005`\xE0\x1C\x80c(\r]\xE9\x14`BW\x80c1\xB5a\xBA\x14`\\W[`\0\x80\xFD[`I`\x02\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`c`\x80\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01`SV\xFE\xA2dipfsX\"\x12 \xE5\xCC\xB9\x8Fl\xEB\xA4}\xCE+\xB9{\xD2\x9E\x02\xCDW\x7F:\x82\x81\xCD\xDAZ\x83bB\xC7M\xC4|\xD8dsolcC\0\x08\n\x003";
    /// The bytecode of the contract.
    pub static RESERVECONFIGURATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10`=W`\x005`\xE0\x1C\x80c(\r]\xE9\x14`BW\x80c1\xB5a\xBA\x14`\\W[`\0\x80\xFD[`I`\x02\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`c`\x80\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01`SV\xFE\xA2dipfsX\"\x12 \xE5\xCC\xB9\x8Fl\xEB\xA4}\xCE+\xB9{\xD2\x9E\x02\xCDW\x7F:\x82\x81\xCD\xDAZ\x83bB\xC7M\xC4|\xD8dsolcC\0\x08\n\x003";
    /// The deployed bytecode of the contract.
    pub static RESERVECONFIGURATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ReserveConfiguration<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ReserveConfiguration<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ReserveConfiguration<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ReserveConfiguration<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ReserveConfiguration<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ReserveConfiguration))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ReserveConfiguration<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESERVECONFIGURATION_ABI.clone(),
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
                RESERVECONFIGURATION_ABI.clone(),
                RESERVECONFIGURATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEBT_CEILING_DECIMALS` (0x280d5de9) function
        pub fn debt_ceiling_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([40, 13, 93, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_RESERVES_COUNT` (0x31b561ba) function
        pub fn max_reserves_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([49, 181, 97, 186], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ReserveConfiguration<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `DEBT_CEILING_DECIMALS` function with signature `DEBT_CEILING_DECIMALS()` and selector `0x280d5de9`
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
    #[ethcall(name = "DEBT_CEILING_DECIMALS", abi = "DEBT_CEILING_DECIMALS()")]
    pub struct DebtCeilingDecimalsCall;
    ///Container type for all input parameters for the `MAX_RESERVES_COUNT` function with signature `MAX_RESERVES_COUNT()` and selector `0x31b561ba`
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
    #[ethcall(name = "MAX_RESERVES_COUNT", abi = "MAX_RESERVES_COUNT()")]
    pub struct MaxReservesCountCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ReserveConfigurationCalls {
        DebtCeilingDecimals(DebtCeilingDecimalsCall),
        MaxReservesCount(MaxReservesCountCall),
    }
    impl ::ethers::core::abi::AbiDecode for ReserveConfigurationCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DebtCeilingDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DebtCeilingDecimals(decoded));
            }
            if let Ok(decoded)
                = <MaxReservesCountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxReservesCount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ReserveConfigurationCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DebtCeilingDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxReservesCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ReserveConfigurationCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DebtCeilingDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxReservesCount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DebtCeilingDecimalsCall> for ReserveConfigurationCalls {
        fn from(value: DebtCeilingDecimalsCall) -> Self {
            Self::DebtCeilingDecimals(value)
        }
    }
    impl ::core::convert::From<MaxReservesCountCall> for ReserveConfigurationCalls {
        fn from(value: MaxReservesCountCall) -> Self {
            Self::MaxReservesCount(value)
        }
    }
    ///Container type for all return fields from the `DEBT_CEILING_DECIMALS` function with signature `DEBT_CEILING_DECIMALS()` and selector `0x280d5de9`
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
    pub struct DebtCeilingDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_RESERVES_COUNT` function with signature `MAX_RESERVES_COUNT()` and selector `0x31b561ba`
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
    pub struct MaxReservesCountReturn(pub u16);
}
