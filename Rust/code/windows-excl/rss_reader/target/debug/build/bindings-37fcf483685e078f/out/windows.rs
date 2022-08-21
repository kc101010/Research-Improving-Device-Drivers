#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Foundation {
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncInfo(::windows::IInspectable);
        unsafe impl ::windows::Interface for IAsyncInfo {
            type Vtable = IAsyncInfo_abi;
            const IID: ::windows::Guid =
                ::windows::Guid::from_values(54, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
        }
        impl IAsyncInfo {
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = self;
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe { (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this)).ok() }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe { (::windows::Interface::vtable(this).10)(::windows::Abi::abi(this)).ok() }
            }
        }
        unsafe impl ::windows::RuntimeType for IAsyncInfo {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
        }
        impl ::std::convert::From<IAsyncInfo> for ::windows::IInspectable {
            fn from(value: IAsyncInfo) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IAsyncInfo> for ::windows::IInspectable {
            fn from(value: &IAsyncInfo) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncInfo_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut AsyncStatus,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::HRESULT,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct AsyncOperationProgressHandler<TResult, TProgress>(
            ::windows::IUnknown,
            ::std::marker::PhantomData<TResult>,
            ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > AsyncOperationProgressHandler<TResult, TProgress>
        {
            pub fn new<
                F: FnMut(
                        &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                        &<TProgress as ::windows::RuntimeType>::DefaultType,
                    ) -> ::windows::Result<()>
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = AsyncOperationProgressHandler_box::<TResult, TProgress, F> {
                    vtable: &AsyncOperationProgressHandler_box::<TResult, TProgress, F>::VTABLE,
                    count: ::windows::RefCount::new(1),
                    invoke,
                };
                unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
            }
            pub fn Invoke<'a>(
                &self,
                asyncinfo: impl ::windows::IntoParam<
                    'a,
                    IAsyncOperationWithProgress<TResult, TProgress>,
                >,
                progressinfo: impl ::windows::IntoParam<'a, TProgress>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).3)(
                        ::windows::Abi::abi(this),
                        asyncinfo.into_param().abi(),
                        progressinfo.into_param().abi(),
                    )
                    .ok()
                }
            }
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress>
        {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{55690902-0aab-421a-8778-f8ce5026d758}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b";")
                    .push_other(<TProgress as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::Interface for AsyncOperationProgressHandler<TResult, TProgress>
        {
            type Vtable = AsyncOperationProgressHandler_abi<TResult, TProgress>;
            const IID : :: windows :: Guid = :: windows :: Guid :: from_signature ( < AsyncOperationProgressHandler < TResult , TProgress > as :: windows :: RuntimeType > :: SIGNATURE ) ;
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct AsyncOperationProgressHandler_abi<TResult, TProgress>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                progressinfo: <TProgress as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
            pub ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        #[repr(C)]
        struct AsyncOperationProgressHandler_box<
            TResult,
            TProgress,
            F: FnMut(
                    &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                    &<TProgress as ::windows::RuntimeType>::DefaultType,
                ) -> ::windows::Result<()>
                + 'static,
        >
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static,
        {
            vtable: *const AsyncOperationProgressHandler_abi<TResult, TProgress>,
            invoke: F,
            count: ::windows::RefCount,
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
                F: FnMut(
                        &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                        &<TProgress as ::windows::RuntimeType>::DefaultType,
                    ) -> ::windows::Result<()>
                    + 'static,
            > AsyncOperationProgressHandler_box<TResult, TProgress, F>
        {
            const VTABLE: AsyncOperationProgressHandler_abi<TResult, TProgress> =
                AsyncOperationProgressHandler_abi::<TResult, TProgress>(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    ::std::marker::PhantomData::<TResult>,
                    ::std::marker::PhantomData::<TProgress>,
                );
            unsafe extern "system" fn QueryInterface(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                * interface = if iid == & < AsyncOperationProgressHandler < TResult , TProgress > as :: windows :: Interface > :: IID || iid == & < :: windows :: IUnknown as :: windows :: Interface > :: IID || iid == & < :: windows :: IAgileObject as :: windows :: Interface > :: IID { & mut ( * this ) . vtable as * mut _ as _ } else { :: std :: ptr :: null_mut ( ) } ;
                if (*interface).is_null() {
                    ::windows::HRESULT(0x8000_4002)
                } else {
                    (*this).count.add_ref();
                    ::windows::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                let remaining = (*this).count.release();
                if remaining == 0 {
                    Box::from_raw(this);
                }
                remaining
            }
            unsafe extern "system" fn Invoke(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                progressinfo: <TProgress as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                ( ( * this ) . invoke ) ( & * ( & asyncinfo as * const < IAsyncOperationWithProgress < TResult , TProgress > as :: windows :: Abi > :: Abi as * const < IAsyncOperationWithProgress < TResult , TProgress > as :: windows :: RuntimeType > :: DefaultType ) , & * ( & progressinfo as * const < TProgress as :: windows :: Abi > :: Abi as * const < TProgress as :: windows :: RuntimeType > :: DefaultType ) , ) . into ( )
            }
        }
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: marker :: Copy,
            :: std :: clone :: Clone,
            :: std :: default :: Default,
            :: std :: fmt :: Debug,
        )]
        #[repr(transparent)]
        pub struct AsyncStatus(pub i32);
        impl AsyncStatus {
            pub const Canceled: Self = Self(2i32);
            pub const Completed: Self = Self(1i32);
            pub const Error: Self = Self(3i32);
            pub const Started: Self = Self(0i32);
        }
        impl ::std::convert::From<i32> for AsyncStatus {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::Abi for AsyncStatus {
            type Abi = Self;
        }
        unsafe impl ::windows::RuntimeType for AsyncStatus {
            type DefaultType = Self;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(
            ::windows::IUnknown,
            ::std::marker::PhantomData<TResult>,
            ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            pub fn new<
                F: FnMut(
                        &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com =
                    AsyncOperationWithProgressCompletedHandler_box::<TResult, TProgress, F> {
                        vtable: &AsyncOperationWithProgressCompletedHandler_box::<
                            TResult,
                            TProgress,
                            F,
                        >::VTABLE,
                        count: ::windows::RefCount::new(1),
                        invoke,
                    };
                unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
            }
            pub fn Invoke<'a>(
                &self,
                asyncinfo: impl ::windows::IntoParam<
                    'a,
                    IAsyncOperationWithProgress<TResult, TProgress>,
                >,
                asyncstatus: AsyncStatus,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).3)(
                        ::windows::Abi::abi(this),
                        asyncinfo.into_param().abi(),
                        asyncstatus,
                    )
                    .ok()
                }
            }
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::RuntimeType
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{e85df41d-6aa7-46e3-a8e2-f009d840c627}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b";")
                    .push_other(<TProgress as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::Interface
            for AsyncOperationWithProgressCompletedHandler<TResult, TProgress>
        {
            type Vtable = AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>;
            const IID : :: windows :: Guid = :: windows :: Guid :: from_signature ( < AsyncOperationWithProgressCompletedHandler < TResult , TProgress > as :: windows :: RuntimeType > :: SIGNATURE ) ;
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
            pub ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        #[repr(C)]
        struct AsyncOperationWithProgressCompletedHandler_box<
            TResult,
            TProgress,
            F: FnMut(
                    &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                    AsyncStatus,
                ) -> ::windows::Result<()>
                + 'static,
        >
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static,
        {
            vtable: *const AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>,
            invoke: F,
            count: ::windows::RefCount,
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
                F: FnMut(
                        &::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            > AsyncOperationWithProgressCompletedHandler_box<TResult, TProgress, F>
        {
            const VTABLE: AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress> =
                AsyncOperationWithProgressCompletedHandler_abi::<TResult, TProgress>(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    ::std::marker::PhantomData::<TResult>,
                    ::std::marker::PhantomData::<TProgress>,
                );
            unsafe extern "system" fn QueryInterface(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                *interface = if iid == &<AsyncOperationWithProgressCompletedHandler<
                    TResult,
                    TProgress,
                > as ::windows::Interface>::IID
                    || iid == &<::windows::IUnknown as ::windows::Interface>::IID
                    || iid == &<::windows::IAgileObject as ::windows::Interface>::IID
                {
                    &mut (*this).vtable as *mut _ as _
                } else {
                    ::std::ptr::null_mut()
                };
                if (*interface).is_null() {
                    ::windows::HRESULT(0x8000_4002)
                } else {
                    (*this).count.add_ref();
                    ::windows::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                let remaining = (*this).count.release();
                if remaining == 0 {
                    Box::from_raw(this);
                }
                remaining
            }
            unsafe extern "system" fn Invoke(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                ( ( * this ) . invoke ) ( & * ( & asyncinfo as * const < IAsyncOperationWithProgress < TResult , TProgress > as :: windows :: Abi > :: Abi as * const < IAsyncOperationWithProgress < TResult , TProgress > as :: windows :: RuntimeType > :: DefaultType ) , asyncstatus , ) . into ( )
            }
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncOperationWithProgress<TResult, TProgress>(
            ::windows::IInspectable,
            ::std::marker::PhantomData<TResult>,
            ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::Interface for IAsyncOperationWithProgress<TResult, TProgress>
        {
            type Vtable = IAsyncOperationWithProgress_abi<TResult, TProgress>;
            const IID : :: windows :: Guid = :: windows :: Guid :: from_signature ( < IAsyncOperationWithProgress < TResult , TProgress > as :: windows :: RuntimeType > :: SIGNATURE ) ;
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > IAsyncOperationWithProgress<TResult, TProgress>
        {
            pub fn SetProgress<'a>(
                &self,
                handler: impl ::windows::IntoParam<
                    'a,
                    AsyncOperationProgressHandler<TResult, TProgress>,
                >,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        handler.into_param().abi(),
                    )
                    .ok()
                }
            }
            pub fn Progress(
                &self,
            ) -> ::windows::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
                let this = self;
                unsafe {
                    let mut result__ : < AsyncOperationProgressHandler < TResult , TProgress > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncOperationProgressHandler<TResult, TProgress>>(result__)
                }
            }
            pub fn SetCompleted<'a>(
                &self,
                handler: impl ::windows::IntoParam<
                    'a,
                    AsyncOperationWithProgressCompletedHandler<TResult, TProgress>,
                >,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).8)(
                        ::windows::Abi::abi(this),
                        handler.into_param().abi(),
                    )
                    .ok()
                }
            }
            pub fn Completed(
                &self,
            ) -> ::windows::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>
            {
                let this = self;
                unsafe {
                    let mut result__: <AsyncOperationWithProgressCompletedHandler<
                        TResult,
                        TProgress,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>(
                            result__,
                        )
                }
            }
            pub fn GetResults(&self) -> ::windows::Result<TResult> {
                let this = self;
                unsafe {
                    let mut result__: <TResult as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<TResult>(result__)
                }
            }
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe { (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this)).ok() }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe { (::windows::Interface::vtable(this).10)(::windows::Abi::abi(this)).ok() }
            }
            pub fn get(&self) -> ::windows::Result<TResult> {
                if self.Status()? == AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::Waiter::new();
                    self.SetCompleted(AsyncOperationWithProgressCompletedHandler::new(
                        move |_sender, _args| {
                            unsafe {
                                signaler.signal();
                            }
                            Ok(())
                        },
                    ))?;
                }
                self.GetResults()
            }
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress>
        {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{b5d036d7-e297-498f-ba60-0289e76e23dd}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b";")
                    .push_other(<TProgress as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::future::Future for IAsyncOperationWithProgress<TResult, TProgress>
        {
            type Output = ::windows::Result<TResult>;
            fn poll(
                self: ::std::pin::Pin<&mut Self>,
                context: &mut ::std::task::Context,
            ) -> ::std::task::Poll<Self::Output> {
                if self.Status()? == AsyncStatus::Started {
                    let waker = context.waker().clone();
                    let _ = self.SetCompleted(AsyncOperationWithProgressCompletedHandler::new(
                        move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        },
                    ));
                    ::std::task::Poll::Pending
                } else {
                    ::std::task::Poll::Ready(self.GetResults())
                }
            }
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::convert::From<IAsyncOperationWithProgress<TResult, TProgress>>
            for ::windows::IInspectable
        {
            fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
                value.0
            }
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>>
            for ::windows::IInspectable
        {
            fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
                value.0.clone()
            }
        }
        impl<
                'a,
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::IntoParam<'a, ::windows::IInspectable>
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<
                'a,
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::IntoParam<'a, ::windows::IInspectable>
            for &'a IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::convert::From<IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo
        {
            fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>>
            for IAsyncInfo
        {
            fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<
                'a,
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::IntoParam<'a, IAsyncInfo>
            for IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::Param::Owned(::std::convert::Into::<IAsyncInfo>::into(self))
            }
        }
        impl<
                'a,
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::windows::IntoParam<'a, IAsyncInfo>
            for &'a IAsyncOperationWithProgress<TResult, TProgress>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::Param::Owned(::std::convert::Into::<IAsyncInfo>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::marker::Send for IAsyncOperationWithProgress<TResult, TProgress>
        {
        }
        unsafe impl<
                TResult: ::windows::RuntimeType + 'static,
                TProgress: ::windows::RuntimeType + 'static,
            > ::std::marker::Sync for IAsyncOperationWithProgress<TResult, TProgress>
        {
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncOperationWithProgress_abi<TResult, TProgress>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                handler: ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                handler: ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut <TResult as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
            pub ::std::marker::PhantomData<TProgress>,
        )
        where
            TResult: ::windows::RuntimeType + 'static,
            TProgress: ::windows::RuntimeType + 'static;
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClass(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClass {
            type Vtable = IUriRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2654363223,
                18610,
                16736,
                [149, 111, 199, 56, 81, 32, 187, 252],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClass_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                puri: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                relativeuri: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
            type Vtable = IUriRuntimeClassWithAbsoluteCanonicalUri_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1972213345,
                8732,
                18447,
                [163, 57, 80, 101, 102, 115, 244, 111],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStringable(::windows::IInspectable);
        unsafe impl ::windows::Interface for IStringable {
            type Vtable = IStringable_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2520162132,
                36534,
                18672,
                [171, 206, 193, 178, 17, 230, 39, 195],
            );
        }
        impl IStringable {
            pub fn ToString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStringable {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
        }
        impl ::std::convert::From<IStringable> for ::windows::IInspectable {
            fn from(value: IStringable) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStringable> for ::windows::IInspectable {
            fn from(value: &IStringable) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStringable_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriEscapeStatics(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriEscapeStatics {
            type Vtable = IUriEscapeStatics_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3251909306,
                51236,
                17490,
                [167, 253, 81, 43, 195, 187, 233, 161],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriEscapeStatics_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                tounescape: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                toescape: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClassFactory(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClassFactory {
            type Vtable = IUriRuntimeClassFactory_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1151957359,
                29246,
                20447,
                [162, 24, 3, 62, 117, 176, 192, 132],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassFactory_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                uri: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                baseuri: ::windows::RawPtr,
                relativeuri: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct Uri(::windows::IInspectable);
        impl Uri {
            pub fn AbsoluteUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn DisplayUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Domain(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Extension(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Fragment(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Host(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Password(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Query(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn RawUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).16)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn SchemeName(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).17)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn UserName(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).18)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Port(&self) -> ::windows::Result<i32> {
                let this = self;
                unsafe {
                    let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<i32>(result__)
                }
            }
            pub fn Suspicious(&self) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).20)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn Equals<'a>(
                &self,
                puri: impl ::windows::IntoParam<'a, Uri>,
            ) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).21)(
                        ::windows::Abi::abi(this),
                        puri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn CombineUri<'a>(
                &self,
                relativeuri: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<Uri> {
                let this = self;
                unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).22)(
                        ::windows::Abi::abi(this),
                        relativeuri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                }
            }
            pub fn ToString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = &::windows::Interface::cast::<IStringable>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn AbsoluteCanonicalUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this =
                    &::windows::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)
                        .unwrap();
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn DisplayIri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this =
                    &::windows::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)
                        .unwrap();
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn UnescapeComponent<'a>(
                tounescape: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<::windows::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        tounescape.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                })
            }
            pub fn EscapeComponent<'a>(
                toescape: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<::windows::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        toescape.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                })
            }
            pub fn CreateUri<'a>(
                uri: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<Uri> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        uri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                })
            }
            pub fn CreateWithRelativeUri<'a>(
                baseuri: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                relativeuri: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
            ) -> ::windows::Result<Uri> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        baseuri.into_param().abi(),
                        relativeuri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                })
            }
            fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> ::windows::Result<R>>(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<Uri, IUriEscapeStatics> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
            fn IUriRuntimeClassFactory<
                R,
                F: FnOnce(&IUriRuntimeClassFactory) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<Uri, IUriRuntimeClassFactory> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        unsafe impl ::windows::RuntimeType for Uri {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})",
            );
        }
        unsafe impl ::windows::Interface for Uri {
            type Vtable = IUriRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2654363223,
                18610,
                16736,
                [149, 111, 199, 56, 81, 32, 187, 252],
            );
        }
        impl ::windows::RuntimeName for Uri {
            const NAME: &'static str = "Windows.Foundation.Uri";
        }
        impl ::std::convert::From<Uri> for ::windows::IInspectable {
            fn from(value: Uri) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&Uri> for ::windows::IInspectable {
            fn from(value: &Uri) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<Uri> for IStringable {
            fn from(value: Uri) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&Uri> for IStringable {
            fn from(value: &Uri) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStringable> for Uri {
            fn into_param(self) -> ::windows::Param<'a, IStringable> {
                ::windows::Param::Owned(::std::convert::Into::<IStringable>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStringable> for &'a Uri {
            fn into_param(self) -> ::windows::Param<'a, IStringable> {
                ::windows::Param::Owned(::std::convert::Into::<IStringable>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        unsafe impl ::std::marker::Send for Uri {}
        unsafe impl ::std::marker::Sync for Uri {}
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Collections {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterator<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterator<T> {
                type Vtable = IIterator_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterator<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterator<T> {
                pub fn Current(&self) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn HasCurrent(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn MoveNext(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterator<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterator<T>>
                for ::windows::IInspectable
            {
                fn from(value: IIterator<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterator<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IIterator<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for IIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let result = self.Current().ok();
                    if result.is_some() {
                        self.MoveNext().ok()?;
                    }
                    result
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterator_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterable<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterable<T> {
                type Vtable = IIterable_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterable<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterable<T> {
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = self;
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterable<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterable<T>>
                for ::windows::IInspectable
            {
                fn from(value: IIterable<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterable<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IIterable<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterable_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IVectorView<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IVectorView<T> {
                type Vtable = IVectorView_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IVectorView<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IVectorView<T> {
                pub fn GetAt(&self, index: u32) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            index,
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn Size(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn IndexOf<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                    index: &mut u32,
                ) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                            index,
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    startindex: u32,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            startindex,
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = &::windows::Interface::cast::<IIterable<T>>(self).unwrap();
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IVectorView<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>>
                for ::windows::IInspectable
            {
                fn from(value: IVectorView<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IVectorView<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>> for IIterable<T> {
                fn from(value: IVectorView<T>) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>> for IIterable<T> {
                fn from(value: &IVectorView<T>) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for &'a IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            pub struct VectorViewIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVectorView<T>,
                current: u32,
                size: u32,
            }
            impl<T: ::windows::RuntimeType> VectorViewIterator<T> {
                pub fn new(vector: IVectorView<T>) -> Self {
                    let size = vector.Size().unwrap();
                    Self {
                        vector,
                        current: 0,
                        size,
                    }
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }
                    let result = self.vector.GetAt(self.current);
                    self.current += 1;
                    result.ok()
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(::std::clone::Clone::clone(self))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVectorView_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                    index: *mut u32,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IVector<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IVector<T> {
                type Vtable = IVector_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IVector<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IVector<T> {
                pub fn GetAt(&self, index: u32) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            index,
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn Size(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn GetView(&self) -> ::windows::Result<IVectorView<T>> {
                    let this = self;
                    unsafe {
                        let mut result__: <IVectorView<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IVectorView<T>>(result__)
                    }
                }
                pub fn IndexOf<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                    index: &mut u32,
                ) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                            index,
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetAt<'a>(
                    &self,
                    index: u32,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            index,
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn InsertAt<'a>(
                    &self,
                    index: u32,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            index,
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RemoveAt(&self, index: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this), index)
                            .ok()
                    }
                }
                pub fn Append<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RemoveAtEnd(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Clear(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn GetMany(
                    &self,
                    startindex: u32,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            startindex,
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn ReplaceAll(
                    &self,
                    items: &[<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).17)(
                            ::windows::Abi::abi(this),
                            items.len() as u32,
                            ::std::mem::transmute(items.as_ptr()),
                        )
                        .ok()
                    }
                }
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = &::windows::Interface::cast::<IIterable<T>>(self).unwrap();
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IVector<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{913337e9-11a1-4345-a3a2-4e7f956e222d}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVector<T>>
                for ::windows::IInspectable
            {
                fn from(value: IVector<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVector<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IVector<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVector<T>> for IIterable<T> {
                fn from(value: IVector<T>) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVector<T>> for IIterable<T> {
                fn from(value: &IVector<T>) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for &'a IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            pub struct VectorIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVector<T>,
                current: u32,
                size: u32,
            }
            impl<T: ::windows::RuntimeType> VectorIterator<T> {
                pub fn new(vector: IVector<T>) -> Self {
                    let size = vector.Size().unwrap();
                    Self {
                        vector,
                        current: 0,
                        size,
                    }
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }
                    let result = self.vector.GetAt(self.current);
                    self.current += 1;
                    result.ok()
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(::std::clone::Clone::clone(self))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVector_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                    index: *mut u32,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    items_array_size: u32,
                    items: *const <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Web {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Syndication {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct ISyndicationNode(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationNode {
                type Vtable = ISyndicationNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1966927736,
                    20984,
                    17856,
                    [169, 245, 241, 113, 157, 236, 63, 178],
                );
            }
            impl ISyndicationNode {
                pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeName<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeNamespace(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeNamespace<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeValue(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeValue<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Language(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetLanguage<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn BaseUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetBaseUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn ElementExtensions(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<ISyndicationNode>,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            ISyndicationNode,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .17 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < ISyndicationNode > > ( result__ )
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for ISyndicationNode {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{753cef78-51f8-45c0-a9f5-f1719dec3fb2}");
            }
            impl ::std::convert::From<ISyndicationNode> for ::windows::IInspectable {
                fn from(value: ISyndicationNode) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&ISyndicationNode> for ::windows::IInspectable {
                fn from(value: &ISyndicationNode) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for ISyndicationNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a ISyndicationNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationNode_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct ISyndicationText(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationText {
                type Vtable = ISyndicationText_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3117178496,
                    12602,
                    16529,
                    [162, 166, 36, 62, 14, 233, 35, 249],
                );
            }
            impl ISyndicationText {
                pub fn Text(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetText<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Type(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetType<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeName<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeNamespace(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeNamespace<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeValue(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeValue<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Language(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetLanguage<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn BaseUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetBaseUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).15)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn ElementExtensions(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<ISyndicationNode>,
                > {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            ISyndicationNode,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .17 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < ISyndicationNode > > ( result__ )
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for ISyndicationText {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{b9cc5e80-313a-4091-a2a6-243e0ee923f9}");
            }
            impl ::std::convert::From<ISyndicationText> for ::windows::IInspectable {
                fn from(value: ISyndicationText) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&ISyndicationText> for ::windows::IInspectable {
                fn from(value: &ISyndicationText) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for ISyndicationText {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a ISyndicationText {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<ISyndicationText> for ISyndicationNode {
                fn from(value: ISyndicationText) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&ISyndicationText> for ISyndicationNode {
                fn from(value: &ISyndicationText) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for ISyndicationText {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for &'a ISyndicationText {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationText_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(C)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct RetrievalProgress {
                pub BytesRetrieved: u32,
                pub TotalBytesToRetrieve: u32,
            }
            impl RetrievalProgress {}
            impl ::std::default::Default for RetrievalProgress {
                fn default() -> Self {
                    Self {
                        BytesRetrieved: 0,
                        TotalBytesToRetrieve: 0,
                    }
                }
            }
            impl ::std::fmt::Debug for RetrievalProgress {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("RetrievalProgress")
                        .field("BytesRetrieved", &format_args!("{:?}", self.BytesRetrieved))
                        .field(
                            "TotalBytesToRetrieve",
                            &format_args!("{:?}", self.TotalBytesToRetrieve),
                        )
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for RetrievalProgress {
                fn eq(&self, other: &Self) -> bool {
                    self.BytesRetrieved == other.BytesRetrieved
                        && self.TotalBytesToRetrieve == other.TotalBytesToRetrieve
                }
            }
            impl ::std::cmp::Eq for RetrievalProgress {}
            unsafe impl ::windows::Abi for RetrievalProgress {
                type Abi = Self;
            }
            unsafe impl ::windows::RuntimeType for RetrievalProgress {
                type DefaultType = Self;
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"struct(Windows.Web.Syndication.RetrievalProgress;u4;u4)",
                );
            }
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct ISyndicationClient(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationClient {
                type Vtable = ISyndicationClient_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2652416439,
                    29257,
                    19269,
                    [178, 41, 125, 248, 149, 165, 161, 245],
                );
            }
            impl ISyndicationClient {
                pub fn MaxResponseBufferSize(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Timeout(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn SetTimeout(&self, value: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn BypassCacheOnRetrieve(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn SetRequestHeader<'a>(
                    &self,
                    name: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            name.into_param().abi(),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RetrieveFeedAsync<'a>(
                    &self,
                    uri: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperationWithProgress<
                        SyndicationFeed,
                        RetrievalProgress,
                    >,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperationWithProgress<
                            SyndicationFeed,
                            RetrievalProgress,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).17)(
                            ::windows::Abi::abi(this),
                            uri.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperationWithProgress<
                            SyndicationFeed,
                            RetrievalProgress,
                        >>(result__)
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for ISyndicationClient {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{9e18a9b7-7249-4b45-b229-7df895a5a1f5}");
            }
            impl ::std::convert::From<ISyndicationClient> for ::windows::IInspectable {
                fn from(value: ISyndicationClient) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&ISyndicationClient> for ::windows::IInspectable {
                fn from(value: &ISyndicationClient) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for ISyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a ISyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationClient_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    name: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    uri: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ISyndicationClientFactory(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationClientFactory {
                type Vtable = ISyndicationClientFactory_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    784642860,
                    42907,
                    16660,
                    [178, 154, 5, 223, 251, 175, 185, 164],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationClientFactory_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct SyndicationClient(::windows::IInspectable);
            impl SyndicationClient {
                pub fn new() -> ::windows::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationClient,
                        ::windows::IActivationFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
                pub fn MaxResponseBufferSize(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Timeout(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn SetTimeout(&self, value: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn BypassCacheOnRetrieve(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn SetRequestHeader<'a>(
                    &self,
                    name: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            name.into_param().abi(),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RetrieveFeedAsync<'a>(
                    &self,
                    uri: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperationWithProgress<
                        SyndicationFeed,
                        RetrievalProgress,
                    >,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperationWithProgress<
                            SyndicationFeed,
                            RetrievalProgress,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).17)(
                            ::windows::Abi::abi(this),
                            uri.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperationWithProgress<
                            SyndicationFeed,
                            RetrievalProgress,
                        >>(result__)
                    }
                }
                fn ISyndicationClientFactory<
                    R,
                    F: FnOnce(&ISyndicationClientFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationClient,
                        ISyndicationClientFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for SyndicationClient {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Syndication.SyndicationClient;{9e18a9b7-7249-4b45-b229-7df895a5a1f5})" ) ;
            }
            unsafe impl ::windows::Interface for SyndicationClient {
                type Vtable = ISyndicationClient_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2652416439,
                    29257,
                    19269,
                    [178, 41, 125, 248, 149, 165, 161, 245],
                );
            }
            impl ::windows::RuntimeName for SyndicationClient {
                const NAME: &'static str = "Windows.Web.Syndication.SyndicationClient";
            }
            impl ::std::convert::From<SyndicationClient> for ::windows::IInspectable {
                fn from(value: SyndicationClient) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&SyndicationClient> for ::windows::IInspectable {
                fn from(value: &SyndicationClient) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for SyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a SyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<SyndicationClient> for ISyndicationClient {
                fn from(value: SyndicationClient) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl ::std::convert::From<&SyndicationClient> for ISyndicationClient {
                fn from(value: &SyndicationClient) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationClient> for SyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationClient> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationClient>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationClient> for &'a SyndicationClient {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationClient> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationClient>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            unsafe impl ::std::marker::Send for SyndicationClient {}
            unsafe impl ::std::marker::Sync for SyndicationClient {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ISyndicationFeed(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationFeed {
                type Vtable = ISyndicationFeed_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2147368146,
                    23398,
                    19810,
                    [132, 3, 27, 193, 13, 145, 13, 107],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationFeed_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    feed: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ISyndicationFeedFactory(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationFeedFactory {
                type Vtable = ISyndicationFeedFactory_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    591864370,
                    35817,
                    18615,
                    [137, 52, 98, 5, 19, 29, 147, 87],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationFeedFactory_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    title: ::windows::RawPtr,
                    subtitle: ::windows::RawPtr,
                    uri: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct SyndicationFeed(::windows::IInspectable);
            impl SyndicationFeed {
                pub fn new() -> ::windows::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationFeed,
                        ::windows::IActivationFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
                pub fn IconUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetIconUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Id(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetId<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Items(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<SyndicationItem>,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            SyndicationItem,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .15 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < SyndicationItem > > ( result__ )
                    }
                }
                pub fn ImageUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).19)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetImageUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).20)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Rights(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).21)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetRights<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).22)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Subtitle(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).23)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetSubtitle<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).24)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Title(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).25)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetTitle<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).26)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn FirstUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).27)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn LastUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).28)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn NextUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).29)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn PreviousUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).30)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn Load<'a>(
                    &self,
                    feed: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).32)(
                            ::windows::Abi::abi(this),
                            feed.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeName<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeNamespace(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeNamespace<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeValue(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeValue<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Language(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetLanguage<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn BaseUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetBaseUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).15)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn ElementExtensions(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<ISyndicationNode>,
                > {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            ISyndicationNode,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .17 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < ISyndicationNode > > ( result__ )
                    }
                }
                pub fn CreateSyndicationFeed<'a>(
                    title: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                    subtitle: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                    uri: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<SyndicationFeed> {
                    Self::ISyndicationFeedFactory(|this| unsafe {
                        let mut result__: <SyndicationFeed as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            title.into_param().abi(),
                            subtitle.into_param().abi(),
                            uri.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<SyndicationFeed>(result__)
                    })
                }
                fn ISyndicationFeedFactory<
                    R,
                    F: FnOnce(&ISyndicationFeedFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationFeed,
                        ISyndicationFeedFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for SyndicationFeed {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Syndication.SyndicationFeed;{7ffe3cd2-5b66-4d62-8403-1bc10d910d6b})" ) ;
            }
            unsafe impl ::windows::Interface for SyndicationFeed {
                type Vtable = ISyndicationFeed_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2147368146,
                    23398,
                    19810,
                    [132, 3, 27, 193, 13, 145, 13, 107],
                );
            }
            impl ::windows::RuntimeName for SyndicationFeed {
                const NAME: &'static str = "Windows.Web.Syndication.SyndicationFeed";
            }
            impl ::std::convert::From<SyndicationFeed> for ::windows::IInspectable {
                fn from(value: SyndicationFeed) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&SyndicationFeed> for ::windows::IInspectable {
                fn from(value: &SyndicationFeed) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for SyndicationFeed {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a SyndicationFeed {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<SyndicationFeed> for ISyndicationNode {
                fn from(value: SyndicationFeed) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&SyndicationFeed> for ISyndicationNode {
                fn from(value: &SyndicationFeed) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for SyndicationFeed {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for &'a SyndicationFeed {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            unsafe impl ::std::marker::Send for SyndicationFeed {}
            unsafe impl ::std::marker::Sync for SyndicationFeed {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ISyndicationItem(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationItem {
                type Vtable = ISyndicationItem_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1418573955,
                    50052,
                    17857,
                    [138, 232, 163, 120, 196, 236, 72, 108],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationItem_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    item: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ISyndicationItemFactory(::windows::IInspectable);
            unsafe impl ::windows::Interface for ISyndicationItemFactory {
                type Vtable = ISyndicationItemFactory_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    622674767,
                    32184,
                    18554,
                    [133, 228, 16, 209, 145, 230, 110, 187],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISyndicationItemFactory_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct SyndicationItem(::windows::IInspectable);
            impl SyndicationItem {
                pub fn new() -> ::windows::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationItem,
                        ::windows::IActivationFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
                pub fn Id(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetId<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Rights(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).18)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetRights<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).19)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Source(&self) -> ::windows::Result<SyndicationFeed> {
                    let this = self;
                    unsafe {
                        let mut result__: <SyndicationFeed as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).20)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<SyndicationFeed>(result__)
                    }
                }
                pub fn SetSource<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, SyndicationFeed>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).21)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Summary(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).22)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetSummary<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).23)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Title(&self) -> ::windows::Result<ISyndicationText> {
                    let this = self;
                    unsafe {
                        let mut result__: <ISyndicationText as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).24)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<ISyndicationText>(result__)
                    }
                }
                pub fn SetTitle<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ISyndicationText>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).25)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn CommentsUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).26)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetCommentsUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).27)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn EditUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).28)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn EditMediaUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).29)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn ETag(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).30)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn ItemUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).31)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn Load<'a>(
                    &self,
                    item: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).32)(
                            ::windows::Abi::abi(this),
                            item.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeName<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeNamespace(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeNamespace<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn NodeValue(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetNodeValue<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Language(&self) -> ::windows::Result<::windows::HSTRING> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HSTRING>(result__)
                    }
                }
                pub fn SetLanguage<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HSTRING>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn BaseUri(&self) -> ::windows::Result<super::super::Foundation::Uri> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Uri as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::Uri>(result__)
                    }
                }
                pub fn SetBaseUri<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, super::super::Foundation::Uri>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).15)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn ElementExtensions(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<ISyndicationNode>,
                > {
                    let this = &::windows::Interface::cast::<ISyndicationNode>(self).unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            ISyndicationNode,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .17 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < ISyndicationNode > > ( result__ )
                    }
                }
                fn ISyndicationItemFactory<
                    R,
                    F: FnOnce(&ISyndicationItemFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        SyndicationItem,
                        ISyndicationItemFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for SyndicationItem {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Web.Syndication.SyndicationItem;{548db883-c384-45c1-8ae8-a378c4ec486c})" ) ;
            }
            unsafe impl ::windows::Interface for SyndicationItem {
                type Vtable = ISyndicationItem_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1418573955,
                    50052,
                    17857,
                    [138, 232, 163, 120, 196, 236, 72, 108],
                );
            }
            impl ::windows::RuntimeName for SyndicationItem {
                const NAME: &'static str = "Windows.Web.Syndication.SyndicationItem";
            }
            impl ::std::convert::From<SyndicationItem> for ::windows::IInspectable {
                fn from(value: SyndicationItem) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&SyndicationItem> for ::windows::IInspectable {
                fn from(value: &SyndicationItem) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for SyndicationItem {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a SyndicationItem {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<SyndicationItem> for ISyndicationNode {
                fn from(value: SyndicationItem) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&SyndicationItem> for ISyndicationNode {
                fn from(value: &SyndicationItem) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for SyndicationItem {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, ISyndicationNode> for &'a SyndicationItem {
                fn into_param(self) -> ::windows::Param<'a, ISyndicationNode> {
                    ::windows::Param::Owned(::std::convert::Into::<ISyndicationNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            unsafe impl ::std::marker::Send for SyndicationItem {}
            unsafe impl ::std::marker::Sync for SyndicationItem {}
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod SystemServices {
                #[repr(transparent)]
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: cmp :: Eq,
                    :: std :: fmt :: Debug,
                )]
                pub struct PSTR(pub *mut u8);
                impl PSTR {
                    pub const NULL: Self = Self(::std::ptr::null_mut());
                    pub fn is_null(&self) -> bool {
                        self.0.is_null()
                    }
                }
                impl ::std::default::Default for PSTR {
                    fn default() -> Self {
                        Self(::std::ptr::null_mut())
                    }
                }
                impl ::std::cmp::PartialEq for PSTR {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                unsafe impl ::windows::Abi for PSTR {
                    type Abi = Self;
                    fn drop_param(param: &mut ::windows::Param<Self>) {
                        if let ::windows::Param::Boxed(value) = param {
                            if !value.0.is_null() {
                                unsafe {
                                    ::std::boxed::Box::from_raw(value.0);
                                }
                            }
                        }
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
                    fn into_param(self) -> ::windows::Param<'a, PSTR> {
                        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                            self.bytes()
                                .chain(::std::iter::once(0))
                                .collect::<std::vec::Vec<u8>>()
                                .into_boxed_slice(),
                        ) as _))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                    fn into_param(self) -> ::windows::Param<'a, PSTR> {
                        ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                            self.bytes()
                                .chain(::std::iter::once(0))
                                .collect::<std::vec::Vec<u8>>()
                                .into_boxed_slice(),
                        ) as _))
                    }
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod UI {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsAndMessaging {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_RESULT(pub i32);
                impl ::std::convert::From<i32> for MESSAGEBOX_RESULT {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_RESULT {
                    type Abi = Self;
                }
                #[repr(transparent)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct HWND(pub isize);
                impl HWND {}
                impl ::std::default::Default for HWND {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HWND {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HWND {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HWND")
                            .field("Value", &format_args!("{:?}", self.0))
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for HWND {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HWND {}
                unsafe impl ::windows::Abi for HWND {
                    type Abi = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_STYLE(pub u32);
                impl ::std::convert::From<u32> for MESSAGEBOX_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_STYLE {
                    type Abi = Self;
                }
                impl ::std::ops::BitOr for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for MESSAGEBOX_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for MESSAGEBOX_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn MessageBoxA<'a>(
                    hwnd: impl ::windows::IntoParam<'a, HWND>,
                    lptext: impl ::windows::IntoParam<'a, super::super::System::SystemServices::PSTR>,
                    lpcaption: impl ::windows::IntoParam<'a, super::super::System::SystemServices::PSTR>,
                    utype: MESSAGEBOX_STYLE,
                ) -> MESSAGEBOX_RESULT {
                    #[link(name = "USER32")]
                    extern "system" {
                        fn MessageBoxA(
                            hwnd: HWND,
                            lptext: super::super::System::SystemServices::PSTR,
                            lpcaption: super::super::System::SystemServices::PSTR,
                            utype: MESSAGEBOX_STYLE,
                        ) -> MESSAGEBOX_RESULT;
                    }
                    MessageBoxA(
                        hwnd.into_param().abi(),
                        lptext.into_param().abi(),
                        lpcaption.into_param().abi(),
                        ::std::mem::transmute(utype),
                    )
                }
            }
        }
    }
}
