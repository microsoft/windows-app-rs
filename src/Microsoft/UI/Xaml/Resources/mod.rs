#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Xaml_Resources\"`*"]
#[repr(transparent)]
pub struct CustomXamlResourceLoader(::windows::core::IUnknown);
impl CustomXamlResourceLoader {
    #[doc = "*Required features: `\"UI_Xaml_Resources\"`*"]
    pub fn new() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Resources\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::core::mem::transmute_copy(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                &mut result__,
            )
            .from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Resources\"`*"]
    pub fn Current() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Resources\"`*"]
    pub fn SetCurrent<'a, Param0: ::windows::core::IntoParam<'a, CustomXamlResourceLoader>>(
        value: Param0,
    ) -> ::windows::core::Result<()> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetCurrent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn ICustomXamlResourceLoaderFactory<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderFactory,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICustomXamlResourceLoaderStatics<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Xaml.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CustomXamlResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CustomXamlResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomXamlResourceLoader {}
impl ::core::fmt::Debug for CustomXamlResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomXamlResourceLoader")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Resources.CustomXamlResourceLoader;{2832c2e5-2ace-5993-a173-3c9c3b992b2e})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_Vtbl;
    const IID: ::windows::core::GUID =
        <ICustomXamlResourceLoader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Resources.CustomXamlResourceLoader";
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: CustomXamlResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: CustomXamlResourceLoader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::core::marker::Sync for CustomXamlResourceLoader {}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomXamlResourceLoader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2832c2e5_2ace_5993_a173_3c9c3b992b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x174d49a6_e1e2_5f7b_a618_a8a953d1b5a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderOverrides(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84bb504c_6730_586a_bd04_9198264b2dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetResource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe08a5a92_b1a2_539a_9d4a_7994e4468cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetCurrent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
