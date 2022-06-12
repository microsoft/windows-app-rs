#[cfg(feature = "UI_Composition")]
pub trait IBrushOverrides_Impl: Sized {
    fn PopulatePropertyInfoOverride(
        &self,
        propertyname: &::windows::core::HSTRING,
        animationpropertyinfo: &::core::option::Option<
            super::super::Composition::AnimationPropertyInfo,
        >,
    ) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Composition")]
impl ::windows::core::RuntimeName for IBrushOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrushOverrides";
}
#[cfg(feature = "UI_Composition")]
impl IBrushOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IBrushOverrides_Impl,
        const OFFSET: isize,
    >() -> IBrushOverrides_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IBrushOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            animationpropertyinfo: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulatePropertyInfoOverride(
                ::core::mem::transmute(&propertyname),
                ::core::mem::transmute(&animationpropertyinfo),
            )
            .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IBrushOverrides, OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IGeneralTransformOverrides_Impl: Sized {
    fn InverseCore(&self) -> ::windows::core::Result<GeneralTransform>;
    fn TryTransformCore(
        &self,
        inpoint: &::windows::Foundation::Point,
        outpoint: &mut ::windows::Foundation::Point,
    ) -> ::windows::core::Result<bool>;
    fn TransformBoundsCore(
        &self,
        rect: &::windows::Foundation::Rect,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>;
}
impl ::windows::core::RuntimeName for IGeneralTransformOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IGeneralTransformOverrides";
}
impl IGeneralTransformOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IGeneralTransformOverrides_Impl,
        const OFFSET: isize,
    >() -> IGeneralTransformOverrides_Vtbl {
        unsafe extern "system" fn InverseCore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InverseCore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryTransformCore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            inpoint: ::windows::Foundation::Point,
            outpoint: *mut ::windows::Foundation::Point,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TryTransformCore(
                ::core::mem::transmute(&inpoint),
                ::core::mem::transmute_copy(&outpoint),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformBoundsCore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IGeneralTransformOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            rect: ::windows::Foundation::Rect,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransformBoundsCore(::core::mem::transmute(&rect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<
                Identity,
                IGeneralTransformOverrides,
                OFFSET,
            >(),
            InverseCore: InverseCore::<Identity, Impl, OFFSET>,
            TryTransformCore: TryTransformCore::<Identity, Impl, OFFSET>,
            TransformBoundsCore: TransformBoundsCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneralTransformOverrides as ::windows::core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceManagerNative_Impl: Sized {
    fn FlushAllSurfacesWithDevice(
        &self,
        device: &::core::option::Option<::windows::core::IUnknown>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceManagerNative {}
impl ISurfaceImageSourceManagerNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceManagerNative_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceManagerNative_Vtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceManagerNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushAllSurfacesWithDevice(::core::mem::transmute(&device))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceManagerNative as ::windows::core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceNative_Impl: Sized {
    fn SetDevice(
        &self,
        device: &::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGIDevice>,
    ) -> ::windows::core::Result<()>;
    fn BeginDraw(
        &self,
        updaterect: &::windows::Win32::Foundation::RECT,
        surface: *mut ::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISurface>,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows::core::Result<()>;
    fn EndDraw(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceNative {}
impl ISurfaceImageSourceNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceNative_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn SetDevice<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: ::windows::Win32::Foundation::RECT,
            surface: *mut *mut ::core::ffi::c_void,
            offset: *mut ::windows::Win32::Foundation::POINT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(
                ::core::mem::transmute(&updaterect),
                ::core::mem::transmute_copy(&surface),
                ::core::mem::transmute_copy(&offset),
            )
            .into()
        }
        unsafe extern "system" fn EndDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceNativeWithD2D_Impl: Sized {
    fn SetDevice(
        &self,
        device: &::core::option::Option<::windows::core::IUnknown>,
    ) -> ::windows::core::Result<()>;
    fn BeginDraw(
        &self,
        updaterect: *const ::windows::Win32::Foundation::RECT,
        iid: *const ::windows::core::GUID,
        updateobject: *mut *mut ::core::ffi::c_void,
        offset: *mut ::windows::Win32::Foundation::POINT,
    ) -> ::windows::core::Result<()>;
    fn EndDraw(&self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&self) -> ::windows::core::Result<()>;
    fn ResumeDraw(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceNativeWithD2D {}
impl ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISurfaceImageSourceNativeWithD2D_Impl,
        const OFFSET: isize,
    >() -> ISurfaceImageSourceNativeWithD2D_Vtbl {
        unsafe extern "system" fn SetDevice<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            device: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: *const ::windows::Win32::Foundation::RECT,
            iid: *const ::windows::core::GUID,
            updateobject: *mut *mut ::core::ffi::c_void,
            offset: *mut ::windows::Win32::Foundation::POINT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDraw(
                ::core::mem::transmute_copy(&updaterect),
                ::core::mem::transmute_copy(&iid),
                ::core::mem::transmute_copy(&updateobject),
                ::core::mem::transmute_copy(&offset),
            )
            .into()
        }
        unsafe extern "system" fn EndDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISurfaceImageSourceNativeWithD2D_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeDraw().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNativeWithD2D as ::windows::core::Interface>::IID
    }
}
pub trait ISwapChainBackgroundPanelNative_Impl: Sized {
    fn SetSwapChain(
        &self,
        swapchain: &::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISwapChainBackgroundPanelNative {}
impl ISwapChainBackgroundPanelNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainBackgroundPanelNative_Impl,
        const OFFSET: isize,
    >() -> ISwapChainBackgroundPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainBackgroundPanelNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchain: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainBackgroundPanelNative as ::windows::core::Interface>::IID
    }
}
pub trait ISwapChainPanelNative_Impl: Sized {
    fn SetSwapChain(
        &self,
        swapchain: &::core::option::Option<::windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISwapChainPanelNative {}
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainPanelNative_Impl,
        const OFFSET: isize,
    >() -> ISwapChainPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainPanelNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchain: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as ::windows::core::Interface>::IID
    }
}
pub trait ISwapChainPanelNative2_Impl: Sized + ISwapChainPanelNative_Impl {
    fn SetSwapChainHandle(
        &self,
        swapchainhandle: ::windows::Win32::Foundation::HANDLE,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISwapChainPanelNative2 {}
impl ISwapChainPanelNative2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISwapChainPanelNative2_Impl,
        const OFFSET: isize,
    >() -> ISwapChainPanelNative2_Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISwapChainPanelNative2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            swapchainhandle: ::windows::Win32::Foundation::HANDLE,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSwapChainHandle(::core::mem::transmute_copy(&swapchainhandle))
                .into()
        }
        Self {
            base__: ISwapChainPanelNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSwapChainHandle: SetSwapChainHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative2 as ::windows::core::Interface>::IID
            || iid == &<ISwapChainPanelNative as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualSurfaceImageSourceNative_Impl: Sized + ISurfaceImageSourceNative_Impl {
    fn Invalidate(
        &self,
        updaterect: &::windows::Win32::Foundation::RECT,
    ) -> ::windows::core::Result<()>;
    fn GetUpdateRectCount(&self) -> ::windows::core::Result<u32>;
    fn GetUpdateRects(
        &self,
        updates: *mut ::windows::Win32::Foundation::RECT,
        count: u32,
    ) -> ::windows::core::Result<()>;
    fn GetVisibleBounds(&self) -> ::windows::core::Result<::windows::Win32::Foundation::RECT>;
    fn RegisterForUpdatesNeeded(
        &self,
        callback: &::core::option::Option<IVirtualSurfaceUpdatesCallbackNative>,
    ) -> ::windows::core::Result<()>;
    fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSourceNative {}
impl IVirtualSurfaceImageSourceNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualSurfaceImageSourceNative_Impl,
        const OFFSET: isize,
    >() -> IVirtualSurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn Invalidate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updaterect: ::windows::Win32::Foundation::RECT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate(::core::mem::transmute(&updaterect)).into()
        }
        unsafe extern "system" fn GetUpdateRectCount<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            count: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateRectCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRects<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            updates: *mut ::windows::Win32::Foundation::RECT,
            count: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpdateRects(
                ::core::mem::transmute_copy(&updates),
                ::core::mem::transmute_copy(&count),
            )
            .into()
        }
        unsafe extern "system" fn GetVisibleBounds<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bounds: *mut ::windows::Win32::Foundation::RECT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVisibleBounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bounds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForUpdatesNeeded(::core::mem::transmute(&callback))
                .into()
        }
        unsafe extern "system" fn Resize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceImageSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            newwidth: i32,
            newheight: i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(
                ::core::mem::transmute_copy(&newwidth),
                ::core::mem::transmute_copy(&newheight),
            )
            .into()
        }
        Self {
            base__: ISurfaceImageSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Identity, Impl, OFFSET>,
            GetUpdateRects: GetUpdateRects::<Identity, Impl, OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Identity, Impl, OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceNative as ::windows::core::Interface>::IID
            || iid == &<ISurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNative_Impl: Sized {
    fn UpdatesNeeded(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVirtualSurfaceUpdatesCallbackNative {}
impl IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IVirtualSurfaceUpdatesCallbackNative_Impl,
        const OFFSET: isize,
    >() -> IVirtualSurfaceUpdatesCallbackNative_Vtbl {
        unsafe extern "system" fn UpdatesNeeded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IVirtualSurfaceUpdatesCallbackNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdatesNeeded().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            UpdatesNeeded: UpdatesNeeded::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceUpdatesCallbackNative as ::windows::core::Interface>::IID
    }
}
pub trait IXamlCompositionBrushBaseOverrides_Impl: Sized {
    fn OnConnected(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDisconnected(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IXamlCompositionBrushBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlCompositionBrushBaseOverrides";
}
impl IXamlCompositionBrushBaseOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlCompositionBrushBaseOverrides_Impl,
        const OFFSET: isize,
    >() -> IXamlCompositionBrushBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected().into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlCompositionBrushBaseOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected().into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<
                Identity,
                IXamlCompositionBrushBaseOverrides,
                OFFSET,
            >(),
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlCompositionBrushBaseOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IXamlLightOverrides_Impl: Sized {
    fn GetId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OnConnected(
        &self,
        newelement: &::core::option::Option<super::UIElement>,
    ) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnDisconnected(
        &self,
        oldelement: &::core::option::Option<super::UIElement>,
    ) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
impl ::windows::core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightOverrides";
}
impl IXamlLightOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IXamlLightOverrides_Impl,
        const OFFSET: isize,
    >() -> IXamlLightOverrides_Vtbl {
        unsafe extern "system" fn GetId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnConnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            newelement: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnConnected(::core::mem::transmute(&newelement)).into()
        }
        unsafe extern "system" fn OnDisconnected<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IXamlLightOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            oldelement: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisconnected(::core::mem::transmute(&oldelement))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IXamlLightOverrides, OFFSET>(
            ),
            GetId: GetId::<Identity, Impl, OFFSET>,
            OnConnected: OnConnected::<Identity, Impl, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlLightOverrides as ::windows::core::Interface>::IID
    }
}
