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
pub struct ElementCompositionPreview(pub ::windows::core::IInspectable);
impl ElementCompositionPreview {
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetElementVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
    ) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetElementChildVisual<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
    ) -> ::windows::core::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetElementChildVisual<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::Visual>,
    >(
        element: Param0,
        visual: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                visual.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetScrollViewerManipulationPropertySet<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Controls::ScrollViewer>,
    >(
        scrollviewer: Param0,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                scrollviewer.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetImplicitShowAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetImplicitHideAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
        Param1: ::windows::core::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn SetIsTranslationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub fn GetPointerPositionPropertySet<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::UIElement>,
    >(
        targetelement: Param0,
    ) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                targetelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
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
unsafe impl ::windows::core::RuntimeType for ElementCompositionPreview {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Hosting.ElementCompositionPreview;{c8ad1ef4-a93f-5a25-85bd-7c498d9856d3})" ) ;
}
unsafe impl ::windows::core::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3);
}
impl ::windows::core::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ElementCompositionPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ElementCompositionPreview> for ::windows::core::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ElementCompositionPreview {}
unsafe impl ::core::marker::Sync for ElementCompositionPreview {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IDesktopWindowXamlSourceNative(pub ::windows::core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    pub unsafe fn AttachToWindow<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Win32::Foundation::HWND>,
    >(
        &self,
        parentwnd: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            parentwnd.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(hwnd),
        )
        .ok()
    }
    pub unsafe fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(message),
            ::core::mem::transmute(result),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0aea2f26_facf_4588_8cf4_34555124db32);
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreview(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc8ad1ef4_a93f_5a25_85bd_7c498d9856d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_abi(
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
pub struct IElementCompositionPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x84da5a6c_0cfa_532b_9b15_ccd986374342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_abi(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        visual: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        scrollviewer: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        animation: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        animation: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        targetelement: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
        feature = "UI_Xaml_Automation",
        feature = "UI_Xaml_Automation_Peers",
        feature = "UI_Xaml_Automation_Provider",
        feature = "UI_Xaml_Controls",
        feature = "UI_Xaml_Controls_Primitives",
        feature = "UI_Xaml_Data",
        feature = "UI_Xaml_Input",
        feature = "UI_Xaml_Media",
        feature = "UI_Xaml_Media_Animation",
        feature = "UI_Xaml_Media_Imaging",
        feature = "UI_Xaml_Media_Media3D"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IFindReferenceTargetsCallback(pub ::windows::core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, IReferenceTrackerTarget>,
    >(
        &self,
        target: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            target.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows::core::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        target: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IReferenceTracker(pub ::windows::core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn FindTrackerTargets<
        'a,
        Param0: ::windows::core::IntoParam<'a, IFindReferenceTargetsCallback>,
    >(
        &self,
        callback: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            callback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetReferenceTrackerManager(
        &self,
    ) -> ::windows::core::Result<IReferenceTrackerManager> {
        let mut result__: <IReferenceTrackerManager as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IReferenceTrackerManager>(result__)
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
impl ::core::convert::From<IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        callback: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IReferenceTrackerExtension(pub ::windows::core::IUnknown);
impl IReferenceTrackerExtension {}
unsafe impl ::windows::core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IReferenceTrackerExtension
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IReferenceTrackerHost(pub ::windows::core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(options),
        )
        .ok()
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetTrackerTarget<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>,
    >(
        &self,
        unknown: Param0,
    ) -> ::windows::core::Result<IReferenceTrackerTarget> {
        let mut result__: <IReferenceTrackerTarget as ::windows::core::Abi>::Abi =
            ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            unknown.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IReferenceTrackerTarget>(result__)
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(bytesallocated),
        )
        .ok()
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(bytesallocated),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        unknown: ::windows::core::RawPtr,
        newreference: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        bytesallocated: u64,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IReferenceTrackerManager(pub ::windows::core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn FindTrackerTargetsCompleted(
        &self,
        findfailed: u8,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(findfailed),
        )
        .ok()
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetReferenceTrackerHost<
        'a,
        Param0: ::windows::core::IntoParam<'a, IReferenceTrackerHost>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            value.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IReferenceTrackerManager
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        findfailed: u8,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IReferenceTrackerTarget(pub ::windows::core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
        ))
    }
    pub unsafe fn Peg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unpeg(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows::core::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceTrackerTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ITrackerOwner(pub ::windows::core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(returnvalue),
        )
        .ok()
    }
    pub unsafe fn DeleteTrackerHandle(
        &self,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(handle),
        )
        .ok()
    }
    pub unsafe fn SetTrackerValue<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>,
    >(
        &self,
        handle: *mut TrackerHandle__,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(handle),
            value.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>,
    ) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(handle),
            ::core::mem::transmute(returnvalue),
        ))
    }
}
unsafe impl ::windows::core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
impl ::core::convert::From<ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handle: *mut TrackerHandle__,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handle: *mut TrackerHandle__,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::windows::core::RawPtr,
    ) -> u8,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TrackerHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
unsafe impl ::windows::core::Abi for TrackerHandle__ {
    type Abi = Self;
}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(1i32);
impl ::core::convert::From<i32>
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    type Abi = Self;
}
