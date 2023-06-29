use crate::domain::usecase::add;
use crate::domain::usecase::add_id::{self, Request as AddIdRequest};
use crate::repository::id::Pool as IdPool;
use crate::repository::item::Pool as ItemPool;

pub type Request = add::Request;
pub type Response = add::Response;
pub type PlanError = add::AddItemError;

pub fn execute(
    planned: &mut dyn ItemPool,
    ids: &mut dyn IdPool,
    request: Request,
) -> Result<Response, PlanError> {
    let response = add::execute(planned, request)?;
    let _ = add_id::execute(ids, AddIdRequest { id: response.id });
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::domain::entity::Item;
    use crate::repository::id::TriePool;
    use crate::repository::item::MemoryPool;

    use super::*;

    #[test]
    fn it_should_return_an_id_when_creating_item_succeeded() {
        let item = Item::new_test();
        let id = item.id();

        let request = Request {
            summary: item.summary().to_owned(),
            content: item.content().to_owned(),
            deadline: *item.deadline(),
            tags: item.tags().clone(),
            priority: item.priority().value(),
        };

        let mut planned: Box<dyn ItemPool> = Box::new(MemoryPool::new());
        let mut ids: Box<dyn IdPool> = Box::new(TriePool::new());
        let res = execute(planned.as_mut(), ids.as_mut(), request);
        assert_eq!(res, Ok(Response { id }));
        assert!(ids.remove(res.unwrap().id));
    }
}
