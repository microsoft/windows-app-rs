pub trait IComponentConnector_Impl: Sized {
    fn Connect(
        &self,
        connectionid: i32,
        target: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
    fn GetBindingConnector(
        &self,
        connectionid: i32,
        target: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<IComponentConnector>;
}
impl ::windows::core::RuntimeName for IComponentConnector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IComponentConnector";
}
impl IComponentConnector_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IComponentConnector_Impl,
        const OFFSET: isize,
    >() -> IComponentConnector_Vtbl {
        unsafe extern "system" fn Connect<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IComponentConnector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionid: i32,
            target: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .Connect(connectionid, ::core::mem::transmute(&target))
                .into()
        }
        unsafe extern "system" fn GetBindingConnector<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IComponentConnector_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            connectionid: i32,
            target: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBindingConnector(connectionid, ::core::mem::transmute(&target)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComponentConnector, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            GetBindingConnector: GetBindingConnector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentConnector as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateComponent_Impl: Sized {
    fn Recycle(&self) -> ::windows::core::Result<()>;
    fn ProcessBindings(
        &self,
        item: &::core::option::Option<::windows::core::IInspectable>,
        itemindex: i32,
        phase: i32,
        nextphase: &mut i32,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataTemplateComponent {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IDataTemplateComponent";
}
impl IDataTemplateComponent_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IDataTemplateComponent_Impl,
        const OFFSET: isize,
    >() -> IDataTemplateComponent_Vtbl {
        unsafe extern "system" fn Recycle<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IDataTemplateComponent_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Recycle().into()
        }
        unsafe extern "system" fn ProcessBindings<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IDataTemplateComponent_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            item: *mut ::core::ffi::c_void,
            itemindex: i32,
            phase: i32,
            nextphase: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .ProcessBindings(
                    ::core::mem::transmute(&item),
                    itemindex,
                    phase,
                    ::core::mem::transmute_copy(&nextphase),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateComponent, OFFSET>(
            ),
            Recycle: Recycle::<Identity, Impl, OFFSET>,
            ProcessBindings: ProcessBindings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateComponent as ::windows::core::Interface>::IID
    }
}
pub trait IMarkupExtensionOverrides_Impl: Sized {
    fn ProvideValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ProvideValueWithIXamlServiceProvider(
        &self,
        serviceprovider: &::core::option::Option<super::IXamlServiceProvider>,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IMarkupExtensionOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IMarkupExtensionOverrides";
}
impl IMarkupExtensionOverrides_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IMarkupExtensionOverrides_Impl,
        const OFFSET: isize,
    >() -> IMarkupExtensionOverrides_Vtbl {
        unsafe extern "system" fn ProvideValue<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IMarkupExtensionOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProvideValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvideValueWithIXamlServiceProvider<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IMarkupExtensionOverrides_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            serviceprovider: ::windows::core::RawPtr,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)
                .ProvideValueWithIXamlServiceProvider(::core::mem::transmute(&serviceprovider))
            {
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
                IMarkupExtensionOverrides,
                OFFSET,
            >(),
            ProvideValue: ProvideValue::<Identity, Impl, OFFSET>,
            ProvideValueWithIXamlServiceProvider: ProvideValueWithIXamlServiceProvider::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtensionOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IProvideValueTarget_Impl: Sized {
    fn TargetObject(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn TargetProperty(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IProvideValueTarget {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IProvideValueTarget";
}
impl IProvideValueTarget_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IProvideValueTarget_Impl,
        const OFFSET: isize,
    >() -> IProvideValueTarget_Vtbl {
        unsafe extern "system" fn TargetObject<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IProvideValueTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetProperty<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IProvideValueTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProvideValueTarget, OFFSET>(),
            TargetObject: TargetObject::<Identity, Impl, OFFSET>,
            TargetProperty: TargetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvideValueTarget as ::windows::core::Interface>::IID
    }
}
pub trait IRootObjectProvider_Impl: Sized {
    fn RootObject(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IRootObjectProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IRootObjectProvider";
}
impl IRootObjectProvider_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IRootObjectProvider_Impl,
        const OFFSET: isize,
    >() -> IRootObjectProvider_Vtbl {
        unsafe extern "system" fn RootObject<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IRootObjectProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRootObjectProvider, OFFSET>(),
            RootObject: RootObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRootObjectProvider as ::windows::core::Interface>::IID
    }
}
pub trait IUriContext_Impl: Sized {
    fn BaseUri(&self) -> ::windows::core::Result<::windows::Foundation::Uri>;
}
impl ::windows::core::RuntimeName for IUriContext {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IUriContext";
}
impl IUriContext_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IUriContext_Impl,
        const OFFSET: isize,
    >() -> IUriContext_Vtbl {
        unsafe extern "system" fn BaseUri<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IUriContext_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BaseUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriContext, OFFSET>(),
            BaseUri: BaseUri::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriContext as ::windows::core::Interface>::IID
    }
}
pub trait IXamlBindScopeDiagnostics_Impl: Sized {
    fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlBindScopeDiagnostics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
}
impl IXamlBindScopeDiagnostics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IXamlBindScopeDiagnostics_Impl,
        const OFFSET: isize,
    >() -> IXamlBindScopeDiagnostics_Vtbl {
        unsafe extern "system" fn Disable<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlBindScopeDiagnostics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            linenumber: i32,
            columnnumber: i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disable(linenumber, columnnumber).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<
                Identity,
                IXamlBindScopeDiagnostics,
                OFFSET,
            >(),
            Disable: Disable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindScopeDiagnostics as ::windows::core::Interface>::IID
    }
}
pub trait IXamlMember_Impl: Sized {
    fn IsAttachable(&self) -> ::windows::core::Result<bool>;
    fn IsDependencyProperty(&self) -> ::windows::core::Result<bool>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TargetType(&self) -> ::windows::core::Result<IXamlType>;
    fn Type(&self) -> ::windows::core::Result<IXamlType>;
    fn GetValue(
        &self,
        instance: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(
        &self,
        instance: &::core::option::Option<::windows::core::IInspectable>,
        value: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlMember {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMember";
}
impl IXamlMember_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IXamlMember_Impl,
        const OFFSET: isize,
    >() -> IXamlMember_Vtbl {
        unsafe extern "system" fn IsAttachable<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsAttachable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDependencyProperty<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDependencyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute(&instance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMember_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .SetValue(
                    ::core::mem::transmute(&instance),
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMember, OFFSET>(),
            IsAttachable: IsAttachable::<Identity, Impl, OFFSET>,
            IsDependencyProperty: IsDependencyProperty::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            TargetType: TargetType::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMember as ::windows::core::Interface>::IID
    }
}
pub trait IXamlMetadataProvider_Impl: Sized {
    fn GetXamlType(
        &self,
        r#type: &::windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows::core::Result<IXamlType>;
    fn GetXamlTypeByFullName(
        &self,
        fullname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<IXamlType>;
    fn GetXmlnsDefinitions(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>>;
}
impl ::windows::core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider";
}
impl IXamlMetadataProvider_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IXamlMetadataProvider_Impl,
        const OFFSET: isize,
    >() -> IXamlMetadataProvider_Vtbl {
        unsafe extern "system" fn GetXamlType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXamlType(::core::mem::transmute(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlTypeByFullName<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXamlTypeByFullName(::core::mem::transmute(&fullname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXmlnsDefinitions<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlMetadataProvider_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXmlnsDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMetadataProvider, OFFSET>(
            ),
            GetXamlType: GetXamlType::<Identity, Impl, OFFSET>,
            GetXamlTypeByFullName: GetXamlTypeByFullName::<Identity, Impl, OFFSET>,
            GetXmlnsDefinitions: GetXmlnsDefinitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMetadataProvider as ::windows::core::Interface>::IID
    }
}
pub trait IXamlType_Impl: Sized {
    fn BaseType(&self) -> ::windows::core::Result<IXamlType>;
    fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsArray(&self) -> ::windows::core::Result<bool>;
    fn IsCollection(&self) -> ::windows::core::Result<bool>;
    fn IsConstructible(&self) -> ::windows::core::Result<bool>;
    fn IsDictionary(&self) -> ::windows::core::Result<bool>;
    fn IsMarkupExtension(&self) -> ::windows::core::Result<bool>;
    fn IsBindable(&self) -> ::windows::core::Result<bool>;
    fn ItemType(&self) -> ::windows::core::Result<IXamlType>;
    fn KeyType(&self) -> ::windows::core::Result<IXamlType>;
    fn BoxedType(&self) -> ::windows::core::Result<IXamlType>;
    fn UnderlyingType(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName>;
    fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateFromString(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetMember(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlMember>;
    fn AddToVector(
        &self,
        instance: &::core::option::Option<::windows::core::IInspectable>,
        value: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
    fn AddToMap(
        &self,
        instance: &::core::option::Option<::windows::core::IInspectable>,
        key: &::core::option::Option<::windows::core::IInspectable>,
        value: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
    fn RunInitializer(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlType {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlType";
}
impl IXamlType_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IXamlType_Impl,
        const OFFSET: isize,
    >() -> IXamlType_Vtbl {
        unsafe extern "system" fn BaseType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BaseType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProperty<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullName<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCollection<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConstructible<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsConstructible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDictionary<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMarkupExtension<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMarkupExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBindable<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsBindable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoxedType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoxedType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlyingType<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnderlyingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateInstance<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromString<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFromString(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMember<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMember(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToVector<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .AddToVector(
                    ::core::mem::transmute(&instance),
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn AddToMap<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            instance: *mut ::core::ffi::c_void,
            key: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .AddToMap(
                    ::core::mem::transmute(&instance),
                    ::core::mem::transmute(&key),
                    ::core::mem::transmute(&value),
                )
                .into()
        }
        unsafe extern "system" fn RunInitializer<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlType_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RunInitializer().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlType, OFFSET>(),
            BaseType: BaseType::<Identity, Impl, OFFSET>,
            ContentProperty: ContentProperty::<Identity, Impl, OFFSET>,
            FullName: FullName::<Identity, Impl, OFFSET>,
            IsArray: IsArray::<Identity, Impl, OFFSET>,
            IsCollection: IsCollection::<Identity, Impl, OFFSET>,
            IsConstructible: IsConstructible::<Identity, Impl, OFFSET>,
            IsDictionary: IsDictionary::<Identity, Impl, OFFSET>,
            IsMarkupExtension: IsMarkupExtension::<Identity, Impl, OFFSET>,
            IsBindable: IsBindable::<Identity, Impl, OFFSET>,
            ItemType: ItemType::<Identity, Impl, OFFSET>,
            KeyType: KeyType::<Identity, Impl, OFFSET>,
            BoxedType: BoxedType::<Identity, Impl, OFFSET>,
            UnderlyingType: UnderlyingType::<Identity, Impl, OFFSET>,
            ActivateInstance: ActivateInstance::<Identity, Impl, OFFSET>,
            CreateFromString: CreateFromString::<Identity, Impl, OFFSET>,
            GetMember: GetMember::<Identity, Impl, OFFSET>,
            AddToVector: AddToVector::<Identity, Impl, OFFSET>,
            AddToMap: AddToMap::<Identity, Impl, OFFSET>,
            RunInitializer: RunInitializer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlType as ::windows::core::Interface>::IID
    }
}
pub trait IXamlTypeResolver_Impl: Sized {
    fn Resolve(
        &self,
        qualifiedtypename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName>;
}
impl ::windows::core::RuntimeName for IXamlTypeResolver {
    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlTypeResolver";
}
impl IXamlTypeResolver_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl,
        Impl: IXamlTypeResolver_Impl,
        const OFFSET: isize,
    >() -> IXamlTypeResolver_Vtbl {
        unsafe extern "system" fn Resolve<
            Identity: ::windows::core::IUnknownImpl,
            Impl: IXamlTypeResolver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            qualifiedtypename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Resolve(::core::mem::transmute(&qualifiedtypename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlTypeResolver, OFFSET>(),
            Resolve: Resolve::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlTypeResolver as ::windows::core::Interface>::IID
    }
}
