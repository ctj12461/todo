use std::error::Error;
use std::sync::Arc;

use snafu::prelude::*;

use crate::domain::usecase::clean;
use crate::repository::Repository;

#[derive(Debug, Snafu)]
struct NoError;

pub fn run(repo: Arc<Repository>) -> Result<(), Box<dyn Error>> {
    repo.apply_finished(|finished| -> Result<(), NoError> {
        clean::execute(finished);
        Ok(())
    })?;

    repo.apply_canceled(|canceled| -> Result<(), NoError> {
        clean::execute(canceled);
        Ok(())
    })?;

    println!("Clean records of finished & canceled items");
    Ok(())
}
