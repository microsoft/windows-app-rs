pub trait IDesktopWindowXamlSourceNative_Impl: Sized {
    fn AttachToWindow(
        &self,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::Result<()>;
    fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::core::Result<()>;
    fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::core::Result<()>;
}
impl IDesktopWindowXamlSourceNative_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IDesktopWindowXamlSourceNative_Impl,
        const OFFSET: isize,
    >() -> IDesktopWindowXamlSourceNative_Vtbl {
        unsafe extern "system" fn AttachToWindow<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IDesktopWindowXamlSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            parentwnd: ::windows::Win32::Foundation::HWND,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .AttachToWindow(::core::mem::transmute_copy(&parentwnd))
                .into()
        }
        unsafe extern "system" fn WindowHandle<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IDesktopWindowXamlSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            hwnd: *mut ::windows::Win32::Foundation::HWND,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .WindowHandle(::core::mem::transmute_copy(&hwnd))
                .into()
        }
        unsafe extern "system" fn PreTranslateMessage<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IDesktopWindowXamlSourceNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
            result: *mut ::windows::Win32::Foundation::BOOL,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .PreTranslateMessage(
                    ::core::mem::transmute_copy(&message),
                    ::core::mem::transmute_copy(&result),
                )
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachToWindow: AttachToWindow::<Identity, Impl, OFFSET>,
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
            PreTranslateMessage: PreTranslateMessage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait IFindReferenceTargetsCallback_Impl: Sized {
    fn FoundTrackerTarget(
        &self,
        target: &::core::option::Option<IReferenceTrackerTarget>,
    ) -> ::windows::core::Result<()>;
}
impl IFindReferenceTargetsCallback_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IFindReferenceTargetsCallback_Impl,
        const OFFSET: isize,
    >() -> IFindReferenceTargetsCallback_Vtbl {
        unsafe extern "system" fn FoundTrackerTarget<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IFindReferenceTargetsCallback_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .FoundTrackerTarget(::core::mem::transmute(&target))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FoundTrackerTarget: FoundTrackerTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindReferenceTargetsCallback as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTracker_Impl: Sized {
    fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn FindTrackerTargets(
        &self,
        callback: &::core::option::Option<IFindReferenceTargetsCallback>,
    ) -> ::windows::core::Result<()>;
    fn GetReferenceTrackerManager(&self) -> ::windows::core::Result<IReferenceTrackerManager>;
    fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn PegFromTrackerSource(&self) -> ::windows::core::Result<()>;
}
impl IReferenceTracker_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IReferenceTracker_Impl,
        const OFFSET: isize,
    >() -> IReferenceTracker_Vtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectFromTrackerSource().into()
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectFromTrackerSource().into()
        }
        unsafe extern "system" fn FindTrackerTargets<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            callback: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .FindTrackerTargets(::core::mem::transmute(&callback))
                .into()
        }
        unsafe extern "system" fn GetReferenceTrackerManager<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReferenceTrackerManager() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefFromTrackerSource().into()
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseFromTrackerSource().into()
        }
        unsafe extern "system" fn PegFromTrackerSource<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTracker_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PegFromTrackerSource().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Identity, Impl, OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Identity, Impl, OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Identity, Impl, OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Identity, Impl, OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Identity, Impl, OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Identity, Impl, OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTracker as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerExtension_Impl: Sized {}
