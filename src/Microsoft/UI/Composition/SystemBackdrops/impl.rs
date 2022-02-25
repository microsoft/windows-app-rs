pub trait ISystemBackdropController_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn SetTargetWithWindowId(
        &self,
        windowid: &super::super::WindowId,
        desktopwindowtarget: &::core::option::Option<::windows::UI::Composition::CompositionTarget>,
    ) -> ::windows::core::Result<bool>;
    fn SetTargetWithCoreWindow(
        &self,
        corewindow: &::core::option::Option<::windows::UI::Core::CoreWindow>,
        compositiontarget: &::core::option::Option<::windows::UI::Composition::CompositionTarget>,
    ) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ISystemBackdropController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropController";
}
impl ISystemBackdropController_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: ISystemBackdropController_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropController_Vtbl {
        unsafe extern "system" fn SetTargetWithWindowId<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::super::WindowId,
            desktopwindowtarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetTargetWithWindowId(
                ::core::mem::transmute(&windowid),
                ::core::mem::transmute(&desktopwindowtarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetWithCoreWindow<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            corewindow: ::windows::core::RawPtr,
            compositiontarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetTargetWithCoreWindow(
                ::core::mem::transmute(&corewindow),
                ::core::mem::transmute(&compositiontarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<
                Identity,
                ISystemBackdropController,
                OFFSET,
            >(),
            SetTargetWithWindowId: SetTargetWithWindowId::<Identity, Impl, OFFSET>,
            SetTargetWithCoreWindow: SetTargetWithCoreWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemBackdropController as ::windows::core::Interface>::IID
    }
}
