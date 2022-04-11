#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentAppWindowBridge(::windows::core::IUnknown);
impl ContentAppWindowBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn GetForWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        childwindowid: Param0,
    ) -> ::windows::core::Result<ContentAppWindowBridge> {
        Self::IContentAppWindowBridgeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForWindowId)(
                ::core::mem::transmute_copy(this),
                childwindowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ContentAppWindowBridge>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn CurrentOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn NativeOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NativeOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SettingsChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveSettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveSettingsChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThemeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveThemeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc(hidden)]
    pub fn IContentAppWindowBridgeStatics<
        R,
        F: FnOnce(&IContentAppWindowBridgeStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ContentAppWindowBridge,
            IContentAppWindowBridgeStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentAppWindowBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentAppWindowBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentAppWindowBridge {}
impl ::core::fmt::Debug for ContentAppWindowBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentAppWindowBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentAppWindowBridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.ContentAppWindowBridge;{22c454e8-3a91-574e-9b39-8d50573ee3f2})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentAppWindowBridge {
    type Vtable = IContentWindowBridge_Vtbl;
    const IID: ::windows::core::GUID = <IContentWindowBridge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentAppWindowBridge {
    const NAME: &'static str = "Microsoft.UI.Content.ContentAppWindowBridge";
}
impl ::core::convert::From<ContentAppWindowBridge> for ::windows::core::IUnknown {
    fn from(value: ContentAppWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentAppWindowBridge> for ::windows::core::IUnknown {
    fn from(value: &ContentAppWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentAppWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentAppWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentAppWindowBridge> for ::windows::core::IInspectable {
    fn from(value: ContentAppWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentAppWindowBridge> for ::windows::core::IInspectable {
    fn from(value: &ContentAppWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentAppWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentAppWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContentAppWindowBridge> for IContentWindowBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentAppWindowBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContentAppWindowBridge> for IContentWindowBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentAppWindowBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentWindowBridge> for ContentAppWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentWindowBridge> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentWindowBridge> for &ContentAppWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentWindowBridge> {
        ::core::convert::TryInto::<IContentWindowBridge>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContentAppWindowBridge {}
unsafe impl ::core::marker::Sync for ContentAppWindowBridge {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ContentCoordinateConversionBehavior(pub i32);
impl ContentCoordinateConversionBehavior {
    pub const Default: Self = Self(0i32);
    pub const Floor: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Ceiling: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentCoordinateConversionBehavior {}
impl ::core::clone::Clone for ContentCoordinateConversionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentCoordinateConversionBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContentCoordinateConversionBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContentCoordinateConversionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentCoordinateConversionBehavior")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentCoordinateConversionBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Content.ContentCoordinateConversionBehavior;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ContentDisplayOrientations(pub i32);
impl ContentDisplayOrientations {
    pub const None: Self = Self(0i32);
    pub const Landscape: Self = Self(1i32);
    pub const Portrait: Self = Self(2i32);
    pub const LandscapeFlipped: Self = Self(4i32);
    pub const PortraitFlipped: Self = Self(8i32);
}
impl ::core::marker::Copy for ContentDisplayOrientations {}
impl ::core::clone::Clone for ContentDisplayOrientations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentDisplayOrientations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContentDisplayOrientations {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContentDisplayOrientations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentDisplayOrientations")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentDisplayOrientations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Content.ContentDisplayOrientations;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentExternalOutputLink(::windows::core::IUnknown);
impl ContentExternalOutputLink {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn PlacementVisual(&self) -> ::windows::core::Result<super::Composition::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlacementVisual)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Composition::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: ::windows::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, ::windows::UI::Color>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBackgroundColor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<ContentExternalOutputLink> {
        Self::IContentExternalOutputLinkStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ContentExternalOutputLink>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IContentExternalOutputLinkStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentExternalOutputLinkStatics<
        R,
        F: FnOnce(&IContentExternalOutputLinkStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ContentExternalOutputLink,
            IContentExternalOutputLinkStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentExternalOutputLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentExternalOutputLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentExternalOutputLink {}
impl ::core::fmt::Debug for ContentExternalOutputLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentExternalOutputLink")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentExternalOutputLink {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Content.ContentExternalOutputLink;{6de3ea12-fc7e-5142-a8b6-46f01758f40d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentExternalOutputLink {
    type Vtable = IContentExternalOutputLink_Vtbl;
    const IID: ::windows::core::GUID =
        <IContentExternalOutputLink as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentExternalOutputLink {
    const NAME: &'static str = "Microsoft.UI.Content.ContentExternalOutputLink";
}
impl ::core::convert::From<ContentExternalOutputLink> for ::windows::core::IUnknown {
    fn from(value: ContentExternalOutputLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentExternalOutputLink> for ::windows::core::IUnknown {
    fn from(value: &ContentExternalOutputLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentExternalOutputLink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ContentExternalOutputLink
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentExternalOutputLink> for ::windows::core::IInspectable {
    fn from(value: ContentExternalOutputLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentExternalOutputLink> for ::windows::core::IInspectable {
    fn from(value: &ContentExternalOutputLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ContentExternalOutputLink
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentExternalOutputLink
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContentExternalOutputLink> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentExternalOutputLink) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContentExternalOutputLink> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentExternalOutputLink) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for ContentExternalOutputLink
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &ContentExternalOutputLink
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContentExternalOutputLink {}
unsafe impl ::core::marker::Sync for ContentExternalOutputLink {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentIsland(::windows::core::IUnknown);
impl ContentIsland {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ActualSize(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn AppData(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppData)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetAppData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAppData)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn CustomProperties(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomProperties)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Composition::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Input_Experimental\"`*"]
    #[cfg(feature = "UI_Input_Experimental")]
    pub fn InputSite(&self) -> ::windows::core::Result<super::Input::Experimental::ExpInputSite> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::Experimental::ExpInputSite>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsContentVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContentVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsContentVisible)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsHitTestAlways(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHitTestAlways)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetIsHitTestAlways(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsHitTestAlways)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSiteVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSiteVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RasterizationScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RequestedSize(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetRequestedSize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRequestedSize)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Root(&self) -> ::windows::core::Result<super::Composition::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Root)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Composition::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetRoot<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::Visual>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRoot)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowBridge(&self) -> ::windows::core::Result<IContentWindowBridge> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IContentWindowBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeLocalCoordinatesWithPointInt<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::PointInt32>,
    >(
        &self,
        screenpositions: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeLocalCoordinatesWithPointInt)(
                ::core::mem::transmute_copy(this),
                screenpositions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeLocalCoordinatesWithPointInts(
        &self,
        screenpositions: &[::windows::Graphics::PointInt32],
    ) -> ::windows::core::Result<::windows::core::Array<::windows::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::Foundation::Point> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeLocalCoordinatesWithPointInts)(
                ::core::mem::transmute_copy(this),
                screenpositions.len() as u32,
                ::core::mem::transmute(screenpositions.as_ptr()),
                ::windows::core::Array::<::windows::Foundation::Point>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeLocalCoordinatesWithRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        &self,
        screenpositions: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeLocalCoordinatesWithRect)(
                ::core::mem::transmute_copy(this),
                screenpositions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeScreenCoordinates<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        localpositions: Param0,
    ) -> ::windows::core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::PointInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeScreenCoordinates)(
                ::core::mem::transmute_copy(this),
                localpositions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::PointInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeScreenCoordinatesWithPoints(
        &self,
        localpositions: &[::windows::Foundation::Point],
    ) -> ::windows::core::Result<::windows::core::Array<::windows::Graphics::PointInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::Graphics::PointInt32> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeScreenCoordinatesWithPoints)(
                ::core::mem::transmute_copy(this),
                localpositions.len() as u32,
                ::core::mem::transmute(localpositions.as_ptr()),
                ::windows::core::Array::<::windows::Graphics::PointInt32>::set_abi_len(
                    &mut result__,
                ),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeScreenCoordinatesWithPointsAndBehavior(
        &self,
        localpositions: &[::windows::Foundation::Point],
        conversionbehavior: ContentCoordinateConversionBehavior,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::Graphics::PointInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::Graphics::PointInt32> =
                ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . ComputeScreenCoordinatesWithPointsAndBehavior ) ( :: core :: mem :: transmute_copy ( this ) , localpositions . len ( ) as u32 , :: core :: mem :: transmute ( localpositions . as_ptr ( ) ) , conversionbehavior , :: windows :: core :: Array :: < ::windows::Graphics:: PointInt32 > :: set_abi_len ( & mut result__ ) , & mut result__ as * mut _ as _ ) . and_then ( || result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ComputeScreenCoordinatesWithRect<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        localpositions: Param0,
    ) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::RectInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ComputeScreenCoordinatesWithRect)(
                ::core::mem::transmute_copy(this),
                localpositions.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::RectInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn CreateStateChangedDeferral(
        &self,
    ) -> ::windows::core::Result<ContentNotificationDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateStateChangedDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentNotificationDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn AutomationProviderRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                ContentIsland,
                ContentIslandAutomationProviderRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomationProviderRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveAutomationProviderRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAutomationProviderRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Closed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<ContentIsland, ContentIslandEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveClosed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveClosed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<ContentIsland, ContentIslandEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<ContentIsland> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ContentIsland>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn FindAllForCurrentThread(
    ) -> ::windows::core::Result<::windows::core::Array<ContentIsland>> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__: ::windows::core::Array<ContentIsland> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllForCurrentThread)(
                ::core::mem::transmute_copy(this),
                ::windows::core::Array::<ContentIsland>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn FindAllForCurrentThreadWithCompositor<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Composition::Compositor>,
    >(
        compositor: Param0,
    ) -> ::windows::core::Result<::windows::core::Array<ContentIsland>> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__: ::windows::core::Array<ContentIsland> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllForCurrentThreadWithCompositor)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                ::windows::core::Array::<ContentIsland>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn FindByVisual<'a, Param0: ::windows::core::IntoParam<'a, super::Composition::Visual>>(
        child: Param0,
    ) -> ::windows::core::Result<ContentIsland> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindByVisual)(
                ::core::mem::transmute_copy(this),
                child.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ContentIsland>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn GetFromId(id: u64) -> ::windows::core::Result<ContentIsland> {
        Self::IContentIslandStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFromId)(
                ::core::mem::transmute_copy(this),
                id,
                &mut result__,
            )
            .from_abi::<ContentIsland>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentIslandStatics<
        R,
        F: FnOnce(&IContentIslandStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentIsland, IContentIslandStatics> =
            ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentIsland {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentIsland {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIsland {}
impl ::core::fmt::Debug for ContentIsland {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIsland").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentIsland {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.ContentIsland;{7f09136b-51a7-5bc6-99b2-44b2415c8517})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentIsland {
    type Vtable = IContentIsland_Vtbl;
    const IID: ::windows::core::GUID = <IContentIsland as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentIsland {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIsland";
}
impl ::core::convert::From<ContentIsland> for ::windows::core::IUnknown {
    fn from(value: ContentIsland) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIsland> for ::windows::core::IUnknown {
    fn from(value: &ContentIsland) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIsland> for ::windows::core::IInspectable {
    fn from(value: ContentIsland) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIsland> for ::windows::core::IInspectable {
    fn from(value: &ContentIsland) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContentIsland> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentIsland) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContentIsland> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentIsland) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &ContentIsland {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContentIsland {}
unsafe impl ::core::marker::Sync for ContentIsland {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentIslandAutomationProviderRequestedEventArgs(::windows::core::IUnknown);
impl ContentIslandAutomationProviderRequestedEventArgs {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutomationProvider)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetAutomationProvider<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAutomationProvider)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetHandled)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ContentIslandAutomationProviderRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentIslandAutomationProviderRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIslandAutomationProviderRequestedEventArgs {}
impl ::core::fmt::Debug for ContentIslandAutomationProviderRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIslandAutomationProviderRequestedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentIslandAutomationProviderRequestedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Content.ContentIslandAutomationProviderRequestedEventArgs;{9fe24bed-2b9c-5137-887f-403c94841824})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentIslandAutomationProviderRequestedEventArgs {
    type Vtable = IContentIslandAutomationProviderRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IContentIslandAutomationProviderRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentIslandAutomationProviderRequestedEventArgs {
    const NAME: &'static str =
        "Microsoft.UI.Content.ContentIslandAutomationProviderRequestedEventArgs";
}
impl ::core::convert::From<ContentIslandAutomationProviderRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: ContentIslandAutomationProviderRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIslandAutomationProviderRequestedEventArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &ContentIslandAutomationProviderRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ContentIslandAutomationProviderRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ContentIslandAutomationProviderRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIslandAutomationProviderRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: ContentIslandAutomationProviderRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIslandAutomationProviderRequestedEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &ContentIslandAutomationProviderRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ContentIslandAutomationProviderRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentIslandAutomationProviderRequestedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIslandAutomationProviderRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ContentIslandAutomationProviderRequestedEventArgs {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentIslandEventArgs(::windows::core::IUnknown);
impl ContentIslandEventArgs {}
impl ::core::clone::Clone for ContentIslandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentIslandEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentIslandEventArgs {}
impl ::core::fmt::Debug for ContentIslandEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentIslandEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentIslandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.ContentIslandEventArgs;{b737c291-f728-5e90-bda4-a9e0efe7c0b9})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentIslandEventArgs {
    type Vtable = IContentIslandEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContentIslandEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentIslandEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentIslandEventArgs";
}
impl ::core::convert::From<ContentIslandEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContentIslandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIslandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContentIslandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentIslandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentIslandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIslandEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContentIslandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIslandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContentIslandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentIslandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentIslandEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIslandEventArgs {}
unsafe impl ::core::marker::Sync for ContentIslandEventArgs {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentNotificationDeferral(::windows::core::IUnknown);
impl ContentNotificationDeferral {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this))
                .ok()
        }
    }
}
impl ::core::clone::Clone for ContentNotificationDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentNotificationDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentNotificationDeferral {}
impl ::core::fmt::Debug for ContentNotificationDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentNotificationDeferral")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentNotificationDeferral {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Content.ContentNotificationDeferral;{d6daa59c-f356-5604-b07b-7edb23f2eb22})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentNotificationDeferral {
    type Vtable = IContentNotificationDeferral_Vtbl;
    const IID: ::windows::core::GUID =
        <IContentNotificationDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentNotificationDeferral {
    const NAME: &'static str = "Microsoft.UI.Content.ContentNotificationDeferral";
}
impl ::core::convert::From<ContentNotificationDeferral> for ::windows::core::IUnknown {
    fn from(value: ContentNotificationDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentNotificationDeferral> for ::windows::core::IUnknown {
    fn from(value: &ContentNotificationDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentNotificationDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ContentNotificationDeferral
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentNotificationDeferral> for ::windows::core::IInspectable {
    fn from(value: ContentNotificationDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentNotificationDeferral> for ::windows::core::IInspectable {
    fn from(value: &ContentNotificationDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ContentNotificationDeferral
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentNotificationDeferral
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentNotificationDeferral {}
unsafe impl ::core::marker::Sync for ContentNotificationDeferral {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentSite(::windows::core::IUnknown);
impl ContentSite {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ActualSize(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetActualSize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector2>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetActualSize)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compositor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Composition::Compositor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::Dispatching::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Input_Experimental\"`*"]
    #[cfg(feature = "UI_Input_Experimental")]
    pub fn InputSite(&self) -> ::windows::core::Result<super::Input::Experimental::ExpInputSite> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputSite)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Input::Experimental::ExpInputSite>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSiteVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSiteVisible)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetIsSiteVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsSiteVisible)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn OverrideScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverrideScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOverrideScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ParentScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetParentScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetParentScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn PhysicalSize(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetPhysicalSize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::SizeInt32>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPhysicalSize)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RasterizationScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RasterizationScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RequestedSize(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedSize)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SiteVisual(&self) -> ::windows::core::Result<super::Composition::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SiteVisual)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Composition::Visual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowBridge(&self) -> ::windows::core::Result<IContentWindowBridge> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IContentWindowBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetWindowBridge<'a, Param0: ::windows::core::IntoParam<'a, IContentWindowBridge>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetWindowBridge)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn TryCreateContentDeferral(&self) -> ::windows::core::Result<ContentNotificationDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateContentDeferral)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentNotificationDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ContentRequestsChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<ContentSite, ContentSiteEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentRequestsChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveContentRequestsChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveContentRequestsChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for ContentSite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentSite {}
impl ::core::fmt::Debug for ContentSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentSite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentSite {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.ContentSite;{6cdc2763-c226-5913-b90b-ec9ff4c34aec})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentSite {
    type Vtable = IContentSite_Vtbl;
    const IID: ::windows::core::GUID = <IContentSite as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentSite {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSite";
}
impl ::core::convert::From<ContentSite> for ::windows::core::IUnknown {
    fn from(value: ContentSite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentSite> for ::windows::core::IUnknown {
    fn from(value: &ContentSite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentSite> for ::windows::core::IInspectable {
    fn from(value: ContentSite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentSite> for ::windows::core::IInspectable {
    fn from(value: &ContentSite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContentSite> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ContentSite) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContentSite> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContentSite) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &ContentSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContentSite {}
unsafe impl ::core::marker::Sync for ContentSite {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct ContentSiteEventArgs(::windows::core::IUnknown);
impl ContentSiteEventArgs {}
impl ::core::clone::Clone for ContentSiteEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentSiteEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentSiteEventArgs {}
impl ::core::fmt::Debug for ContentSiteEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentSiteEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentSiteEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.ContentSiteEventArgs;{7853c397-1667-596d-9b7d-dc6941ded8a4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentSiteEventArgs {
    type Vtable = IContentSiteEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContentSiteEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentSiteEventArgs {
    const NAME: &'static str = "Microsoft.UI.Content.ContentSiteEventArgs";
}
impl ::core::convert::From<ContentSiteEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContentSiteEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentSiteEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContentSiteEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentSiteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentSiteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentSiteEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContentSiteEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentSiteEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContentSiteEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentSiteEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ContentSiteEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentSiteEventArgs {}
unsafe impl ::core::marker::Sync for ContentSiteEventArgs {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ContentSizePolicy(pub i32);
impl ContentSizePolicy {
    pub const None: Self = Self(0i32);
    pub const ResizeContentToParentWindow: Self = Self(1i32);
    pub const ResizeParentWindowToContent: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentSizePolicy {}
impl ::core::clone::Clone for ContentSizePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ContentSizePolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ContentSizePolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for ContentSizePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentSizePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ContentSizePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Content.ContentSizePolicy;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct CoreWindowSiteBridge(::windows::core::IUnknown);
impl CoreWindowSiteBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn OverrideScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverrideScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOverrideScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ContentIsland>>(
        &self,
        content: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Connect)(
                ::core::mem::transmute_copy(this),
                content.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn TryCreatePopupSiteBridge(&self) -> ::windows::core::Result<PopupWindowSiteBridge> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreatePopupSiteBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PopupWindowSiteBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowBridge(&self) -> ::windows::core::Result<CoreWindowTopLevelWindowBridge> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CoreWindowTopLevelWindowBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Create<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Composition::Compositor>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Core::CoreWindow>,
    >(
        compositor: Param0,
        corewindow: Param1,
    ) -> ::windows::core::Result<CoreWindowSiteBridge> {
        Self::ICoreWindowSiteBridgeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                corewindow.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CoreWindowSiteBridge>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ICoreWindowSiteBridgeStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreWindowSiteBridgeStatics<
        R,
        F: FnOnce(&ICoreWindowSiteBridgeStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CoreWindowSiteBridge,
            ICoreWindowSiteBridgeStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CoreWindowSiteBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWindowSiteBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowSiteBridge {}
impl ::core::fmt::Debug for CoreWindowSiteBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowSiteBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowSiteBridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.CoreWindowSiteBridge;{9adb8288-eb40-5f4c-b08e-78077a2eae27})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWindowSiteBridge {
    type Vtable = ICoreWindowSiteBridge_Vtbl;
    const IID: ::windows::core::GUID = <ICoreWindowSiteBridge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.CoreWindowSiteBridge";
}
impl ::core::convert::From<CoreWindowSiteBridge> for ::windows::core::IUnknown {
    fn from(value: CoreWindowSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindowSiteBridge> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWindowSiteBridge> for ::windows::core::IInspectable {
    fn from(value: CoreWindowSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindowSiteBridge> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWindowSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CoreWindowSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWindowSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CoreWindowSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CoreWindowSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWindowSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for &CoreWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::core::convert::TryInto::<IContentSiteBridge>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreWindowSiteBridge {}
unsafe impl ::core::marker::Sync for CoreWindowSiteBridge {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct CoreWindowTopLevelWindowBridge(::windows::core::IUnknown);
impl CoreWindowTopLevelWindowBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn CurrentOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn NativeOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NativeOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SettingsChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveSettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveSettingsChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThemeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveThemeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for CoreWindowTopLevelWindowBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CoreWindowTopLevelWindowBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CoreWindowTopLevelWindowBridge {}
impl ::core::fmt::Debug for CoreWindowTopLevelWindowBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CoreWindowTopLevelWindowBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CoreWindowTopLevelWindowBridge {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Content.CoreWindowTopLevelWindowBridge;{22c454e8-3a91-574e-9b39-8d50573ee3f2})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CoreWindowTopLevelWindowBridge {
    type Vtable = IContentWindowBridge_Vtbl;
    const IID: ::windows::core::GUID = <IContentWindowBridge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CoreWindowTopLevelWindowBridge {
    const NAME: &'static str = "Microsoft.UI.Content.CoreWindowTopLevelWindowBridge";
}
impl ::core::convert::From<CoreWindowTopLevelWindowBridge> for ::windows::core::IUnknown {
    fn from(value: CoreWindowTopLevelWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindowTopLevelWindowBridge> for ::windows::core::IUnknown {
    fn from(value: &CoreWindowTopLevelWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CoreWindowTopLevelWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CoreWindowTopLevelWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CoreWindowTopLevelWindowBridge> for ::windows::core::IInspectable {
    fn from(value: CoreWindowTopLevelWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CoreWindowTopLevelWindowBridge> for ::windows::core::IInspectable {
    fn from(value: &CoreWindowTopLevelWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CoreWindowTopLevelWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CoreWindowTopLevelWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CoreWindowTopLevelWindowBridge> for IContentWindowBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: CoreWindowTopLevelWindowBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CoreWindowTopLevelWindowBridge> for IContentWindowBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: &CoreWindowTopLevelWindowBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentWindowBridge> for CoreWindowTopLevelWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentWindowBridge> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentWindowBridge> for &CoreWindowTopLevelWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentWindowBridge> {
        ::core::convert::TryInto::<IContentWindowBridge>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CoreWindowTopLevelWindowBridge {}
unsafe impl ::core::marker::Sync for CoreWindowTopLevelWindowBridge {}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct DesktopChildSiteBridge(::windows::core::IUnknown);
impl DesktopChildSiteBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn OverrideScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverrideScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOverrideScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ContentIsland>>(
        &self,
        content: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Connect)(
                ::core::mem::transmute_copy(this),
                content.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn TryCreatePopupSiteBridge(&self) -> ::windows::core::Result<PopupWindowSiteBridge> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreatePopupSiteBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PopupWindowSiteBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ChildWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChildWindowId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ResizePolicy(&self) -> ::windows::core::Result<ContentSizePolicy> {
        let this = self;
        unsafe {
            let mut result__: ContentSizePolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResizePolicy)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentSizePolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetResizePolicy(&self, value: ContentSizePolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetResizePolicy)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowBridge(&self) -> ::windows::core::Result<ContentAppWindowBridge> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentAppWindowBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Input_Experimental\"`*"]
    #[cfg(feature = "UI_Input_Experimental")]
    pub fn NavigateFocus<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Input::Experimental::ExpFocusNavigationRequest>,
    >(
        &self,
        request: Param0,
    ) -> ::windows::core::Result<super::Input::Experimental::ExpFocusNavigationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigateFocus)(
                ::core::mem::transmute_copy(this),
                request.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::Input::Experimental::ExpFocusNavigationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn PreTranslateKeyboardMessage(
        &self,
        windowhandle: u64,
        keyboardmessage: u32,
        virtualkey: u32,
        keyinfo: u64,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreTranslateKeyboardMessage)(
                ::core::mem::transmute_copy(this),
                windowhandle,
                keyboardmessage,
                virtualkey,
                keyinfo,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Input_Experimental\"`*"]
    #[cfg(feature = "UI_Input_Experimental")]
    pub fn TakeFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                DesktopChildSiteBridge,
                super::Input::Experimental::ExpNavigateFocusRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TakeFocusRequested)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveTakeFocusRequested<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveTakeFocusRequested)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Create<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Composition::Compositor>,
        Param1: ::windows::core::IntoParam<'a, super::WindowId>,
    >(
        compositor: Param0,
        parentwindowid: Param1,
    ) -> ::windows::core::Result<DesktopChildSiteBridge> {
        Self::IDesktopChildSiteBridgeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                parentwindowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<DesktopChildSiteBridge>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IDesktopChildSiteBridgeStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesktopChildSiteBridgeStatics<
        R,
        F: FnOnce(&IDesktopChildSiteBridgeStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DesktopChildSiteBridge,
            IDesktopChildSiteBridgeStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DesktopChildSiteBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DesktopChildSiteBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DesktopChildSiteBridge {}
impl ::core::fmt::Debug for DesktopChildSiteBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DesktopChildSiteBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DesktopChildSiteBridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.DesktopChildSiteBridge;{d07f33bd-2527-5bd8-8d84-9e7320dd3fde})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DesktopChildSiteBridge {
    type Vtable = IDesktopChildSiteBridge_Vtbl;
    const IID: ::windows::core::GUID = <IDesktopChildSiteBridge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DesktopChildSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.DesktopChildSiteBridge";
}
impl ::core::convert::From<DesktopChildSiteBridge> for ::windows::core::IUnknown {
    fn from(value: DesktopChildSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopChildSiteBridge> for ::windows::core::IUnknown {
    fn from(value: &DesktopChildSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DesktopChildSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DesktopChildSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DesktopChildSiteBridge> for ::windows::core::IInspectable {
    fn from(value: DesktopChildSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DesktopChildSiteBridge> for ::windows::core::IInspectable {
    fn from(value: &DesktopChildSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DesktopChildSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DesktopChildSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DesktopChildSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopChildSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopChildSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopChildSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for DesktopChildSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &DesktopChildSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DesktopChildSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: DesktopChildSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DesktopChildSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: &DesktopChildSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for DesktopChildSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for &DesktopChildSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::core::convert::TryInto::<IContentSiteBridge>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DesktopChildSiteBridge {}
unsafe impl ::core::marker::Sync for DesktopChildSiteBridge {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentAppWindowBridgeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentAppWindowBridgeStatics {
    type Vtable = IContentAppWindowBridgeStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84b9cd9c_e822_5f12_b21d_31a409b804f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentAppWindowBridgeStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        childwindowid: super::WindowId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentExternalOutputLink(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentExternalOutputLink {
    type Vtable = IContentExternalOutputLink_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6de3ea12_fc7e_5142_a8b6_46f01758f40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentExternalOutputLink_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub PlacementVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    PlacementVisual: usize,
    pub BackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentExternalOutputLinkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentExternalOutputLinkStatics {
    type Vtable = IContentExternalOutputLinkStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb758f401_833e_587d_b0cd_a3934eba3721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentExternalOutputLinkStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Create: usize,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIsland(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIsland {
    type Vtable = IContentIsland_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7f09136b_51a7_5bc6_99b2_44b2415c8517);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIsland_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    pub AppData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAppData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CustomProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input_Experimental")]
    pub InputSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Experimental"))]
    InputSite: usize,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsHitTestAlways: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsHitTestAlways: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub RequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    pub SetRequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Root: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Root: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetRoot: usize,
    pub WindowBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ComputeLocalCoordinatesWithPointInt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenpositions: ::windows::Graphics::PointInt32,
        result__: *mut ::windows::Foundation::Point,
    )
        -> ::windows::core::HRESULT,
    pub ComputeLocalCoordinatesWithPointInts: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenPositions_array_size: u32,
        screenpositions: *const ::windows::Graphics::PointInt32,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Foundation::Point,
    )
        -> ::windows::core::HRESULT,
    pub ComputeLocalCoordinatesWithRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        screenpositions: ::windows::Graphics::RectInt32,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub ComputeScreenCoordinates: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localpositions: ::windows::Foundation::Point,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub ComputeScreenCoordinatesWithPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localPositions_array_size: u32,
        localpositions: *const ::windows::Foundation::Point,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Graphics::PointInt32,
    )
        -> ::windows::core::HRESULT,
    pub ComputeScreenCoordinatesWithPointsAndBehavior:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            localPositions_array_size: u32,
            localpositions: *const ::windows::Foundation::Point,
            conversionbehavior: ContentCoordinateConversionBehavior,
            result_size__: *mut u32,
            result__: *mut *mut ::windows::Graphics::PointInt32,
        ) -> ::windows::core::HRESULT,
    pub ComputeScreenCoordinatesWithRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localpositions: ::windows::Foundation::Rect,
        result__: *mut ::windows::Graphics::RectInt32,
    )
        -> ::windows::core::HRESULT,
    pub CreateStateChangedDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub AutomationProviderRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAutomationProviderRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub Closed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIslandAutomationProviderRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIslandAutomationProviderRequestedEventArgs {
    type Vtable = IContentIslandAutomationProviderRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9fe24bed_2b9c_5137_887f_403c94841824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandAutomationProviderRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AutomationProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAutomationProvider: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIslandEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIslandEventArgs {
    type Vtable = IContentIslandEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb737c291_f728_5e90_bda4_a9e0efe7c0b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIslandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIslandFactory {
    type Vtable = IContentIslandFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x82383f52_e81a_5ec9_a954_bac8a931ba7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIslandStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIslandStatics {
    type Vtable = IContentIslandStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdc482863_efb3_5aad_9df5_d78dd0c7fb72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIslandStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Create: usize,
    pub FindAllForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub FindAllForCurrentThreadWithCompositor:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            compositor: ::windows::core::RawPtr,
            result_size__: *mut u32,
            result__: *mut *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    FindAllForCurrentThreadWithCompositor: usize,
    #[cfg(feature = "UI_Composition")]
    pub FindByVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    FindByVisual: usize,
    pub GetFromId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: u64,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentNotificationDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentNotificationDeferral {
    type Vtable = IContentNotificationDeferral_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd6daa59c_f356_5604_b07b_7edb23f2eb22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentNotificationDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Complete:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentSite(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentSite {
    type Vtable = IContentSite_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cdc2763_c226_5913_b90b_ec9ff4c34aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSite_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    pub SetActualSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    #[cfg(feature = "UI_Input_Experimental")]
    pub InputSite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Experimental"))]
    InputSite: usize,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsSiteVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub OverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetOverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub ParentScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetParentScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub PhysicalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub SetPhysicalSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub RequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub SiteVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SiteVisual: usize,
    pub WindowBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetWindowBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryCreateContentDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ContentRequestsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveContentRequestsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct IContentSiteBridge(::windows::core::IUnknown);
impl IContentSiteBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn OverrideScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverrideScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOverrideScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ContentIsland>>(
        &self,
        content: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Connect)(
                ::core::mem::transmute_copy(this),
                content.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn TryCreatePopupSiteBridge(&self) -> ::windows::core::Result<PopupWindowSiteBridge> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreatePopupSiteBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PopupWindowSiteBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::convert::From<IContentSiteBridge> for ::windows::core::IUnknown {
    fn from(value: IContentSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentSiteBridge> for ::windows::core::IUnknown {
    fn from(value: &IContentSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContentSiteBridge> for ::windows::core::IInspectable {
    fn from(value: IContentSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentSiteBridge> for ::windows::core::IInspectable {
    fn from(value: &IContentSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContentSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IContentSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContentSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContentSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &IContentSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IContentSiteBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContentSiteBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentSiteBridge {}
impl ::core::fmt::Debug for IContentSiteBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentSiteBridge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContentSiteBridge {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{908ae78c-7dcc-5ea9-83e1-435c7fddf646}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContentSiteBridge {
    type Vtable = IContentSiteBridge_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x908ae78c_7dcc_5ea9_83e1_435c7fddf646);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteBridge_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsConnected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub OverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetOverrideScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub Connect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        content: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryCreatePopupSiteBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentSiteEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentSiteEventArgs {
    type Vtable = IContentSiteEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7853c397_1667_596d_9b7d_dc6941ded8a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentSiteFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentSiteFactory {
    type Vtable = IContentSiteFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x72fb98d5_b28a_57f1_91fa_24c014a342c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentSiteFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct IContentWindowBridge(::windows::core::IUnknown);
impl IContentWindowBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn CurrentOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__: super::DisplayId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DisplayId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn DisplayScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn NativeOrientation(&self) -> ::windows::core::Result<ContentDisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__: ContentDisplayOrientations = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NativeOrientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ContentDisplayOrientations>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn WindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SettingsChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveSettingsChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveSettingsChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn StateChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStateChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn ThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                IContentWindowBridge,
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThemeChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn RemoveThemeChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveThemeChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<IContentWindowBridge> for ::windows::core::IUnknown {
    fn from(value: IContentWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentWindowBridge> for ::windows::core::IUnknown {
    fn from(value: &IContentWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContentWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContentWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContentWindowBridge> for ::windows::core::IInspectable {
    fn from(value: IContentWindowBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContentWindowBridge> for ::windows::core::IInspectable {
    fn from(value: &IContentWindowBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContentWindowBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IContentWindowBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContentWindowBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContentWindowBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentWindowBridge {}
impl ::core::fmt::Debug for IContentWindowBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentWindowBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContentWindowBridge {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{22c454e8-3a91-574e-9b39-8d50573ee3f2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IContentWindowBridge {
    type Vtable = IContentWindowBridge_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x22c454e8_3a91_574e_9b39_8d50573ee3f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentWindowBridge_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurrentOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentDisplayOrientations,
    ) -> ::windows::core::HRESULT,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows::core::HRESULT,
    pub DisplayScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub NativeOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentDisplayOrientations,
    ) -> ::windows::core::HRESULT,
    pub WindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub SettingsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveSettingsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ThemeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveThemeChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowSiteBridge(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowSiteBridge {
    type Vtable = ICoreWindowSiteBridge_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9adb8288_eb40_5f4c_b08e_78077a2eae27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowSiteBridge_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WindowBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreWindowSiteBridgeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreWindowSiteBridgeStatics {
    type Vtable = ICoreWindowSiteBridgeStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfbc56793_11f2_5754_ad70_30e82c07fa35);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreWindowSiteBridgeStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: ::windows::core::RawPtr,
        corewindow: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Create: usize,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopChildSiteBridge(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopChildSiteBridge {
    type Vtable = IDesktopChildSiteBridge_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd07f33bd_2527_5bd8_8d84_9e7320dd3fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopChildSiteBridge_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ChildWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub ResizePolicy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ContentSizePolicy,
    ) -> ::windows::core::HRESULT,
    pub SetResizePolicy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ContentSizePolicy,
    ) -> ::windows::core::HRESULT,
    pub WindowBridge: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input_Experimental")]
    pub NavigateFocus: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        request: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Experimental"))]
    NavigateFocus: usize,
    pub PreTranslateKeyboardMessage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowhandle: u64,
        keyboardmessage: u32,
        virtualkey: u32,
        keyinfo: u64,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input_Experimental")]
    pub TakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input_Experimental"))]
    TakeFocusRequested: usize,
    pub RemoveTakeFocusRequested: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDesktopChildSiteBridgeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDesktopChildSiteBridgeStatics {
    type Vtable = IDesktopChildSiteBridgeStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84f2d231_6771_501e_a1a9_5c0dde844c45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopChildSiteBridgeStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: ::windows::core::RawPtr,
        parentwindowid: super::WindowId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Create: usize,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupWindowSiteBridge(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupWindowSiteBridge {
    type Vtable = IPopupWindowSiteBridge_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf038cf34_ab18_5bfa_a8f1_734d5c0ca1e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupWindowSiteBridge_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Anchored: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAnchored: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub PopupWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveAndResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupWindowSiteBridgeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupWindowSiteBridgeStatics {
    type Vtable = IPopupWindowSiteBridgeStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3e316c2f_417d_5496_b68c_95750f571c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupWindowSiteBridgeStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Content\"`*"]
#[repr(transparent)]
pub struct PopupWindowSiteBridge(::windows::core::IUnknown);
impl PopupWindowSiteBridge {
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn OverrideScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OverrideScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetOverrideScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetOverrideScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ContentIsland>>(
        &self,
        content: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Connect)(
                ::core::mem::transmute_copy(this),
                content.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn TryCreatePopupSiteBridge(&self) -> ::windows::core::Result<PopupWindowSiteBridge> {
        let this = &::windows::core::Interface::cast::<IContentSiteBridge>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreatePopupSiteBridge)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PopupWindowSiteBridge>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Anchored(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Anchored)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn SetAnchored(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAnchored)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn PopupWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__: super::WindowId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PopupWindowId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::WindowId>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Hide)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn MoveAndResize<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Graphics::RectInt32>,
    >(
        &self,
        rect: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveAndResize)(
                ::core::mem::transmute_copy(this),
                rect.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn Show(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Show)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Content\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPopupWindowSiteBridgeStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPopupWindowSiteBridgeStatics<
        R,
        F: FnOnce(&IPopupWindowSiteBridgeStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PopupWindowSiteBridge,
            IPopupWindowSiteBridgeStatics,
        > = ::windows::core::FactoryCache::from_library(b"Microsoft.UI.Input.dll\0");
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopupWindowSiteBridge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopupWindowSiteBridge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopupWindowSiteBridge {}
impl ::core::fmt::Debug for PopupWindowSiteBridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupWindowSiteBridge")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PopupWindowSiteBridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Content.PopupWindowSiteBridge;{f038cf34-ab18-5bfa-a8f1-734d5c0ca1e6})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PopupWindowSiteBridge {
    type Vtable = IPopupWindowSiteBridge_Vtbl;
    const IID: ::windows::core::GUID = <IPopupWindowSiteBridge as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PopupWindowSiteBridge {
    const NAME: &'static str = "Microsoft.UI.Content.PopupWindowSiteBridge";
}
impl ::core::convert::From<PopupWindowSiteBridge> for ::windows::core::IUnknown {
    fn from(value: PopupWindowSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupWindowSiteBridge> for ::windows::core::IUnknown {
    fn from(value: &PopupWindowSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopupWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopupWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupWindowSiteBridge> for ::windows::core::IInspectable {
    fn from(value: PopupWindowSiteBridge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupWindowSiteBridge> for ::windows::core::IInspectable {
    fn from(value: &PopupWindowSiteBridge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopupWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PopupWindowSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PopupWindowSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PopupWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PopupWindowSiteBridge> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PopupWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for PopupWindowSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &PopupWindowSiteBridge
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PopupWindowSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: PopupWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PopupWindowSiteBridge> for IContentSiteBridge {
    type Error = ::windows::core::Error;
    fn try_from(value: &PopupWindowSiteBridge) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for PopupWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContentSiteBridge> for &PopupWindowSiteBridge {
    fn into_param(self) -> ::windows::core::Param<'a, IContentSiteBridge> {
        ::core::convert::TryInto::<IContentSiteBridge>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PopupWindowSiteBridge {}
unsafe impl ::core::marker::Sync for PopupWindowSiteBridge {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
