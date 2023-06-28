pub mod id;
pub mod item;

use std::sync::Mutex;

use id::Pool as IdPool;
use item::Pool as ItemPool;

pub struct Data {
    pub planned: Box<dyn ItemPool>,
    pub finished: Box<dyn ItemPool>,
    pub canceled: Box<dyn ItemPool>,
    pub ids: Box<dyn IdPool>,
}

pub struct Repository {
    inner: Mutex<Data>,
}

impl Repository {
    pub fn new(data: Data) -> Self {
        Self {
            inner: Mutex::new(data),
        }
    }

    pub fn apply_planned<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let planned = &mut data.planned;
        f(planned.as_mut())
    }

    pub fn apply_finished<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let finished = &mut data.finished;
        f(finished.as_mut())
    }

    pub fn apply_canceled<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let canceled = &mut data.canceled;
        f(canceled.as_mut())
    }

    pub fn apply_planned_ids<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool, &mut dyn IdPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let planned = &mut data.planned;
        let ids = &mut data.ids;
        f(planned.as_mut(), ids.as_mut())
    }

    pub fn apply_planned_finished_canceled<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool, &mut dyn ItemPool, &mut dyn ItemPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let planned = &mut data.planned;
        let finished = &mut data.finished;
        let canceled = &mut data.canceled;
        f(planned.as_mut(), finished.as_mut(), canceled.as_mut())
    }

    pub fn apply_planned_finished_ids<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool, &mut dyn ItemPool, &mut dyn IdPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let planned = &mut data.planned;
        let finished = &mut data.finished;
        let ids = &mut data.ids;
        f(planned.as_mut(), finished.as_mut(), ids.as_mut())
    }

    pub fn apply_planned_canceled_ids<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn ItemPool, &mut dyn ItemPool, &mut dyn IdPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let planned = &mut data.planned;
        let canceled = &mut data.canceled;
        let ids = &mut data.ids;
        f(planned.as_mut(), canceled.as_mut(), ids.as_mut())
    }

    pub fn apply_ids<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut dyn IdPool) -> T,
    {
        let data = &mut *self.inner.lock().unwrap();
        let ids = &mut data.ids;
        f(ids.as_mut())
    }
}
