use makepad_widgets::LiveId;

/// 用于在after_apply中收集应用的属性
pub type LiveProps = Vec<(LiveId, LivePropsValue)>;
/// 基础应用类型的属性的值
pub type LivePropBasicValue = Option<Vec<LiveId>>;

/// 用于表示不同类型组件需要收集的应用属性的值的枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LivePropsValue {
    /// 基础
    Basic(LivePropBasicValue),
    /// 带有插槽的
    Slot(LiveProps),
}

impl LivePropsValue {
    pub fn is_basic(&self) -> bool {
        matches!(self, LivePropsValue::Basic(_))
    }

    pub fn is_slot(&self) -> bool {
        matches!(self, LivePropsValue::Slot(_))
    }
}

impl From<Option<Vec<LiveId>>> for LivePropsValue {
    fn from(value: Option<Vec<LiveId>>) -> Self {
        LivePropsValue::Basic(value)
    }
}

impl From<Vec<(LiveId, LivePropsValue)>> for LivePropsValue {
    fn from(value: Vec<(LiveId, LivePropsValue)>) -> Self {
        LivePropsValue::Slot(value)
    }
}

