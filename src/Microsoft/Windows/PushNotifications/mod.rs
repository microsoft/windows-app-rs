#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xda28bbcb_7695_5d38_af82_f30b72fef1f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Uri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::DateTime,
    ) -> ::windows::core::HRESULT,
    pub Close:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationCreateChannelResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4df3717f_5d33_56e9_b381_1b350c95722e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationCreateChannelResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Channel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PushNotificationChannelStatus,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x902f4aba_ff63_5dfe_a88f_15cc6bed55ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Register:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterAll:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChannelAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        remoteid: ::windows::core::GUID,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub PushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePushReceived: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationManagerStatics {
    type Vtable = IPushNotificationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x71329470_1b55_58dc_a00c_68c26f2d8bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Default: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfbd4ec53_bb83_5495_8777_d3cf13e4299c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Payload: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Canceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationChannel(::windows::core::IUnknown);
impl PushNotificationChannel {
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Uri(&self) -> ::windows::core::Result<::windows::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<::windows::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::DateTime>::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationTime)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
impl ::core::clone::Clone for PushNotificationChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationChannel {}
impl ::core::fmt::Debug for PushNotificationChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannel")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannel {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationChannel;{da28bbcb-7695-5d38-af82-f30b72fef1f6})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: ::windows::core::GUID =
        <IPushNotificationChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationChannel";
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: PushNotificationChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationChannel> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationChannel
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationChannel {}
unsafe impl ::core::marker::Sync for PushNotificationChannel {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PushNotificationChannelStatus(pub i32);
impl PushNotificationChannelStatus {
    pub const InProgress: Self = Self(0i32);
    pub const InProgressRetry: Self = Self(1i32);
    pub const CompletedSuccess: Self = Self(2i32);
    pub const CompletedFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for PushNotificationChannelStatus {}
impl ::core::clone::Clone for PushNotificationChannelStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PushNotificationChannelStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PushNotificationChannelStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PushNotificationChannelStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationChannelStatus")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationChannelStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationCreateChannelResult(::windows::core::IUnknown);
impl PushNotificationCreateChannelResult {
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Channel(&self) -> ::windows::core::Result<PushNotificationChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Channel)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationChannel>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::HRESULT>::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PushNotificationChannelStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PushNotificationChannelStatus>::zeroed();
            (::windows::core::Interface::vtable(this).Status)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationChannelStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PushNotificationCreateChannelResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelResult {}
impl ::core::fmt::Debug for PushNotificationCreateChannelResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationCreateChannelResult")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelResult {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult;{4df3717f-5d33-56e9-b381-1b350c95722e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationCreateChannelResult {
    type Vtable = IPushNotificationCreateChannelResult_Vtbl;
    const IID: ::windows::core::GUID =
        <IPushNotificationCreateChannelResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationCreateChannelResult {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationCreateChannelResult";
}
impl ::core::convert::From<PushNotificationCreateChannelResult> for ::windows::core::IUnknown {
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationCreateChannelResult> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationCreateChannelResult> for ::windows::core::IInspectable {
    fn from(value: PushNotificationCreateChannelResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationCreateChannelResult> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationCreateChannelResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationCreateChannelResult
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationCreateChannelResult {}
unsafe impl ::core::marker::Sync for PushNotificationCreateChannelResult {}
#[repr(C)]
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
pub struct PushNotificationCreateChannelStatus {
    pub status: PushNotificationChannelStatus,
    pub extendedError: ::windows::core::HRESULT,
    pub retryCount: u32,
}
impl ::core::marker::Copy for PushNotificationCreateChannelStatus {}
impl ::core::clone::Clone for PushNotificationCreateChannelStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PushNotificationCreateChannelStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PushNotificationCreateChannelStatus")
            .field("status", &self.status)
            .field("extendedError", &self.extendedError)
            .field("retryCount", &self.retryCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PushNotificationCreateChannelStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PushNotificationCreateChannelStatus {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"struct(Microsoft.Windows.PushNotifications.PushNotificationCreateChannelStatus;enum(Microsoft.Windows.PushNotifications.PushNotificationChannelStatus;i4);struct(Windows.Foundation.HResult;i4);u4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PushNotificationCreateChannelStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<PushNotificationCreateChannelStatus>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for PushNotificationCreateChannelStatus {}
impl ::core::default::Default for PushNotificationCreateChannelStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationManager(::windows::core::IUnknown);
impl PushNotificationManager {
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Register(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Unregister(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Unregister)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn UnregisterAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterAll)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn CreateChannelAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(
        &self,
        remoteid: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperationWithProgress<
            PushNotificationCreateChannelResult,
            PushNotificationCreateChannelStatus,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateChannelAsync)(
                ::windows::core::Interface::as_raw(this),
                remoteid.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperationWithProgress<
                PushNotificationCreateChannelResult,
                PushNotificationCreateChannelStatus,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn PushReceived<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                PushNotificationManager,
                PushNotificationReceivedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).PushReceived)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn RemovePushReceived<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePushReceived)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Default() -> ::windows::core::Result<PushNotificationManager> {
        Self::IPushNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Default)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<PushNotificationManager>(result__)
        })
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for PushNotificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationManager {}
impl ::core::fmt::Debug for PushNotificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationManager")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationManager {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationManager;{902f4aba-ff63-5dfe-a88f-15cc6bed55ff})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationManager {
    type Vtable = IPushNotificationManager_Vtbl;
    const IID: ::windows::core::GUID =
        <IPushNotificationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationManager {
    const NAME: &'static str = "Microsoft.Windows.PushNotifications.PushNotificationManager";
}
impl ::core::convert::From<PushNotificationManager> for ::windows::core::IUnknown {
    fn from(value: PushNotificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationManager> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PushNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PushNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationManager> for ::windows::core::IInspectable {
    fn from(value: PushNotificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationManager> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PushNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationManager {}
unsafe impl ::core::marker::Sync for PushNotificationManager {}
#[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
#[repr(transparent)]
pub struct PushNotificationReceivedEventArgs(::windows::core::IUnknown);
impl PushNotificationReceivedEventArgs {
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn Payload(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<u8>>::zeroed();
            (::windows::core::Interface::vtable(this).Payload)(
                ::windows::core::Interface::as_raw(this),
                ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::core::Result<::windows::ApplicationModel::Background::BackgroundTaskDeferral>
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::ApplicationModel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
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
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Canceled)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_PushNotifications\"`*"]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCanceled)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for PushNotificationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PushNotificationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationReceivedEventArgs {}
impl ::core::fmt::Debug for PushNotificationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationReceivedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs;{fbd4ec53-bb83-5495-8777-d3cf13e4299c})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IPushNotificationReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.PushNotifications.PushNotificationReceivedEventArgs";
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PushNotificationReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PushNotificationReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PushNotificationReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PushNotificationReceivedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PushNotificationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for PushNotificationReceivedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
