#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod context {
    use convert_case::{Case, Casing};
    use serde::Serialize;
    use typesafe_builders::prelude::*;
    pub struct Context {
        pub pallet: Pallet,
        pub cargo: Cargo,
        pub test: Test,
        pub mock: Mock,
        pub benchmarking: Benchmarking,
        pub weights: Weights,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Context {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Context",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pallet",
                    &self.pallet,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "cargo",
                    &self.cargo,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "test",
                    &self.test,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mock",
                    &self.mock,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "benchmarking",
                    &self.benchmarking,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "weights",
                    &self.weights,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(dead_code)]
    #[allow(non_upper_case_globals)]
    pub struct GenericContextBuilder<
        const PALLET_SET: bool,
        const CARGO_SET: bool,
        const TEST_SET: bool,
        const MOCK_SET: bool,
        const BENCHMARKING_SET: bool,
        const WEIGHTS_SET: bool,
    > {
        pallet: Option<Pallet>,
        cargo: Option<Cargo>,
        test: Option<Test>,
        mock: Option<Mock>,
        benchmarking: Option<Benchmarking>,
        weights: Option<Weights>,
    }
    pub type ContextBuilder = GenericContextBuilder<
        false,
        false,
        false,
        false,
        false,
        false,
    >;
    #[allow(non_upper_case_globals)]
    impl<
        const cargo_set: bool,
        const test_set: bool,
        const mock_set: bool,
        const benchmarking_set: bool,
        const weights_set: bool,
    > GenericContextBuilder<
        false,
        cargo_set,
        test_set,
        mock_set,
        benchmarking_set,
        weights_set,
    > {
        #[allow(dead_code)]
        pub fn pallet(
            self,
            pallet: Pallet,
        ) -> GenericContextBuilder<
            true,
            cargo_set,
            test_set,
            mock_set,
            benchmarking_set,
            weights_set,
        > {
            GenericContextBuilder {
                pallet: Some(pallet),
                cargo: self.cargo,
                test: self.test,
                mock: self.mock,
                benchmarking: self.benchmarking,
                weights: self.weights,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    impl<
        const pallet_set: bool,
        const test_set: bool,
        const mock_set: bool,
        const benchmarking_set: bool,
        const weights_set: bool,
    > GenericContextBuilder<
        pallet_set,
        false,
        test_set,
        mock_set,
        benchmarking_set,
        weights_set,
    > {
        #[allow(dead_code)]
        pub fn cargo(
            self,
            cargo: Cargo,
        ) -> GenericContextBuilder<
            pallet_set,
            true,
            test_set,
            mock_set,
            benchmarking_set,
            weights_set,
        > {
            GenericContextBuilder {
                cargo: Some(cargo),
                pallet: self.pallet,
                test: self.test,
                mock: self.mock,
                benchmarking: self.benchmarking,
                weights: self.weights,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    impl<
        const pallet_set: bool,
        const cargo_set: bool,
        const mock_set: bool,
        const benchmarking_set: bool,
        const weights_set: bool,
    > GenericContextBuilder<
        pallet_set,
        cargo_set,
        false,
        mock_set,
        benchmarking_set,
        weights_set,
    > {
        #[allow(dead_code)]
        pub fn test(
            self,
            test: Test,
        ) -> GenericContextBuilder<
            pallet_set,
            cargo_set,
            true,
            mock_set,
            benchmarking_set,
            weights_set,
        > {
            GenericContextBuilder {
                test: Some(test),
                pallet: self.pallet,
                cargo: self.cargo,
                mock: self.mock,
                benchmarking: self.benchmarking,
                weights: self.weights,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    impl<
        const pallet_set: bool,
        const cargo_set: bool,
        const test_set: bool,
        const benchmarking_set: bool,
        const weights_set: bool,
    > GenericContextBuilder<
        pallet_set,
        cargo_set,
        test_set,
        false,
        benchmarking_set,
        weights_set,
    > {
        #[allow(dead_code)]
        pub fn mock(
            self,
            mock: Mock,
        ) -> GenericContextBuilder<
            pallet_set,
            cargo_set,
            test_set,
            true,
            benchmarking_set,
            weights_set,
        > {
            GenericContextBuilder {
                mock: Some(mock),
                pallet: self.pallet,
                cargo: self.cargo,
                test: self.test,
                benchmarking: self.benchmarking,
                weights: self.weights,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    impl<
        const pallet_set: bool,
        const cargo_set: bool,
        const test_set: bool,
        const mock_set: bool,
        const weights_set: bool,
    > GenericContextBuilder<
        pallet_set,
        cargo_set,
        test_set,
        mock_set,
        false,
        weights_set,
    > {
        #[allow(dead_code)]
        pub fn benchmarking(
            self,
            benchmarking: Benchmarking,
        ) -> GenericContextBuilder<
            pallet_set,
            cargo_set,
            test_set,
            mock_set,
            true,
            weights_set,
        > {
            GenericContextBuilder {
                benchmarking: Some(benchmarking),
                pallet: self.pallet,
                cargo: self.cargo,
                test: self.test,
                mock: self.mock,
                weights: self.weights,
            }
        }
    }
    #[allow(non_upper_case_globals)]
    impl<
        const pallet_set: bool,
        const cargo_set: bool,
        const test_set: bool,
        const mock_set: bool,
        const benchmarking_set: bool,
    > GenericContextBuilder<
        pallet_set,
        cargo_set,
        test_set,
        mock_set,
        benchmarking_set,
        false,
    > {
        #[allow(dead_code)]
        pub fn weights(
            self,
            weights: Weights,
        ) -> GenericContextBuilder<
            pallet_set,
            cargo_set,
            test_set,
            mock_set,
            benchmarking_set,
            true,
        > {
            GenericContextBuilder {
                weights: Some(weights),
                pallet: self.pallet,
                cargo: self.cargo,
                test: self.test,
                mock: self.mock,
                benchmarking: self.benchmarking,
            }
        }
    }
    impl GenericContextBuilder<true, true, true, true, true, true> {
        /// Infallible build the instance.
        #[allow(dead_code)]
        pub fn build(self) -> Context {
            Context {
                pallet: self.pallet.unwrap(),
                cargo: self.cargo.unwrap(),
                test: self.test.unwrap(),
                mock: self.mock.unwrap(),
                benchmarking: self.benchmarking.unwrap(),
                weights: self.weights.unwrap(),
            }
        }
    }
    impl Context {
        #[allow(dead_code)]
        fn builder() -> GenericContextBuilder<false, false, false, false, false, false> {
            GenericContextBuilder {
                pallet: None,
                cargo: None,
                test: None,
                mock: None,
                benchmarking: None,
                weights: None,
            }
        }
    }
    pub struct Pallet {
        pub license_header: String,
        pub name: String,
        pub storage: Storage,
        pub event: Event,
        pub call: Call,
        pub error: Error,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Pallet {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Pallet",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "license_header",
                    &self.license_header,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "storage",
                    &self.storage,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "event",
                    &self.event,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "call",
                    &self.call,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error",
                    &self.error,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl Pallet {
        pub fn folder_name(&self) -> String {
            let canon = self.name.to_case(Case::Snake);
            canon.strip_prefix("pallet_").unwrap_or(&canon).to_string()
        }
    }
    pub struct Storage {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Storage {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Storage",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Event {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Event {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Event",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Call {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Call {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Call",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Error {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Error",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Cargo {
        version: String,
        author: String,
        description: String,
        license: String,
        repository: String,
        edition: String,
        homepage: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Cargo {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Cargo",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "author",
                    &self.author,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "description",
                    &self.description,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "license",
                    &self.license,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "repository",
                    &self.repository,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "edition",
                    &self.edition,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "homepage",
                    &self.homepage,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Test {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Test {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Test",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Mock {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Mock {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Mock",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Benchmarking {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Benchmarking {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Benchmarking",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct Weights {
        dummy: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Weights {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Weights",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dummy",
                    &self.dummy,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub mod presets {
        use super::*;
        pub fn preset() {
            Context::builder().test(Test { dummy: true })
        }
        pub fn substrate() -> Context {
            Context {
                pallet: Pallet {
                    name: "template".to_string(),
                    storage: Storage { dummy: true },
                    event: Event { dummy: true },
                    call: Call { dummy: true },
                    error: Error { dummy: true },
                    license_header: "// This file is part of Substrate.\n\n// Copyright (C) Parity Technologies (UK) Ltd.\n// SPDX-License-Identifier: Apache-2.0\n\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n// \thttp://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n"
                        .to_string(),
                },
                cargo: Cargo {
                    version: "0.1.0".to_string(),
                    author: "Parity Technologies <admin@parity.io>".to_string(),
                    edition: "2021".to_string(),
                    license: "Apache-2.0".to_string(),
                    description: "FRAME pallet <TODO FAIL-CI>".to_string(),
                    repository: "https://github.com/paritytech/substrate".to_string(),
                    homepage: "https://substrate.io".to_string(),
                },
                mock: Mock { dummy: true },
                test: Test { dummy: true },
                benchmarking: Benchmarking { dummy: true },
                weights: Weights { dummy: true },
            }
        }
    }
}
use context::*;
use std::path::{Path, PathBuf};
use tera::Tera;
use inquire::{Confirm, Text};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            ::core::panicking::panic_fmt(
                format_args!(
                    "internal error: entered unreachable code: {0}",
                    format_args!("Parsing error(s) in static files: {0}", e)
                ),
            );
        }
    };
    let mut context = presets::substrate();
    let name = prompt_name()?;
    context.pallet.name = name;
    let path = prompt_path(&context.pallet.folder_name())?;
    let root_dir = PathBuf::from(context.pallet.folder_name());
    if root_dir.exists() {
        {
            {
                let lvl = ::log::Level::Error;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        format_args!(
                            "Directory \'{0}\' already exists!", root_dir.display()
                        ),
                        lvl,
                        &("framy", "framy", "src/main.rs", 33u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            ::std::process::exit(1);
        };
    }
    let src = root_dir.join("src");
    std::fs::create_dir_all(&src)?;
    let root_files = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([("Cargo.tera", "Cargo.toml")]),
    );
    let src_files = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            ("tests.tera", "tests.rs"),
            ("mock.tera", "mock.rs"),
            ("weights.tera", "weights.rs"),
            ("benchmarking_v2.tera", "benchmarking.rs"),
            ("lib.tera", "lib.rs"),
        ]),
    );
    for (template, file) in root_files {
        render_to_file(&tera, template, &context, &root_dir.join(file))?;
    }
    for (template, file) in src_files {
        render_to_file(&tera, template, &context, &src.join(file))?;
    }
    Ok(())
}
fn render_to_file(
    tera: &Tera,
    template: &str,
    context: &Context,
    file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = tera.render(template, &tera::Context::from_serialize(&context)?)?;
    std::fs::write(file, rendered)?;
    Ok(())
}
fn prompt_name() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let name = Text::new("Name:").with_initial_value("pallet-").prompt()?;
        if name.starts_with("pallet-") {
            break Ok(name);
        } else {
            let ans = Confirm::new("Are you sure?")
                .with_default(false)
                .with_help_message("Pallet names usually start with 'pallet-'.")
                .prompt();
            if match ans {
                Ok(true) => true,
                _ => false,
            } {
                break Ok(name);
            }
        }
    }
}
fn prompt_path(initial: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    loop {
        let path = Text::new("Folder:").with_initial_value(initial).prompt()?;
        let path = PathBuf::from(path);
        if !path.exists() {
            break Ok(path);
        } else {
            let ans = Confirm::new("Are you sure?")
                .with_default(false)
                .with_help_message("The folder already exists.")
                .prompt();
            if match ans {
                Ok(true) => true,
                _ => false,
            } {
                break Ok(path);
            }
        }
    }
}
