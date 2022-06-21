#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
pub struct ActivationRegistrationManager {}
impl ActivationRegistrationManager {
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn RegisterForFileTypeActivation<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        supportedfiletypes: &[::windows::core::HSTRING],
        logo: Param1,
        displayname: Param2,
        supportedverbs: &[::windows::core::HSTRING],
        exepath: Param4,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RegisterForFileTypeActivation)(
                ::windows::core::Interface::as_raw(this),
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
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
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
            (::windows::core::Interface::vtable(this).RegisterForProtocolActivation)(
                ::windows::core::Interface::as_raw(this),
                scheme.into_param().abi(),
                logo.into_param().abi(),
                displayname.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn RegisterForStartupActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        taskid: Param0,
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).RegisterForStartupActivation)(
                ::windows::core::Interface::as_raw(this),
                taskid.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn UnregisterForFileTypeActivation<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        filetypes: &[::windows::core::HSTRING],
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).UnregisterForFileTypeActivation)(
                ::windows::core::Interface::as_raw(this),
                filetypes.len() as u32,
                ::core::mem::transmute(filetypes.as_ptr()),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn UnregisterForProtocolActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        scheme: Param0,
        exepath: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).UnregisterForProtocolActivation)(
                ::windows::core::Interface::as_raw(this),
                scheme.into_param().abi(),
                exepath.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn UnregisterForStartupActivation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        taskid: Param0,
    ) -> ::windows::core::Result<()> {
        Self::IActivationRegistrationManagerStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).UnregisterForStartupActivation)(
                ::windows::core::Interface::as_raw(this),
                taskid.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
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
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
#[repr(transparent)]
pub struct AppActivationArguments(::windows::core::IUnknown);
impl AppActivationArguments {
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ExtendedActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ExtendedActivationKind>::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ExtendedActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Data)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for AppActivationArguments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppActivationArguments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppActivationArguments {}
impl ::core::fmt::Debug for AppActivationArguments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppActivationArguments")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppActivationArguments {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.AppLifecycle.AppActivationArguments;{14f99eaf-1580-5062-bdc8-d5d1c31138fb})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
    const IID: ::windows::core::GUID = <IAppActivationArguments as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppActivationArguments {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppActivationArguments";
}
impl ::core::convert::From<AppActivationArguments> for ::windows::core::IUnknown {
    fn from(value: AppActivationArguments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::core::IUnknown {
    fn from(value: &AppActivationArguments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppActivationArguments> for ::windows::core::IInspectable {
    fn from(value: AppActivationArguments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppActivationArguments> for ::windows::core::IInspectable {
    fn from(value: &AppActivationArguments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppActivationArguments {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AppActivationArguments
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppActivationArguments {}
unsafe impl ::core::marker::Sync for AppActivationArguments {}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
#[repr(transparent)]
pub struct AppInstance(::windows::core::IUnknown);
impl AppInstance {
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn UnregisterKey(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterKey)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn RedirectActivationToAsync<
        'a,
        Param0: ::windows::core::IntoParam<'a, AppActivationArguments>,
    >(
        &self,
        args: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RedirectActivationToAsync)(
                ::windows::core::Interface::as_raw(this),
                args.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn GetActivatedEventArgs(&self) -> ::windows::core::Result<AppActivationArguments> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetActivatedEventArgs)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppActivationArguments>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn Activated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventHandler<AppActivationArguments>>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Activated)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn RemoveActivated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveActivated)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Key)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn IsCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrent)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).ProcessId)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn GetCurrent() -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrent)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn GetInstances(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetInstances)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AppInstance>>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn FindOrRegisterForKey<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        key: Param0,
    ) -> ::windows::core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).FindOrRegisterForKey)(
                ::windows::core::Interface::as_raw(this),
                key.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<AppInstance>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
    pub fn Restart<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        arguments: Param0,
    ) -> ::windows::core::Result<::windows::ApplicationModel::Core::AppRestartFailureReason> {
        Self::IAppInstanceStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::windows::ApplicationModel::Core::AppRestartFailureReason,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Restart)(
                ::windows::core::Interface::as_raw(this),
                arguments.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::ApplicationModel::Core::AppRestartFailureReason>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics2<
        R,
        F: FnOnce(&IAppInstanceStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppInstance, IAppInstanceStatics2> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstance {}
impl ::core::fmt::Debug for AppInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.AppLifecycle.AppInstance;{75766ae4-0239-5a26-b9da-d5bfc75a4866})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows::core::GUID = <IAppInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppInstance {
    const NAME: &'static str = "Microsoft.Windows.AppLifecycle.AppInstance";
}
impl ::core::convert::From<AppInstance> for ::windows::core::IUnknown {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IUnknown {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppInstance> for ::windows::core::IInspectable {
    fn from(value: AppInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppInstance> for ::windows::core::IInspectable {
    fn from(value: &AppInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppInstance {}
unsafe impl ::core::marker::Sync for AppInstance {}
#[doc = "*Required features: `\"Windows_AppLifecycle\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedActivationKind(pub i32);
impl ExtendedActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
    pub const Push: Self = Self(5000i32);
    pub const AppNotification: Self = Self(5001i32);
}
impl ::core::marker::Copy for ExtendedActivationKind {}
impl ::core::clone::Clone for ExtendedActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExtendedActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedActivationKind")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.AppLifecycle.ExtendedActivationKind;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationRegistrationManagerStatics {
    type Vtable = IActivationRegistrationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5ac4e92e_017b_5d68_8198_f68636ab99d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationRegistrationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RegisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        supportedFileTypes_array_size: u32,
        supportedfiletypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        supportedVerbs_array_size: u32,
        supportedverbs: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RegisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        logo: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RegisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForFileTypeActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fileTypes_array_size: u32,
        filetypes: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForProtocolActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        exepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub UnregisterForStartupActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppActivationArguments(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppActivationArguments {
    type Vtable = IAppActivationArguments_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x14f99eaf_1580_5062_bdc8_d5d1c31138fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppActivationArguments_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ExtendedActivationKind,
    ) -> ::windows::core::HRESULT,
    pub Data: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstance(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75766ae4_0239_5a26_b9da_d5bfc75a4866);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstance_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UnregisterKey:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RedirectActivationToAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        args: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetActivatedEventArgs: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Activated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Key: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ProcessId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstanceStatics {
    type Vtable = IAppInstanceStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4f414b25_8330_5a9b_bbc1_8229d479649d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetInstances: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub FindOrRegisterForKey: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstanceStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppInstanceStatics2 {
    type Vtable = IAppInstanceStatics2_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfe9f1885_7160_5397_ba9b_5890b24fdc04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstanceStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Restart: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::ApplicationModel::Core::AppRestartFailureReason,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
