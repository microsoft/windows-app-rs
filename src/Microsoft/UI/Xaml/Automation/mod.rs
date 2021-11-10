#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AnnotationPatternIdentifiers(pub ::windows::core::IInspectable);
impl AnnotationPatternIdentifiers {
    pub fn AnnotationTypeIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AnnotationTypeNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AuthorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DateTimeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn TargetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAnnotationPatternIdentifiersStatics<
        R,
        F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AnnotationPatternIdentifiers,
            IAnnotationPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AnnotationPatternIdentifiers;{92d76915-0cd3-59cd-8ae0-c9004628ba1e})" ) ;
}
unsafe impl ::windows::core::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x92d76915_0cd3_59cd_8ae0_c9004628ba1e);
}
impl ::windows::core::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::core::marker::Sync for AnnotationPatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: AnnotationType = AnnotationType(60000i32);
    pub const SpellingError: AnnotationType = AnnotationType(60001i32);
    pub const GrammarError: AnnotationType = AnnotationType(60002i32);
    pub const Comment: AnnotationType = AnnotationType(60003i32);
    pub const FormulaError: AnnotationType = AnnotationType(60004i32);
    pub const TrackChanges: AnnotationType = AnnotationType(60005i32);
    pub const Header: AnnotationType = AnnotationType(60006i32);
    pub const Footer: AnnotationType = AnnotationType(60007i32);
    pub const Highlighted: AnnotationType = AnnotationType(60008i32);
    pub const Endnote: AnnotationType = AnnotationType(60009i32);
    pub const Footnote: AnnotationType = AnnotationType(60010i32);
    pub const InsertionChange: AnnotationType = AnnotationType(60011i32);
    pub const DeletionChange: AnnotationType = AnnotationType(60012i32);
    pub const MoveChange: AnnotationType = AnnotationType(60013i32);
    pub const FormatChange: AnnotationType = AnnotationType(60014i32);
    pub const UnsyncedChange: AnnotationType = AnnotationType(60015i32);
    pub const EditingLockedChange: AnnotationType = AnnotationType(60016i32);
    pub const ExternalChange: AnnotationType = AnnotationType(60017i32);
    pub const ConflictingChange: AnnotationType = AnnotationType(60018i32);
    pub const Author: AnnotationType = AnnotationType(60019i32);
    pub const AdvancedProofingIssue: AnnotationType = AnnotationType(60020i32);
    pub const DataValidationError: AnnotationType = AnnotationType(60021i32);
    pub const CircularReferenceError: AnnotationType = AnnotationType(60022i32);
}
impl ::core::convert::From<i32> for AnnotationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AnnotationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AnnotationType;i4)",
    );
}
impl ::windows::core::DefaultType for AnnotationType {
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
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: AutomationActiveEnd = AutomationActiveEnd(0i32);
    pub const Start: AutomationActiveEnd = AutomationActiveEnd(1i32);
    pub const End: AutomationActiveEnd = AutomationActiveEnd(2i32);
}
impl ::core::convert::From<i32> for AutomationActiveEnd {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationActiveEnd {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationActiveEnd;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationActiveEnd {
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
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: AutomationAnimationStyle = AutomationAnimationStyle(0i32);
    pub const LasVegasLights: AutomationAnimationStyle = AutomationAnimationStyle(1i32);
    pub const BlinkingBackground: AutomationAnimationStyle = AutomationAnimationStyle(2i32);
    pub const SparkleText: AutomationAnimationStyle = AutomationAnimationStyle(3i32);
    pub const MarchingBlackAnts: AutomationAnimationStyle = AutomationAnimationStyle(4i32);
    pub const MarchingRedAnts: AutomationAnimationStyle = AutomationAnimationStyle(5i32);
    pub const Shimmer: AutomationAnimationStyle = AutomationAnimationStyle(6i32);
    pub const Other: AutomationAnimationStyle = AutomationAnimationStyle(7i32);
}
impl ::core::convert::From<i32> for AutomationAnimationStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationAnimationStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationAnimationStyle;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationAnimationStyle {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AutomationAnnotation(pub ::windows::core::IInspectable);
impl AutomationAnnotation {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AutomationAnnotation,
            ::windows::core::IActivationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Type(&self) -> ::windows::core::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__: AnnotationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AnnotationType>(result__)
        }
    }
    pub fn SetType(&self, value: AnnotationType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn Element(&self) -> ::windows::core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn SetElement<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(
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
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn CreateInstance(r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn CreateWithElementParameter<
        'a,
        Param1: ::windows::core::IntoParam<'a, super::UIElement>,
    >(
        r#type: AnnotationType,
        element: Param1,
    ) -> ::windows::core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                r#type,
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn TypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ElementProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
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
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ClearValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ReadLocalValue<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
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
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
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
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    pub fn Dispatcher(&self) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IAutomationAnnotationFactory<
        R,
        F: FnOnce(&IAutomationAnnotationFactory) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationFactory,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationAnnotationStatics<
        R,
        F: FnOnce(&IAutomationAnnotationStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationAnnotation {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationAnnotation;{c2cc46ad-1414-5f1b-808a-89e5d53d82fe})" ) ;
}
unsafe impl ::windows::core::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc2cc46ad_1414_5f1b_808a_89e5d53d82fe);
}
impl ::windows::core::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationAnnotation";
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: AutomationAnnotation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IUnknown {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: AutomationAnnotation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows::core::IInspectable {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AutomationAnnotation
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AutomationAnnotation> for super::DependencyObject {
    fn from(value: AutomationAnnotation) -> Self {
        ::core::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::core::convert::From<&AutomationAnnotation> for super::DependencyObject {
    fn from(value: &AutomationAnnotation) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(
            ::core::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::core::marker::Send for AutomationAnnotation {}
unsafe impl ::core::marker::Sync for AutomationAnnotation {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: AutomationBulletStyle = AutomationBulletStyle(0i32);
    pub const HollowRoundBullet: AutomationBulletStyle = AutomationBulletStyle(1i32);
    pub const FilledRoundBullet: AutomationBulletStyle = AutomationBulletStyle(2i32);
    pub const HollowSquareBullet: AutomationBulletStyle = AutomationBulletStyle(3i32);
    pub const FilledSquareBullet: AutomationBulletStyle = AutomationBulletStyle(4i32);
    pub const DashBullet: AutomationBulletStyle = AutomationBulletStyle(5i32);
    pub const Other: AutomationBulletStyle = AutomationBulletStyle(6i32);
}
impl ::core::convert::From<i32> for AutomationBulletStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationBulletStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationBulletStyle;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationBulletStyle {
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
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: AutomationCaretBidiMode = AutomationCaretBidiMode(0i32);
    pub const RTL: AutomationCaretBidiMode = AutomationCaretBidiMode(1i32);
}
impl ::core::convert::From<i32> for AutomationCaretBidiMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretBidiMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretBidiMode;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationCaretBidiMode {
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
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: AutomationCaretPosition = AutomationCaretPosition(0i32);
    pub const EndOfLine: AutomationCaretPosition = AutomationCaretPosition(1i32);
    pub const BeginningOfLine: AutomationCaretPosition = AutomationCaretPosition(2i32);
}
impl ::core::convert::From<i32> for AutomationCaretPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationCaretPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretPosition;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationCaretPosition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AutomationElementIdentifiers(pub ::windows::core::IInspectable);
impl AutomationElementIdentifiers {
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AutomationIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn BoundingRectangleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ClassNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ClickablePointProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HasKeyboardFocusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HelpTextProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsContentElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsControlElementProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsKeyboardFocusableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsOffscreenProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsPasswordProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ItemStatusProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ItemTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LabeledByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn NameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn OrientationProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LiveSettingProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ControlledPeersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn PositionInSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SizeOfSetProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AnnotationsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsPeripheralProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FullDescriptionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DescribedByProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FlowsToProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FlowsFromProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CultureProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HeadingLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsDialogProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAutomationElementIdentifiersStatics<
        R,
        F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AutomationElementIdentifiers,
            IAutomationElementIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationElementIdentifiers;{2fb51a33-b0cf-5a4c-9ed3-267eca7aeefc})" ) ;
}
unsafe impl ::windows::core::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2fb51a33_b0cf_5a4c_9ed3_267eca7aeefc);
}
impl ::windows::core::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationElementIdentifiers";
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::core::marker::Sync for AutomationElementIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: AutomationFlowDirections = AutomationFlowDirections(0i32);
    pub const RightToLeft: AutomationFlowDirections = AutomationFlowDirections(1i32);
    pub const BottomToTop: AutomationFlowDirections = AutomationFlowDirections(2i32);
    pub const Vertical: AutomationFlowDirections = AutomationFlowDirections(3i32);
}
impl ::core::convert::From<i32> for AutomationFlowDirections {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationFlowDirections {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationFlowDirections;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationFlowDirections {
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
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: AutomationOutlineStyles = AutomationOutlineStyles(0i32);
    pub const Outline: AutomationOutlineStyles = AutomationOutlineStyles(1i32);
    pub const Shadow: AutomationOutlineStyles = AutomationOutlineStyles(2i32);
    pub const Engraved: AutomationOutlineStyles = AutomationOutlineStyles(3i32);
    pub const Embossed: AutomationOutlineStyles = AutomationOutlineStyles(4i32);
}
impl ::core::convert::From<i32> for AutomationOutlineStyles {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationOutlineStyles {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationOutlineStyles;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationOutlineStyles {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AutomationProperties(pub ::windows::core::IInspectable);
impl AutomationProperties {
    #[cfg(feature = "UI_Dispatching")]
    pub fn AcceleratorKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAcceleratorKey<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAcceleratorKey<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessKeyProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAccessKey<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AutomationIdProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetAutomationId<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetAutomationId<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).14)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn HelpTextProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetHelpText<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetHelpText<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).17)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsRequiredForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetIsRequiredForForm<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetIsRequiredForForm<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).20)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ItemStatusProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetItemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetItemStatus<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).23)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ItemTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetItemType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetItemType<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).26)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LabeledByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn GetLabeledBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<super::UIElement> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn SetLabeledBy<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, super::UIElement>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).29)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn NameProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetName<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetName<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).32)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LiveSettingProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn GetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<Peers::AutomationLiveSetting> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLiveSetting = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationLiveSetting>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn SetLiveSetting<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).35)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AccessibilityViewProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn GetAccessibilityView<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<Peers::AccessibilityView> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AccessibilityView = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AccessibilityView>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn SetAccessibilityView<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: Peers::AccessibilityView,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).38)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn ControlledPeersProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn GetControlledPeers<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::UIElement>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::UIElement>>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn PositionInSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetPositionInSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).43)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SizeOfSetProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetSizeOfSet<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).46)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).49)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn AnnotationsProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    pub fn GetAnnotations<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<AutomationAnnotation>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AutomationAnnotation>>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn GetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<Peers::AutomationLandmarkType> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLandmarkType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).53)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationLandmarkType>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn SetLandmarkType<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).54)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetLocalizedLandmarkType<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetLocalizedLandmarkType<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).57)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsPeripheralProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).59)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetIsPeripheral<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).60)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsDataValidForFormProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetIsDataValidForForm<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).62)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetIsDataValidForForm<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).63)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FullDescriptionProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetFullDescription<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).65)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetFullDescription<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).66)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn LocalizedControlTypeProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).67)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetLocalizedControlType<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> =
                ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).68)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetLocalizedControlType<
        'a,
        Param0: ::windows::core::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).69)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DescribedByProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetDescribedBy<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).71)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FlowsToProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetFlowsTo<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).73)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn FlowsFromProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).74)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetFlowsFrom<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVector<super::DependencyObject>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).75)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn CultureProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).77)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetCulture<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).78)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn HeadingLevelProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn GetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<Peers::AutomationHeadingLevel> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationHeadingLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).80)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationHeadingLevel>(result__)
        })
    }
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub fn SetHeadingLevel<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).81)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn IsDialogProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).82)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn GetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).83)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn SetIsDialog<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::core::Interface::vtable(this).84)(
                ::core::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn IAutomationPropertiesStatics<
        R,
        F: FnOnce(&IAutomationPropertiesStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            AutomationProperties,
            IAutomationPropertiesStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationProperties {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationProperties;{525c6a71-dd8a-52a0-977b-db1b02f8e896})" ) ;
}
unsafe impl ::windows::core::Interface for AutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x525c6a71_dd8a_52a0_977b_db1b02f8e896);
}
impl ::windows::core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: AutomationProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: AutomationProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a AutomationProperties
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationProperties {}
unsafe impl ::core::marker::Sync for AutomationProperties {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct AutomationProperty(pub ::windows::core::IInspectable);
impl AutomationProperty {}
unsafe impl ::windows::core::RuntimeType for AutomationProperty {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationProperty;{5ca6b2c8-ff86-5a41-aa18-6948fae592cf})" ) ;
}
unsafe impl ::windows::core::Interface for AutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5ca6b2c8_ff86_5a41_aa18_6948fae592cf);
}
impl ::windows::core::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperty";
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: AutomationProperty) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IUnknown {
    fn from(value: &AutomationProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: AutomationProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows::core::IInspectable {
    fn from(value: &AutomationProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutomationProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AutomationProperty {}
unsafe impl ::core::marker::Sync for AutomationProperty {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: AutomationStyleId = AutomationStyleId(70001i32);
    pub const Heading2: AutomationStyleId = AutomationStyleId(70002i32);
    pub const Heading3: AutomationStyleId = AutomationStyleId(70003i32);
    pub const Heading4: AutomationStyleId = AutomationStyleId(70004i32);
    pub const Heading5: AutomationStyleId = AutomationStyleId(70005i32);
    pub const Heading6: AutomationStyleId = AutomationStyleId(70006i32);
    pub const Heading7: AutomationStyleId = AutomationStyleId(70007i32);
    pub const Heading8: AutomationStyleId = AutomationStyleId(70008i32);
    pub const Heading9: AutomationStyleId = AutomationStyleId(70009i32);
    pub const Title: AutomationStyleId = AutomationStyleId(70010i32);
    pub const Subtitle: AutomationStyleId = AutomationStyleId(70011i32);
    pub const Normal: AutomationStyleId = AutomationStyleId(70012i32);
    pub const Emphasis: AutomationStyleId = AutomationStyleId(70013i32);
    pub const Quote: AutomationStyleId = AutomationStyleId(70014i32);
    pub const BulletedList: AutomationStyleId = AutomationStyleId(70015i32);
}
impl ::core::convert::From<i32> for AutomationStyleId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationStyleId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationStyleId;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationStyleId {
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
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(0i32);
    pub const Single: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(1i32);
    pub const WordsOnly: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(2i32);
    pub const Double: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(3i32);
    pub const Dot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(4i32);
    pub const Dash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(5i32);
    pub const DashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(6i32);
    pub const DashDotDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(7i32);
    pub const Wavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(8i32);
    pub const ThickSingle: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(9i32);
    pub const DoubleWavy: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(10i32);
    pub const ThickWavy: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(11i32);
    pub const LongDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(12i32);
    pub const ThickDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(13i32);
    pub const ThickDashDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(14i32);
    pub const ThickDashDotDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(15i32);
    pub const ThickDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(16i32);
    pub const ThickLongDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(17i32);
    pub const Other: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(18i32);
}
impl ::core::convert::From<i32> for AutomationTextDecorationLineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationTextDecorationLineStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationTextDecorationLineStyle {
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
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: AutomationTextEditChangeType = AutomationTextEditChangeType(0i32);
    pub const AutoCorrect: AutomationTextEditChangeType = AutomationTextEditChangeType(1i32);
    pub const Composition: AutomationTextEditChangeType = AutomationTextEditChangeType(2i32);
    pub const CompositionFinalized: AutomationTextEditChangeType =
        AutomationTextEditChangeType(3i32);
}
impl ::core::convert::From<i32> for AutomationTextEditChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AutomationTextEditChangeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationTextEditChangeType;i4)",
    );
}
impl ::windows::core::DefaultType for AutomationTextEditChangeType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DockPatternIdentifiers(pub ::windows::core::IInspectable);
impl DockPatternIdentifiers {
    pub fn DockPositionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDockPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DockPatternIdentifiers,
            IDockPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DockPatternIdentifiers;{75574f99-d145-547e-972b-7d879f93c03e})" ) ;
}
unsafe impl ::windows::core::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75574f99_d145_547e_972b_7d879f93c03e);
}
impl ::windows::core::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DockPatternIdentifiers";
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DockPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DockPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DockPatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: DockPosition = DockPosition(0i32);
    pub const Left: DockPosition = DockPosition(1i32);
    pub const Bottom: DockPosition = DockPosition(2i32);
    pub const Right: DockPosition = DockPosition(3i32);
    pub const Fill: DockPosition = DockPosition(4i32);
    pub const None: DockPosition = DockPosition(5i32);
}
impl ::core::convert::From<i32> for DockPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DockPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DockPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.DockPosition;i4)",
    );
}
impl ::windows::core::DefaultType for DockPosition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DragPatternIdentifiers(pub ::windows::core::IInspectable);
impl DragPatternIdentifiers {
    pub fn DropEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DropEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn GrabbedItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsGrabbedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDragPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DragPatternIdentifiers,
            IDragPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DragPatternIdentifiers;{aa2fdfd5-fb45-5d2b-8d92-a8e7b07061c2})" ) ;
}
unsafe impl ::windows::core::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xaa2fdfd5_fb45_5d2b_8d92_a8e7b07061c2);
}
impl ::windows::core::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DragPatternIdentifiers";
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DragPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DragPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DragPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct DropTargetPatternIdentifiers(pub ::windows::core::IInspectable);
impl DropTargetPatternIdentifiers {
    pub fn DropTargetEffectProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DropTargetEffectsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDropTargetPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            DropTargetPatternIdentifiers,
            IDropTargetPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DropTargetPatternIdentifiers;{133e8ff3-1ddd-5cbb-b908-1484d7c04da7})" ) ;
}
unsafe impl ::windows::core::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x133e8ff3_1ddd_5cbb_b908_1484d7c04da7);
}
impl ::windows::core::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DropTargetPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ExpandCollapsePatternIdentifiers(pub ::windows::core::IInspectable);
impl ExpandCollapsePatternIdentifiers {
    pub fn ExpandCollapseStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IExpandCollapsePatternIdentifiersStatics<
        R,
        F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ExpandCollapsePatternIdentifiers,
            IExpandCollapsePatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers;{cec15d9f-8630-569a-86a0-524bbea618ff})" ) ;
}
unsafe impl ::windows::core::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcec15d9f_8630_569a_86a0_524bbea618ff);
}
impl ::windows::core::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
    pub const Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
    pub const PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
    pub const LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
}
impl ::core::convert::From<i32> for ExpandCollapseState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExpandCollapseState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ExpandCollapseState;i4)",
    );
}
impl ::windows::core::DefaultType for ExpandCollapseState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct GridItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl GridItemPatternIdentifiers {
    pub fn ColumnProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ColumnSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ContainingGridProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowSpanProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            GridItemPatternIdentifiers,
            IGridItemPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.GridItemPatternIdentifiers;{93609087-1114-557d-b17b-f801e41cebbb})" ) ;
}
unsafe impl ::windows::core::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93609087_1114_557d_b17b_f801e41cebbb);
}
impl ::windows::core::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct GridPatternIdentifiers(pub ::windows::core::IInspectable);
impl GridPatternIdentifiers {
    pub fn ColumnCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowCountProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            GridPatternIdentifiers,
            IGridPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.GridPatternIdentifiers;{e5e1e250-c37c-54a2-8c61-1d9ccd3bb39c})" ) ;
}
unsafe impl ::windows::core::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5e1e250_c37c_54a2_8c61_1d9ccd3bb39c);
}
impl ::windows::core::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridPatternIdentifiers";
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a GridPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GridPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridPatternIdentifiers {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x92d76915_0cd3_59cd_8ae0_c9004628ba1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers_abi(
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
pub struct IAnnotationPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x20a136e2_4a47_5de5_9e1e_ecfc6d92f52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc2cc46ad_1414_5f1b_808a_89e5d53d82fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotation_abi(
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
        result__: *mut AnnotationType,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        value: AnnotationType,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
#[doc(hidden)]
pub struct IAutomationAnnotationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactory_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x95f82773_eac5_572e_87de_24d9514b9a89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory_abi(
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
        r#type: AnnotationType,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
        r#type: AnnotationType,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
#[doc(hidden)]
pub struct IAutomationAnnotationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc5abdc1e_fc26_5444_a8b3_59b2c0a95578);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2fb51a33_b0cf_5a4c_9ed3_267eca7aeefc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers_abi(
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
pub struct IAutomationElementIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x72af6b8c_3e12_5e7a_a2ec_26dc193f9df9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x525c6a71_dd8a_52a0_977b_db1b02f8e896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_abi(
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
pub struct IAutomationPropertiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb1e3e0f3_112f_5966_87dc_7862d4ad50e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
        value: ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut Peers::AutomationLiveSetting,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut Peers::AccessibilityView,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: Peers::AccessibilityView,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(
        feature = "UI_Composition",
        feature = "UI_Dispatching",
        feature = "UI_Input",
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
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut Peers::AutomationLandmarkType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut Peers::AutomationHeadingLevel,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers"))]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Dispatching", feature = "UI_Xaml_Automation_Peers")))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
    #[cfg(feature = "UI_Dispatching")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        element: ::windows::core::RawPtr,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationProperty(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5ca6b2c8_ff86_5a41_aa18_6948fae592cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperty_abi(
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
pub struct IDockPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x75574f99_d145_547e_972b_7d879f93c03e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers_abi(
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
pub struct IDockPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02d5a72c_f49d_53a9_b9fb_af2719d16ccf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics_abi(
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
pub struct IDragPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xaa2fdfd5_fb45_5d2b_8d92_a8e7b07061c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiers_abi(
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
pub struct IDragPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x482eee70_0bfc_5552_9e7d_8dffc526b2f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x133e8ff3_1ddd_5cbb_b908_1484d7c04da7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers_abi(
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
pub struct IDropTargetPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6da6f0bd_b942_5283_be35_501ae87f88c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcec15d9f_8630_569a_86a0_524bbea618ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers_abi(
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
pub struct IExpandCollapsePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x283101f4_c40c_55bf_a23b_d62b73b6aa35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics_abi(
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
pub struct IGridItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x93609087_1114_557d_b17b_f801e41cebbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiers_abi(
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
pub struct IGridItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8072bc18_87d0_5a02_a0a1_f9aec968c0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe5e1e250_c37c_54a2_8c61_1d9ccd3bb39c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers_abi(
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
pub struct IGridPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe861604c_101f_5a6d_a308_3714f510f744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x70e4c847_2b82_5ecf_b808_e9d453c1fe53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers_abi(
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
pub struct IMultipleViewPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xac71daef_d094_5c90_94af_1fa474ab45fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc114db37_6a75_5ef1_a542_d3b13f92cbfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers_abi(
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
pub struct IRangeValuePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0aaa9ad7_f9b8_52a1_bc96_2a97fe389ed0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x04f1a4b8_edc7_55f2_96df_a9c7e809372e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers_abi(
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
pub struct IScrollPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0f94f2f0_e0d2_5a24_b415_8d1506ce47aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut f64,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce3a549d_a2cb_594d_a2a4_44778c09cca5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers_abi(
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
pub struct ISelectionItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2b8ead7c_4e03_5b84_9e34_8b7384cbd862);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x401743d2_1fba_5d05_b89f_631676453237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers_abi(
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
pub struct ISelectionPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xf3ed111b_b20a_5e5e_a232_07f607fd5c07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdca2ec46_8564_5c9c_ba90_2c08455f697b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers_abi(
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
pub struct ISpreadsheetItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7eb10f80_8d3a_59ad_a2b9_05d8cecf18db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics_abi(
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
pub struct IStylesPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x13aeca5e_b496_5df5_aea5_330e1f0490eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiers_abi(
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
pub struct IStylesPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb232287a_bc4c_581e_a33c_3d6aee10d04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb4de5d03_a5b4_5ca1_8715_16c8c6a10fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers_abi(
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
pub struct ITableItemPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x81d24bd7_66fb_53ef_9b32_d00f9c240a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3d7f9c0b_ff8f_50fa_bc01_2cc3c2e06e2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers_abi(
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
pub struct ITablePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3660935e_bcbb_5848_8e9a_264854f7a19a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa0d2df4c_ba59_51d9_9c01_034d7941c280);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers_abi(
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
pub struct ITogglePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x862920b5_dcb3_5691_a456_c2f15c476dfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics_abi(
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
pub struct ITransformPattern2Identifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6ef7595c_db8c_51b0_878b_34b7ef12f4da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2Identifiers_abi(
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
pub struct ITransformPattern2IdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xd9876ff5_89ed_5333_8111_ad25a28bee8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2348187b_c50f_5a0e_bc05_305ac71b3b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers_abi(
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
pub struct ITransformPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcb7d84e4_5429_5188_8aa0_5f96558a8790);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfb493395_fb97_59d5_9323_4651ce964b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers_abi(
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
pub struct IValuePatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2019faf5_ce64_59a7_bc13_0677c3146724);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbec579e1_91be_5d8f_aaca_6ad8839872d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers_abi(
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
pub struct IWindowPatternIdentifiersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStatics_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x06762744_d3d7_5441_b879_373681d47f64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::core::RawPtr,
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
        result__: *mut ::windows::core::RawPtr,
    ) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct MultipleViewPatternIdentifiers(pub ::windows::core::IInspectable);
impl MultipleViewPatternIdentifiers {
    pub fn CurrentViewProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SupportedViewsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IMultipleViewPatternIdentifiersStatics<
        R,
        F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            MultipleViewPatternIdentifiers,
            IMultipleViewPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.MultipleViewPatternIdentifiers;{70e4c847-2b82-5ecf-b808-e9d453c1fe53})" ) ;
}
unsafe impl ::windows::core::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x70e4c847_2b82_5ecf_b808_e9d453c1fe53);
}
impl ::windows::core::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::core::marker::Sync for MultipleViewPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct RangeValuePatternIdentifiers(pub ::windows::core::IInspectable);
impl RangeValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LargeChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MaximumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MinimumProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SmallChangeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IRangeValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            RangeValuePatternIdentifiers,
            IRangeValuePatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.RangeValuePatternIdentifiers;{c114db37-6a75-5ef1-a542-d3b13f92cbfe})" ) ;
}
unsafe impl ::windows::core::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xc114db37_6a75_5ef1_a542_d3b13f92cbfe);
}
impl ::windows::core::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for RangeValuePatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
    pub const ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
    pub const Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
}
impl ::core::convert::From<i32> for RowOrColumnMajor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RowOrColumnMajor {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.RowOrColumnMajor;i4)",
    );
}
impl ::windows::core::DefaultType for RowOrColumnMajor {
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
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: ScrollAmount = ScrollAmount(0i32);
    pub const SmallDecrement: ScrollAmount = ScrollAmount(1i32);
    pub const NoAmount: ScrollAmount = ScrollAmount(2i32);
    pub const LargeIncrement: ScrollAmount = ScrollAmount(3i32);
    pub const SmallIncrement: ScrollAmount = ScrollAmount(4i32);
}
impl ::core::convert::From<i32> for ScrollAmount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ScrollAmount {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ScrollAmount;i4)",
    );
}
impl ::windows::core::DefaultType for ScrollAmount {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ScrollPatternIdentifiers(pub ::windows::core::IInspectable);
impl ScrollPatternIdentifiers {
    pub fn HorizontallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HorizontalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HorizontalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn NoScroll() -> ::windows::core::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        })
    }
    pub fn VerticallyScrollableProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn VerticalScrollPercentProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn VerticalViewSizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IScrollPatternIdentifiersStatics<
        R,
        F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ScrollPatternIdentifiers,
            IScrollPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ScrollPatternIdentifiers;{04f1a4b8-edc7-55f2-96df-a9c7e809372e})" ) ;
}
unsafe impl ::windows::core::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x04f1a4b8_edc7_55f2_96df_a9c7e809372e);
}
impl ::windows::core::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::core::marker::Sync for ScrollPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SelectionItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl SelectionItemPatternIdentifiers {
    pub fn IsSelectedProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SelectionContainerProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            SelectionItemPatternIdentifiers,
            ISelectionItemPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SelectionItemPatternIdentifiers;{ce3a549d-a2cb-594d-a2a4-44778c09cca5})" ) ;
}
unsafe impl ::windows::core::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xce3a549d_a2cb_594d_a2a4_44778c09cca5);
}
impl ::windows::core::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SelectionPatternIdentifiers(pub ::windows::core::IInspectable);
impl SelectionPatternIdentifiers {
    pub fn CanSelectMultipleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsSelectionRequiredProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SelectionProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            SelectionPatternIdentifiers,
            ISelectionPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SelectionPatternIdentifiers;{401743d2-1fba-5d05-b89f-631676453237})" ) ;
}
unsafe impl ::windows::core::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x401743d2_1fba_5d05_b89f_631676453237);
}
impl ::windows::core::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct SpreadsheetItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl SpreadsheetItemPatternIdentifiers {
    pub fn FormulaProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISpreadsheetItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            SpreadsheetItemPatternIdentifiers,
            ISpreadsheetItemPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers;{dca2ec46-8564-5c9c-ba90-2c08455f697b})" ) ;
}
unsafe impl ::windows::core::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xdca2ec46_8564_5c9c_ba90_2c08455f697b);
}
impl ::windows::core::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct StylesPatternIdentifiers(pub ::windows::core::IInspectable);
impl StylesPatternIdentifiers {
    pub fn ExtendedPropertiesProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillPatternColorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillPatternStyleProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ShapeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn StyleIdProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn StyleNameProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IStylesPatternIdentifiersStatics<
        R,
        F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            StylesPatternIdentifiers,
            IStylesPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.StylesPatternIdentifiers;{13aeca5e-b496-5df5-aea5-330e1f0490eb})" ) ;
}
unsafe impl ::windows::core::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x13aeca5e_b496_5df5_aea5_330e1f0490eb);
}
impl ::windows::core::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.StylesPatternIdentifiers";
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::core::marker::Sync for StylesPatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: SupportedTextSelection = SupportedTextSelection(0i32);
    pub const Single: SupportedTextSelection = SupportedTextSelection(1i32);
    pub const Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
}
impl ::core::convert::From<i32> for SupportedTextSelection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SupportedTextSelection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.SupportedTextSelection;i4)",
    );
}
impl ::windows::core::DefaultType for SupportedTextSelection {
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
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
    pub const KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
    pub const LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
    pub const LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
    pub const RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
    pub const RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
}
impl ::core::convert::From<i32> for SynchronizedInputType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SynchronizedInputType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.SynchronizedInputType;i4)",
    );
}
impl ::windows::core::DefaultType for SynchronizedInputType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TableItemPatternIdentifiers(pub ::windows::core::IInspectable);
impl TableItemPatternIdentifiers {
    pub fn ColumnHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowHeaderItemsProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITableItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            TableItemPatternIdentifiers,
            ITableItemPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TableItemPatternIdentifiers;{b4de5d03-a5b4-5ca1-8715-16c8c6a10fcc})" ) ;
}
unsafe impl ::windows::core::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xb4de5d03_a5b4_5ca1_8715_16c8c6a10fcc);
}
impl ::windows::core::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TableItemPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TablePatternIdentifiers(pub ::windows::core::IInspectable);
impl TablePatternIdentifiers {
    pub fn ColumnHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowHeadersProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowOrColumnMajorProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITablePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            TablePatternIdentifiers,
            ITablePatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TablePatternIdentifiers;{3d7f9c0b-ff8f-50fa-bc01-2cc3c2e06e2c})" ) ;
}
unsafe impl ::windows::core::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3d7f9c0b_ff8f_50fa_bc01_2cc3c2e06e2c);
}
impl ::windows::core::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TablePatternIdentifiers";
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TablePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TablePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TablePatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TogglePatternIdentifiers(pub ::windows::core::IInspectable);
impl TogglePatternIdentifiers {
    pub fn ToggleStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITogglePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            TogglePatternIdentifiers,
            ITogglePatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TogglePatternIdentifiers;{a0d2df4c-ba59-51d9-9c01-034d7941c280})" ) ;
}
unsafe impl ::windows::core::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa0d2df4c_ba59_51d9_9c01_034d7941c280);
}
impl ::windows::core::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TogglePatternIdentifiers";
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TogglePatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: ToggleState = ToggleState(0i32);
    pub const On: ToggleState = ToggleState(1i32);
    pub const Indeterminate: ToggleState = ToggleState(2i32);
}
impl ::core::convert::From<i32> for ToggleState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ToggleState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ToggleState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ToggleState;i4)",
    );
}
impl ::windows::core::DefaultType for ToggleState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TransformPattern2Identifiers(pub ::windows::core::IInspectable);
impl TransformPattern2Identifiers {
    pub fn CanZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ZoomLevelProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MaxZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MinZoomProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPattern2IdentifiersStatics<
        R,
        F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            TransformPattern2Identifiers,
            ITransformPattern2IdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TransformPattern2Identifiers;{6ef7595c-db8c-51b0-878b-34b7ef12f4da})" ) ;
}
unsafe impl ::windows::core::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6ef7595c_db8c_51b0_878b_34b7ef12f4da);
}
impl ::windows::core::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPattern2Identifiers";
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::core::marker::Sync for TransformPattern2Identifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct TransformPatternIdentifiers(pub ::windows::core::IInspectable);
impl TransformPatternIdentifiers {
    pub fn CanMoveProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanResizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanRotateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            TransformPatternIdentifiers,
            ITransformPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TransformPatternIdentifiers;{2348187b-c50f-5a0e-bc05-305ac71b3b6b})" ) ;
}
unsafe impl ::windows::core::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2348187b_c50f_5a0e_bc05_305ac71b3b6b);
}
impl ::windows::core::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPatternIdentifiers";
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TransformPatternIdentifiers {}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct ValuePatternIdentifiers(pub ::windows::core::IInspectable);
impl ValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            ValuePatternIdentifiers,
            IValuePatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ValuePatternIdentifiers;{fb493395-fb97-59d5-9323-4651ce964b55})" ) ;
}
unsafe impl ::windows::core::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfb493395_fb97_59d5_9323_4651ce964b55);
}
impl ::windows::core::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ValuePatternIdentifiers";
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a ValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ValuePatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: WindowInteractionState = WindowInteractionState(0i32);
    pub const Closing: WindowInteractionState = WindowInteractionState(1i32);
    pub const ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
    pub const BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
    pub const NotResponding: WindowInteractionState = WindowInteractionState(4i32);
}
impl ::core::convert::From<i32> for WindowInteractionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WindowInteractionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.WindowInteractionState;i4)",
    );
}
impl ::windows::core::DefaultType for WindowInteractionState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: clone :: Clone,
    :: core :: fmt :: Debug,
)]
pub struct WindowPatternIdentifiers(pub ::windows::core::IInspectable);
impl WindowPatternIdentifiers {
    pub fn CanMaximizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanMinimizeProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsModalProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsTopmostProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn WindowInteractionStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn WindowVisualStateProperty() -> ::windows::core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(
                ::core::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IWindowPatternIdentifiersStatics<
        R,
        F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<
            WindowPatternIdentifiers,
            IWindowPatternIdentifiersStatics,
        > = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE : :: windows :: core :: ConstBuffer = :: windows :: core :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.WindowPatternIdentifiers;{bec579e1-91be-5d8f-aaca-6ad8839872d2})" ) ;
}
unsafe impl ::windows::core::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbec579e1_91be_5d8f_aaca_6ad8839872d2);
}
impl ::windows::core::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.WindowPatternIdentifiers";
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IUnknown {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown>
    for &'a WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows::core::IInspectable {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable>
    for &'a WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::core::marker::Sync for WindowPatternIdentifiers {}
#[derive(
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: marker :: Copy,
    :: core :: clone :: Clone,
    :: core :: default :: Default,
    :: core :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: WindowVisualState = WindowVisualState(0i32);
    pub const Maximized: WindowVisualState = WindowVisualState(1i32);
    pub const Minimized: WindowVisualState = WindowVisualState(2i32);
}
impl ::core::convert::From<i32> for WindowVisualState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WindowVisualState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.WindowVisualState;i4)",
    );
}
impl ::windows::core::DefaultType for WindowVisualState {
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
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: ZoomUnit = ZoomUnit(0i32);
    pub const LargeDecrement: ZoomUnit = ZoomUnit(1i32);
    pub const SmallDecrement: ZoomUnit = ZoomUnit(2i32);
    pub const LargeIncrement: ZoomUnit = ZoomUnit(3i32);
    pub const SmallIncrement: ZoomUnit = ZoomUnit(4i32);
}
impl ::core::convert::From<i32> for ZoomUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ZoomUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows::core::ConstBuffer =
        ::windows::core::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Automation.ZoomUnit;i4)");
}
impl ::windows::core::DefaultType for ZoomUnit {
    type DefaultType = Self;
}
