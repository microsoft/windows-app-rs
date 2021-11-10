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
pub struct AnimatedAcceptVisualSource(pub ::windows::core::IInspectable);
impl AnimatedAcceptVisualSource {
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
            AnimatedAcceptVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedAcceptVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedAcceptVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedAcceptVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource";
}
impl ::core::convert::From<AnimatedAcceptVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedAcceptVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedAcceptVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedAcceptVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedAcceptVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedBackVisualSource(pub ::windows::core::IInspectable);
impl AnimatedBackVisualSource {
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
            AnimatedBackVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedBackVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedBackVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedBackVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource";
}
impl ::core::convert::From<AnimatedBackVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedBackVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedBackVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedBackVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedBackVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedBackVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedBackVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedBackVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource> for AnimatedBackVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedChevronDownSmallVisualSource(pub ::windows::core::IInspectable);
impl AnimatedChevronDownSmallVisualSource {
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
            AnimatedChevronDownSmallVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedChevronDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedChevronDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedChevronDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedChevronDownSmallVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedChevronDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedChevronDownSmallVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronDownSmallVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedChevronRightDownSmallVisualSource(pub ::windows::core::IInspectable);
impl AnimatedChevronRightDownSmallVisualSource {
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
            AnimatedChevronRightDownSmallVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronRightDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedChevronRightDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedChevronRightDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedChevronUpDownSmallVisualSource(pub ::windows::core::IInspectable);
impl AnimatedChevronUpDownSmallVisualSource {
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
            AnimatedChevronUpDownSmallVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedChevronUpDownSmallVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedChevronUpDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedChevronUpDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource";
}
impl ::core::convert::From<AnimatedChevronUpDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedChevronUpDownSmallVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedFindVisualSource(pub ::windows::core::IInspectable);
impl AnimatedFindVisualSource {
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
            AnimatedFindVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedFindVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedFindVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedFindVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource";
}
impl ::core::convert::From<AnimatedFindVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedFindVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedFindVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnimatedFindVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedFindVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedFindVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedFindVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedFindVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource> for AnimatedFindVisualSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedGlobalNavigationButtonVisualSource(pub ::windows::core::IInspectable);
impl AnimatedGlobalNavigationButtonVisualSource {
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
            AnimatedGlobalNavigationButtonVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedGlobalNavigationButtonVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedGlobalNavigationButtonVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedGlobalNavigationButtonVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource";
}
impl ::core::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IUnknown
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::core::IInspectable
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnimatedSettingsVisualSource(pub ::windows::core::IInspectable);
impl AnimatedSettingsVisualSource {
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
            AnimatedSettingsVisualSource,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
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
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    pub fn Markers(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView<::windows::core::HSTRING, f64>,
    > {
        let this = &::windows::core::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: core :: HSTRING , f64 > > ( result__ )
        }
    }
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AnimatedSettingsVisualSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::core::Interface for AnimatedSettingsVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x294765c3_70e3_555c_9657_01fc4051169d);
}
impl ::windows::core::RuntimeName for AnimatedSettingsVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource";
}
impl ::core::convert::From<AnimatedSettingsVisualSource> for ::windows::core::IUnknown {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnimatedSettingsVisualSource> for ::windows::core::IUnknown {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnimatedSettingsVisualSource> for ::windows::core::IInspectable {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnimatedSettingsVisualSource> for ::windows::core::IInspectable {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimatedVisualSource> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
