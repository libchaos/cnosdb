use std::sync::Arc;

use crate::execution::ddl::DDLExecution;
use crate::metadata::MetaDataRef;
use datafusion::scheduler::Scheduler;
use spi::query::{
    execution::{QueryExecution, QueryExecutionFactory, QueryStateMachineRef},
    logical_planner::Plan,
    optimizer::Optimizer,
};

use super::query::SqlQueryExecution;

pub struct SqlQueryExecutionFactory {
    catalog: MetaDataRef,
    optimizer: Arc<dyn Optimizer + Send + Sync>,
    // TODO 需要封装 scheduler
    scheduler: Arc<Scheduler>,
}

impl SqlQueryExecutionFactory {
    #[inline(always)]
    pub fn new(
        catalog: MetaDataRef,
        optimizer: Arc<dyn Optimizer + Send + Sync>,
        scheduler: Arc<Scheduler>,
    ) -> Self {
        Self {
            catalog,
            optimizer,
            scheduler,
        }
    }
}

impl QueryExecutionFactory for SqlQueryExecutionFactory {
    fn create_query_execution(
        &self,
        plan: Plan,
        state_machine: QueryStateMachineRef,
    ) -> Box<dyn QueryExecution> {
        match plan {
            Plan::Query(query_plan) => Box::new(SqlQueryExecution::new(
                state_machine,
                query_plan,
                self.optimizer.clone(),
                self.scheduler.clone(),
            )),
            Plan::DDL(ddl_plan) => Box::new(DDLExecution::new(
                state_machine,
                ddl_plan,
                self.catalog.clone(),
            )),
        }
    }
}
