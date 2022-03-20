#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    clashing_extern_declarations,
    clippy::all
)]
#[cfg(feature = "UI_Input_Interop")]
pub mod Interop;
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CrossSlideThresholds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CrossSlideThresholds")
            .field("SelectionStart", &self.SelectionStart)
            .field("SpeedBumpStart", &self.SpeedBumpStart)
            .field("SpeedBumpEnd", &self.SpeedBumpEnd)
            .field("RearrangeStart", &self.RearrangeStart)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CrossSlideThresholds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CrossSlideThresholds {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<CrossSlideThresholds>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for CrossSlideThresholds {}
impl ::core::default::Default for CrossSlideThresholds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct CrossSlidingEventArgs(::windows::core::IUnknown);
impl CrossSlidingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlidingState(&self) -> ::windows::core::Result<CrossSlidingState> {
        let this = self;
        unsafe {
            let mut result__: CrossSlidingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlidingState)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CrossSlidingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for CrossSlidingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CrossSlidingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CrossSlidingEventArgs {}
impl ::core::fmt::Debug for CrossSlidingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.CrossSlidingEventArgs;{7679641f-ba9f-543c-a7c8-6229a98f89ef})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICrossSlidingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CrossSlidingEventArgs";
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CrossSlidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CrossSlidingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CrossSlidingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CrossSlidingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CrossSlidingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CrossSlidingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CrossSlidingEventArgs {}
unsafe impl ::core::marker::Sync for CrossSlidingEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CrossSlidingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CrossSlidingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for CrossSlidingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CrossSlidingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CrossSlidingState {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.CrossSlidingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct DraggingEventArgs(::windows::core::IUnknown);
impl DraggingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DraggingState(&self) -> ::windows::core::Result<DraggingState> {
        let this = self;
        unsafe {
            let mut result__: DraggingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DraggingState)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<DraggingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for DraggingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DraggingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DraggingEventArgs {}
impl ::core::fmt::Debug for DraggingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.DraggingEventArgs;{3efb1b75-3d3b-550e-963d-0828ca76128a})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDraggingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DraggingEventArgs";
}
impl ::core::convert::From<DraggingEventArgs> for ::windows::core::IUnknown {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DraggingEventArgs> for ::windows::core::IInspectable {
    fn from(value: DraggingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DraggingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DraggingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DraggingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DraggingEventArgs {}
unsafe impl ::core::marker::Sync for DraggingEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DraggingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DraggingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for DraggingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DraggingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DraggingState {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.DraggingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct GestureRecognizer(::windows::core::IUnknown);
impl GestureRecognizer {
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
            GestureRecognizer,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn AutoProcessInertia(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoProcessInertia)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetAutoProcessInertia(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAutoProcessInertia)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideExact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideExact)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideExact(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCrossSlideExact)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideHorizontally(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideHorizontally)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCrossSlideHorizontally)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSlideThresholds(&self) -> ::windows::core::Result<CrossSlideThresholds> {
        let this = self;
        unsafe {
            let mut result__: CrossSlideThresholds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSlideThresholds)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CrossSlideThresholds>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCrossSlideThresholds<
        'a,
        Param0: ::windows::core::IntoParam<'a, CrossSlideThresholds>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCrossSlideThresholds)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GestureSettings(&self) -> ::windows::core::Result<GestureSettings> {
        let this = self;
        unsafe {
            let mut result__: GestureSettings = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GestureSettings)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<GestureSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetGestureSettings(&self, value: GestureSettings) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetGestureSettings)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInertial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInertial)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PivotCenter(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PivotCenter)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetPivotCenter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPivotCenter)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PivotRadius)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetPivotRadius(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPivotRadius)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaExpansionDeceleration)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaExpansionDeceleration)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaExpansion(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaExpansion)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaExpansion(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaExpansion)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaRotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaRotationAngle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaRotationAngle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaRotationDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaRotationDeceleration)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaRotationDeceleration)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaTranslationDeceleration(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaTranslationDeceleration)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaTranslationDeceleration)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn InertiaTranslationDisplacement(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InertiaTranslationDisplacement)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInertiaTranslationDisplacement)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationExact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationExact)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetManipulationExact(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetManipulationExact)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn MouseWheelParameters(&self) -> ::windows::core::Result<MouseWheelParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MouseWheelParameters)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MouseWheelParameters>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ShowGestureFeedback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowGestureFeedback)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetShowGestureFeedback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetShowGestureFeedback)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CanBeDoubleTap<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanBeDoubleTap)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CompleteGesture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).CompleteGesture)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessDownEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ProcessDownEvent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessMoveEvents<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Collections::IVector<PointerPoint>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ProcessMoveEvents)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessMouseWheelEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ProcessMouseWheelEvent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                isshiftkeydown,
                iscontrolkeydown,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessInertia(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ProcessInertia)(::core::mem::transmute_copy(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ProcessUpEvent<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ProcessUpEvent)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Tapped<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tapped)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveTapped)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RightTapped<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RightTapped)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveRightTapped<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRightTapped)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Holding<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Holding)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveHolding<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveHolding)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Dragging<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dragging)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveDragging<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDragging)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationStarted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationStartedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationStarted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveManipulationStarted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationStarted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationUpdated<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationUpdatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationUpdated)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveManipulationUpdated<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationUpdated)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationInertiaStarting<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationInertiaStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationInertiaStarting)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveManipulationInertiaStarting<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationInertiaStarting)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ManipulationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                GestureRecognizer,
                ManipulationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManipulationCompleted)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveManipulationCompleted<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveManipulationCompleted)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CrossSliding<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CrossSliding)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveCrossSliding<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCrossSliding)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for GestureRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GestureRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GestureRecognizer {}
