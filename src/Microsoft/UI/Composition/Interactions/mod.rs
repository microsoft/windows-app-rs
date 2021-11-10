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
pub struct CompositionConditionalValue(pub ::windows::core::IInspectable);
impl CompositionConditionalValue {
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<CompositionConditionalValue> {
        Self::ICompositionConditionalValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<CompositionConditionalValue>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ICompositionConditionalValueStatics<
        R,
        F: FnOnce(&ICompositionConditionalValueStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            CompositionConditionalValue,
            ICompositionConditionalValueStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionConditionalValue {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.CompositionConditionalValue;{3743dda0-fbe2-5ecf-9e80-4638a011f707})" ) ;
}
unsafe impl ::windows::core::Interface for CompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3743dda0_fbe2_5ecf_9e80_4638a011f707);
}
impl ::windows::core::RuntimeName for CompositionConditionalValue {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.CompositionConditionalValue";
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::core::IUnknown {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::core::IUnknown {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CompositionConditionalValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for ::windows::core::IInspectable {
    fn from(value: CompositionConditionalValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for ::windows::core::IInspectable {
    fn from(value: &CompositionConditionalValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompositionConditionalValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionConditionalValue> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionConditionalValue> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionConditionalValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for CompositionConditionalValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CompositionConditionalValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CompositionConditionalValue> for super::CompositionObject {
    fn from(value: CompositionConditionalValue) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositionConditionalValue> for super::CompositionObject {
    fn from(value: &CompositionConditionalValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &CompositionConditionalValue {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for CompositionConditionalValue {}
unsafe impl ::core::marker::Sync for CompositionConditionalValue {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct CompositionInteractionSourceCollection(pub ::windows::core::IInspectable);
impl CompositionInteractionSourceCollection {
    pub fn Count(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ICompositionInteractionSource>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn First(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IIterator<ICompositionInteractionSource>,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IIterator<
                ICompositionInteractionSource,
            >>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionInteractionSourceCollection {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.CompositionInteractionSourceCollection;{9aa1b86b-b002-5e2e-bb2b-0e2c547445e1})" ) ;
}
unsafe impl ::windows::core::Interface for CompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9aa1b86b_b002_5e2e_bb2b_0e2c547445e1);
}
impl ::windows::core::RuntimeName for CompositionInteractionSourceCollection {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.CompositionInteractionSourceCollection";
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for ::windows::core::IUnknown {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for ::windows::core::IUnknown {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection>
    for ::windows::core::IInspectable
{
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection>
    for ::windows::core::IInspectable
{
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection>
    for ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection>
    for ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>
{
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a>
    ::windows::core::IntoParam<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > for &CompositionInteractionSourceCollection
{
    fn into_param(
        self,
    ) -> ::windows::core::Param<
        'a,
        ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
    > {
        ::core::convert::TryInto::<
            ::windows::Foundation::Collections::IIterable<ICompositionInteractionSource>,
        >::try_into(self)
        .map(::windows::core::Param::Owned)
        .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CompositionInteractionSourceCollection>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CompositionInteractionSourceCollection>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &CompositionInteractionSourceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: CompositionInteractionSourceCollection) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&CompositionInteractionSourceCollection> for super::CompositionObject {
    fn from(value: &CompositionInteractionSourceCollection) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &CompositionInteractionSourceCollection
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for CompositionInteractionSourceCollection {}
unsafe impl ::core::marker::Sync for CompositionInteractionSourceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &CompositionInteractionSourceCollection {
    type Item = ICompositionInteractionSource;
    type IntoIter = ::windows::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionConditionalValue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionConditionalValue {
    type Vtable = ICompositionConditionalValue_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3743dda0_fbe2_5ecf_9e80_4638a011f707);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValue_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionConditionalValueStatics {
    type Vtable = ICompositionConditionalValueStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdf133c1f_a185_536c_b54b_8f369212a581);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionConditionalValueStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ICompositionInteractionSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionInteractionSource {
    type Vtable = ICompositionInteractionSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x711c72c0_c406_4a12_859b_b44f651af046);
}
impl ICompositionInteractionSource {}
unsafe impl ::windows::core::RuntimeType for ICompositionInteractionSource {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{711c72c0-c406-4a12-859b-b44f651af046}");
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::core::IUnknown {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::core::IUnknown {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICompositionInteractionSource> for ::windows::core::IInspectable {
    fn from(value: ICompositionInteractionSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionInteractionSource> for ::windows::core::IInspectable {
    fn from(value: &ICompositionInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ICompositionInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSource_abi(
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
pub struct ICompositionInteractionSourceCollection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionInteractionSourceCollection {
    type Vtable = ICompositionInteractionSourceCollection_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9aa1b86b_b002_5e2e_bb2b_0e2c547445e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionInteractionSourceCollection_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x099e0124_dadf_5bc6_a895_90387657550f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionSourceConfiguration_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02d8ec1f_8f04_505e_bd1e_47b2a204de51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        modifiers: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        modifiers: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        modifiers: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        amount: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        animation: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        velocityinpixelspersecond: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: f32,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        animation: ::windows::core::RawPtr,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        velocityinpercentpersecond: f32,
        centerpoint: ::windows::Foundation::Numerics::Vector3,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTracker2 {
    type Vtable = IInteractionTracker2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x396d7fb1_2fad_5508_8591_4ff0dc5a7484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker2_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTracker3 {
    type Vtable = IInteractionTracker3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x239752cf_266c_5acb_acc3_b3e3ecaf4d3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker3_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        modifiers: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTracker4 {
    type Vtable = IInteractionTracker4_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa9a9ce02_53c9_5690_a575_f340b7c2fdf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker4_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        amount: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTracker5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTracker5 {
    type Vtable = IInteractionTracker5_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdbfcd333_c3bf_5057_a45e_25edf06ebd8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTracker5_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::Foundation::Numerics::Vector3,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7464035c_cfce_56da_9472_420f276bd0a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerCustomAnimationStateEnteredArgs2 {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x06b99fbc_d6a8_5ae3_88b8_e91621becbd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerCustomAnimationStateEnteredArgs2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x199094ab_15fd_539c_97b8_964a8196f777);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerIdleStateEnteredArgs2 {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4eb213c0_931c_5164_8965_11c0186d3390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerIdleStateEnteredArgs2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4d3a0c6b_c508_5029_a47a_cbf64636f010);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifier_abi(
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
pub struct IInteractionTrackerInertiaModifierFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaModifierFactory {
    type Vtable = IInteractionTrackerInertiaModifierFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6dee5b33_0b5a_57b1_8537_93d4fd038f9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaModifierFactory_abi(
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
pub struct IInteractionTrackerInertiaMotion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91f662c0_3141_5b5e_862f_cfc60bee8cd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotion_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaMotionStatics {
    type Vtable = IInteractionTrackerInertiaMotionStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb0185a4f_0059_52c6_a660_9aed0c44ff7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaMotionStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8c7482e0_185d_56b1_b67f_fca4fcd13cd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotion_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerInertiaNaturalMotionStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x860ec143_f165_5298_abf2_47369dd07f10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaNaturalMotionStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1a2b20cd_3371_53ff_a560_f4847b467d73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValue_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaRestingValueStatics {
    type Vtable = IInteractionTrackerInertiaRestingValueStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcf0f0414_7fdf_5284_aeef_28b71b62aa4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaRestingValueStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5b76c949_a4d0_5c9d_9292_7013ae9656c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc42d7e8f_7199_57a9_8aec_8727552b13e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInertiaStateEnteredArgs3 {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce726ca0_1c04_531b_9951_4aec996952e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInertiaStateEnteredArgs3_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x70d29b84_0931_5f17_a8a1_82f8f8782532);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerInteractingStateEnteredArgs2 {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f1ff38d_2f51_5ceb_8d09_bda1519f9342);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerInteractingStateEnteredArgs2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct IInteractionTrackerOwner(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerOwner {
    type Vtable = IInteractionTrackerOwner_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8869779d_1d2a_5816_836a_68a910507d87);
}
impl IInteractionTrackerOwner {
    #[cfg(feature = "UI_Dispatching")]
    pub fn CustomAnimationStateEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerCustomAnimationStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IdleStateEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerIdleStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn InertiaStateEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerInertiaStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn InteractingStateEntered<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerInteractingStateEnteredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn RequestIgnored<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerRequestIgnoredArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ValuesChanged<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTrackerValuesChangedArgs>,
    >(
        &self,
        sender: Param0,
        args: Param1,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                sender.into_param().abi(),
                args.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInteractionTrackerOwner {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"{8869779d-1d2a-5816-836a-68a910507d87}");
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::core::IUnknown {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::core::IUnknown {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInteractionTrackerOwner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInteractionTrackerOwner> for ::windows::core::IInspectable {
    fn from(value: IInteractionTrackerOwner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInteractionTrackerOwner> for ::windows::core::IInspectable {
    fn from(value: &IInteractionTrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a IInteractionTrackerOwner
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerOwner_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        sender: ::windows::core::RawPtr,
        args: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc276205e_f7a5_5ba2_ad45_d12c3c339149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerRequestIgnoredArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics {
    type Vtable = IInteractionTrackerStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7ac9867a_e16e_56ef_9809_f6e404240f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        owner: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerStatics2 {
    type Vtable = IInteractionTrackerStatics2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x25658e4c_b99f_5108_aab7_1cc44f11508b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerStatics2_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        boundtracker1: ::windows::core::RawPtr,
        boundtracker2: ::windows::core::RawPtr,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        boundtracker1: ::windows::core::RawPtr,
        boundtracker2: ::windows::core::RawPtr,
        result__: *mut InteractionBindingAxisModes,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9b495bed_1cf7_55c1_82b9_8022cbf3c766);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerValuesChangedArgs_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4b8ed310_cb61_5f0a_b99a_940cdd2c42b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifier_abi(
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
pub struct IInteractionTrackerVector2InertiaModifierFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaModifierFactory {
    type Vtable = IInteractionTrackerVector2InertiaModifierFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1b3fd240_ba66_5296_b801_62a2a3606613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaModifierFactory_abi(
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
pub struct IInteractionTrackerVector2InertiaNaturalMotion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x097ba1a6_e077_52d1_86d3_38e3f6619ddf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotion_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInteractionTrackerVector2InertiaNaturalMotionStatics {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotionStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcc24ab87_9131_5286_b3ce_1ef97e0974e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInteractionTrackerVector2InertiaNaturalMotionStatics_abi(
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        compositor: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xea595c95_b9cb_5cd4_bb9c_4934ff329063);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut VisualInteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: VisualInteractionSourceRedirectionMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionChainingMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: InteractionSourceMode,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pointerpoint: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Input"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSource2 {
    type Vtable = IVisualInteractionSource2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xff1132ba_dc0d_519e_be49_be301e52306a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::Foundation::Numerics::Vector3,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        conditionalvalues: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSource3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSource3 {
    type Vtable = IVisualInteractionSource3_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd523bd66_a05d_5417_8e07_84ae3caf9752);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSource3_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceObjectFactory {
    type Vtable = IVisualInteractionSourceObjectFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfeb73102_238c_52aa_8e03_b68d5ecc44b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceObjectFactory_abi(
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
pub struct IVisualInteractionSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics {
    type Vtable = IVisualInteractionSourceStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5fc9c763_e2e5_530e_87cd_b93118ade8a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics_abi(
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
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Input"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        source: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Input")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVisualInteractionSourceStatics2 {
    type Vtable = IVisualInteractionSourceStatics2_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa6b494fe_12a1_5a73_b87e_4c4ef58eac6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceStatics2_abi(
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
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Input"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        source: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Input")))] usize,
);
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionBindingAxisModes(pub u32);
impl InteractionBindingAxisModes {
    pub const None: InteractionBindingAxisModes = InteractionBindingAxisModes(0u32);
    pub const PositionX: InteractionBindingAxisModes = InteractionBindingAxisModes(1u32);
    pub const PositionY: InteractionBindingAxisModes = InteractionBindingAxisModes(2u32);
    pub const Scale: InteractionBindingAxisModes = InteractionBindingAxisModes(4u32);
}
impl ::core::convert::From<u32> for InteractionBindingAxisModes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionBindingAxisModes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionBindingAxisModes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionBindingAxisModes;u4)",
    );
}
impl ::windows::core::DefaultType for InteractionBindingAxisModes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for InteractionBindingAxisModes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for InteractionBindingAxisModes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for InteractionBindingAxisModes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for InteractionBindingAxisModes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for InteractionBindingAxisModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
pub struct InteractionChainingMode(pub i32);
impl InteractionChainingMode {
    pub const Auto: InteractionChainingMode = InteractionChainingMode(0i32);
    pub const Always: InteractionChainingMode = InteractionChainingMode(1i32);
    pub const Never: InteractionChainingMode = InteractionChainingMode(2i32);
}
impl ::core::convert::From<i32> for InteractionChainingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionChainingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionChainingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionChainingMode;i4)",
    );
}
impl ::windows::core::DefaultType for InteractionChainingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionSourceConfiguration(pub ::windows::core::IInspectable);
impl InteractionSourceConfiguration {
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(
        &self,
        value: InteractionSourceRedirectionMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceConfiguration {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionSourceConfiguration;{099e0124-dadf-5bc6-a895-90387657550f})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionSourceConfiguration {
    type Vtable = IInteractionSourceConfiguration_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x099e0124_dadf_5bc6_a895_90387657550f);
}
impl ::windows::core::RuntimeName for InteractionSourceConfiguration {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionSourceConfiguration";
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::core::IUnknown {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for ::windows::core::IInspectable {
    fn from(value: InteractionSourceConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionSourceConfiguration> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionSourceConfiguration>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionSourceConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: InteractionSourceConfiguration) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionSourceConfiguration> for super::CompositionObject {
    fn from(value: &InteractionSourceConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionSourceConfiguration
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionSourceConfiguration {}
unsafe impl ::core::marker::Sync for InteractionSourceConfiguration {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionSourceMode(pub i32);
impl InteractionSourceMode {
    pub const Disabled: InteractionSourceMode = InteractionSourceMode(0i32);
    pub const EnabledWithInertia: InteractionSourceMode = InteractionSourceMode(1i32);
    pub const EnabledWithoutInertia: InteractionSourceMode = InteractionSourceMode(2i32);
}
impl ::core::convert::From<i32> for InteractionSourceMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionSourceMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionSourceMode;i4)",
    );
}
impl ::windows::core::DefaultType for InteractionSourceMode {
    type DefaultType = Self;
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
pub struct InteractionSourceRedirectionMode(pub i32);
impl InteractionSourceRedirectionMode {
    pub const Disabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(0i32);
    pub const Enabled: InteractionSourceRedirectionMode = InteractionSourceRedirectionMode(1i32);
}
impl ::core::convert::From<i32> for InteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionSourceRedirectionMode;i4)",
    );
}
impl ::windows::core::DefaultType for InteractionSourceRedirectionMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTracker(pub ::windows::core::IInspectable);
impl InteractionTracker {
    pub fn InteractionSources(
        &self,
    ) -> ::windows::core::Result<CompositionInteractionSourceCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<CompositionInteractionSourceCollection>(result__)
        }
    }
    pub fn IsPositionRoundingSuggested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn MaxPosition(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetMaxPosition<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MaxScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetMaxScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MinPosition(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn SetMinPosition<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn MinScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetMinScale(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Owner(&self) -> ::windows::core::Result<IInteractionTrackerOwner> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IInteractionTrackerOwner>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn PositionInertiaDecayRate(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .20 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation:: IReference :: < ::windows::Foundation::Numerics:: Vector3 > > ( result__ )
        }
    }
    pub fn SetPositionInertiaDecayRate<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn ScaleInertiaDecayRate(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<f32>>(result__)
        }
    }
    pub fn SetScaleInertiaDecayRate<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::IReference<f32>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn AdjustPositionXIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    pub fn AdjustPositionYIfGreaterThanThreshold(
        &self,
        adjustment: f32,
        positionthreshold: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                adjustment,
                positionthreshold,
            )
            .ok()
        }
    }
    pub fn ConfigurePositionXInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ConfigurePositionYInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ConfigureScaleInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerInertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TryUpdatePosition<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdatePositionBy<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                amount.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TryUpdatePositionWithAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdatePositionWithAdditionalVelocity<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpixelspersecond: Param0,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                velocityinpixelspersecond.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdateScale<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: f32,
        centerpoint: Param1,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                value,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TryUpdateScaleWithAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        animation: Param0,
        centerpoint: Param1,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                animation.into_param().abi(),
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdateScaleWithAdditionalVelocity<
        'a,
        Param1: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        velocityinpercentpersecond: f32,
        centerpoint: Param1,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                velocityinpercentpersecond,
                centerpoint.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureCenterPointXInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureCenterPointYInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ConfigureVector2PositionInertiaModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<InteractionTrackerVector2InertiaModifier>,
        >,
    >(
        &self,
        modifiers: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker3>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                modifiers.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TryUpdatePositionWithOption<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn TryUpdatePositionByWithOption<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        amount: Param0,
        option: InteractionTrackerClampingOption,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                amount.into_param().abi(),
                option,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn TryUpdatePositionWithOption2<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
        option: InteractionTrackerClampingOption,
        posupdateoption: InteractionTrackerPositionUpdateOption,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IInteractionTracker5>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
                option,
                posupdateoption,
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CreateWithOwner<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Compositor>,
        Param1: ::windows::core::IntoParam<'a, IInteractionTrackerOwner>,
    >(
        compositor: Param0,
        owner: Param1,
    ) -> ::windows::core::Result<InteractionTracker> {
        Self::IInteractionTrackerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                owner.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTracker>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetBindingMode<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
        axismode: InteractionBindingAxisModes,
    ) -> ::windows::core::Result<()> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                axismode,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetBindingMode<
        'a,
        Param0: ::windows::core::IntoParam<'a, InteractionTracker>,
        Param1: ::windows::core::IntoParam<'a, InteractionTracker>,
    >(
        boundtracker1: Param0,
        boundtracker2: Param1,
    ) -> ::windows::core::Result<InteractionBindingAxisModes> {
        Self::IInteractionTrackerStatics2(|this| unsafe {
            let mut result__: InteractionBindingAxisModes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                boundtracker1.into_param().abi(),
                boundtracker2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionBindingAxisModes>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerStatics<
        R,
        F: FnOnce(&IInteractionTrackerStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInteractionTrackerStatics2<
        R,
        F: FnOnce(&IInteractionTrackerStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTracker,
            IInteractionTrackerStatics2,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTracker {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTracker;{02d8ec1f-8f04-505e-bd1e-47b2a204de51})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTracker {
    type Vtable = IInteractionTracker_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02d8ec1f_8f04_505e_bd1e_47b2a204de51);
}
impl ::windows::core::RuntimeName for InteractionTracker {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.InteractionTracker";
}
impl ::core::convert::From<InteractionTracker> for ::windows::core::IUnknown {
    fn from(value: InteractionTracker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::core::IUnknown {
    fn from(value: &InteractionTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTracker> for ::windows::core::IInspectable {
    fn from(value: InteractionTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTracker> for ::windows::core::IInspectable {
    fn from(value: &InteractionTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTracker> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTracker) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTracker> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTracker) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTracker> for super::CompositionObject {
    fn from(value: InteractionTracker) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTracker> for super::CompositionObject {
    fn from(value: &InteractionTracker) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &InteractionTracker {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTracker {}
unsafe impl ::core::marker::Sync for InteractionTracker {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerClampingOption(pub i32);
impl InteractionTrackerClampingOption {
    pub const Auto: InteractionTrackerClampingOption = InteractionTrackerClampingOption(0i32);
    pub const Disabled: InteractionTrackerClampingOption = InteractionTrackerClampingOption(1i32);
}
impl ::core::convert::From<i32> for InteractionTrackerClampingOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionTrackerClampingOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerClampingOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionTrackerClampingOption;i4)",
    );
}
impl ::windows::core::DefaultType for InteractionTrackerClampingOption {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerCustomAnimationStateEnteredArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerCustomAnimationStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            IInteractionTrackerCustomAnimationStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerCustomAnimationStateEnteredArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs;{7464035c-cfce-56da-9472-420f276bd0a5})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerCustomAnimationStateEnteredArgs {
    type Vtable = IInteractionTrackerCustomAnimationStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7464035c_cfce_56da_9472_420f276bd0a5);
}
impl ::windows::core::RuntimeName for InteractionTrackerCustomAnimationStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerCustomAnimationStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerCustomAnimationStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerCustomAnimationStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerCustomAnimationStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerCustomAnimationStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerCustomAnimationStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerIdleStateEnteredArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerIdleStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this =
            &::windows::core::Interface::cast::<IInteractionTrackerIdleStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerIdleStateEnteredArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs;{199094ab-15fd-539c-97b8-964a8196f777})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerIdleStateEnteredArgs {
    type Vtable = IInteractionTrackerIdleStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x199094ab_15fd_539c_97b8_964a8196f777);
}
impl ::windows::core::RuntimeName for InteractionTrackerIdleStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerIdleStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerIdleStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerIdleStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerIdleStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerIdleStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerIdleStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerIdleStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaModifier(pub ::windows::core::IInspectable);
impl InteractionTrackerInertiaModifier {
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaModifier {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaModifier;{4d3a0c6b-c508-5029-a47a-cbf64636f010})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaModifier {
    type Vtable = IInteractionTrackerInertiaModifier_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4d3a0c6b_c508_5029_a47a_cbf64636f010);
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaModifier {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaModifier";
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaModifier) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaModifier {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaMotion(pub ::windows::core::IInspectable);
impl InteractionTrackerInertiaMotion {
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Motion(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetMotion<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<InteractionTrackerInertiaMotion> {
        Self::IInteractionTrackerInertiaMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaMotion>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaMotionStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTrackerInertiaMotion,
            IInteractionTrackerInertiaMotionStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaMotion {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaMotion;{91f662c0-3141-5b5e-862f-cfc60bee8cd6})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaMotion {
    type Vtable = IInteractionTrackerInertiaMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x91f662c0_3141_5b5e_862f_cfc60bee8cd6);
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for InteractionTrackerInertiaModifier {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::core::convert::From<InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaMotion {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaNaturalMotion(pub ::windows::core::IInspectable);
impl InteractionTrackerInertiaNaturalMotion {
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::ScalarNaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ScalarNaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ScalarNaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<InteractionTrackerInertiaNaturalMotion> {
        Self::IInteractionTrackerInertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaNaturalMotionStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaNaturalMotionStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTrackerInertiaNaturalMotion,
            IInteractionTrackerInertiaNaturalMotionStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaNaturalMotion {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion;{8c7482e0-185d-56b1-b67f-fca4fcd13cd2})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaNaturalMotion {
    type Vtable = IInteractionTrackerInertiaNaturalMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8c7482e0_185d_56b1_b67f_fca4fcd13cd2);
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaNaturalMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaNaturalMotion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::core::convert::From<InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaNaturalMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaNaturalMotion> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaNaturalMotion {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaRestingValue(pub ::windows::core::IInspectable);
impl InteractionTrackerInertiaRestingValue {
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RestingValue(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetRestingValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<InteractionTrackerInertiaRestingValue> {
        Self::IInteractionTrackerInertiaRestingValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerInertiaRestingValue>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerInertiaRestingValueStatics<
        R,
        F: FnOnce(&IInteractionTrackerInertiaRestingValueStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTrackerInertiaRestingValue,
            IInteractionTrackerInertiaRestingValueStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaRestingValue {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue;{1a2b20cd-3371-53ff-a560-f4847b467d73})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaRestingValue {
    type Vtable = IInteractionTrackerInertiaRestingValue_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1a2b20cd_3371_53ff_a560_f4847b467d73);
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaRestingValue {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaRestingValue";
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerInertiaRestingValue>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerInertiaRestingValue>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerInertiaRestingValue) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue>
    for InteractionTrackerInertiaModifier
{
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(self),
        )
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerInertiaModifier>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerInertiaModifier> {
        ::windows::core::Param::Owned(
            ::core::convert::Into::<InteractionTrackerInertiaModifier>::into(
                ::core::clone::Clone::clone(self),
            ),
        )
    }
}
impl ::core::convert::From<InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: InteractionTrackerInertiaRestingValue) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaRestingValue> for super::CompositionObject {
    fn from(value: &InteractionTrackerInertiaRestingValue) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerInertiaRestingValue
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaRestingValue {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaRestingValue {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInertiaStateEnteredArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerInertiaStateEnteredArgs {
    pub fn ModifiedRestingPosition(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IReference<::windows::Foundation::Numerics::Vector3>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            ( :: windows :: core :: Interface :: vtable ( this ) .6 ) ( :: core :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation:: IReference :: < ::windows::Foundation::Numerics:: Vector3 > > ( result__ )
        }
    }
    pub fn ModifiedRestingScale(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::IReference<f32>>(result__)
        }
    }
    pub fn NaturalRestingPosition(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn NaturalRestingScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn PositionVelocityInPixelsPerSecond(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ScaleVelocityInPercentPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn IsInertiaFromImpulse(&self) -> ::windows::core::Result<bool> {
        let this =
            &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this =
            &::windows::core::Interface::cast::<IInteractionTrackerInertiaStateEnteredArgs3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInertiaStateEnteredArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs;{5b76c949-a4d0-5c9d-9292-7013ae9656c7})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInertiaStateEnteredArgs {
    type Vtable = IInteractionTrackerInertiaStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5b76c949_a4d0_5c9d_9292_7013ae9656c7);
}
impl ::windows::core::RuntimeName for InteractionTrackerInertiaStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInertiaStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInertiaStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerInertiaStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInertiaStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInertiaStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInertiaStateEnteredArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerInteractingStateEnteredArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerInteractingStateEnteredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn IsFromBinding(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<
            IInteractionTrackerInteractingStateEnteredArgs2,
        >(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerInteractingStateEnteredArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs;{70d29b84-0931-5f17-a8a1-82f8f8782532})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerInteractingStateEnteredArgs {
    type Vtable = IInteractionTrackerInteractingStateEnteredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x70d29b84_0931_5f17_a8a1_82f8f8782532);
}
impl ::windows::core::RuntimeName for InteractionTrackerInteractingStateEnteredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerInteractingStateEnteredArgs";
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::core::IUnknown
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerInteractingStateEnteredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerInteractingStateEnteredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerInteractingStateEnteredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerInteractingStateEnteredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerInteractingStateEnteredArgs {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct InteractionTrackerPositionUpdateOption(pub i32);
impl InteractionTrackerPositionUpdateOption {
    pub const Default: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(0i32);
    pub const AllowActiveCustomScaleAnimation: InteractionTrackerPositionUpdateOption =
        InteractionTrackerPositionUpdateOption(1i32);
}
impl ::core::convert::From<i32> for InteractionTrackerPositionUpdateOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InteractionTrackerPositionUpdateOption {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerPositionUpdateOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.InteractionTrackerPositionUpdateOption;i4)",
    );
}
impl ::windows::core::DefaultType for InteractionTrackerPositionUpdateOption {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerRequestIgnoredArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerRequestIgnoredArgs {
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerRequestIgnoredArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs;{c276205e-f7a5-5ba2-ad45-d12c3c339149})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerRequestIgnoredArgs {
    type Vtable = IInteractionTrackerRequestIgnoredArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc276205e_f7a5_5ba2_ad45_d12c3c339149);
}
impl ::windows::core::RuntimeName for InteractionTrackerRequestIgnoredArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerRequestIgnoredArgs";
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerRequestIgnoredArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerRequestIgnoredArgs>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerRequestIgnoredArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerRequestIgnoredArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerRequestIgnoredArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerRequestIgnoredArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerValuesChangedArgs(pub ::windows::core::IInspectable);
impl InteractionTrackerValuesChangedArgs {
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerValuesChangedArgs {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs;{9b495bed-1cf7-55c1-82b9-8022cbf3c766})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerValuesChangedArgs {
    type Vtable = IInteractionTrackerValuesChangedArgs_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9b495bed_1cf7_55c1_82b9_8022cbf3c766);
}
impl ::windows::core::RuntimeName for InteractionTrackerValuesChangedArgs {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerValuesChangedArgs";
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::core::IUnknown {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerValuesChangedArgs> for ::windows::core::IInspectable {
    fn from(value: InteractionTrackerValuesChangedArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerValuesChangedArgs> for ::windows::core::IInspectable {
    fn from(value: &InteractionTrackerValuesChangedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerValuesChangedArgs
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerValuesChangedArgs {}
unsafe impl ::core::marker::Sync for InteractionTrackerValuesChangedArgs {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaModifier(pub ::windows::core::IInspectable);
impl InteractionTrackerVector2InertiaModifier {
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaModifier {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier;{4b8ed310-cb61-5f0a-b99a-940cdd2c42b1})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaModifier {
    type Vtable = IInteractionTrackerVector2InertiaModifier_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x4b8ed310_cb61_5f0a_b99a_940cdd2c42b1);
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaModifier {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaModifier";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for ::windows::core::IUnknown {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::core::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier>
    for super::IAnimationObject
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for super::IAnimationObject
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaModifier>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(value: &InteractionTrackerVector2InertiaModifier) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: InteractionTrackerVector2InertiaModifier) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaModifier> for super::CompositionObject {
    fn from(value: &InteractionTrackerVector2InertiaModifier) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaModifier
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaModifier {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaModifier {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct InteractionTrackerVector2InertiaNaturalMotion(pub ::windows::core::IInspectable);
impl InteractionTrackerVector2InertiaNaturalMotion {
    pub fn Condition(&self) -> ::windows::core::Result<super::ExpressionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ExpressionAnimation>(result__)
        }
    }
    pub fn SetCondition<'a, Param0: ::windows::core::IntoParam<'a, super::ExpressionAnimation>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn NaturalMotion(&self) -> ::windows::core::Result<super::Vector2NaturalMotionAnimation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Vector2NaturalMotionAnimation>(result__)
        }
    }
    pub fn SetNaturalMotion<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::Vector2NaturalMotionAnimation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(
        compositor: Param0,
    ) -> ::windows::core::Result<InteractionTrackerVector2InertiaNaturalMotion> {
        Self::IInteractionTrackerVector2InertiaNaturalMotionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                compositor.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InteractionTrackerVector2InertiaNaturalMotion>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IInteractionTrackerVector2InertiaNaturalMotionStatics<
        R,
        F: FnOnce(
            &IInteractionTrackerVector2InertiaNaturalMotionStatics,
        ) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            InteractionTrackerVector2InertiaNaturalMotion,
            IInteractionTrackerVector2InertiaNaturalMotionStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InteractionTrackerVector2InertiaNaturalMotion {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion;{097ba1a6-e077-52d1-86d3-38e3f6619ddf})" ) ;
}
unsafe impl ::windows::core::Interface for InteractionTrackerVector2InertiaNaturalMotion {
    type Vtable = IInteractionTrackerVector2InertiaNaturalMotion_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x097ba1a6_e077_52d1_86d3_38e3f6619ddf);
}
impl ::windows::core::RuntimeName for InteractionTrackerVector2InertiaNaturalMotion {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Interactions.InteractionTrackerVector2InertiaNaturalMotion";
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::core::IUnknown
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::core::IUnknown
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::core::IInspectable
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::core::IInspectable
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::IAnimationObject
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InteractionTrackerVector2InertiaNaturalMotion>
    for ::windows::Foundation::IClosable
{
    type Error = ::windows::core::Error;
    fn try_from(
        value: &InteractionTrackerVector2InertiaNaturalMotion,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::Into::<InteractionTrackerVector2InertiaModifier>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for InteractionTrackerVector2InertiaModifier
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, InteractionTrackerVector2InertiaModifier>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, InteractionTrackerVector2InertiaModifier> {
        ::windows::core::Param::Owned(::core::convert::Into::<
            InteractionTrackerVector2InertiaModifier,
        >::into(::core::clone::Clone::clone(self)))
    }
}
impl ::core::convert::From<InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&InteractionTrackerVector2InertiaNaturalMotion>
    for super::CompositionObject
{
    fn from(value: &InteractionTrackerVector2InertiaNaturalMotion) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject>
    for &InteractionTrackerVector2InertiaNaturalMotion
{
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for InteractionTrackerVector2InertiaNaturalMotion {}
unsafe impl ::core::marker::Sync for InteractionTrackerVector2InertiaNaturalMotion {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct VisualInteractionSource(pub ::windows::core::IInspectable);
impl VisualInteractionSource {
    pub fn IsPositionXRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionXRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn IsPositionYRailsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPositionYRailsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ManipulationRedirectionMode(
        &self,
    ) -> ::windows::core::Result<VisualInteractionSourceRedirectionMode> {
        let this = self;
        unsafe {
            let mut result__: VisualInteractionSourceRedirectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VisualInteractionSourceRedirectionMode>(result__)
        }
    }
    pub fn SetManipulationRedirectionMode(
        &self,
        value: VisualInteractionSourceRedirectionMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionXChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionXChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionXSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionXSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionYChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetPositionYChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PositionYSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetPositionYSourceMode(
        &self,
        value: InteractionSourceMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScaleChainingMode(&self) -> ::windows::core::Result<InteractionChainingMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionChainingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionChainingMode>(result__)
        }
    }
    pub fn SetScaleChainingMode(
        &self,
        value: InteractionChainingMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ScaleSourceMode(&self) -> ::windows::core::Result<InteractionSourceMode> {
        let this = self;
        unsafe {
            let mut result__: InteractionSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceMode>(result__)
        }
    }
    pub fn SetScaleSourceMode(&self, value: InteractionSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<super::Visual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Visual>(result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn TryRedirectForManipulation<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::super::Input::PointerPoint>,
    >(
        &self,
        pointerpoint: Param0,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                pointerpoint.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DeltaPosition(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn DeltaScale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn PositionVelocity(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector3> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: ::windows::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn ScaleVelocity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureCenterPointXModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureCenterPointYModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureDeltaPositionXModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureDeltaPositionYModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ConfigureDeltaScaleModifiers<
        'a,
        Param0: ::windows::core::IntoParam<
            'a,
            ::windows::Foundation::Collections::IIterable<CompositionConditionalValue>,
        >,
    >(
        &self,
        conditionalvalues: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                conditionalvalues.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn PointerWheelConfig(&self) -> ::windows::core::Result<InteractionSourceConfiguration> {
        let this = &::windows::core::Interface::cast::<IVisualInteractionSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<InteractionSourceConfiguration>(result__)
        }
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Input"))]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(
        source: Param0,
    ) -> ::windows::core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Input"))]
    pub fn CreateFromIVisualElement<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::IVisualElement>,
    >(
        source: Param0,
    ) -> ::windows::core::Result<VisualInteractionSource> {
        Self::IVisualInteractionSourceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                source.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VisualInteractionSource>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PopulatePropertyInfo<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::AnimationPropertyInfo>,
    >(
        &self,
        propertyname: Param0,
        propertyinfo: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IAnimationObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                propertyinfo.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::Compositor> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Compositor>(result__)
        }
    }
    pub fn Properties(&self) -> ::windows::core::Result<super::CompositionPropertySet> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::CompositionPropertySet>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn StartAnimation<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param1: ::windows::core::IntoParam<'a, super::CompositionAnimation>,
    >(
        &self,
        propertyname: Param0,
        animation: Param1,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetComment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ImplicitAnimations(
        &self,
    ) -> ::windows::core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::ImplicitAnimationCollection>(result__)
        }
    }
    pub fn SetImplicitAnimations<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ImplicitAnimationCollection>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StartAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn StopAnimationGroup<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::ICompositionAnimationBase>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn TryGetAnimationController<
        'a,
        Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        &self,
        propertyname: Param0,
    ) -> ::windows::core::Result<super::AnimationController> {
        let this = &::windows::core::Interface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::AnimationController>(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<::windows::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok()
        }
    }
    pub fn IVisualInteractionSourceStatics<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVisualInteractionSourceStatics2<
        R,
        F: FnOnce(&IVisualInteractionSourceStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            VisualInteractionSource,
            IVisualInteractionSourceStatics2,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VisualInteractionSource {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Composition.Interactions.VisualInteractionSource;{ea595c95-b9cb-5cd4-bb9c-4934ff329063})" ) ;
}
unsafe impl ::windows::core::Interface for VisualInteractionSource {
    type Vtable = IVisualInteractionSource_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xea595c95_b9cb_5cd4_bb9c_4934ff329063);
}
impl ::windows::core::RuntimeName for VisualInteractionSource {
    const NAME: &'static str = "Microsoft.UI.Composition.Interactions.VisualInteractionSource";
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::core::IUnknown {
    fn from(value: VisualInteractionSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::core::IUnknown {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VisualInteractionSource> for ::windows::core::IInspectable {
    fn from(value: VisualInteractionSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VisualInteractionSource> for ::windows::core::IInspectable {
    fn from(value: &VisualInteractionSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a VisualInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ICompositionInteractionSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionInteractionSource> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionInteractionSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionInteractionSource>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionInteractionSource> {
        ::core::convert::TryInto::<ICompositionInteractionSource>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for super::IAnimationObject {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IAnimationObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::IAnimationObject> {
        ::core::convert::TryInto::<super::IAnimationObject>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VisualInteractionSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VisualInteractionSource> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &VisualInteractionSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for VisualInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::Foundation::IClosable>
    for &VisualInteractionSource
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::Foundation::IClosable> {
        ::core::convert::TryInto::<::windows::Foundation::IClosable>::try_into(self)
            .map(::windows::core::Param::Owned)
            .unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<VisualInteractionSource> for super::CompositionObject {
    fn from(value: VisualInteractionSource) -> Self {
        ::core::convert::Into::<super::CompositionObject>::into(&value)
    }
}
impl ::core::convert::From<&VisualInteractionSource> for super::CompositionObject {
    fn from(value: &VisualInteractionSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::CompositionObject> for &VisualInteractionSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::CompositionObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::CompositionObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for VisualInteractionSource {}
unsafe impl ::core::marker::Sync for VisualInteractionSource {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VisualInteractionSourceRedirectionMode(pub i32);
impl VisualInteractionSourceRedirectionMode {
    pub const Off: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(0i32);
    pub const CapableTouchpadOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(1i32);
    pub const PointerWheelOnly: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(2i32);
    pub const CapableTouchpadAndPointerWheel: VisualInteractionSourceRedirectionMode =
        VisualInteractionSourceRedirectionMode(3i32);
}
impl ::core::convert::From<i32> for VisualInteractionSourceRedirectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VisualInteractionSourceRedirectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VisualInteractionSourceRedirectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Interactions.VisualInteractionSourceRedirectionMode;i4)",
    );
}
impl ::windows::core::DefaultType for VisualInteractionSourceRedirectionMode {
    type DefaultType = Self;
}
