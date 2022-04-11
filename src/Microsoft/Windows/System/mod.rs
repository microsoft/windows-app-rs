#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[cfg(feature = "Windows_System_Power")]
pub mod Power;
#[doc = "*Required features: `\"Windows_System\"`*"]
#[repr(transparent)]
pub struct EnvironmentManager(::windows::core::IUnknown);
impl EnvironmentManager {
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn GetEnvironmentVariables(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnvironmentVariables)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::core::HSTRING,
                ::windows::core::HSTRING,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn GetEnvironmentVariable<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnvironmentVariable)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn SetEnvironmentVariable<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetEnvironmentVariable)(
                ::core::mem::transmute_copy(this),
                name.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn AppendToPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).AppendToPath)(
                ::core::mem::transmute_copy(this),
                path.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn RemoveFromPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveFromPath)(
                ::core::mem::transmute_copy(this),
                path.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn AddExecutableFileExtension<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        pathext: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).AddExecutableFileExtension)(
                ::core::mem::transmute_copy(this),
                pathext.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn RemoveExecutableFileExtension<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        pathext: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveExecutableFileExtension)(
                ::core::mem::transmute_copy(this),
                pathext.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn GetForProcess() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForProcess)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn GetForUser() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn GetForMachine() -> ::windows::core::Result<EnvironmentManager> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForMachine)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<EnvironmentManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Windows_System\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IEnvironmentManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnvironmentManagerStatics<
        R,
        F: FnOnce(&IEnvironmentManagerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            EnvironmentManager,
            IEnvironmentManagerStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.WindowsAppRuntime.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EnvironmentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EnvironmentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnvironmentManager {}
impl ::core::fmt::Debug for EnvironmentManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnvironmentManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EnvironmentManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Windows.System.EnvironmentManager;{d1b239bb-7013-5176-b02a-63477410d986})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
    const IID: ::windows::core::GUID = <IEnvironmentManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnvironmentManager {
    const NAME: &'static str = "Microsoft.Windows.System.EnvironmentManager";
}
impl ::core::convert::From<EnvironmentManager> for ::windows::core::IUnknown {
    fn from(value: EnvironmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnvironmentManager> for ::windows::core::IUnknown {
    fn from(value: &EnvironmentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnvironmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnvironmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnvironmentManager> for ::windows::core::IInspectable {
    fn from(value: EnvironmentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnvironmentManager> for ::windows::core::IInspectable {
    fn from(value: &EnvironmentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnvironmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnvironmentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnvironmentManager {}
unsafe impl ::core::marker::Sync for EnvironmentManager {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnvironmentManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnvironmentManager {
    type Vtable = IEnvironmentManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd1b239bb_7013_5176_b02a_63477410d986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManager_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetEnvironmentVariables: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetEnvironmentVariable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AppendToPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RemoveFromPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub AddExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub RemoveExecutableFileExtension: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnvironmentManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnvironmentManagerStatics {
    type Vtable = IEnvironmentManagerStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x407b1522_6156_5398_93fd_d6411c35e7b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnvironmentManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForProcess: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetForMachine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
