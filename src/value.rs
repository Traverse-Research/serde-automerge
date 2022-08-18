use std::ops::{Deref, DerefMut};

use anyhow::Result;
use automerge::{
    transaction::{Transactable, Transaction},
    Automerge, AutomergeError, ObjId, ObjType, Prop, ScalarValue, ROOT,
};

pub trait AutomergePuttable {
    fn put<T: Transactable>(
        &self,
        obj: &ObjId,
        prop: Prop,
        tr: &mut T,
    ) -> Result<Option<ObjId>, AutomergeError>;
}

impl<S> AutomergePuttable for S
where
    S: Copy,
    ScalarValue: From<S>,
{
    fn put<T: Transactable>(
        &self,
        obj: &ObjId,
        prop: Prop,
        tr: &mut T,
    ) -> Result<Option<ObjId>, AutomergeError> {
        tr.put(&obj, prop, ScalarValue::from(*self))?;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct Value<V>
where
    V: AutomergePuttable,
{
    value: V,
    obj: ObjId,
    prop: Prop,
}

impl<V: AutomergePuttable> Value<V> {
    pub fn new<O: Into<ObjId>, P: Into<Prop>>(value: V, obj: O, prop: P) -> Self {
        Self {
            value,
            obj: obj.into(),
            prop: prop.into(),
        }
    }
    pub fn get_mut<'v, 't, T: Transactable>(
        &'v mut self,
        tr: &'t mut T,
    ) -> ValueGuard<'v, 't, V, T> {
        ValueGuard {
            value: self,
            transaction: tr,
        }
    }
}

impl<V: AutomergePuttable> Deref for Value<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct ValueGuard<'v, 't, V, T>
where
    V: AutomergePuttable,
    T: Transactable,
{
    value: &'v mut Value<V>,
    transaction: &'t mut T,
}

impl<'v, 't, V: AutomergePuttable, T: Transactable> Deref for ValueGuard<'v, 't, V, T> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<'v, 't, V: AutomergePuttable, T: Transactable> DerefMut for ValueGuard<'v, 't, V, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value.value
    }
}
impl<'v, 't, V: AutomergePuttable, T: Transactable> Drop for ValueGuard<'v, 't, V, T> {
    fn drop(&mut self) {
        if let Some(obj) = self
            .value
            .value
            .put(&self.value.obj, self.value.prop.clone(), self.transaction)
            .unwrap()
        {
            self.value.obj = obj;
        }
    }
}

#[derive(Debug)]
struct Point(f64, f64, f64);

impl AutomergePuttable for Point {
    fn put<T: Transactable>(
        &self,
        obj: &ObjId,
        prop: Prop,
        tr: &mut T,
    ) -> Result<Option<ObjId>, AutomergeError> {
        let obj = tr.put_object(obj, prop, ObjType::Map)?;
        self.0.put(&obj, "x".into(), tr)?;
        self.1.put(&obj, "y".into(), tr)?;
        self.2.put(&obj, "z".into(), tr)?;
        Ok(Some(obj))
    }
}

pub fn fake_main() -> Result<()> {
    let mut doc = Automerge::new();
    let mut value = Value::new(Point(1.0, 2.0, 3.0), ROOT, "float");

    let mut tr = doc.transaction();
    *value.get_mut(&mut tr) = Point(4.0, 5.0, 6.0);
    //value.0.get_mut(&mut tr) = 7.0;
    let hash = tr.commit();

    for change in doc.get_changes(&[]).unwrap() {
        println!(
            "{}: {:?}",
            change.message().unwrap(),
            doc.get_at(ROOT, "float", &[change.hash])
        );
    }

    Ok(())
}
/*
pub enum Value {
    F64(f64, ObjId, Prop),
}

pub struct ValueGuard<'v, 'd, 't> {
    value: &'v mut Value,
    transaction: &'t mut Transaction<'d>,
}

impl Value {
    pub fn from_f64<P: Into<Prop>>(value: f64, obj: ObjId, prop: P) -> Self {
        Self::F64(value, obj, prop.into())
    }
    pub fn get_mut<'v, 'd, 't>(
        &'v mut self,
        tr: &'t mut Transaction<'d>,
    ) -> ValueGuard<'v, 'd, 't> {
        ValueGuard::new(self, tr)
    }
    pub(crate) fn put(&self, tr: &mut Transaction<'_>) -> Result<(), AutomergeError> {
        match self {
            Self::F64(value, obj, prop) => tr.put(obj, prop.clone(), *value),
        }
    }
}

impl<'v, 'd, 't> ValueGuard<'v, 'd, 't> {
    pub(crate) fn new(value: &'v mut Value, transaction: &'t mut Transaction<'d>) -> Self {
        Self { value, transaction }
    }
}

impl<'v, 'd, 't> Deref for ValueGuard<'v, 'd, 't> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        match self.value {
            Value::F64(v, _, _) => v,
        }
    }
}
impl<'v, 'd, 't> DerefMut for ValueGuard<'v, 'd, 't> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.value {
            Value::F64(v, _, _) => v,
        }
    }
}

impl<'v, 'd, 't> Drop for ValueGuard<'v, 'd, 't> {
    fn drop(&mut self) {
        self.value.put(self.transaction).unwrap();
    }
}

pub fn fake_main() -> Result<()> {
    let mut doc = Automerge::new();
    let mut value = Value::from_f64(1.0, ROOT, "float");

    doc.transact_with::<_, _, AutomergeError, _, ()>(
        |_| {
            automerge::transaction::CommitOptions::default()
                .with_message("Add initial value".to_owned())
        },
        |tx| {
            let initial_value: f64 = 0.0;
            tx.put(ROOT, "float", initial_value)?;
            Ok(())
        },
    )
    .map_err(|e| e.error)?;


    Ok(())
}
*/
