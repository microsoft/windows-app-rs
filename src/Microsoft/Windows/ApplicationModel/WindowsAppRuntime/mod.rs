#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
pub struct DeploymentManager {}
impl DeploymentManager {
    #[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
    pub fn GetStatus() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatus)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    #[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
    pub fn Initialize() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Initialize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentManagerStatics<
        R,
        F: FnOnce(&IDeploymentManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DeploymentManager,
            IDeploymentManagerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for DeploymentManager {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentManager";
}
#[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
#[repr(transparent)]
pub struct DeploymentResult(::windows::core::IUnknown);
impl DeploymentResult {
    #[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
    pub fn Status(&self) -> ::windows::core::Result<DeploymentStatus> {
        let this = self;
        unsafe {
            let mut result__: DeploymentStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
    pub fn CreateInstance(
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
    ) -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentResultFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                status,
                extendederror,
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeploymentResultFactory<
        R,
        F: FnOnce(&IDeploymentResultFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DeploymentResult,
            IDeploymentResultFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeploymentResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeploymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentResult {}
impl ::core::fmt::Debug for DeploymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentResult {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult;{27203f62-463d-587a-8eb7-870098901078})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
    const IID: ::windows::core::GUID = <IDeploymentResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeploymentResult {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult";
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: DeploymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: &DeploymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: DeploymentResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: &DeploymentResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[doc = "*Required features: 'Windows_ApplicationModel_WindowsAppRuntime'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeploymentStatus(pub i32);
impl DeploymentStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Ok: Self = Self(1i32);
    pub const PackageInstallRequired: Self = Self(2i32);
    pub const PackageInstallFailed: Self = Self(3i32);
}
impl ::core::marker::Copy for DeploymentStatus {}
impl ::core::clone::Clone for DeploymentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeploymentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeploymentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeploymentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentStatus;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeploymentManagerStatics {
    type Vtable = IDeploymentManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6782a9d0_bfd0_50ea_81b0_32e9ed37cdf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27203f62_463d_587a_8eb7_870098901078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DeploymentStatus,
    ) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeploymentResultFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeploymentResultFactory {
    type Vtable = IDeploymentResultFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xacd7bdae_4ae6_5cac_8205_1e8c305f953b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResultFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
