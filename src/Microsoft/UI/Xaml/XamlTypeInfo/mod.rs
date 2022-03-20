#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlControlsXamlMetaDataProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProvider {
    type Vtable = IXamlControlsXamlMetaDataProvider_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXamlControlsXamlMetaDataProviderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXamlControlsXamlMetaDataProviderStatics {
    type Vtable = IXamlControlsXamlMetaDataProviderStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlControlsXamlMetaDataProviderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Initialize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`, `\"UI_Xaml_Markup\"`*"]
#[cfg(feature = "UI_Xaml_Markup")]
#[repr(transparent)]
pub struct XamlControlsXamlMetaDataProvider(::windows::core::IUnknown);
#[cfg(feature = "UI_Xaml_Markup")]
impl XamlControlsXamlMetaDataProvider {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
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
    #[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`*"]
    pub fn Initialize() -> ::windows::core::Result<()> {
        Self::IXamlControlsXamlMetaDataProviderStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).Initialize)(::core::mem::transmute_copy(this))
                .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`, `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).GetXamlType)(
                ::core::mem::transmute_copy(this),
                r#type.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`, `\"UI_Xaml_Markup\"`*"]
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
            (::windows::core::Interface::vtable(this).GetXamlTypeByFullName)(
                ::core::mem::transmute_copy(this),
                fullname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Markup::IXamlType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_XamlTypeInfo\"`, `\"UI_Xaml_Markup\"`*"]
    #[cfg(feature = "UI_Xaml_Markup")]
    pub fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<super::Markup::XmlnsDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<super::Markup::XmlnsDefinition> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetXmlnsDefinitions)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<super::Markup::XmlnsDefinition>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc(hidden)]
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
impl ::core::clone::Clone for XamlControlsXamlMetaDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::cmp::PartialEq for XamlControlsXamlMetaDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::cmp::Eq for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::fmt::Debug for XamlControlsXamlMetaDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XamlControlsXamlMetaDataProvider")
            .field(&self.0)
            .finish()
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::RuntimeType for XamlControlsXamlMetaDataProvider {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider;{a96251f0-2214-5d53-8746-ce99a2593cd7})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::windows::core::Interface for XamlControlsXamlMetaDataProvider {
    type Vtable = super::Markup::IXamlMetadataProvider_Vtbl;
    const IID: ::windows::core::GUID =
        <super::Markup::IXamlMetadataProvider as ::windows::core::Interface>::IID;
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::windows::core::RuntimeName for XamlControlsXamlMetaDataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::core::IUnknown {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::core::IUnknown {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<XamlControlsXamlMetaDataProvider> for ::windows::core::IInspectable {
    fn from(value: XamlControlsXamlMetaDataProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::From<&XamlControlsXamlMetaDataProvider> for ::windows::core::IInspectable {
    fn from(value: &XamlControlsXamlMetaDataProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::TryFrom<XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    type Error = ::windows::core::Error;
    fn try_from(value: XamlControlsXamlMetaDataProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl ::core::convert::TryFrom<&XamlControlsXamlMetaDataProvider>
    for super::Markup::IXamlMetadataProvider
{
    type Error = ::windows::core::Error;
    fn try_from(value: &XamlControlsXamlMetaDataProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
impl<'a> ::windows::core::IntoParam<'a, super::Markup::IXamlMetadataProvider>
    for &XamlControlsXamlMetaDataProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, super::Markup::IXamlMetadataProvider> {
        ::core::convert::TryInto::<super::Markup::IXamlMetadataProvider>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Send for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "UI_Xaml_Markup")]
unsafe impl ::core::marker::Sync for XamlControlsXamlMetaDataProvider {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
