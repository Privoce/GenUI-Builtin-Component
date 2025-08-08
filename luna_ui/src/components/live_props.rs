use makepad_widgets::{LiveId, LiveIdAsProp, LiveProp};

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

    /// 构建路径并插入到给定的路径列表中
    /// - 在insert函数中，路径会被处理，第二个参数为该路径是否为深度路径
    /// - 深度路径：在这里并不是指Slot，而是指基础应用类型的属性的值是Some的情况，例如: Margin, Padding等
    pub fn build_paths_and_insert(
        &self,
        paths: &mut Vec<LiveProp>,
        insert: &mut dyn FnMut(&Vec<LiveProp>, Option<&Vec<LiveId>>),
    ) -> () {
        match self {
            LivePropsValue::Basic(fields) => {
                if let Some(fields) = fields {
                    for field in fields {
                        // 需要使用临时量来处理，因为field是需要push一个insert一个的
                        let mut tmp_paths = paths.clone();
                        tmp_paths.push(field.as_field());
                        insert(&tmp_paths, Some(fields));
                    }
                } else {
                    insert(paths, None);
                }
            }
            LivePropsValue::Slot(items) => {
                for (id, value) in items {
                    paths.push(id.as_field());
                    // value需要依据类型来处理, 递归调用
                    value.build_paths_and_insert(paths, insert);
                    paths.pop();
                }
            }
        }
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
