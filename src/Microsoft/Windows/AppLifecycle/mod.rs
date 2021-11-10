#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub struct ActivationRegistrationManager {}
impl ActivationRegistrationManager {
    pub fn RegisterForFileTypeActivation<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        supportedfiletypes : & [ < :: windows :: core :: HSTRING as :: windows :: core :: DefaultType > :: DefaultType ],
        logo: Param1,
        displayname: Param2,
        supportedverbs : & [ < :: windows :: core :: HSTRING as :: windows :: core :: DefaultType > :: DefaultType ],
        exepath: Param4,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                supportedfiletypes.len() as u32,
                ::core::mem::transmute(supportedfiletypes.as_ptr()),
                logo.into_param().abi(),
                displayname.into_param().abi(),
                supportedverbs.len() as u32,
                ::core::mem::transmute(supportedverbs.as_ptr()),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn RegisterForProtocolActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        scheme: Param0,
        logo: Param1,
        displayname: Param2,
        exepath: Param3,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                scheme.into_param().abi(),
                logo.into_param().abi(),
                displayname.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn RegisterForStartupActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        taskid: Param0,
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                taskid.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn UnregisterForFileTypeActivation<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        filetypes: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType],
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                filetypes.len() as u32,
                ::core::mem::transmute(filetypes.as_ptr()),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn UnregisterForProtocolActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        scheme: Param0,
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                scheme.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn UnregisterForStartupActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        taskid: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                taskid.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IActivationRegistrationManagerStatics<
        R,
        F: FnOnce(&IActivationRegistrationManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ActivationRegistrationManager,
            IActivationRegistrationManagerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ActivationRegistrationManager {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.ActivationRegistrationManager";
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppActivationArguments(pub ::windows::core::IInspectable);
impl AppActivationArguments {
    pub fn Kind(&self) -> ::windows::core::Result<ExtendedActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ExtendedActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ExtendedActivationKind>(result__)
        }
    }
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationArguments {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppLifecycle.AppActivationArguments;{14f99eaf-1580-5062-bdc8-d5d1c31138fb})" ) ;
}
unsafe impl ::windows::core::Interface for AppActivationArguments {
    type Vtable = IAppActivationArguments_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14f99eaf_1580_5062_bdc8_d5d1c31138fb);
}
impl ::windows::core::RuntimeName for AppActivationArguments {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
}
impl ::core::convert::From<AppActivationArguments> for ::windows::core::IUnknown {
    fn from(value: AppActivationArguments) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::core::IUnknown {
    fn from(value: &AppActivationArguments) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppActivationArguments> for ::windows::core::IInspectable {
    fn from(value: AppActivationArguments) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::core::IInspectable {
    fn from(value: &AppActivationArguments) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppActivationArguments
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppActivationArguments {}
unsafe impl ::core::marker::Sync for AppActivationArguments {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AppInstance(pub ::windows::core::IInspectable);
impl AppInstance {
    pub fn UnregisterKey(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn RedirectActivationToAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppActivationArguments>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                args.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetActivatedEventArgs(&self) -> ::windows::core::Result<AppActivationArguments> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppActivationArguments>(result__)
        }
    }
    pub fn Activated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventHandler<AppActivationArguments>>,
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
    pub fn RemoveActivated<
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
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn GetCurrent() -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    pub fn GetInstances(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    pub fn FindOrRegisterForKey<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        key: Param0,
    ) -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                key.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.AppLifecycle.AppInstance;{75766ae4-0239-5a26-b9da-d5bfc75a4866})",
    );
}
unsafe impl ::windows::core::Interface for AppInstance {
    type Vtable = IAppInstance_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75766ae4_0239_5a26_b9da_d5bfc75a4866);
}
impl ::windows::core::RuntimeName for AppInstance {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
}
impl ::core::convert::From<AppInstance> for ::windows::core::IUnknown {
    fn from(value: AppInstance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IUnknown {
    fn from(value: &AppInstance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppInstance> for ::windows::core::IInspectable {
    fn from(value: AppInstance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IInspectable {
    fn from(value: &AppInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ExtendedActivationKind(pub i32);
impl ExtendedActivationKind {
    pub const Launch: ExtendedActivationKind = ExtendedActivationKind(0i32);
    pub const Search: ExtendedActivationKind = ExtendedActivationKind(1i32);
    pub const ShareTarget: ExtendedActivationKind = ExtendedActivationKind(2i32);
    pub const File: ExtendedActivationKind = ExtendedActivationKind(3i32);
    pub const Protocol: ExtendedActivationKind = ExtendedActivationKind(4i32);
    pub const FileOpenPicker: ExtendedActivationKind = ExtendedActivationKind(5i32);
    pub const FileSavePicker: ExtendedActivationKind = ExtendedActivationKind(6i32);
    pub const CachedFileUpdater: ExtendedActivationKind = ExtendedActivationKind(7i32);
    pub const ContactPicker: ExtendedActivationKind = ExtendedActivationKind(8i32);
    pub const Device: ExtendedActivationKind = ExtendedActivationKind(9i32);
    pub const PrintTaskSettings: ExtendedActivationKind = ExtendedActivationKind(10i32);
    pub const CameraSettings: ExtendedActivationKind = ExtendedActivationKind(11i32);
    pub const RestrictedLaunch: ExtendedActivationKind = ExtendedActivationKind(12i32);
    pub const AppointmentsProvider: ExtendedActivationKind = ExtendedActivationKind(13i32);
    pub const Contact: ExtendedActivationKind = ExtendedActivationKind(14i32);
    pub const LockScreenCall: ExtendedActivationKind = ExtendedActivationKind(15i32);
    pub const VoiceCommand: ExtendedActivationKind = ExtendedActivationKind(16i32);
    pub const LockScreen: ExtendedActivationKind = ExtendedActivationKind(17i32);
    pub const PickerReturned: ExtendedActivationKind = ExtendedActivationKind(1000i32);
    pub const WalletAction: ExtendedActivationKind = ExtendedActivationKind(1001i32);
    pub const PickFileContinuation: ExtendedActivationKind = ExtendedActivationKind(1002i32);
    pub const PickSaveFileContinuation: ExtendedActivationKind = ExtendedActivationKind(1003i32);
    pub const PickFolderContinuation: ExtendedActivationKind = ExtendedActivationKind(1004i32);
    pub const WebAuthenticationBrokerContinuation: ExtendedActivationKind =
        ExtendedActivationKind(1005i32);
    pub const WebAccountProvider: ExtendedActivationKind = ExtendedActivationKind(1006i32);
    pub const ComponentUI: ExtendedActivationKind = ExtendedActivationKind(1007i32);
    pub const ProtocolForResults: ExtendedActivationKind = ExtendedActivationKind(1009i32);
    pub const ToastNotification: ExtendedActivationKind = ExtendedActivationKind(1010i32);
    pub const Print3DWorkflow: ExtendedActivationKind = ExtendedActivationKind(1011i32);
    pub const DialReceiver: ExtendedActivationKind = ExtendedActivationKind(1012i32);
    pub const DevicePairing: ExtendedActivationKind = ExtendedActivationKind(1013i32);
    pub const UserDataAccountsProvider: ExtendedActivationKind = ExtendedActivationKind(1014i32);
    pub const FilePickerExperience: ExtendedActivationKind = ExtendedActivationKind(1015i32);
    pub const LockScreenComponent: ExtendedActivationKind = ExtendedActivationKind(1016i32);
    pub const ContactPanel: ExtendedActivationKind = ExtendedActivationKind(1017i32);
    pub const PrintWorkflowForegroundTask: ExtendedActivationKind = ExtendedActivationKind(1018i32);
    pub const GameUIProvider: ExtendedActivationKind = ExtendedActivationKind(1019i32);
    pub const StartupTask: ExtendedActivationKind = ExtendedActivationKind(1020i32);
    pub const CommandLineLaunch: ExtendedActivationKind = ExtendedActivationKind(1021i32);
    pub const BarcodeScannerProvider: ExtendedActivationKind = ExtendedActivationKind(1022i32);
    pub const PrintSupportJobUI: ExtendedActivationKind = ExtendedActivationKind(1023i32);
    pub const PrintSupportSettingsUI: ExtendedActivationKind = ExtendedActivationKind(1024i32);
    pub const PhoneCallActivation: ExtendedActivationKind = ExtendedActivationKind(1025i32);
    pub const VpnForeground: ExtendedActivationKind = ExtendedActivationKind(1026i32);
    pub const Push: ExtendedActivationKind = ExtendedActivationKind(5000i32);
}
impl ::core::convert::From<i32> for ExtendedActivationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
    );
}
impl ::windows::core::DefaultType for ExtendedActivationKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivationRegistrationManagerStatics {
    type Vtable = IActivationRegistrationManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5ac4e92e_017b_5d68_8198_f68636ab99d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics_abi(
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
        supportedFileTypes_array_size: u32,
        supportedfiletypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        supportedVerbs_array_size: u32,
        supportedverbs: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        fileTypes_array_size: u32,
        filetypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppActivationArguments(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppActivationArguments {
    type Vtable = IAppActivationArguments_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14f99eaf_1580_5062_bdc8_d5d1c31138fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationArguments_abi(
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
        result__: *mut ExtendedActivationKind,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstance(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppInstance {
    type Vtable = IAppInstance_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75766ae4_0239_5a26_b9da_d5bfc75a4866);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppInstanceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4f414b25_8330_5a9b_bbc1_8229d479649d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
