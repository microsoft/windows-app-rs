#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct ElementCompositionPreview(::windows::core::IUnknown);
impl ElementCompositionPreview {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
    ) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetElementVisual)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
    ) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetElementChildVisual)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetElementChildVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::Visual>,
    >(
        element: Param0,
        visual: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetElementChildVisual)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                visual.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`, `\"UI_Xaml_Controls\"`*"]
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub fn GetScrollViewerManipulationPropertySet<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::ScrollViewer>,
    >(
        scrollviewer: Param0,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetScrollViewerManipulationPropertySet)(
                ::windows::core::Interface::as_raw(this),
                scrollviewer.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitShowAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetImplicitShowAnimation)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn SetImplicitHideAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetImplicitHideAnimation)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetIsTranslationEnabled)(
                ::windows::core::Interface::as_raw(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`, `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetPointerPositionPropertySet<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
    >(
        targetelement: Param0,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetPointerPositionPropertySet)(
                ::windows::core::Interface::as_raw(this),
                targetelement.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IElementCompositionPreviewStatics<
        R,
        F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ElementCompositionPreview,
            IElementCompositionPreviewStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ElementCompositionPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElementCompositionPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElementCompositionPreview {}
impl ::core::fmt::Debug for ElementCompositionPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElementCompositionPreview")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Hosting.ElementCompositionPreview;{c8ad1ef4-a93f-5a25-85bd-7c498d9856d3})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows::core::GUID =
        <IElementCompositionPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn AttachToWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::HWND>,
    >(
        &self,
        parentwnd: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachToWindow)(
            ::windows::core::Interface::as_raw(self),
            parentwnd.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WindowHandle)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(hwnd),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreTranslateMessage)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(message),
            ::core::mem::transmute(result),
        )
        .ok()
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0aea2f26_facf_4588_8cf4_34555124db32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AttachToWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub WindowHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub PreTranslateMessage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElementCompositionPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84da5a6c_0cfa_532b_9b15_ccd986374342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Composition")]
    pub GetElementVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetElementChildVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetElementChildVisual: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        visual: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetElementChildVisual: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub GetScrollViewerManipulationPropertySet:
        unsafe extern "system" fn(
            this: *mut ::core::ffi::c_void,
            scrollviewer: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))]
    GetScrollViewerManipulationPropertySet: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitShowAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitShowAnimation: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetImplicitHideAnimation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        animation: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetImplicitHideAnimation: usize,
    pub SetIsTranslationEnabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        element: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPointerPositionPropertySet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        targetelement: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPointerPositionPropertySet: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(::windows::core::IUnknown);
impl IFindReferenceTargetsCallback {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn FoundTrackerTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, IReferenceTrackerTarget>,
    >(
        &self,
        target: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FoundTrackerTarget)(
            ::windows::core::Interface::as_raw(self),
            target.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFindReferenceTargetsCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFindReferenceTargetsCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindReferenceTargetsCallback {}
impl ::core::fmt::Debug for IFindReferenceTargetsCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindReferenceTargetsCallback")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTracker(::windows::core::IUnknown);
impl IReferenceTracker {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectFromTrackerSource)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectFromTrackerSource)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn FindTrackerTargets<
        'a,
        Param0: ::windows::core::IntoParam<'a, IFindReferenceTargetsCallback>,
    >(
        &self,
        callback: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindTrackerTargets)(
            ::windows::core::Interface::as_raw(self),
            callback.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn GetReferenceTrackerManager(
        &self,
    ) -> ::windows::core::Result<IReferenceTrackerManager> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).GetReferenceTrackerManager)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IReferenceTrackerManager>(result__)
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRefFromTrackerSource)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseFromTrackerSource)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PegFromTrackerSource)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
}
impl ::core::convert::From<IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTracker {}
impl ::core::fmt::Debug for IReferenceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ConnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisconnectFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        callback: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddRefFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PegFromTrackerSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerExtension(::windows::core::IUnknown);
impl IReferenceTrackerExtension {}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IReferenceTrackerExtension
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerExtension {}
impl ::core::fmt::Debug for IReferenceTrackerExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerExtension")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerHost(::windows::core::IUnknown);
impl IReferenceTrackerHost {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectUnusedReferenceSources)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(options),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseDisconnectedReferenceSources)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyEndOfReferenceTrackingOnThread)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn GetTrackerTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>,
    >(
        &self,
        unknown: Param0,
    ) -> ::windows::core::Result<IReferenceTrackerTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).GetTrackerTarget)(
            ::windows::core::Interface::as_raw(self),
            unknown.into_param().abi(),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<IReferenceTrackerTarget>(result__)
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddMemoryPressure)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(bytesallocated),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMemoryPressure)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(bytesallocated),
        )
        .ok()
    }
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerHost {}
impl ::core::fmt::Debug for IReferenceTrackerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerHost")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    )
        -> ::windows::core::HRESULT,
    pub ReleaseDisconnectedReferenceSources:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unknown: *mut ::core::ffi::c_void,
        newreference: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerManager(::windows::core::IUnknown);
impl IReferenceTrackerManager {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReferenceTrackingStarted)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn FindTrackerTargetsCompleted(
        &self,
        findfailed: u8,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindTrackerTargetsCompleted)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(findfailed),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReferenceTrackingCompleted)(
            ::windows::core::Interface::as_raw(self),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn SetReferenceTrackerHost<
        'a,
        Param0: ::windows::core::IntoParam<'a, IReferenceTrackerHost>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReferenceTrackerHost)(
            ::windows::core::Interface::as_raw(self),
            value.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IReferenceTrackerManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerManager {}
impl ::core::fmt::Debug for IReferenceTrackerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerManager")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ReferenceTrackingStarted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        findfailed: u8,
    ) -> ::windows::core::HRESULT,
    pub ReferenceTrackingCompleted:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct IReferenceTrackerTarget(::windows::core::IUnknown);
impl IReferenceTrackerTarget {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self)
            .AddRefFromReferenceTracker)(
            ::windows::core::Interface::as_raw(self)
        ))
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self)
            .ReleaseFromReferenceTracker)(
            ::windows::core::Interface::as_raw(self)
        ))
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn Peg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Peg)(::windows::core::Interface::as_raw(self))
            .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn Unpeg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unpeg)(::windows::core::Interface::as_raw(self))
            .ok()
    }
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerTarget {}
impl ::core::fmt::Debug for IReferenceTrackerTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerTarget")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddRefFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseFromReferenceTracker:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unpeg:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
pub struct ITrackerOwner(::windows::core::IUnknown);
impl ITrackerOwner {
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateTrackerHandle)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(returnvalue),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn DeleteTrackerHandle(
        &self,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteTrackerHandle)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(handle),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn SetTrackerValue<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>,
    >(
        &self,
        handle: *mut TrackerHandle__,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrackerValue)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(handle),
            value.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
    pub unsafe fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>,
    ) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self)
            .TryGetSafeTrackerValue)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(handle),
            ::core::mem::transmute(returnvalue),
        ))
    }
}
impl ::core::convert::From<ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrackerOwner {}
impl ::core::fmt::Debug for ITrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handle: *mut TrackerHandle__,
        returnvalue: *mut *mut ::core::ffi::c_void,
    ) -> u8,
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TrackerHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for TrackerHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<TrackerHandle__>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(pub i32);
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(0i32);
#[doc = "*Required features: `\"UI_Xaml_Hosting\"`*"]
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(1i32);
impl ::core::marker::Copy
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
}
impl ::core::clone::Clone
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    type Abi = Self;
}
impl ::core::fmt::Debug
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple(
            "__MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001",
        )
        .field(&self.0)
        .finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
