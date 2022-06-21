#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedAcceptVisualSource(::windows::core::IUnknown);
impl AnimatedAcceptVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedAcceptVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedAcceptVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedAcceptVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedAcceptVisualSource {}
impl ::core::fmt::Debug for AnimatedAcceptVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedAcceptVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedAcceptVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedAcceptVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedAcceptVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource";
}
impl ::core::convert::From<AnimatedAcceptVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedAcceptVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedAcceptVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedAcceptVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedAcceptVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedAcceptVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedAcceptVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedBackVisualSource(::windows::core::IUnknown);
impl AnimatedBackVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedBackVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedBackVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedBackVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedBackVisualSource {}
impl ::core::fmt::Debug for AnimatedBackVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedBackVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedBackVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedBackVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedBackVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource";
}
impl ::core::convert::From<AnimatedBackVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedBackVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedBackVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedBackVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedBackVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedBackVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedBackVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource> for AnimatedBackVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedBackVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedBackVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedChevronDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronDownSmallVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedChevronDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedChevronDownSmallVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronRightDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronRightDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedChevronRightDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronRightDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronRightDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronRightDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronRightDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronRightDownSmallVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronRightDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedChevronRightDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronRightDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronRightDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronRightDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronRightDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronRightDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedChevronUpDownSmallVisualSource(::windows::core::IUnknown);
impl AnimatedChevronUpDownSmallVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedChevronUpDownSmallVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedChevronUpDownSmallVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedChevronUpDownSmallVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedChevronUpDownSmallVisualSource {}
impl ::core::fmt::Debug for AnimatedChevronUpDownSmallVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedChevronUpDownSmallVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronUpDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedChevronUpDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedChevronUpDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronUpDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronUpDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedChevronUpDownSmallVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedChevronUpDownSmallVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedChevronUpDownSmallVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedFindVisualSource(::windows::core::IUnknown);
impl AnimatedFindVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedFindVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedFindVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedFindVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedFindVisualSource {}
impl ::core::fmt::Debug for AnimatedFindVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedFindVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedFindVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedFindVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedFindVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource";
}
impl ::core::convert::From<AnimatedFindVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedFindVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedFindVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedFindVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedFindVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedFindVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedFindVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource> for AnimatedFindVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedFindVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedFindVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedGlobalNavigationButtonVisualSource(::windows::core::IUnknown);
impl AnimatedGlobalNavigationButtonVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedGlobalNavigationButtonVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedGlobalNavigationButtonVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedGlobalNavigationButtonVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedGlobalNavigationButtonVisualSource {}
impl ::core::fmt::Debug for AnimatedGlobalNavigationButtonVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedGlobalNavigationButtonVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedGlobalNavigationButtonVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedGlobalNavigationButtonVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedGlobalNavigationButtonVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource";
}
impl ::core::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedGlobalNavigationButtonVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedGlobalNavigationButtonVisualSource {}
#[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
#[repr(transparent)]
pub struct AnimatedSettingsVisualSource(::windows::core::IUnknown);
impl AnimatedSettingsVisualSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnimatedSettingsVisualSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateAnimatedVisual)(
                ::windows::core::Interface::as_raw(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) . Markers ) ( :: windows :: core :: Interface :: as_raw ( this ) , result__ . as_mut_ptr ( ) ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Controls_AnimatedVisuals\"`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetColorProperty)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for AnimatedSettingsVisualSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnimatedSettingsVisualSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnimatedSettingsVisualSource {}
impl ::core::fmt::Debug for AnimatedSettingsVisualSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimatedSettingsVisualSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedSettingsVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnimatedSettingsVisualSource {
    type Vtable = super::IAnimatedVisualSource_Vtbl;
    const IID: ::windows::core::GUID =
        <super::IAnimatedVisualSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnimatedSettingsVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource";
}
impl ::core::convert::From<AnimatedSettingsVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedSettingsVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnimatedSettingsVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedSettingsVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource2> {
        ::core::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AnimatedSettingsVisualSource {}
unsafe impl ::core::marker::Sync for AnimatedSettingsVisualSource {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
