use crate::repository::item::Pool;

pub fn execute(pool: &mut dyn Pool) {
    pool.clear();
}
