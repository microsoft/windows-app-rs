pub trait IExpCompositionPropertyChanged_Impl: Sized {
    fn SetPropertyChangedListener(
        &self,
        property: ExpExpressionNotificationProperty,
        listener: &::core::option::Option<IExpCompositionPropertyChangedListener>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IExpCompositionPropertyChanged {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Experimental.IExpCompositionPropertyChanged";
}
impl IExpCompositionPropertyChanged_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IExpCompositionPropertyChanged_Impl,
        const OFFSET: isize,
    >() -> IExpCompositionPropertyChanged_Vtbl {
        unsafe extern "system" fn SetPropertyChangedListener<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChanged_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            property: ExpExpressionNotificationProperty,
            listener: ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .SetPropertyChangedListener(property, ::core::mem::transmute(&listener))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<
                Identity,
                IExpCompositionPropertyChanged,
                OFFSET,
            >(),
            SetPropertyChangedListener: SetPropertyChangedListener::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpCompositionPropertyChanged as ::windows::core::Interface>::IID
    }
}
pub trait IExpCompositionPropertyChangedListener_Impl: Sized {
    fn NotifyBooleanPropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: bool,
    ) -> ::windows::core::Result<()>;
    fn NotifyColorPropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::UI::Color,
    ) -> ::windows::core::Result<()>;
    fn NotifyMatrix3x2PropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()>;
    fn NotifyMatrix4x4PropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()>;
    fn NotifyReferencePropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
    ) -> ::windows::core::Result<()>;
    fn NotifySinglePropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: f32,
    ) -> ::windows::core::Result<()>;
    fn NotifyVector2PropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>;
    fn NotifyVector3PropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::Result<()>;
    fn NotifyVector4PropertyChanged(
        &self,
        target: &::core::option::Option<super::CompositionObject>,
        property: ExpExpressionNotificationProperty,
        value: &::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IExpCompositionPropertyChangedListener {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Experimental.IExpCompositionPropertyChangedListener";
}
impl IExpCompositionPropertyChangedListener_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IExpCompositionPropertyChangedListener_Impl,
        const OFFSET: isize,
    >() -> IExpCompositionPropertyChangedListener_Vtbl {
        unsafe extern "system" fn NotifyBooleanPropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyBooleanPropertyChanged(::core::mem::transmute(&target), property, value)
                .into()
        }
        unsafe extern "system" fn NotifyColorPropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyColorPropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn NotifyMatrix3x2PropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::Foundation::Numerics::Matrix3x2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyMatrix3x2PropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn NotifyMatrix4x4PropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::Foundation::Numerics::Matrix4x4,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyMatrix4x4PropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn NotifyReferencePropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyReferencePropertyChanged(::core::mem::transmute(&target), property)
                .into()
        }
        unsafe extern "system" fn NotifySinglePropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifySinglePropertyChanged(::core::mem::transmute(&target), property, value)
                .into()
        }
        unsafe extern "system" fn NotifyVector2PropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::Foundation::Numerics::Vector2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyVector2PropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn NotifyVector3PropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::Foundation::Numerics::Vector3,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyVector3PropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn NotifyVector4PropertyChanged<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositionPropertyChangedListener_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            target: ::windows::core::RawPtr,
            property: ExpExpressionNotificationProperty,
            value: ::windows::Foundation::Numerics::Vector4,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .NotifyVector4PropertyChanged(
                    ::core::mem::transmute(&target),
                    property,
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<
                Identity,
                IExpCompositionPropertyChangedListener,
                OFFSET,
            >(),
            NotifyBooleanPropertyChanged: NotifyBooleanPropertyChanged::<Identity, Impl, OFFSET>,
            NotifyColorPropertyChanged: NotifyColorPropertyChanged::<Identity, Impl, OFFSET>,
            NotifyMatrix3x2PropertyChanged: NotifyMatrix3x2PropertyChanged::<Identity, Impl, OFFSET>,
            NotifyMatrix4x4PropertyChanged: NotifyMatrix4x4PropertyChanged::<Identity, Impl, OFFSET>,
            NotifyReferencePropertyChanged: NotifyReferencePropertyChanged::<Identity, Impl, OFFSET>,
            NotifySinglePropertyChanged: NotifySinglePropertyChanged::<Identity, Impl, OFFSET>,
            NotifyVector2PropertyChanged: NotifyVector2PropertyChanged::<Identity, Impl, OFFSET>,
            NotifyVector3PropertyChanged: NotifyVector3PropertyChanged::<Identity, Impl, OFFSET>,
            NotifyVector4PropertyChanged: NotifyVector4PropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpCompositionPropertyChangedListener as ::windows::core::Interface>::IID
    }
}
pub trait IExpCompositor_Impl: Sized {
    fn OpenSharedManipulationTransformFromHandle(
        &self,
        handle: u64,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IExpCompositor {
    const NAME: &'static str = "Microsoft.UI.Composition.Experimental.IExpCompositor";
}
impl IExpCompositor_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IExpCompositor_Impl,
        const OFFSET: isize,
    >() -> IExpCompositor_Vtbl {
        unsafe extern "system" fn OpenSharedManipulationTransformFromHandle<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpCompositor_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handle: u64,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenSharedManipulationTransformFromHandle(handle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IExpCompositor, OFFSET>(),
            OpenSharedManipulationTransformFromHandle: OpenSharedManipulationTransformFromHandle::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpCompositor as ::windows::core::Interface>::IID
    }
}
pub trait IExpVisual_Impl: Sized {
    fn SetInteraction(
        &self,
        interaction: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IExpVisual {
    const NAME: &'static str = "Microsoft.UI.Composition.Experimental.IExpVisual";
}
impl IExpVisual_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IExpVisual_Impl,
        const OFFSET: isize,
    >() -> IExpVisual_Vtbl {
        unsafe extern "system" fn SetInteraction<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IExpVisual_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            interaction: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .SetInteraction(::core::mem::transmute(&interaction))
                .into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IExpVisual, OFFSET>(),
            SetInteraction: SetInteraction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExpVisual as ::windows::core::Interface>::IID
    }
}
