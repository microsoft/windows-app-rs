#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct Binding(::windows::core::IUnknown);
impl Binding {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Path)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::PropertyPath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::PropertyPath>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPath)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<BindingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<BindingMode>::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<BindingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetMode(&self, value: BindingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMode)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Source)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSource)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RelativeSource(&self) -> ::windows::core::Result<RelativeSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).RelativeSource)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<RelativeSource>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetRelativeSource<'a, Param0: ::windows::core::IntoParam<'a, RelativeSource>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetRelativeSource)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ElementName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).ElementName)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetElementName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetElementName)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Converter(&self) -> ::windows::core::Result<IValueConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Converter)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<IValueConverter>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverter<'a, Param0: ::windows::core::IntoParam<'a, IValueConverter>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetConverter)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ConverterParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ConverterParameter)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverterParameter<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetConverterParameter)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ConverterLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).ConverterLanguage)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverterLanguage<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetConverterLanguage)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn FallbackValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FallbackValue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetFallbackValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetFallbackValue)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn TargetNullValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TargetNullValue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetTargetNullValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTargetNullValue)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UpdateSourceTrigger(&self) -> ::windows::core::Result<UpdateSourceTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UpdateSourceTrigger>::zeroed();
            (::windows::core::Interface::vtable(this).UpdateSourceTrigger)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<UpdateSourceTrigger>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetUpdateSourceTrigger(
        &self,
        value: UpdateSourceTrigger,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetUpdateSourceTrigger)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<Binding> {
        Self::IBindingFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<Binding>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<Binding> {
        Self::IBindingFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<Binding>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBindingFactory<R, F: FnOnce(&IBindingFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Binding, IBindingFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Binding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Binding {}
impl ::core::fmt::Debug for Binding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Binding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Binding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.Binding;{501ea0e8-edd4-59de-8845-76af2eabbe00})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Binding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows::core::GUID = <IBinding as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Binding {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.Binding";
}
impl ::core::convert::From<Binding> for ::windows::core::IUnknown {
    fn from(value: Binding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Binding> for ::windows::core::IUnknown {
    fn from(value: &Binding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Binding> for ::windows::core::IInspectable {
    fn from(value: Binding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Binding> for ::windows::core::IInspectable {
    fn from(value: &Binding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Binding> for BindingBase {
    fn from(value: Binding) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Binding> for BindingBase {
    fn from(value: &Binding) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingBase> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, BindingBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingBase> for &Binding {
    fn into_param(self) -> ::windows::core::Param<'a, BindingBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<BindingBase>::into(self))
    }
}
impl ::core::convert::From<Binding> for super::DependencyObject {
    fn from(value: Binding) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Binding> for super::DependencyObject {
    fn from(value: &Binding) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Binding {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Binding {}
unsafe impl ::core::marker::Sync for Binding {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingBase(::windows::core::IUnknown);
impl BindingBase {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<BindingBase> {
        Self::IBindingBaseFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<BindingBase>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<BindingBase> {
        Self::IBindingBaseFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<BindingBase>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBindingBaseFactory<R, F: FnOnce(&IBindingBaseFactory) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BindingBase, IBindingBaseFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BindingBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingBase {}
impl ::core::fmt::Debug for BindingBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.BindingBase;{91ddd141-5944-50ef-b85e-218e463f7a73})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingBase {
    type Vtable = IBindingBase_Vtbl;
    const IID: ::windows::core::GUID = <IBindingBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingBase";
}
impl ::core::convert::From<BindingBase> for ::windows::core::IUnknown {
    fn from(value: BindingBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingBase> for ::windows::core::IUnknown {
    fn from(value: &BindingBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingBase> for ::windows::core::IInspectable {
    fn from(value: BindingBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingBase> for ::windows::core::IInspectable {
    fn from(value: &BindingBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingBase> for super::DependencyObject {
    fn from(value: BindingBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BindingBase> for super::DependencyObject {
    fn from(value: &BindingBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BindingBase {}
unsafe impl ::core::marker::Sync for BindingBase {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingExpression(::windows::core::IUnknown);
impl BindingExpression {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn DataItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DataItem)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ParentBinding(&self) -> ::windows::core::Result<Binding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).ParentBinding)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<Binding>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UpdateSource(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).UpdateSource)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
}
impl ::core::clone::Clone for BindingExpression {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingExpression {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingExpression {}
impl ::core::fmt::Debug for BindingExpression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingExpression").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingExpression {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.BindingExpression;{4c023916-37bc-5b07-bc9d-15c547bd9b26})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingExpression {
    type Vtable = IBindingExpression_Vtbl;
    const IID: ::windows::core::GUID = <IBindingExpression as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingExpression {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingExpression";
}
impl ::core::convert::From<BindingExpression> for ::windows::core::IUnknown {
    fn from(value: BindingExpression) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpression> for ::windows::core::IUnknown {
    fn from(value: &BindingExpression) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpression> for ::windows::core::IInspectable {
    fn from(value: BindingExpression) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpression> for ::windows::core::IInspectable {
    fn from(value: &BindingExpression) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpression> for BindingExpressionBase {
    fn from(value: BindingExpression) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BindingExpression> for BindingExpressionBase {
    fn from(value: &BindingExpression) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingExpressionBase> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, BindingExpressionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingExpressionBase> for &BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, BindingExpressionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<BindingExpressionBase>::into(self))
    }
}
unsafe impl ::core::marker::Send for BindingExpression {}
unsafe impl ::core::marker::Sync for BindingExpression {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingExpressionBase(::windows::core::IUnknown);
impl BindingExpressionBase {}
impl ::core::clone::Clone for BindingExpressionBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingExpressionBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingExpressionBase {}
impl ::core::fmt::Debug for BindingExpressionBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingExpressionBase")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingExpressionBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.BindingExpressionBase;{8825e5a9-d9a3-5e87-bcd8-c63133d29029})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
    const IID: ::windows::core::GUID = <IBindingExpressionBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingExpressionBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingExpressionBase";
}
impl ::core::convert::From<BindingExpressionBase> for ::windows::core::IUnknown {
    fn from(value: BindingExpressionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpressionBase> for ::windows::core::IUnknown {
    fn from(value: &BindingExpressionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpressionBase> for ::windows::core::IInspectable {
    fn from(value: BindingExpressionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpressionBase> for ::windows::core::IInspectable {
    fn from(value: &BindingExpressionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a BindingExpressionBase
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BindingExpressionBase {}
unsafe impl ::core::marker::Sync for BindingExpressionBase {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
impl ::core::marker::Copy for BindingMode {}
impl ::core::clone::Clone for BindingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BindingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BindingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BindingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingMode {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Data.BindingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingOperations(::windows::core::IUnknown);
impl BindingOperations {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetBinding<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param2: ::windows::core::IntoParam<'a, BindingBase>,
    >(
        target: Param0,
        dp: Param1,
        binding: Param2,
    ) -> ::windows::core::Result<()> {
        Self::IBindingOperationsStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).SetBinding)(
                ::windows::core::Interface::as_raw(this),
                target.into_param().abi(),
                dp.into_param().abi(),
                binding.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc(hidden)]
    pub fn IBindingOperationsStatics<
        R,
        F: FnOnce(&IBindingOperationsStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            BindingOperations,
            IBindingOperationsStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BindingOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingOperations {}
impl ::core::fmt::Debug for BindingOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.BindingOperations;{9a319b95-aabe-5075-b227-8eb07e443d8b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingOperations {
    type Vtable = IBindingOperations_Vtbl;
    const IID: ::windows::core::GUID = <IBindingOperations as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingOperations {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.BindingOperations";
}
impl ::core::convert::From<BindingOperations> for ::windows::core::IUnknown {
    fn from(value: BindingOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingOperations> for ::windows::core::IUnknown {
    fn from(value: &BindingOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingOperations> for ::windows::core::IInspectable {
    fn from(value: BindingOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingOperations> for ::windows::core::IInspectable {
    fn from(value: &BindingOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BindingOperations {}
unsafe impl ::core::marker::Sync for BindingOperations {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CollectionViewSource(::windows::core::IUnknown);
impl CollectionViewSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CollectionViewSource,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Source)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetSource)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn View(&self) -> ::windows::core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).View)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ICollectionView>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSourceGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsSourceGrouped)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetIsSourceGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsSourceGrouped)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ItemsPath(&self) -> ::windows::core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).ItemsPath)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::PropertyPath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetItemsPath<'a, Param0: ::windows::core::IntoParam<'a, super::PropertyPath>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetItemsPath)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).SourceProperty)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ViewProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).ViewProperty)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSourceGroupedProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).IsSourceGroupedProperty)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ItemsPathProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).ItemsPathProperty)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICollectionViewSourceStatics<
        R,
        F: FnOnce(&ICollectionViewSourceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CollectionViewSource,
            ICollectionViewSourceStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CollectionViewSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CollectionViewSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CollectionViewSource {}
impl ::core::fmt::Debug for CollectionViewSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CollectionViewSource")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CollectionViewSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.CollectionViewSource;{a45e3b3a-f31e-5bbb-8a7c-70cf5c64bc3f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
    const IID: ::windows::core::GUID = <ICollectionViewSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CollectionViewSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.CollectionViewSource";
}
impl ::core::convert::From<CollectionViewSource> for ::windows::core::IUnknown {
    fn from(value: CollectionViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CollectionViewSource> for ::windows::core::IUnknown {
    fn from(value: &CollectionViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CollectionViewSource> for ::windows::core::IInspectable {
    fn from(value: CollectionViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CollectionViewSource> for ::windows::core::IInspectable {
    fn from(value: &CollectionViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CollectionViewSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CollectionViewSource> for super::DependencyObject {
    fn from(value: CollectionViewSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CollectionViewSource> for super::DependencyObject {
    fn from(value: &CollectionViewSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CollectionViewSource {}
unsafe impl ::core::marker::Sync for CollectionViewSource {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CurrentChangingEventArgs(::windows::core::IUnknown);
impl CurrentChangingEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCancelable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsCancelable)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateWithCancelableParameter(
        iscancelable: bool,
    ) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCancelableParameter)(
                ::windows::core::Interface::as_raw(this),
                iscancelable,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateWithCancelableParameter_compose<T: ::windows::core::Compose>(
        iscancelable: bool,
        compose: T,
    ) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCancelableParameter)(
                ::windows::core::Interface::as_raw(this),
                iscancelable,
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrentChangingEventArgsFactory<
        R,
        F: FnOnce(&ICurrentChangingEventArgsFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CurrentChangingEventArgs,
            ICurrentChangingEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CurrentChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentChangingEventArgs {}
impl ::core::fmt::Debug for CurrentChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentChangingEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentChangingEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Data.CurrentChangingEventArgs;{63e42ed6-e14a-51ea-9cb1-72f9c907dc80})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <ICurrentChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CurrentChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.CurrentChangingEventArgs";
}
impl ::core::convert::From<CurrentChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CurrentChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CurrentChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrentChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CurrentChangingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CurrentChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CurrentChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CurrentChangingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CurrentChangingEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CurrentChangingEventArgs {}
unsafe impl ::core::marker::Sync for CurrentChangingEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CurrentChangingEventHandler(pub ::windows::core::IUnknown);
impl CurrentChangingEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<CurrentChangingEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = CurrentChangingEventHandlerBox::<F> {
            vtable: &CurrentChangingEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, CurrentChangingEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Invoke)(
                ::windows::core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct CurrentChangingEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<CurrentChangingEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const CurrentChangingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<CurrentChangingEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > CurrentChangingEventHandlerBox<F>
{
    const VTABLE: CurrentChangingEventHandler_Vtbl = CurrentChangingEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<CurrentChangingEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for CurrentChangingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentChangingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentChangingEventHandler {}
impl ::core::fmt::Debug for CurrentChangingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentChangingEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for CurrentChangingEventHandler {
    type Vtable = CurrentChangingEventHandler_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3d2a98dd_95b3_5fd5_93b4_a1a2599f225c);
}
unsafe impl ::windows::core::RuntimeType for CurrentChangingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{3d2a98dd-95b3-5fd5-93b4-a1a2599f225c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CurrentChangingEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct DataErrorsChangedEventArgs(::windows::core::IUnknown);
impl DataErrorsChangedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn PropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).PropertyName)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetPropertyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPropertyName)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        name: Param0,
    ) -> ::windows::core::Result<DataErrorsChangedEventArgs> {
        Self::IDataErrorsChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                name.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<DataErrorsChangedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataErrorsChangedEventArgsFactory<
        R,
        F: FnOnce(&IDataErrorsChangedEventArgsFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DataErrorsChangedEventArgs,
            IDataErrorsChangedEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DataErrorsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataErrorsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataErrorsChangedEventArgs {}
impl ::core::fmt::Debug for DataErrorsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataErrorsChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataErrorsChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Data.DataErrorsChangedEventArgs;{d026dd64-5f26-5f15-a86a-0dec8a431796})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DataErrorsChangedEventArgs {
    type Vtable = IDataErrorsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IDataErrorsChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataErrorsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.DataErrorsChangedEventArgs";
}
impl ::core::convert::From<DataErrorsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DataErrorsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataErrorsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DataErrorsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DataErrorsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a DataErrorsChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DataErrorsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DataErrorsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DataErrorsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DataErrorsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for DataErrorsChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DataErrorsChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DataErrorsChangedEventArgs {}
unsafe impl ::core::marker::Sync for DataErrorsChangedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinding(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBinding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x501ea0e8_edd4_59de_8845_76af2eabbe00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Path: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut BindingMode,
    ) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: BindingMode,
    ) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RelativeSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetRelativeSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ElementName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetElementName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Converter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetConverter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ConverterParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetConverterParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ConverterLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetConverterLanguage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub FallbackValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFallbackValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TargetNullValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTargetNullValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub UpdateSourceTrigger: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut UpdateSourceTrigger,
    ) -> ::windows::core::HRESULT,
    pub SetUpdateSourceTrigger: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: UpdateSourceTrigger,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingBase {
    type Vtable = IBindingBase_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91ddd141_5944_50ef_b85e_218e463f7a73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBase_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingBaseFactory {
    type Vtable = IBindingBaseFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc8a866c5_f6f3_5f7a_9592_d385af48bd8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBaseFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpression(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpression {
    type Vtable = IBindingExpression_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4c023916_37bc_5b07_bc9d_15c547bd9b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpression_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DataItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ParentBinding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub UpdateSource:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8825e5a9_d9a3_5e87_bcd8_c63133d29029);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBase_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionBaseFactory {
    type Vtable = IBindingExpressionBaseFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x41d643b9_2629_5451_a716_596c0848b5dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBaseFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionFactory {
    type Vtable = IBindingExpressionFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x086cae14_81a1_588b_b619_05ee84c0f089);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingFactory {
    type Vtable = IBindingFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcb2de749_b115_5f67_b64a_797d54885d5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingOperations(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingOperations {
    type Vtable = IBindingOperations_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9a319b95_aabe_5075_b227_8eb07e443d8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperations_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingOperationsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingOperationsStatics {
    type Vtable = IBindingOperationsStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1e1bdbd3_fca5_5c85_b87d_b504cd8fa8ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperationsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetBinding: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: ::windows::core::RawPtr,
        dp: ::windows::core::RawPtr,
        binding: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionView(::windows::core::IUnknown);
impl ICollectionView {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CurrentItem)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).CurrentPosition)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCurrentAfterLast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAfterLast)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCurrentBeforeFirst(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentBeforeFirst)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CollectionGroups(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CollectionGroups)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IObservableVector<
                ::windows::core::IInspectable,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn HasMoreItems(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HasMoreItems)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<::windows::core::IInspectable>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).CurrentChanged)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveCurrentChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCurrentChanged)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentChanging<
        'a,
        Param0: ::windows::core::IntoParam<'a, CurrentChangingEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).CurrentChanging)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveCurrentChanging<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveCurrentChanging)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentTo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        item: Param0,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentTo)(
                ::windows::core::Interface::as_raw(this),
                item.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToPosition(&self, index: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToPosition)(
                ::windows::core::Interface::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToFirst(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToFirst)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToLast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToLast)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToNext)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToPrevious(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToPrevious)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).LoadMoreItemsAsync)(
                ::windows::core::Interface::as_raw(this),
                count,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn First(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterator<::windows::core::IInspectable>,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).First)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ::windows::core::IInspectable,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn VectorChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::VectorChangedEventHandler<
                ::windows::core::IInspectable,
            >,
        >,
    >(
        &self,
        vhnd: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).VectorChanged)(
                ::windows::core::Interface::as_raw(this),
                vhnd.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveVectorChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveVectorChanged)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(
                ::windows::core::Interface::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Size)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetView(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView<::windows::core::IInspectable>,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<
                ::windows::core::IInspectable,
            >>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
        index: &mut u32,
    ) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetAt)(
                ::windows::core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        index: u32,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).InsertAt)(
                ::windows::core::Interface::as_raw(this),
                index,
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAt)(
                ::windows::core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Append)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAtEnd)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [::core::option::Option<::windows::core::IInspectable>],
    ) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(
                ::windows::core::Interface::as_raw(this),
                startindex,
                items.len() as u32,
                ::core::mem::transmute_copy(&items),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ReplaceAll(
        &self,
        items: &[::core::option::Option<::windows::core::IInspectable>],
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ReplaceAll)(
                ::windows::core::Interface::as_raw(this),
                items.len() as u32,
                ::core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<ICollectionView> for ::windows::core::IUnknown {
    fn from(value: ICollectionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionView> for ::windows::core::IUnknown {
    fn from(value: &ICollectionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionView> for ::windows::core::IInspectable {
    fn from(value: ICollectionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionView> for ::windows::core::IInspectable {
    fn from(value: &ICollectionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICollectionView>
    for ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICollectionView>
    for ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
    > for ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
    > for &ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ICollectionView>
    for ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICollectionView>
    for ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > for ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > for &ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ICollectionView>
    for ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICollectionView>
    for ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
    > for ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
    > for &ICollectionView
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IVector<::windows::core::IInspectable>,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ICollectionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionView {}
impl ::core::fmt::Debug for ICollectionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionView {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{f8bb90d8-e008-5d65-8c97-7bb790a4230c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
impl ::core::iter::IntoIterator for ICollectionView {
    type Item = ::windows::core::IInspectable;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
impl ::core::iter::IntoIterator for &ICollectionView {
    type Item = ::windows::core::IInspectable;
    type IntoIter = ::windows::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::windows::Foundation::Collections::VectorIterator::new(
            ::core::convert::TryInto::try_into(self).ok(),
        )
    }
}
unsafe impl ::windows::core::Interface for ICollectionView {
    type Vtable = ICollectionView_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf8bb90d8_e008_5d65_8c97_7bb790a4230c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionView_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurrentItem: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CurrentPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub IsCurrentAfterLast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsCurrentBeforeFirst: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub CollectionGroups: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub HasMoreItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub CurrentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCurrentChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub CurrentChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveCurrentChanging: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        item: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentToPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentToFirst: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentToLast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentToNext: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MoveCurrentToPrevious: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub LoadMoreItemsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionViewFactory(::windows::core::IUnknown);
impl ICollectionViewFactory {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateView(&self) -> ::windows::core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateView)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<ICollectionView>(result__)
        }
    }
}
impl ::core::convert::From<ICollectionViewFactory> for ::windows::core::IUnknown {
    fn from(value: ICollectionViewFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewFactory> for ::windows::core::IUnknown {
    fn from(value: &ICollectionViewFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionViewFactory> for ::windows::core::IInspectable {
    fn from(value: ICollectionViewFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewFactory> for ::windows::core::IInspectable {
    fn from(value: &ICollectionViewFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICollectionViewFactory
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICollectionViewFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionViewFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionViewFactory {}
impl ::core::fmt::Debug for ICollectionViewFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionViewFactory")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionViewFactory {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{d971f795-5728-5bef-9602-43f2c4250e56}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICollectionViewFactory {
    type Vtable = ICollectionViewFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd971f795_5728_5bef_9602_43f2c4250e56);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateView: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionViewGroup(::windows::core::IUnknown);
impl ICollectionViewGroup {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Group)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GroupItems(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IObservableVector<::windows::core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GroupItems)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IObservableVector<
                ::windows::core::IInspectable,
            >>(result__)
        }
    }
}
impl ::core::convert::From<ICollectionViewGroup> for ::windows::core::IUnknown {
    fn from(value: ICollectionViewGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewGroup> for ::windows::core::IUnknown {
    fn from(value: &ICollectionViewGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionViewGroup> for ::windows::core::IInspectable {
    fn from(value: ICollectionViewGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewGroup> for ::windows::core::IInspectable {
    fn from(value: &ICollectionViewGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICollectionViewGroup
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICollectionViewGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionViewGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionViewGroup {}
impl ::core::fmt::Debug for ICollectionViewGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionViewGroup")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionViewGroup {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{96a08da8-be38-5ae0-903d-6fb6111e61f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICollectionViewGroup {
    type Vtable = ICollectionViewGroup_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x96a08da8_be38_5ae0_903d_6fb6111e61f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewGroup_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Group: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GroupItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICollectionViewSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa45e3b3a_f31e_5bbb_8a7c_70cf5c64bc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub View: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsSourceGrouped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsSourceGrouped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ItemsPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub SetItemsPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICollectionViewSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICollectionViewSourceStatics {
    type Vtable = ICollectionViewSourceStatics_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe282f10f_d4b1_5769_8a11_30f739e6113b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SourceProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ViewProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsSourceGroupedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub ItemsPathProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x63e42ed6_e14a_51ea_9cb1_72f9c907dc80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsCancelable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentChangingEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentChangingEventArgsFactory {
    type Vtable = ICurrentChangingEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3670f48a_ac2c_5352_8a4b_6b977a08e5f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub CreateWithCancelableParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iscancelable: bool,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICustomProperty(::windows::core::IUnknown);
impl ICustomProperty {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Type)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Name)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(
        &self,
        target: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                target.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        target: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                target.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetIndexedValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        target: Param0,
        index: Param1,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedValue)(
                ::windows::core::Interface::as_raw(this),
                target.into_param().abi(),
                index.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetIndexedValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        target: Param0,
        value: Param1,
        index: Param2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIndexedValue)(
                ::windows::core::Interface::as_raw(this),
                target.into_param().abi(),
                value.into_param().abi(),
                index.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanWrite)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanRead)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICustomProperty> for ::windows::core::IUnknown {
    fn from(value: ICustomProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomProperty> for ::windows::core::IUnknown {
    fn from(value: &ICustomProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomProperty> for ::windows::core::IInspectable {
    fn from(value: ICustomProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomProperty> for ::windows::core::IInspectable {
    fn from(value: &ICustomProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomProperty {}
impl ::core::fmt::Debug for ICustomProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomProperty {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{30da92c0-23e8-42a0-ae7c-734a0e5d2782}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomProperty {
    type Vtable = ICustomProperty_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x30da92c0_23e8_42a0_ae7c_734a0e5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomProperty_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetIndexedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        index: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetIndexedValue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        target: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        index: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub CanRead: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICustomPropertyProvider(::windows::core::IUnknown);
impl ICustomPropertyProvider {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetCustomProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::core::Result<ICustomProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetCustomProperty)(
                ::windows::core::Interface::as_raw(this),
                name.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ICustomProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetIndexedProperty<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
    >(
        &self,
        name: Param0,
        r#type: Param1,
    ) -> ::windows::core::Result<ICustomProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedProperty)(
                ::windows::core::Interface::as_raw(this),
                name.into_param().abi(),
                r#type.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<ICustomProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetStringRepresentation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).GetStringRepresentation)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::UI::Xaml::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).Type)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Xaml::Interop::TypeName>(result__)
        }
    }
}
impl ::core::convert::From<ICustomPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: ICustomPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: &ICustomPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomPropertyProvider> for ::windows::core::IInspectable {
    fn from(value: ICustomPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomPropertyProvider> for ::windows::core::IInspectable {
    fn from(value: &ICustomPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICustomPropertyProvider
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomPropertyProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomPropertyProvider {}
impl ::core::fmt::Debug for ICustomPropertyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomPropertyProvider")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomPropertyProvider {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{7c925755-3e48-42b4-8677-76372267033f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomPropertyProvider {
    type Vtable = ICustomPropertyProvider_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7c925755_3e48_42b4_8677_76372267033f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomPropertyProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetCustomProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetIndexedProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        r#type: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub GetStringRepresentation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataErrorsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataErrorsChangedEventArgs {
    type Vtable = IDataErrorsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd026dd64_5f26_5f15_a86a_0dec8a431796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataErrorsChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataErrorsChangedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDataErrorsChangedEventArgsFactory {
    type Vtable = IDataErrorsChangedEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62d0bd1e_b85f_5fcc_842a_7cb0dda37fe5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataErrorsChangedEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemIndexRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeba09846_2554_5b86_ac17_614f05105fa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRange_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FirstIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub LastIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemIndexRangeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItemIndexRangeFactory {
    type Vtable = IItemIndexRangeFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9fc73213_eda0_5238_aa2c_401c9921f0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRangeFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        firstindex: i32,
        length: u32,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct IItemsRangeInfo(::windows::core::IUnknown);
impl IItemsRangeInfo {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RangesChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ItemIndexRange>,
        Param1: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IVectorView<ItemIndexRange>,
        >,
    >(
        &self,
        visiblerange: Param0,
        trackeditems: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RangesChanged)(
                ::windows::core::Interface::as_raw(this),
                visiblerange.into_param().abi(),
                trackeditems.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
}
impl ::core::convert::From<IItemsRangeInfo> for ::windows::core::IUnknown {
    fn from(value: IItemsRangeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemsRangeInfo> for ::windows::core::IUnknown {
    fn from(value: &IItemsRangeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IItemsRangeInfo> for ::windows::core::IInspectable {
    fn from(value: IItemsRangeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemsRangeInfo> for ::windows::core::IInspectable {
    fn from(value: &IItemsRangeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IItemsRangeInfo> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IItemsRangeInfo) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IItemsRangeInfo> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IItemsRangeInfo) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IItemsRangeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemsRangeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemsRangeInfo {}
impl ::core::fmt::Debug for IItemsRangeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemsRangeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IItemsRangeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{b8376d08-85fb-563b-8273-39ef2d138256}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IItemsRangeInfo {
    type Vtable = IItemsRangeInfo_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb8376d08_85fb_563b_8273_39ef2d138256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsRangeInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RangesChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        visiblerange: ::windows::core::RawPtr,
        trackeditems: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct INotifyDataErrorInfo(::windows::core::IUnknown);
impl INotifyDataErrorInfo {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn HasErrors(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HasErrors)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ErrorsChanged<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::EventHandler<DataErrorsChangedEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ErrorsChanged)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemoveErrorsChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveErrorsChanged)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetErrors<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterable<::windows::core::IInspectable>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetErrors)(
                ::windows::core::Interface::as_raw(this),
                propertyname.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IIterable<
                ::windows::core::IInspectable,
            >>(result__)
        }
    }
}
impl ::core::convert::From<INotifyDataErrorInfo> for ::windows::core::IUnknown {
    fn from(value: INotifyDataErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyDataErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &INotifyDataErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotifyDataErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotifyDataErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INotifyDataErrorInfo> for ::windows::core::IInspectable {
    fn from(value: INotifyDataErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyDataErrorInfo> for ::windows::core::IInspectable {
    fn from(value: &INotifyDataErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INotifyDataErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a INotifyDataErrorInfo
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotifyDataErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotifyDataErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyDataErrorInfo {}
impl ::core::fmt::Debug for INotifyDataErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyDataErrorInfo")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyDataErrorInfo {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{0ee6c2cc-273e-567d-bc0a-1dd87ee51eba}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for INotifyDataErrorInfo {
    type Vtable = INotifyDataErrorInfo_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0ee6c2cc_273e_567d_bc0a_1dd87ee51eba);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyDataErrorInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HasErrors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ErrorsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveErrorsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub GetErrors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct INotifyPropertyChanged(::windows::core::IUnknown);
impl INotifyPropertyChanged {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn PropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, PropertyChangedEventHandler>,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::core::mem::MaybeUninit::<::windows::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).PropertyChanged)(
                ::windows::core::Interface::as_raw(this),
                handler.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RemovePropertyChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemovePropertyChanged)(
                ::windows::core::Interface::as_raw(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
impl ::core::convert::From<INotifyPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: INotifyPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: &INotifyPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INotifyPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: INotifyPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: &INotifyPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a INotifyPropertyChanged
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotifyPropertyChanged {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotifyPropertyChanged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyPropertyChanged {}
impl ::core::fmt::Debug for INotifyPropertyChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyPropertyChanged")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyPropertyChanged {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{90b17601-b065-586e-83d9-9adc3a695284}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for INotifyPropertyChanged {
    type Vtable = INotifyPropertyChanged_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x90b17601_b065_586e_83d9_9adc3a695284);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyPropertyChanged_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemovePropertyChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x63d0c952_396b_54f4_af8c_ba8724a427bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PropertyName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyChangedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPropertyChangedEventArgsFactory {
    type Vtable = IPropertyChangedEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7c0c27a8_0b41_5070_b160_fc9ae960a36c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRelativeSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRelativeSource {
    type Vtable = IRelativeSource_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7ffc8126_5dd8_58bb_b686_c71eddea07b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut RelativeSourceMode,
    ) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: RelativeSourceMode,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRelativeSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRelativeSourceFactory {
    type Vtable = IRelativeSourceFactory_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8518522c_85e3_5ae1_b9e9_28ea43c2051e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSourceFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        baseinterface: *mut ::core::ffi::c_void,
        innerinterface: *mut *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ISelectionInfo(::windows::core::IUnknown);
impl ISelectionInfo {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SelectRange<'a, Param0: ::windows::core::IntoParam<'a, ItemIndexRange>>(
        &self,
        itemindexrange: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SelectRange)(
                ::windows::core::Interface::as_raw(this),
                itemindexrange.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn DeselectRange<'a, Param0: ::windows::core::IntoParam<'a, ItemIndexRange>>(
        &self,
        itemindexrange: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).DeselectRange)(
                ::windows::core::Interface::as_raw(this),
                itemindexrange.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSelected(&self, index: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsSelected)(
                ::windows::core::Interface::as_raw(this),
                index,
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetSelectedRanges(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<ItemIndexRange>>
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).GetSelectedRanges)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::Collections::IVectorView<ItemIndexRange>>(result__)
        }
    }
}
impl ::core::convert::From<ISelectionInfo> for ::windows::core::IUnknown {
    fn from(value: ISelectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionInfo> for ::windows::core::IUnknown {
    fn from(value: &ISelectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISelectionInfo> for ::windows::core::IInspectable {
    fn from(value: ISelectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionInfo> for ::windows::core::IInspectable {
    fn from(value: &ISelectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISelectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionInfo {}
impl ::core::fmt::Debug for ISelectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISelectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{1b84c26b-9532-5803-935b-a03bf7e875dc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISelectionInfo {
    type Vtable = ISelectionInfo_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1b84c26b_9532_5803_935b_a03bf7e875dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SelectRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemindexrange: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub DeselectRange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        itemindexrange: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        index: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetSelectedRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ISupportIncrementalLoading(::windows::core::IUnknown);
impl ISupportIncrementalLoading {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn LoadMoreItemsAsync(
        &self,
        count: u32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).LoadMoreItemsAsync)(
                ::windows::core::Interface::as_raw(this),
                count,
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::Foundation::IAsyncOperation<LoadMoreItemsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn HasMoreItems(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HasMoreItems)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ISupportIncrementalLoading> for ::windows::core::IUnknown {
    fn from(value: ISupportIncrementalLoading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportIncrementalLoading> for ::windows::core::IUnknown {
    fn from(value: &ISupportIncrementalLoading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISupportIncrementalLoading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ISupportIncrementalLoading
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISupportIncrementalLoading> for ::windows::core::IInspectable {
    fn from(value: ISupportIncrementalLoading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportIncrementalLoading> for ::windows::core::IInspectable {
    fn from(value: &ISupportIncrementalLoading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ISupportIncrementalLoading
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ISupportIncrementalLoading
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportIncrementalLoading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportIncrementalLoading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportIncrementalLoading {}
impl ::core::fmt::Debug for ISupportIncrementalLoading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportIncrementalLoading")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISupportIncrementalLoading {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{d8f9b586-a64a-5ff8-868e-204e144f2cf4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISupportIncrementalLoading {
    type Vtable = ISupportIncrementalLoading_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd8f9b586_a64a_5ff8_868e_204e144f2cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportIncrementalLoading_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub LoadMoreItemsAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub HasMoreItems: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct IValueConverter(::windows::core::IUnknown);
impl IValueConverter {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Convert<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
        targettype: Param1,
        parameter: Param2,
        language: Param3,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Convert)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
                targettype.into_param().abi(),
                parameter.into_param().abi(),
                language.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ConvertBack<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, ::windows::UI::Xaml::Interop::TypeName>,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        value: Param0,
        targettype: Param1,
        parameter: Param2,
        language: Param3,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ConvertBack)(
                ::windows::core::Interface::as_raw(this),
                value.into_param().abi(),
                targettype.into_param().abi(),
                parameter.into_param().abi(),
                language.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IValueConverter> for ::windows::core::IUnknown {
    fn from(value: IValueConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueConverter> for ::windows::core::IUnknown {
    fn from(value: &IValueConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IValueConverter> for ::windows::core::IInspectable {
    fn from(value: IValueConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueConverter> for ::windows::core::IInspectable {
    fn from(value: &IValueConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IValueConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValueConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueConverter {}
impl ::core::fmt::Debug for IValueConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IValueConverter {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{afdd2bff-10f5-5173-b7c0-3590bd96cb35}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IValueConverter {
    type Vtable = IValueConverter_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xafdd2bff_10f5_5173_b7c0_3590bd96cb35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueConverter_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Convert: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        targettype: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        parameter: *mut ::core::ffi::c_void,
        language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ConvertBack: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        targettype: ::core::mem::ManuallyDrop<::windows::UI::Xaml::Interop::TypeName>,
        parameter: *mut ::core::ffi::c_void,
        language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ItemIndexRange(::windows::core::IUnknown);
impl ItemIndexRange {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn FirstIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).FirstIndex)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).Length)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn LastIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).LastIndex)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance(firstindex: i32, length: u32) -> ::windows::core::Result<ItemIndexRange> {
        Self::IItemIndexRangeFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                firstindex,
                length,
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<ItemIndexRange>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance_compose<T: ::windows::core::Compose>(
        firstindex: i32,
        length: u32,
        compose: T,
    ) -> ::windows::core::Result<ItemIndexRange> {
        Self::IItemIndexRangeFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                firstindex,
                length,
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<ItemIndexRange>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IItemIndexRangeFactory<
        R,
        F: FnOnce(&IItemIndexRangeFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ItemIndexRange, IItemIndexRangeFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ItemIndexRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItemIndexRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemIndexRange {}
impl ::core::fmt::Debug for ItemIndexRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemIndexRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ItemIndexRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.ItemIndexRange;{eba09846-2554-5b86-ac17-614f05105fa2})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
    const IID: ::windows::core::GUID = <IItemIndexRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ItemIndexRange {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.ItemIndexRange";
}
impl ::core::convert::From<ItemIndexRange> for ::windows::core::IUnknown {
    fn from(value: ItemIndexRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemIndexRange> for ::windows::core::IUnknown {
    fn from(value: &ItemIndexRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ItemIndexRange> for ::windows::core::IInspectable {
    fn from(value: ItemIndexRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemIndexRange> for ::windows::core::IInspectable {
    fn from(value: &ItemIndexRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ItemIndexRange {}
unsafe impl ::core::marker::Sync for ItemIndexRange {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
pub struct LoadMoreItemsResult {
    pub Count: u32,
}
impl ::core::marker::Copy for LoadMoreItemsResult {}
impl ::core::clone::Clone for LoadMoreItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LoadMoreItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LoadMoreItemsResult")
            .field("Count", &self.Count)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for LoadMoreItemsResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoadMoreItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.UI.Xaml.Data.LoadMoreItemsResult;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for LoadMoreItemsResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ::windows::core::memcmp(
                self as *const _ as _,
                other as *const _ as _,
                core::mem::size_of::<LoadMoreItemsResult>(),
            ) == 0
        }
    }
}
impl ::core::cmp::Eq for LoadMoreItemsResult {}
impl ::core::default::Default for LoadMoreItemsResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct PropertyChangedEventArgs(::windows::core::IUnknown);
impl PropertyChangedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn PropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<
                ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
            >::zeroed();
            (::windows::core::Interface::vtable(this).PropertyName)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        name: Param0,
    ) -> ::windows::core::Result<PropertyChangedEventArgs> {
        Self::IPropertyChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                name.into_param().abi(),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<PropertyChangedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance_compose<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        T: ::windows::core::Compose,
    >(
        name: Param0,
        compose: T,
    ) -> ::windows::core::Result<PropertyChangedEventArgs> {
        Self::IPropertyChangedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                name.into_param().abi(),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<PropertyChangedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPropertyChangedEventArgsFactory<
        R,
        F: FnOnce(&IPropertyChangedEventArgsFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            PropertyChangedEventArgs,
            IPropertyChangedEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyChangedEventArgs {}
impl ::core::fmt::Debug for PropertyChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyChangedEventArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Data.PropertyChangedEventArgs;{63d0c952-396b-54f4-af8c-ba8724a427bf})" ) ;
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID =
        <IPropertyChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PropertyChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.PropertyChangedEventArgs";
}
impl ::core::convert::From<PropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a PropertyChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for PropertyChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a PropertyChangedEventArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for PropertyChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct PropertyChangedEventHandler(pub ::windows::core::IUnknown);
impl PropertyChangedEventHandler {
    pub fn new<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PropertyChangedEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = PropertyChangedEventHandlerBox::<F> {
            vtable: &PropertyChangedEventHandlerBox::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Invoke<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
        Param1: ::windows::core::IntoParam<'a, PropertyChangedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Invoke)(
                ::windows::core::Interface::as_raw(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
struct PropertyChangedEventHandlerBox<
    F: FnMut(
            &::core::option::Option<::windows::core::IInspectable>,
            &::core::option::Option<PropertyChangedEventArgs>,
        ) -> ::windows::core::Result<()>
        + ::core::marker::Send
        + 'static,
> {
    vtable: *const PropertyChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<
        F: FnMut(
                &::core::option::Option<::windows::core::IInspectable>,
                &::core::option::Option<PropertyChangedEventArgs>,
            ) -> ::windows::core::Result<()>
            + ::core::marker::Send
            + 'static,
    > PropertyChangedEventHandlerBox<F>
{
    const VTABLE: PropertyChangedEventHandler_Vtbl = PropertyChangedEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: ::windows::core::RawPtr,
        iid: &::windows::core::GUID,
        interface: *mut *const ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PropertyChangedEventHandler as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID
            || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for PropertyChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyChangedEventHandler {}
impl ::core::fmt::Debug for PropertyChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyChangedEventHandler")
            .field(&self.0)
            .finish()
    }
}
unsafe impl ::windows::core::Interface for PropertyChangedEventHandler {
    type Vtable = PropertyChangedEventHandler_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe3de52f6_1e32_5da6_bb2d_b5b6096c962d);
}
unsafe impl ::windows::core::RuntimeType for PropertyChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{e3de52f6-1e32-5da6-bb2d-b5b6096c962d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertyChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sender: *mut ::core::ffi::c_void,
        e: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct RelativeSource(::windows::core::IUnknown);
impl RelativeSource {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ClearValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ReadLocalValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnimationBaseValue)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::core::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i64>::zeroed();
            (::windows::core::Interface::vtable(this).RegisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                result__.as_mut_ptr(),
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).UnregisterPropertyChangedCallback)(
                ::windows::core::Interface::as_raw(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).Dispatcher)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Dispatching\"`*"]
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).DispatcherQueue)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<RelativeSourceMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RelativeSourceMode>::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(
                ::windows::core::Interface::as_raw(this),
                result__.as_mut_ptr(),
            )
            .from_abi::<RelativeSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetMode(&self, value: RelativeSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetMode)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<RelativeSource> {
        Self::IRelativeSourceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::ptr::null_mut(),
                &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<RelativeSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(
        compose: T,
    ) -> ::windows::core::Result<RelativeSource> {
        Self::IRelativeSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::RawPtr>::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(&derived__),
                base__ as *mut _ as _,
                result__.as_mut_ptr(),
            )
            .from_abi::<RelativeSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRelativeSourceFactory<
        R,
        F: FnOnce(&IRelativeSourceFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RelativeSource, IRelativeSourceFactory> =
            ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RelativeSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RelativeSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RelativeSource {}
impl ::core::fmt::Debug for RelativeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RelativeSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RelativeSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Xaml.Data.RelativeSource;{7ffc8126-5dd8-58bb-b686-c71eddea07b2})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RelativeSource {
    type Vtable = IRelativeSource_Vtbl;
    const IID: ::windows::core::GUID = <IRelativeSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RelativeSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.RelativeSource";
}
impl ::core::convert::From<RelativeSource> for ::windows::core::IUnknown {
    fn from(value: RelativeSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RelativeSource> for ::windows::core::IUnknown {
    fn from(value: &RelativeSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RelativeSource> for ::windows::core::IInspectable {
    fn from(value: RelativeSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RelativeSource> for ::windows::core::IInspectable {
    fn from(value: &RelativeSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RelativeSource> for super::DependencyObject {
    fn from(value: RelativeSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RelativeSource> for super::DependencyObject {
    fn from(value: &RelativeSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RelativeSource {}
unsafe impl ::core::marker::Sync for RelativeSource {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
impl ::core::marker::Copy for RelativeSourceMode {}
impl ::core::clone::Clone for RelativeSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RelativeSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RelativeSourceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for RelativeSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RelativeSourceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RelativeSourceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Data.RelativeSourceMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
impl ::core::marker::Copy for UpdateSourceTrigger {}
impl ::core::clone::Clone for UpdateSourceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateSourceTrigger {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UpdateSourceTrigger {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateSourceTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSourceTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UpdateSourceTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Data.UpdateSourceTrigger;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