impl ::core::fmt::Debug for GestureRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GestureRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.GestureRecognizer;{cda89afc-6bd0-595c-ba37-545fce5bf016})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows::core::GUID = <IGestureRecognizer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.GestureRecognizer";
}
impl ::core::convert::From<GestureRecognizer> for ::windows::core::IUnknown {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows::core::IUnknown {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GestureRecognizer> for ::windows::core::IInspectable {
    fn from(value: GestureRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GestureRecognizer> for ::windows::core::IInspectable {
    fn from(value: &GestureRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GestureRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GestureRecognizer {}
unsafe impl ::core::marker::Sync for GestureRecognizer {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GestureSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GestureSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for GestureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GestureSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for GestureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.GestureSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct HoldingEventArgs(::windows::core::IUnknown);
impl HoldingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn HoldingState(&self) -> ::windows::core::Result<HoldingState> {
        let this = self;
        unsafe {
            let mut result__: HoldingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoldingState)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HoldingState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for HoldingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HoldingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HoldingEventArgs {}
impl ::core::fmt::Debug for HoldingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.HoldingEventArgs;{8e449e85-d223-533c-b0b2-bf7c6d10c2db})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IHoldingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.HoldingEventArgs";
}
impl ::core::convert::From<HoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: HoldingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HoldingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &HoldingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HoldingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HoldingEventArgs {}
unsafe impl ::core::marker::Sync for HoldingEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HoldingState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HoldingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for HoldingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HoldingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HoldingState {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.HoldingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICrossSlidingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICrossSlidingEventArgs {
    type Vtable = ICrossSlidingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7679641f_ba9f_543c_a7c8_6229a98f89ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CrossSlidingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CrossSlidingState,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDraggingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDraggingEventArgs {
    type Vtable = IDraggingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3efb1b75_3d3b_550e_963d_0828ca76128a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DraggingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DraggingState,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGestureRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcda89afc_6bd0_595c_ba37_545fce5bf016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AutoProcessInertia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CrossSlideThresholds,
    ) -> ::windows::core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CrossSlideThresholds,
    ) -> ::windows::core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut GestureSettings,
    ) -> ::windows::core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: GestureSettings,
    ) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub PivotCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPivotCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    )
        -> ::windows::core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    )
        -> ::windows::core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub CompleteGesture:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ProcessMoveEvents: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
        isshiftkeydown: bool,
        iscontrolkeydown: bool,
    ) -> ::windows::core::HRESULT,
    pub ProcessInertia:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessUpEvent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Tapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRightTapped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Holding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveHolding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Dragging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDragging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ManipulationUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveManipulationUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub ManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    )
        -> ::windows::core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub CrossSliding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCrossSliding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHoldingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHoldingEventArgs {
    type Vtable = IHoldingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8e449e85_d223_533c_b0b2_bf7c6d10c2db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HoldingState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut HoldingState,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputCursor {
    type Vtable = IInputCursor_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x359b15f9_19c2_5714_8432_75176826406b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputCursorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputCursorFactory {
    type Vtable = IInputCursorFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f47647b_4be0_53e9_be7e_c38d5459db6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputDesktopResourceCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1df2777f_7c90_58fc_a7a3_d5736c6510fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ModuleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub ResourceId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputDesktopResourceCursorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputDesktopResourceCursorStatics {
    type Vtable = IInputDesktopResourceCursorStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf440dc37_a0b6_56eb_bcec_b024f2233d47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourceid: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        modulename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        resourceid: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputKeyboardSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputKeyboardSourceStatics {
    type Vtable = IInputKeyboardSourceStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf4e1563d_8c2e_5bcd_b784_47adeaa3cd7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetKeyStateForCurrentThread: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        virtualkey: ::windows::System::VirtualKey,
        result__: *mut ::windows::UI::Core::CoreVirtualKeyStates,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputLightDismissAction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputLightDismissAction {
    type Vtable = IInputLightDismissAction_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe8a39502_a860_502f_8c10_3646d43aecf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissAction_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Dismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDismissed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputLightDismissActionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputLightDismissActionStatics {
    type Vtable = IInputLightDismissActionStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed9b8def_6496_5169_984d_d44b4e690623);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissActionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputLightDismissEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x078660ee_07ca_5808_b982_e6e899cf098c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputObject {
    type Vtable = IInputObject_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x42edbc88_d386_544d_b1b8_68617fe68282);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObject_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputObjectFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputObjectFactory {
    type Vtable = IInputObjectFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf7786bc2_b0b8_5961_9a57_ae199d452106);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObjectFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPointerSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPointerSource {
    type Vtable = IInputPointerSource_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6a6c2764_c3f4_5be5_8447_c9a98766c240);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetCursor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub DeviceKinds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputPointerSourceDeviceKinds,
    ) -> ::windows::core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerRoutedAway: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerRoutedAway: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerRoutedReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerRoutedTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerRoutedTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputSystemCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputSystemCursor {
    type Vtable = IInputSystemCursor_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x59f538e7_c500_59ab_8b54_0bc6100fd49e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CursorShape: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut InputSystemCursorShape,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputSystemCursorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputSystemCursorStatics {
    type Vtable = IInputSystemCursorStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd3860bb6_698a_5814_aedd_c2fa8bba5a02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        r#type: InputSystemCursorShape,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e0249d4_46e4_5559_aee3_fa45ce2a7f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xacf9ef71_6e15_56ab_9260_f0d3ce5f66e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4a616613_eef1_5f1b_a768_0775478d49d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x406e1961_0c98_5fc0_b3d8_116492ef0053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cumulative: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub Delta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationDelta,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub Velocities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ManipulationVelocities,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMouseWheelParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6d98be40_1d56_51d1_aa0d_f325439cd009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CharTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetCharTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub DeltaScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub PageTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub SetPageTranslation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x865b188c_2ed5_5df8_829f_ac0701d5c51a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub KeyModifiers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::System::VirtualKeyModifiers,
    ) -> ::windows::core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetIntermediateTransformedPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    )
        -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0d430ee6_252c_59a4_b2a2_d44264dc6a40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FrameId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub PointerId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub Properties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub GetTransformedPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPointProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd760ed77_4b10_57a5_b3cc_d9bf3413e996);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContactRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub Orientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerUpdateKind,
    ) -> ::windows::core::HRESULT,
    pub Pressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub TouchConfidence: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Twist: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub XTilt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub YTilt: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct IPointerPointTransform(::windows::core::IUnknown);
impl IPointerPointTransform {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Inverse(&self) -> ::windows::core::Result<IPointerPointTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Inverse)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IPointerPointTransform>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TryTransform<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        inpoint: Param0,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTransform)(
                ::core::mem::transmute_copy(this),
                inpoint.into_param().abi(),
                outpoint,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TryTransformBounds<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Rect>,
    >(
        &self,
        inrect: Param0,
        outrect: &mut ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryTransformBounds)(
                ::core::mem::transmute_copy(this),
                inrect.into_param().abi(),
                outrect,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows::core::IUnknown {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows::core::IUnknown {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPointerPointTransform> for ::windows::core::IInspectable {
    fn from(value: IPointerPointTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPointerPointTransform> for ::windows::core::IInspectable {
    fn from(value: &IPointerPointTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPointerPointTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IPointerPointTransform
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPointerPointTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPointerPointTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPointerPointTransform {}
impl ::core::fmt::Debug for IPointerPointTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPointerPointTransform")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{db4791bc-994d-54c7-92ef-66ea1de9b43c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPointerPointTransform {
    type Vtable = IPointerPointTransform_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdb4791bc_994d_54c7_92ef_66ea1de9b43c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Inverse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inpoint: ::windows::Foundation::Point,
        outpoint: *mut ::windows::Foundation::Point,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub TryTransformBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inrect: ::windows::Foundation::Rect,
        outrect: *mut ::windows::Foundation::Rect,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPredictor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPredictor {
    type Vtable = IPointerPredictor_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x12c100ec_2100_565f_a60c_f1187f438828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PredictionTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub SetPredictionTime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::TimeSpan,
    ) -> ::windows::core::HRESULT,
    pub GetPredictedPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::core::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerPredictorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPointerPredictorStatics {
    type Vtable = IPointerPredictorStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x78a8ef30_3e5c_55cd_8f85_65ac09b1a987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateForInputPointerSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputpointersource: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRightTappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8ff73b39_887e_50a4_8500_77953039dcb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITappedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc3a01bb5_6076_5e0f_871a_9d94a6a8f82b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PointerDeviceType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut PointerDeviceType,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Point,
    ) -> ::windows::core::HRESULT,
    pub TapCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputCursor(::windows::core::IUnknown);
impl InputCursor {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
impl ::core::clone::Clone for InputCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputCursor {}
impl ::core::fmt::Debug for InputCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputCursor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputCursor;{359b15f9-19c2-5714-8432-75176826406b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputCursor {
    type Vtable = IInputCursor_Vtbl;
    const IID: ::windows::core::GUID = <IInputCursor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCursor";
}
impl ::core::convert::From<InputCursor> for ::windows::core::IUnknown {
    fn from(value: InputCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputCursor> for ::windows::core::IUnknown {
    fn from(value: &InputCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputCursor> for ::windows::core::IInspectable {
    fn from(value: InputCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputCursor> for ::windows::core::IInspectable {
    fn from(value: &InputCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InputCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputCursor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputCursor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &InputCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InputCursor {}
unsafe impl ::core::marker::Sync for InputCursor {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputDesktopResourceCursor(::windows::core::IUnknown);
impl InputDesktopResourceCursor {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ModuleName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModuleName)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ResourceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Create(resourceid: u32) -> ::windows::core::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                resourceid,
                &mut result__,
            )
            .from_abi::<InputDesktopResourceCursor>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CreateFromModule<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        modulename: Param0,
        resourceid: u32,
    ) -> ::windows::core::Result<InputDesktopResourceCursor> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromModule)(
                ::core::mem::transmute_copy(this),
                modulename.into_param().abi(),
                resourceid,
                &mut result__,
            )
            .from_abi::<InputDesktopResourceCursor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputDesktopResourceCursorStatics<
        R,
        F: FnOnce(&IInputDesktopResourceCursorStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InputDesktopResourceCursor,
            IInputDesktopResourceCursorStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputDesktopResourceCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputDesktopResourceCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputDesktopResourceCursor {}
impl ::core::fmt::Debug for InputDesktopResourceCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputDesktopResourceCursor")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputDesktopResourceCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputDesktopResourceCursor;{1df2777f-7c90-58fc-a7a3-d5736c6510fd})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputDesktopResourceCursor {
    type Vtable = IInputDesktopResourceCursor_Vtbl;
    const IID: ::windows::core::GUID =
        <IInputDesktopResourceCursor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopResourceCursor";
}
impl ::core::convert::From<InputDesktopResourceCursor> for ::windows::core::IUnknown {
    fn from(value: InputDesktopResourceCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputDesktopResourceCursor> for ::windows::core::IUnknown {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputDesktopResourceCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputDesktopResourceCursor> for ::windows::core::IInspectable {
    fn from(value: InputDesktopResourceCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputDesktopResourceCursor> for ::windows::core::IInspectable {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InputDesktopResourceCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputDesktopResourceCursor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputDesktopResourceCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputDesktopResourceCursor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InputDesktopResourceCursor
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InputDesktopResourceCursor> for InputCursor {
    fn from(value: InputDesktopResourceCursor) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputDesktopResourceCursor> for InputCursor {
    fn from(value: &InputDesktopResourceCursor) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputCursor> for InputDesktopResourceCursor {
    fn into_param(self) -> ::windows::core::Param<'a, InputCursor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputCursor> for &InputDesktopResourceCursor {
    fn into_param(self) -> ::windows::core::Param<'a, InputCursor> {
        ::windows::core::Param::Owned(::core::convert::Into::<InputCursor>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputDesktopResourceCursor {}
unsafe impl ::core::marker::Sync for InputDesktopResourceCursor {}
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct InputKeyboardSource {}
impl InputKeyboardSource {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetKeyStateForCurrentThread(
        virtualkey: ::windows::System::VirtualKey,
    ) -> ::windows::core::Result<::windows::UI::Core::CoreVirtualKeyStates> {
        Self::IInputKeyboardSourceStatics(|this| unsafe {
            let mut result__: ::windows::UI::Core::CoreVirtualKeyStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetKeyStateForCurrentThread)(
                ::core::mem::transmute_copy(this),
                virtualkey,
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreVirtualKeyStates>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputKeyboardSourceStatics<
        R,
        F: FnOnce(&IInputKeyboardSourceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InputKeyboardSource,
            IInputKeyboardSourceStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputKeyboardSource";
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputLightDismissAction(::windows::core::IUnknown);
impl InputLightDismissAction {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Dismissed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<
                InputLightDismissAction,
                InputLightDismissEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Dismissed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemoveDismissed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDismissed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetForWindowId<'a, Param0: ::windows::core::IntoParam<'a, super::WindowId>>(
        windowid: Param0,
    ) -> ::windows::core::Result<InputLightDismissAction> {
        Self::IInputLightDismissActionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForWindowId)(
                ::core::mem::transmute_copy(this),
                windowid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InputLightDismissAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Input\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IInputLightDismissActionStatics<
        R,
        F: FnOnce(&IInputLightDismissActionStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InputLightDismissAction,
            IInputLightDismissActionStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputLightDismissAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputLightDismissAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputLightDismissAction {}
impl ::core::fmt::Debug for InputLightDismissAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputLightDismissAction")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputLightDismissAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputLightDismissAction;{e8a39502-a860-502f-8c10-3646d43aecf1})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputLightDismissAction {
    type Vtable = IInputLightDismissAction_Vtbl;
    const IID: ::windows::core::GUID =
        <IInputLightDismissAction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissAction";
}
impl ::core::convert::From<InputLightDismissAction> for ::windows::core::IUnknown {
    fn from(value: InputLightDismissAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputLightDismissAction> for ::windows::core::IUnknown {
    fn from(value: &InputLightDismissAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputLightDismissAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputLightDismissAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputLightDismissAction> for ::windows::core::IInspectable {
    fn from(value: InputLightDismissAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputLightDismissAction> for ::windows::core::IInspectable {
    fn from(value: &InputLightDismissAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputLightDismissAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InputLightDismissAction
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputLightDismissAction> for InputObject {
    fn from(value: InputLightDismissAction) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputLightDismissAction> for InputObject {
    fn from(value: &InputLightDismissAction) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputObject> for InputLightDismissAction {
    fn into_param(self) -> ::windows::core::Param<'a, InputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputObject> for &InputLightDismissAction {
    fn into_param(self) -> ::windows::core::Param<'a, InputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<InputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputLightDismissAction {}
unsafe impl ::core::marker::Sync for InputLightDismissAction {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputLightDismissEventArgs(::windows::core::IUnknown);
impl InputLightDismissEventArgs {}
impl ::core::clone::Clone for InputLightDismissEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputLightDismissEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputLightDismissEventArgs {}
impl ::core::fmt::Debug for InputLightDismissEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputLightDismissEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputLightDismissEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputLightDismissEventArgs;{078660ee-07ca-5808-b982-e6e899cf098c})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputLightDismissEventArgs {
    type Vtable = IInputLightDismissEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IInputLightDismissEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissEventArgs";
}
impl ::core::convert::From<InputLightDismissEventArgs> for ::windows::core::IUnknown {
    fn from(value: InputLightDismissEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputLightDismissEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InputLightDismissEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputLightDismissEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputLightDismissEventArgs> for ::windows::core::IInspectable {
    fn from(value: InputLightDismissEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputLightDismissEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InputLightDismissEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InputLightDismissEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InputLightDismissEventArgs {}
unsafe impl ::core::marker::Sync for InputLightDismissEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputObject(::windows::core::IUnknown);
impl InputObject {
    #[doc = "*Required features: `\"UI_Input\"`, `\"UI_Dispatching\"`*"]
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
}
impl ::core::clone::Clone for InputObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputObject {}
impl ::core::fmt::Debug for InputObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputObject;{42edbc88-d386-544d-b1b8-68617fe68282})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputObject {
    type Vtable = IInputObject_Vtbl;
    const IID: ::windows::core::GUID = <IInputObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputObject {
    const NAME: &'static str = "Microsoft.UI.Input.InputObject";
}
impl ::core::convert::From<InputObject> for ::windows::core::IUnknown {
    fn from(value: InputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputObject> for ::windows::core::IUnknown {
    fn from(value: &InputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputObject> for ::windows::core::IInspectable {
    fn from(value: InputObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputObject> for ::windows::core::IInspectable {
    fn from(value: &InputObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InputObject {}
unsafe impl ::core::marker::Sync for InputObject {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputPointerSource(::windows::core::IUnknown);
impl InputPointerSource {
    #[doc = "*Required features: `\"UI_Input\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Cursor(&self) -> ::windows::core::Result<InputCursor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cursor)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputCursor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCursor<'a, Param0: ::windows::core::IntoParam<'a, InputCursor>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCursor)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DeviceKinds(&self) -> ::windows::core::Result<InputPointerSourceDeviceKinds> {
        let this = self;
        unsafe {
            let mut result__: InputPointerSourceDeviceKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceKinds)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputPointerSourceDeviceKinds>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerCaptureLost<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerCaptureLost)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerCaptureLost<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerCaptureLost)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerEntered<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerEntered)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerEntered)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerExited<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerExited)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerExited<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerExited)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerMoved<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerMoved)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerMoved<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerMoved)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerPressed<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerPressed)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerPressed<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerPressed)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerReleased<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerReleased)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerReleased<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerReleased)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerRoutedAway<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerRoutedAway)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerRoutedAway<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerRoutedAway)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerRoutedReleased<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerRoutedReleased)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerRoutedReleased<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerRoutedReleased)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerRoutedTo<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerRoutedTo)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerRoutedTo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerRoutedTo)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerWheelChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerWheelChanged)(
                ::core::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn RemovePointerWheelChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePointerWheelChanged)(
                ::core::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for InputPointerSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputPointerSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputPointerSource {}
impl ::core::fmt::Debug for InputPointerSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPointerSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputPointerSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputPointerSource;{6a6c2764-c3f4-5be5-8447-c9a98766c240})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputPointerSource {
    type Vtable = IInputPointerSource_Vtbl;
    const IID: ::windows::core::GUID = <IInputPointerSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPointerSource";
}
impl ::core::convert::From<InputPointerSource> for ::windows::core::IUnknown {
    fn from(value: InputPointerSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPointerSource> for ::windows::core::IUnknown {
    fn from(value: &InputPointerSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputPointerSource> for ::windows::core::IInspectable {
    fn from(value: InputPointerSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPointerSource> for ::windows::core::IInspectable {
    fn from(value: &InputPointerSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputPointerSource> for InputObject {
    fn from(value: InputPointerSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputPointerSource> for InputObject {
    fn from(value: &InputPointerSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputObject> for InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, InputObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputObject> for &InputPointerSource {
    fn into_param(self) -> ::windows::core::Param<'a, InputObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<InputObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputPointerSource {}
unsafe impl ::core::marker::Sync for InputPointerSource {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputPointerSourceDeviceKinds(pub u32);
impl InputPointerSourceDeviceKinds {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl ::core::marker::Copy for InputPointerSourceDeviceKinds {}
impl ::core::clone::Clone for InputPointerSourceDeviceKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputPointerSourceDeviceKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InputPointerSourceDeviceKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputPointerSourceDeviceKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPointerSourceDeviceKinds")
            .field(&self.0)
            .finish()
    }
}
impl ::core::ops::BitOr for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InputPointerSourceDeviceKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InputPointerSourceDeviceKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InputPointerSourceDeviceKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputPointerSourceDeviceKinds;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct InputSystemCursor(::windows::core::IUnknown);
impl InputSystemCursor {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CursorShape(&self) -> ::windows::core::Result<InputSystemCursorShape> {
        let this = self;
        unsafe {
            let mut result__: InputSystemCursorShape = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorShape)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InputSystemCursorShape>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Create(r#type: InputSystemCursorShape) -> ::windows::core::Result<InputSystemCursor> {
        Self::IInputSystemCursorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(
                ::core::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<InputSystemCursor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputSystemCursorStatics<
        R,
        F: FnOnce(&IInputSystemCursorStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InputSystemCursor,
            IInputSystemCursorStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InputSystemCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputSystemCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputSystemCursor {}
impl ::core::fmt::Debug for InputSystemCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputSystemCursor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputSystemCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.InputSystemCursor;{59f538e7-c500-59ab-8b54-0bc6100fd49e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputSystemCursor {
    type Vtable = IInputSystemCursor_Vtbl;
    const IID: ::windows::core::GUID = <IInputSystemCursor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputSystemCursor";
}
impl ::core::convert::From<InputSystemCursor> for ::windows::core::IUnknown {
    fn from(value: InputSystemCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputSystemCursor> for ::windows::core::IUnknown {
    fn from(value: &InputSystemCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputSystemCursor> for ::windows::core::IInspectable {
    fn from(value: InputSystemCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputSystemCursor> for ::windows::core::IInspectable {
    fn from(value: &InputSystemCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InputSystemCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputSystemCursor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputSystemCursor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputSystemCursor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InputSystemCursor> for InputCursor {
    fn from(value: InputSystemCursor) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&InputSystemCursor> for InputCursor {
    fn from(value: &InputSystemCursor) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputCursor> for InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, InputCursor> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, InputCursor> for &InputSystemCursor {
    fn into_param(self) -> ::windows::core::Param<'a, InputCursor> {
        ::windows::core::Param::Owned(::core::convert::Into::<InputCursor>::into(self))
    }
}
unsafe impl ::core::marker::Send for InputSystemCursor {}
unsafe impl ::core::marker::Sync for InputSystemCursor {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InputSystemCursorShape(pub i32);
impl InputSystemCursorShape {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
    pub const AppStarting: Self = Self(16i32);
}
impl ::core::marker::Copy for InputSystemCursorShape {}
impl ::core::clone::Clone for InputSystemCursorShape {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputSystemCursorShape {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InputSystemCursorShape {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputSystemCursorShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputSystemCursorShape")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputSystemCursorShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Input.InputSystemCursorShape;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationCompletedEventArgs(::windows::core::IUnknown);
impl ManipulationCompletedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationCompletedEventArgs {}
impl ::core::fmt::Debug for ManipulationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationCompletedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationCompletedEventArgs;{0e0249d4-46e4-5559-aee3-fa45ce2a7f56})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationCompletedEventArgs {
    type Vtable = IManipulationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IManipulationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationCompletedEventArgs";
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationCompletedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationCompletedEventArgs {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct ManipulationDelta {
    pub Translation: ::windows::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
impl ::core::marker::Copy for ManipulationDelta {}
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ManipulationDelta {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationDelta")
            .field("Translation", &self.Translation)
            .field("Scale", &self.Scale)
            .field("Rotation", &self.Rotation)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ManipulationDelta {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ManipulationDelta {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"struct(Microsoft.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ManipulationDelta {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<ManipulationDelta>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for ManipulationDelta {}
impl ::core::default::Default for ManipulationDelta {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventArgs(::windows::core::IUnknown);
impl ManipulationInertiaStartingEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Delta(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationInertiaStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationInertiaStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationInertiaStartingEventArgs {}
impl ::core::fmt::Debug for ManipulationInertiaStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationInertiaStartingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationInertiaStartingEventArgs;{acf9ef71-6e15-56ab-9260-f0d3ce5f66e8})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = IManipulationInertiaStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IManipulationInertiaStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationInertiaStartingEventArgs";
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationInertiaStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationInertiaStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationInertiaStartingEventArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &ManipulationInertiaStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationInertiaStartingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationInertiaStartingEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationInertiaStartingEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationStartedEventArgs(::windows::core::IUnknown);
impl ManipulationStartedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationStartedEventArgs {}
impl ::core::fmt::Debug for ManipulationStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationStartedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationStartedEventArgs;{4a616613-eef1-5f1b-a768-0775478d49d4})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationStartedEventArgs {
    type Vtable = IManipulationStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IManipulationStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationStartedEventArgs";
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationStartedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationStartedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationStartedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct ManipulationUpdatedEventArgs(::windows::core::IUnknown);
impl ManipulationUpdatedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Cumulative(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cumulative)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Delta(&self) -> ::windows::core::Result<ManipulationDelta> {
        let this = self;
        unsafe {
            let mut result__: ManipulationDelta = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delta)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationDelta>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Velocities(&self) -> ::windows::core::Result<ManipulationVelocities> {
        let this = self;
        unsafe {
            let mut result__: ManipulationVelocities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Velocities)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ManipulationVelocities>(result__)
        }
    }
}
impl ::core::clone::Clone for ManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ManipulationUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ManipulationUpdatedEventArgs {}
impl ::core::fmt::Debug for ManipulationUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ManipulationUpdatedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Input.ManipulationUpdatedEventArgs;{406e1961-0c98-5fc0-b3d8-116492ef0053})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = IManipulationUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IManipulationUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationUpdatedEventArgs";
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ManipulationUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ManipulationUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ManipulationUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ManipulationUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ManipulationUpdatedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ManipulationUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for ManipulationUpdatedEventArgs {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Input\"`*"]
pub struct ManipulationVelocities {
    pub Linear: ::windows::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
impl ::core::marker::Copy for ManipulationVelocities {}
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ManipulationVelocities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ManipulationVelocities")
            .field("Linear", &self.Linear)
            .field("Angular", &self.Angular)
            .field("Expansion", &self.Expansion)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for ManipulationVelocities {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ManipulationVelocities {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"struct(Microsoft.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)" ) ;
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for ManipulationVelocities {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<ManipulationVelocities>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for ManipulationVelocities {}
impl ::core::default::Default for ManipulationVelocities {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct MouseWheelParameters(::windows::core::IUnknown);
impl MouseWheelParameters {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CharTranslation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharTranslation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetCharTranslation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCharTranslation)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DeltaScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaScale)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetDeltaScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDeltaScale)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn DeltaRotationAngle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeltaRotationAngle)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetDeltaRotationAngle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDeltaRotationAngle)(
                ::core::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PageTranslation(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageTranslation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetPageTranslation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Point>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPageTranslation)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for MouseWheelParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MouseWheelParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MouseWheelParameters {}
impl ::core::fmt::Debug for MouseWheelParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MouseWheelParameters")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.MouseWheelParameters;{6d98be40-1d56-51d1-aa0d-f325439cd009})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MouseWheelParameters {
    type Vtable = IMouseWheelParameters_Vtbl;
    const IID: ::windows::core::GUID = <IMouseWheelParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.MouseWheelParameters";
}
impl ::core::convert::From<MouseWheelParameters> for ::windows::core::IUnknown {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows::core::IUnknown {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MouseWheelParameters> for ::windows::core::IInspectable {
    fn from(value: MouseWheelParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MouseWheelParameters> for ::windows::core::IInspectable {
    fn from(value: &MouseWheelParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MouseWheelParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a MouseWheelParameters
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MouseWheelParameters {}
unsafe impl ::core::marker::Sync for MouseWheelParameters {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const Touchpad: Self = Self(3i32);
}
impl ::core::marker::Copy for PointerDeviceType {}
impl ::core::clone::Clone for PointerDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PointerDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.PointerDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerEventArgs(::windows::core::IUnknown);
impl PointerEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CurrentPoint(&self) -> ::windows::core::Result<PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentPoint)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerPoint>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
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
    #[doc = "*Required features: `\"UI_Input\"`*"]
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
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn KeyModifiers(&self) -> ::windows::core::Result<::windows::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: ::windows::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyModifiers)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetIntermediatePoints(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<PointerPoint>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetIntermediateTransformedPoints<
        'a,
        Param0: ::windows::core::IntoParam<'a, IPointerPointTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIntermediateTransformedPoints)(
                ::core::mem::transmute_copy(this),
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<PointerPoint>>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerEventArgs {}
impl ::core::fmt::Debug for PointerEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerEventArgs;{865b188c-2ed5-5df8-829f-ac0701d5c51a})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerEventArgs {
    type Vtable = IPointerEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPointerEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.PointerEventArgs";
}
impl ::core::convert::From<PointerEventArgs> for ::windows::core::IUnknown {
    fn from(value: PointerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PointerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerEventArgs> for ::windows::core::IInspectable {
    fn from(value: PointerEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PointerEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PointerEventArgs {}
unsafe impl ::core::marker::Sync for PointerEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerPoint(::windows::core::IUnknown);
impl PointerPoint {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInContact(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInContact)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerId)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Properties(&self) -> ::windows::core::Result<PointerPointProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerPointProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Timestamp)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetTransformedPoint<
        'a,
        Param0: ::windows::core::IntoParam<'a, IPointerPointTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::core::Result<PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTransformedPoint)(
                ::core::mem::transmute_copy(this),
                transform.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PointerPoint>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPoint {}
impl ::core::fmt::Debug for PointerPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPoint;{0d430ee6-252c-59a4-b2a2-d44264dc6a40})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerPoint {
    type Vtable = IPointerPoint_Vtbl;
    const IID: ::windows::core::GUID = <IPointerPoint as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
impl ::core::convert::From<PointerPoint> for ::windows::core::IUnknown {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows::core::IUnknown {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPoint> for ::windows::core::IInspectable {
    fn from(value: PointerPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPoint> for ::windows::core::IInspectable {
    fn from(value: &PointerPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PointerPoint {}
unsafe impl ::core::marker::Sync for PointerPoint {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerPointProperties(::windows::core::IUnknown);
impl PointerPointProperties {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn ContactRect(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactRect)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsBarrelButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarrelButtonPressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsCanceled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCanceled)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsEraser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEraser)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsHorizontalMouseWheel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHorizontalMouseWheel)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInRange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInRange)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsInverted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInverted)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsLeftButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLeftButtonPressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsMiddleButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMiddleButtonPressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsPrimary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrimary)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsRightButtonPressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRightButtonPressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsXButton1Pressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsXButton1Pressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn IsXButton2Pressed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsXButton2Pressed)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn MouseWheelDelta(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MouseWheelDelta)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerUpdateKind(&self) -> ::windows::core::Result<PointerUpdateKind> {
        let this = self;
        unsafe {
            let mut result__: PointerUpdateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerUpdateKind)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerUpdateKind>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pressure)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TouchConfidence(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TouchConfidence)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Twist(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Twist)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn XTilt(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XTilt)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn YTilt(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).YTilt)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for PointerPointProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPointProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPointProperties {}
impl ::core::fmt::Debug for PointerPointProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPointProperties")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerPointProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPointProperties;{d760ed77-4b10-57a5-b3cc-d9bf3413e996})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerPointProperties {
    type Vtable = IPointerPointProperties_Vtbl;
    const IID: ::windows::core::GUID = <IPointerPointProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
impl ::core::convert::From<PointerPointProperties> for ::windows::core::IUnknown {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows::core::IUnknown {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPointProperties> for ::windows::core::IInspectable {
    fn from(value: PointerPointProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPointProperties> for ::windows::core::IInspectable {
    fn from(value: &PointerPointProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerPointProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PointerPointProperties
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PointerPointProperties {}
unsafe impl ::core::marker::Sync for PointerPointProperties {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct PointerPredictor(::windows::core::IUnknown);
impl PointerPredictor {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PredictionTime(&self) -> ::windows::core::Result<::windows::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PredictionTime)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn SetPredictionTime<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::TimeSpan>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPredictionTime)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn GetPredictedPoints<'a, Param0: ::windows::core::IntoParam<'a, PointerPoint>>(
        &self,
        point: Param0,
    ) -> ::windows::core::Result<::windows::core::Array<PointerPoint>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<PointerPoint> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPredictedPoints)(
                ::core::mem::transmute_copy(this),
                point.into_param().abi(),
                ::windows::core::Array::<PointerPoint>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn CreateForInputPointerSource<
        'a,
        Param0: ::windows::core::IntoParam<'a, InputPointerSource>,
    >(
        inputpointersource: Param0,
    ) -> ::windows::core::Result<PointerPredictor> {
        Self::IPointerPredictorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForInputPointerSource)(
                ::core::mem::transmute_copy(this),
                inputpointersource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PointerPredictor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPointerPredictorStatics<
        R,
        F: FnOnce(&IPointerPredictorStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PointerPredictor,
            IPointerPredictorStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerPredictor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerPredictor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerPredictor {}
impl ::core::fmt::Debug for PointerPredictor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerPredictor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerPredictor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.PointerPredictor;{12c100ec-2100-565f-a60c-f1187f438828})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PointerPredictor {
    type Vtable = IPointerPredictor_Vtbl;
    const IID: ::windows::core::GUID = <IPointerPredictor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPredictor";
}
impl ::core::convert::From<PointerPredictor> for ::windows::core::IUnknown {
    fn from(value: PointerPredictor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPredictor> for ::windows::core::IUnknown {
    fn from(value: &PointerPredictor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerPredictor> for ::windows::core::IInspectable {
    fn from(value: PointerPredictor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerPredictor> for ::windows::core::IInspectable {
    fn from(value: &PointerPredictor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PointerPredictor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PointerPredictor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PointerPredictor> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PointerPredictor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &PointerPredictor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PointerPredictor {}
unsafe impl ::core::marker::Sync for PointerPredictor {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PointerUpdateKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PointerUpdateKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PointerUpdateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpdateKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.PointerUpdateKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct RightTappedEventArgs(::windows::core::IUnknown);
impl RightTappedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
}
impl ::core::clone::Clone for RightTappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RightTappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RightTappedEventArgs {}
impl ::core::fmt::Debug for RightTappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RightTappedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.RightTappedEventArgs;{8ff73b39-887e-50a4-8500-77953039dcb4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RightTappedEventArgs {
    type Vtable = IRightTappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IRightTappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.RightTappedEventArgs";
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RightTappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RightTappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RightTappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RightTappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a RightTappedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RightTappedEventArgs {}
unsafe impl ::core::marker::Sync for RightTappedEventArgs {}
#[doc = "*Required features: `\"UI_Input\"`*"]
#[repr(transparent)]
pub struct TappedEventArgs(::windows::core::IUnknown);
impl TappedEventArgs {
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn PointerDeviceType(&self) -> ::windows::core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__: PointerDeviceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PointerDeviceType)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PointerDeviceType>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Input\"`*"]
    pub fn TapCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TapCount)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TappedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TappedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TappedEventArgs {}
impl ::core::fmt::Debug for TappedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TappedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TappedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Input.TappedEventArgs;{c3a01bb5-6076-5e0f-871a-9d94a6a8f82b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for TappedEventArgs {
    type Vtable = ITappedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ITappedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TappedEventArgs";
}
impl ::core::convert::From<TappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TappedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TappedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TappedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TappedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TappedEventArgs {}
unsafe impl ::core::marker::Sync for TappedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