impl IReferenceTrackerExtension_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IReferenceTrackerExtension_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerExtension_Vtbl {
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerExtension as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerHost_Impl: Sized {
    fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::core::Result<()>;
    fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()>;
    fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()>;
    fn GetTrackerTarget(
        &self,
        unknown: &::core::option::Option<::windows::core::IUnknown>,
    ) -> ::windows::core::Result<IReferenceTrackerTarget>;
    fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()>;
    fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerHost_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IReferenceTrackerHost_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerHost_Vtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            options : __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .DisconnectUnusedReferenceSources(::core::mem::transmute_copy(&options))
                .into()
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDisconnectedReferenceSources().into()
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyEndOfReferenceTrackingOnThread().into()
        }
        unsafe extern "system" fn GetTrackerTarget<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            unknown: *mut ::core::ffi::c_void,
            newreference: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrackerTarget(::core::mem::transmute(&unknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *newreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemoryPressure<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytesallocated: u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .AddMemoryPressure(::core::mem::transmute_copy(&bytesallocated))
                .into()
        }
        unsafe extern "system" fn RemoveMemoryPressure<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerHost_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            bytesallocated: u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .RemoveMemoryPressure(::core::mem::transmute_copy(&bytesallocated))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<
                Identity,
                Impl,
                OFFSET,
            >,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<
                Identity,
                Impl,
                OFFSET,
            >,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetTrackerTarget: GetTrackerTarget::<Identity, Impl, OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Identity, Impl, OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerHost as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerManager_Impl: Sized {
    fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()>;
    fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::core::Result<()>;
    fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()>;
    fn SetReferenceTrackerHost(
        &self,
        value: &::core::option::Option<IReferenceTrackerHost>,
    ) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerManager_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IReferenceTrackerManager_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerManager_Vtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReferenceTrackingStarted().into()
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            findfailed: u8,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .FindTrackerTargetsCompleted(::core::mem::transmute_copy(&findfailed))
                .into()
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReferenceTrackingCompleted().into()
        }
        unsafe extern "system" fn SetReferenceTrackerHost<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerManager_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .SetReferenceTrackerHost(::core::mem::transmute(&value))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Identity, Impl, OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Identity, Impl, OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Identity, Impl, OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerManager as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerTarget_Impl: Sized {
    fn AddRefFromReferenceTracker(&self) -> u32;
    fn ReleaseFromReferenceTracker(&self) -> u32;
    fn Peg(&self) -> ::windows::core::Result<()>;
    fn Unpeg(&self) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerTarget_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IReferenceTrackerTarget_Impl,
        const OFFSET: isize,
    >() -> IReferenceTrackerTarget_Vtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefFromReferenceTracker()
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseFromReferenceTracker()
        }
        unsafe extern "system" fn Peg<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Peg().into()
        }
        unsafe extern "system" fn Unpeg<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IReferenceTrackerTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unpeg().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Identity, Impl, OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Identity, Impl, OFFSET>,
            Peg: Peg::<Identity, Impl, OFFSET>,
            Unpeg: Unpeg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerTarget as ::windows::core::Interface>::IID
    }
}
pub trait ITrackerOwner_Impl: Sized {
    fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::core::Result<()>;
    fn DeleteTrackerHandle(&self, handle: *mut TrackerHandle__) -> ::windows::core::Result<()>;
    fn SetTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        value: &::core::option::Option<::windows::core::IUnknown>,
    ) -> ::windows::core::Result<()>;
    fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>,
    ) -> u8;
}
impl ITrackerOwner_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: ITrackerOwner_Impl,
        const OFFSET: isize,
    >() -> ITrackerOwner_Vtbl {
        unsafe extern "system" fn CreateTrackerHandle<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            returnvalue: *mut *mut TrackerHandle__,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .CreateTrackerHandle(::core::mem::transmute_copy(&returnvalue))
                .into()
        }
        unsafe extern "system" fn DeleteTrackerHandle<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .DeleteTrackerHandle(::core::mem::transmute_copy(&handle))
                .into()
        }
        unsafe extern "system" fn SetTrackerValue<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .SetTrackerValue(
                    ::core::mem::transmute_copy(&handle),
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<
            Identity: ::windows::core::IUnknownImpl,
            Impl: ITrackerOwner_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: *mut TrackerHandle__,
            returnvalue: *mut *mut ::core::ffi::c_void,
        ) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TryGetSafeTrackerValue(
                ::core::mem::transmute_copy(&handle),
                ::core::mem::transmute_copy(&returnvalue),
            )
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Identity, Impl, OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Identity, Impl, OFFSET>,
            SetTrackerValue: SetTrackerValue::<Identity, Impl, OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrackerOwner as ::windows::core::Interface>::IID
    }
}
