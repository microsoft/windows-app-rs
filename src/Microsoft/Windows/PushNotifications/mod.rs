#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationActivationInfo {
    type Vtable = IPushNotificationActivationInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf50b1226_8eb2_57b9_891f_9ceb3948c5c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfo_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut PushNotificationRegistrationActivators,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditions_array_size: u32,
        conditions: *const ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfoFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationActivationInfoFactory {
    type Vtable = IPushNotificationActivationInfoFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x353b2c3f_bc64_5b92_be1d_a76a2cde2bf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationActivationInfoFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        activators: PushNotificationRegistrationActivators,
        taskclsid: ::windows::core::GUID,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        activators: PushNotificationRegistrationActivators,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationChannel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd4ef2a0_3985_5848_8426_813de636f825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationChannelFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationChannelFactory {
    type Vtable = IPushNotificationChannelFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf065caec_3fd1_50c7_9434_0e29ffe7ba5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannelFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        channel: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df3717f_5d33_56e9_b381_1b350c95722e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut PushNotificationChannelStatus,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResultFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationCreateChannelResultFactory {
    type Vtable = IPushNotificationCreateChannelResultFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6162a948_0dda_53ae_abcd_5b47f336d612);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResultFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        channel: ::windows::core::RawPtr,
        extendederror: ::windows::core::HRESULT,
        status: PushNotificationChannelStatus,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationManagerStatics {
    type Vtable = IPushNotificationManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x214c19b4_31b3_5e12_9efe_bebc0279bf8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        details: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        remoteid: ::windows::core::GUID,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        activators: PushNotificationRegistrationActivators,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa8fe81bc_d2d9_52cf_ab1b_6fde711862ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::core::GUID,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PushNotificationActivationInfo(pub ::windows::core::IInspectable);
impl PushNotificationActivationInfo {
    pub fn TaskClsid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Activators(&self) -> ::windows::core::Result<PushNotificationRegistrationActivators> {
        let this = self;
        unsafe {
            let mut result__: PushNotificationRegistrationActivators = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationRegistrationActivators>(result__)
        }
    }
    pub fn GetConditions(
        &self,
    ) -> ::windows::core::Result<
        ::windows::core::Array<::windows::ApplicationModel::Background::IBackgroundCondition>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<
                ::windows::ApplicationModel::Background::IBackgroundCondition,
            > = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<
                    ::windows::ApplicationModel::Background::IBackgroundCondition,
                >::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn SetConditions(
        &self,
        conditions : & [ < ::windows::ApplicationModel::Background:: IBackgroundCondition as :: windows :: core :: DefaultType > :: DefaultType ],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                conditions.len() as u32,
                ::core::mem::transmute(conditions.as_ptr()),
            )
            .ok()
        }
    }
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(
        activators: PushNotificationRegistrationActivators,
        taskclsid: Param1,
    ) -> ::windows::core::Result<PushNotificationActivationInfo> {
        Self::IPushNotificationActivationInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                activators,
                taskclsid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PushNotificationActivationInfo>(result__)
        })
    }
    pub fn CreateInstance2(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::core::Result<PushNotificationActivationInfo> {
        Self::IPushNotificationActivationInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                activators,
                &mut result__,
            )
            .from_abi::<PushNotificationActivationInfo>(result__)
        })
    }
    pub fn IPushNotificationActivationInfoFactory<
        R,
        F: FnOnce(&IPushNotificationActivationInfoFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PushNotificationActivationInfo,
            IPushNotificationActivationInfoFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationActivationInfo {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationActivationInfo;{f50b1226-8eb2-57b9-891f-9ceb3948c5c3})" ) ;
}
unsafe impl ::windows::core::Interface for PushNotificationActivationInfo {
    type Vtable = IPushNotificationActivationInfo_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf50b1226_8eb2_57b9_891f_9ceb3948c5c3);
}
impl ::windows::core::RuntimeName for PushNotificationActivationInfo {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationActivationInfo";
}
impl ::core::convert::From<PushNotificationActivationInfo> for ::windows::core::IUnknown {
    fn from(value: PushNotificationActivationInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PushNotificationActivationInfo> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationActivationInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PushNotificationActivationInfo> for ::windows::core::IInspectable {
    fn from(value: PushNotificationActivationInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PushNotificationActivationInfo> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationActivationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationActivationInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PushNotificationActivationInfo {}
unsafe impl ::core::marker::Sync for PushNotificationActivationInfo {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PushNotificationChannel(pub ::windows::core::IInspectable);
impl PushNotificationChannel {
    pub fn Uri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    pub fn ExpirationTime(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn PushReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                PushNotificationChannel,
                PushNotificationReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePushReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CreateInstance<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Networking::PushNotifications::PushNotificationChannel,
        >,
    >(
        channel: Param0,
    ) -> ::windows::core::Result<PushNotificationChannel> {
        Self::IPushNotificationChannelFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                channel.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PushNotificationChannel>(result__)
        })
    }
    pub fn IPushNotificationChannelFactory<
        R,
        F: FnOnce(&IPushNotificationChannelFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PushNotificationChannel,
            IPushNotificationChannelFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannel {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationChannel;{cd4ef2a0-3985-5848-8426-813de636f825})" ) ;
}
unsafe impl ::windows::core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcd4ef2a0_3985_5848_8426_813de636f825);
}
impl ::windows::core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationChannel";
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: PushNotificationChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: PushNotificationChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationChannel
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PushNotificationChannelStatus(pub i32);
impl PushNotificationChannelStatus {
    pub const InProgress: PushNotificationChannelStatus = PushNotificationChannelStatus(0i32);
    pub const InProgressRetry: PushNotificationChannelStatus = PushNotificationChannelStatus(1i32);
    pub const CompletedSuccess: PushNotificationChannelStatus = PushNotificationChannelStatus(2i32);
    pub const CompletedFailure: PushNotificationChannelStatus = PushNotificationChannelStatus(3i32);
}
impl ::core::convert::From<i32> for PushNotificationChannelStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PushNotificationChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4)",
    );
}
impl ::windows::core::DefaultType for PushNotificationChannelStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PushNotificationCreateChannelResult(pub ::windows::core::IInspectable);
impl PushNotificationCreateChannelResult {
    pub fn Channel(&self) -> ::windows::core::Result<PushNotificationChannel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationChannel>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<PushNotificationChannelStatus> {
        let this = self;
        unsafe {
            let mut result__: PushNotificationChannelStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PushNotificationChannelStatus>(result__)
        }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, PushNotificationChannel>>(
        channel: Param0,
        extendederror: ::windows::core::HRESULT,
        status: PushNotificationChannelStatus,
    ) -> ::windows::core::Result<PushNotificationCreateChannelResult> {
        Self::IPushNotificationCreateChannelResultFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                channel.into_param().abi(),
                extendederror,
                status,
                &mut result__,
            )
            .from_abi::<PushNotificationCreateChannelResult>(result__)
        })
    }
    pub fn IPushNotificationCreateChannelResultFactory<
        R,
        F: FnOnce(&IPushNotificationCreateChannelResultFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PushNotificationCreateChannelResult,
            IPushNotificationCreateChannelResultFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelResult {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult;{4df3717f-5d33-56e9-b381-1b350c95722e})" ) ;
}
unsafe impl ::windows::core::Interface for PushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df3717f_5d33_56e9_b381_1b350c95722e);
}
impl ::windows::core::RuntimeName for PushNotificationCreateChannelResult {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult";
}
impl ::core::convert::From<PushNotificationCreateChannelResult> for ::windows::core::IUnknown {
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PushNotificationCreateChannelResult> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PushNotificationCreateChannelResult> for ::windows::core::IInspectable {
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PushNotificationCreateChannelResult> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PushNotificationCreateChannelResult {}
unsafe impl ::core::marker::Sync for PushNotificationCreateChannelResult {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PushNotificationCreateChannelStatus {
    pub status: PushNotificationChannelStatus,
    pub extendedError: ::windows::core::HRESULT,
    pub retryCount: u32,
}
impl PushNotificationCreateChannelStatus {}
impl ::core::default::Default for PushNotificationCreateChannelStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PushNotificationCreateChannelStatus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PushNotificationCreateChannelStatus")
            .field("status", &self.status)
            .field("extendedError", &self.extendedError)
            .field("retryCount", &self.retryCount)
            .finish()
    }
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelStatus {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.extendedError == other.extendedError
            && self.retryCount == other.retryCount
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelStatus {}
unsafe impl ::windows::core::Abi for PushNotificationCreateChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelStatus {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"struct(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelStatus;enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4);struct(Windows.Foundation.HResult;i4);u4)" ) ;
}
impl ::windows::core::DefaultType for PushNotificationCreateChannelStatus {
    type DefaultType = Self;
}
pub struct PushNotificationManager {}
impl PushNotificationManager {
    pub fn RegisterActivator<
        'a,
        Param0: ::windows::core::IntoParam<'a, PushNotificationActivationInfo>,
    >(
        details: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                details.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn UnregisterActivator(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::core::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                activators,
            )
            .ok()
        })
    }
    pub fn UnregisterAllActivators() -> ::windows::core::Result<()> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok()
        })
    }
    pub fn CreateChannelAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(
        remoteid: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperationWithProgress<
            PushNotificationCreateChannelResult,
            PushNotificationCreateChannelStatus,
        >,
    > {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                remoteid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncOperationWithProgress<
                PushNotificationCreateChannelResult,
                PushNotificationCreateChannelStatus,
            >>(result__)
        })
    }
    pub fn IsActivatorSupported(
        activators: PushNotificationRegistrationActivators,
    ) -> ::windows::core::Result<bool> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                activators,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn IPushNotificationManagerStatics<
        R,
        F: FnOnce(&IPushNotificationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PushNotificationManager,
            IPushNotificationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PushNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationManager";
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct PushNotificationReceivedEventArgs(pub ::windows::core::IInspectable);
impl PushNotificationReceivedEventArgs {
    pub fn Payload(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<u8>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn GetDeferral(
        &self,
    ) -> ::windows::core::Result<::windows::ApplicationModel::Background::BackgroundTaskDeferral>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::ApplicationModel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    pub fn Canceled<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::ApplicationModel::Background::BackgroundTaskCanceledEventHandler,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs;{a8fe81bc-d2d9-52cf-ab1b-6fde711862ea})" ) ;
}
unsafe impl ::windows::core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa8fe81bc_d2d9_52cf_ab1b_6fde711862ea);
}
impl ::windows::core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs";
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PushNotificationRegistrationActivators(pub u32);
impl PushNotificationRegistrationActivators {
    pub const Undefined: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(0u32);
    pub const PushTrigger: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(1u32);
    pub const ComActivator: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(2u32);
    pub const ProtocolActivator: PushNotificationRegistrationActivators =
        PushNotificationRegistrationActivators(4u32);
}
impl ::core::convert::From<u32> for PushNotificationRegistrationActivators {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PushNotificationRegistrationActivators {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PushNotificationRegistrationActivators {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationRegistrationActivators;u4)",
    );
}
impl ::windows::core::DefaultType for PushNotificationRegistrationActivators {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PushNotificationRegistrationActivators {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PushNotificationRegistrationActivators {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PushNotificationRegistrationActivators {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PushNotificationRegistrationActivators {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PushNotificationRegistrationActivators {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
