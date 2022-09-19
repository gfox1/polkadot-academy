#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://docs.substrate.io/v3/runtime/frame)
///			types needed to add this pallet to a
///			runtime.
///
pub mod pallet {
    use codec::{Codec, FullEncode};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        fn on_value_update(new_value: Self::ValueType);
        type ValueType: From<u32>
            + Codec
            + Default
            + TypeInfo
            + FullEncode
            + sp_std::ops::AddAssign
            + PartialOrd
            + Copy;
        const MAX_VALUE: u32;
    }
    #[allow(type_alias_bounds)]
    pub type Values<T: Config> = StorageMap<
        _GeneratedPrefixForStorageValues<T>,
        Twox64Concat,
        T::AccountId,
        T::ValueType,
        ValueQuery,
    >;
    #[allow(type_alias_bounds)]
    pub type Counter<T: Config> =
        StorageValue<_GeneratedPrefixForStorageCounter<T>, u32, ValueQuery>;
    ///
    ///			The [pallet](https://docs.substrate.io/v3/runtime/frame#pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(PhantomData<T>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    impl<T: Config> Pallet<T> {
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            if !<Values<T>>::contains_key(&who) {
                if value > T::MAX_VALUE.into() {
                    return Err("failed".into());
                }
                Counter::<T>::mutate(|x| *x += 1);
                let value: T::ValueType = value.into();
                <Values<T>>::insert(who, value);
                T::on_value_update(value);
            } else {
                return Err("already submitted".into());
            }
            Ok(())
        }
        pub fn other_signed_extrinsic(origin: OriginFor<T>) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            Ok(())
        }
    }
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            if n % 10u32.into() == sp_runtime::traits::Zero::zero() {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(
                                &["count of users is "],
                                &[::core::fmt::ArgumentV1::new_display(&Counter::<T>::get())],
                            ),
                            lvl,
                            &(
                                "simple_pallet::pallet",
                                "simple_pallet::pallet",
                                "src/lib.rs",
                                75u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
            }
            0
        }
        fn on_finalize(_n: T::BlockNumber) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata(
        ) -> frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata>
        {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            None
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn module_name() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: module_name :: < Self > () . expect ("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn accumulate(
            acc: &mut frame_support::sp_std::vec::Vec<frame_support::traits::PalletInfoData>,
        ) {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            acc.push(item);
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = < Values < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            {
                let mut storage_info = < Counter < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        #[codec(index = 0u8)]
        set_value {
            #[allow(missing_docs)]
            value: u32,
        },
        #[codec(index = 1u8)]
        other_signed_extrinsic {},
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::set_value { ref value } => fmt
                        .debug_struct("Call::set_value")
                        .field("value", &value)
                        .finish(),
                    Self::other_signed_extrinsic {} => {
                        fmt.debug_struct("Call::other_signed_extrinsic").finish()
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::set_value { ref value } => Self::set_value {
                        value: core::clone::Clone::clone(value),
                    },
                    Self::other_signed_extrinsic {} => Self::other_signed_extrinsic {},
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::set_value { value }, Self::set_value { value: _0 }) => {
                        true && value == _0
                    }
                    (Self::other_signed_extrinsic {}, Self::other_signed_extrinsic {}) => true,
                    (Self::__Ignore { .. }, Self::set_value { .. }) => false,
                    (Self::__Ignore { .. }, Self::other_signed_extrinsic { .. }) => false,
                    (Self::set_value { .. }, Self::__Ignore { .. }) => false,
                    (Self::set_value { .. }, Self::other_signed_extrinsic { .. }) => false,
                    (Self::other_signed_extrinsic { .. }, Self::__Ignore { .. }) => false,
                    (Self::other_signed_extrinsic { .. }, Self::set_value { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::set_value { ref value } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(value, __codec_dest_edqy);
                    }
                    Call::other_signed_extrinsic {} => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::set_value {
                            value: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::set_value::value`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::other_signed_extrinsic {})
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn't exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "simple_pallet::pallet"))
                    .type_params(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([::scale_info::TypeParameter::new(
                            "T",
                            ::core::option::Option::None,
                        )]),
                    ))
                    .docs_always(&[
                        "Contains one variant per dispatchable that can be called by an extrinsic.",
                    ])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("set_value", |v| {
                                v.index(0u8 as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<u32>()
                                            .name("value")
                                            .type_name("u32")
                                            .docs_always(&[])
                                    }))
                                    .docs_always(&[])
                            })
                            .variant("other_signed_extrinsic", |v| {
                                v.index(1u8 as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named())
                                    .docs_always(&[])
                            }),
                    )
            }
        };
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `set_value`.
        pub fn new_call_variant_set_value(value: u32) -> Self {
            Self::set_value { value }
        }
        ///Create a call with the variant `other_signed_extrinsic`.
        pub fn new_call_variant_other_signed_extrinsic() -> Self {
            Self::other_signed_extrinsic {}
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::set_value { ref value } => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                            &__pallet_base_weight,
                            (value,),
                        );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                            &__pallet_base_weight,
                            (value,),
                        );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                            &__pallet_base_weight,
                            (value,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::other_signed_extrinsic {} => {
                    let __pallet_base_weight = 0;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<()>>::weigh_data(
                        &__pallet_base_weight,
                        (),
                    );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                            &__pallet_base_weight,
                            (),
                        );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<()>>::pays_fee(
                        &__pallet_base_weight,
                        (),
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(&["__Ignore cannot be used"], &[]),
                        )],
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::set_value { .. } => "set_value",
                Self::other_signed_extrinsic { .. } => "other_signed_extrinsic",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["set_value", "other_signed_extrinsic"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::set_value { value } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "set_value",
                                    "simple_pallet::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("src/lib.rs"),
                                    Some(4u32),
                                    Some("simple_pallet::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::set_value(origin, value)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::other_signed_extrinsic {} => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "other_signed_extrinsic",
                                    "simple_pallet::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("src/lib.rs"),
                                    Some(4u32),
                                    Some("simple_pallet::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::other_signed_extrinsic(origin)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ));
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support :: metadata :: PalletStorageMetadata { prefix : < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed") , entries : { # [allow (unused_mut)] let mut entries = :: alloc :: vec :: Vec :: new () ; { < Values < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } { < Counter < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } entries } , }
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageValues<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageValues<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Values";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageCounter<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageCounter<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Counter";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "simple_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(4u32),
                            Some("simple_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "simple_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(4u32),
                            Some("simple_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "simple_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(4u32),
                            Some("simple_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "simple_pallet::pallet",
                            "src/lib.rs",
                            4u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
