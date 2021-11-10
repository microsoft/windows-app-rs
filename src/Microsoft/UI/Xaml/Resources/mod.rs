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
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CustomXamlResourceLoader(pub ::windows::core::IInspectable);
impl CustomXamlResourceLoader {
    pub fn Current() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn SetCurrent<'a, Param0: ::windows::core::IntoParam<'a, CustomXamlResourceLoader>>(
        value: Param0,
    ) -> ::windows::core::Result<()> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn new() -> ::windows::core::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                &mut result__,
            )
            .from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn ICustomXamlResourceLoaderStatics<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICustomXamlResourceLoaderFactory<
        R,
        F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CustomXamlResourceLoader,
            ICustomXamlResourceLoaderFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Resources.CustomXamlResourceLoader;{2832c2e5-2ace-5993-a173-3c9c3b992b2e})" ) ;
}
unsafe impl ::windows::core::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2832c2e5_2ace_5993_a173_3c9c3b992b2e);
}
impl ::windows::core::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Microsoft.UI.Xaml.Resources.CustomXamlResourceLoader";
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IUnknown {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CustomXamlResourceLoader> for ::windows::core::IInspectable {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CustomXamlResourceLoader
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::core::marker::Sync for CustomXamlResourceLoader {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2832c2e5_2ace_5993_a173_3c9c3b992b2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x174d49a6_e1e2_5f7b_a618_a8a953d1b5a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_abi(
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
        baseinterface: ::windows::core::RawPtr,
        innerinterface: *mut ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84bb504c_6730_586a_bd04_9198264b2dc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_abi(
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
        resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        objecttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        propertytype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe08a5a92_b1a2_539a_9d4a_7994e4468cd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_abi(
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
