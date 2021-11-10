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
pub struct IXamlControlsXamlMetaDataProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProvider {
    type Vtable = IXamlControlsXamlMetaDataProvider_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_abi(
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
pub struct IXamlControlsXamlMetaDataProviderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProviderStatics {
    type Vtable = IXamlControlsXamlMetaDataProviderStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_abi(
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
);
#[cfg(feature = "UI_Xaml_Markup")]
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct XamlControlsXamlMetaDataProvider(pub ::windows::core::IInspectable);
#[cfg(feature = "UI_Xaml_Markup")]
impl XamlControlsXamlMetaDataProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Initialize() -> ::windows::core::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        })
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXamlType<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
    >(
        &self,
        r#type: Param0,
    ) -> ::windows::core::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXamlTypeByFullName<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        fullname: Param0,
    ) -> ::windows::core::Result<super::Markup::IXamlType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<super::Markup::XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<super::Markup::XmlnsDefinition> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<super::Markup::XmlnsDefinition>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    pub fn IXamlControlsXamlMetaDataProviderStatics<
        R,
        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            XamlControlsXamlMetaDataProvider,
            IXamlControlsXamlMetaDataProviderStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider;{a96251f0-2214-5d53-8746-ce99a2593cd7})" ) ;
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = super::Markup::IXamlMetadataProvider_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa96251f0_2214_5d53_8746_ce99a2593cd7);
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::windows::core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::core::IUnknown {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::core::IUnknown {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::core::IInspectable {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        value.0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::core::IInspectable {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for &XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Send for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Sync for XamlControlsXamlMetaDataProvider {}
