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
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISystemBackdropController_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropController_Vtbl {
        unsafe extern "system" fn SetTargetWithWindowId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::super::WindowId,
            desktopwindowtarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetTargetWithWindowId(
                ::core::mem::transmute(&windowid),
                ::core::mem::transmute(&desktopwindowtarget),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetWithCoreWindow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropController_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            corewindow: ::windows::core::RawPtr,
            compositiontarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetTargetWithCoreWindow(
                ::core::mem::transmute(&corewindow),
                ::core::mem::transmute(&compositiontarget),
            ) {
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
pub trait ISystemBackdropControllerWithTargets_Impl:
    Sized + ::windows::Foundation::IClosable_Impl + ISystemBackdropController_Impl
{
    fn State(&self) -> ::windows::core::Result<SystemBackdropState>;
    fn AddSystemBackdropTarget(
        &self,
        systembackdroptarget: &::core::option::Option<super::ICompositionSupportsSystemBackdrop>,
    ) -> ::windows::core::Result<bool>;
    fn RemoveAllSystemBackdropTargets(&self) -> ::windows::core::Result<()>;
    fn RemoveSystemBackdropTarget(
        &self,
        systembackdroptarget: &::core::option::Option<super::ICompositionSupportsSystemBackdrop>,
    ) -> ::windows::core::Result<bool>;
    fn SetSystemBackdropConfiguration(
        &self,
        configuration: &::core::option::Option<SystemBackdropConfiguration>,
    ) -> ::windows::core::Result<()>;
    fn StateChanged(
        &self,
        handler: &::core::option::Option<
            ::windows::Foundation::TypedEventHandler<
                ISystemBackdropControllerWithTargets,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISystemBackdropControllerWithTargets {
    const NAME: &'static str =
        "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropControllerWithTargets";
}
impl ISystemBackdropControllerWithTargets_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ISystemBackdropControllerWithTargets_Impl,
        const OFFSET: isize,
    >() -> ISystemBackdropControllerWithTargets_Vtbl {
        unsafe extern "system" fn State<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut SystemBackdropState,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSystemBackdropTarget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            systembackdroptarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddSystemBackdropTarget(::core::mem::transmute(&systembackdroptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllSystemBackdropTargets<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllSystemBackdropTargets().into()
        }
        unsafe extern "system" fn RemoveSystemBackdropTarget<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            systembackdroptarget: ::windows::core::RawPtr,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoveSystemBackdropTarget(::core::mem::transmute(&systembackdroptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemBackdropConfiguration<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            configuration: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSystemBackdropConfiguration(::core::mem::transmute(&configuration))
                .into()
        }
        unsafe extern "system" fn StateChanged<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: ::windows::core::RawPtr,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StateChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ISystemBackdropControllerWithTargets_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStateChanged(::core::mem::transmute(&token))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<
                Identity,
                ISystemBackdropControllerWithTargets,
                OFFSET,
            >(),
            State: State::<Identity, Impl, OFFSET>,
            AddSystemBackdropTarget: AddSystemBackdropTarget::<Identity, Impl, OFFSET>,
            RemoveAllSystemBackdropTargets: RemoveAllSystemBackdropTargets::<Identity, Impl, OFFSET>,
            RemoveSystemBackdropTarget: RemoveSystemBackdropTarget::<Identity, Impl, OFFSET>,
            SetSystemBackdropConfiguration: SetSystemBackdropConfiguration::<Identity, Impl, OFFSET>,
            StateChanged: StateChanged::<Identity, Impl, OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemBackdropControllerWithTargets as ::windows::core::Interface>::IID
    }
}
