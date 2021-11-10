#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub struct DeploymentManager {}
impl DeploymentManager {
    pub fn GetStatus() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
    pub fn Initialize() -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DeploymentResult(pub ::windows::core::IInspectable);
impl DeploymentResult {
    pub fn Status(&self) -> ::windows::core::Result<DeploymentStatus> {
        let this = self;
        unsafe {
            let mut result__: DeploymentStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DeploymentStatus>(result__)
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
    pub fn CreateInstance(
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
    ) -> ::windows::core::Result<DeploymentResult> {
        Self::IDeploymentResultFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                status,
                extendederror,
                &mut result__,
            )
            .from_abi::<DeploymentResult>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for DeploymentResult {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult;{27203f62-463d-587a-8eb7-870098901078})" ) ;
}
unsafe impl ::windows::core::Interface for DeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27203f62_463d_587a_8eb7_870098901078);
}
impl ::windows::core::RuntimeName for DeploymentResult {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentResult";
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: DeploymentResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IUnknown {
    fn from(value: &DeploymentResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: DeploymentResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeploymentResult> for ::windows::core::IInspectable {
    fn from(value: &DeploymentResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeploymentResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DeploymentResult {}
unsafe impl ::core::marker::Sync for DeploymentResult {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DeploymentStatus(pub i32);
impl DeploymentStatus {
    pub const Unknown: DeploymentStatus = DeploymentStatus(0i32);
    pub const Ok: DeploymentStatus = DeploymentStatus(1i32);
    pub const PackageInstallRequired: DeploymentStatus = DeploymentStatus(2i32);
    pub const PackageInstallFailed: DeploymentStatus = DeploymentStatus(3i32);
}
impl ::core::convert::From<i32> for DeploymentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DeploymentStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DeploymentStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.WindowsAppRuntime.DeploymentStatus;i4)",
    );
}
impl ::windows::core::DefaultType for DeploymentStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeploymentManagerStatics {
    type Vtable = IDeploymentManagerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6782a9d0_bfd0_50ea_81b0_32e9ed37cdf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentManagerStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeploymentResult {
    type Vtable = IDeploymentResult_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x27203f62_463d_587a_8eb7_870098901078);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_abi(
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
        result__: *mut DeploymentStatus,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::HRESULT,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeploymentResultFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeploymentResultFactory {
    type Vtable = IDeploymentResultFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xacd7bdae_4ae6_5cac_8205_1e8c305f953b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResultFactory_abi(
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
        status: DeploymentStatus,
        extendederror: ::windows::core::HRESULT,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
