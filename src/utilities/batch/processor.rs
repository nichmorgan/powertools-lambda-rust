use futures::{future, Future};
use std::any::Any;

use crate::utilities::{dto::sqs_event::SQSEvent, typing::lambda_context::LambdaContext};
use async_trait::async_trait;

use super::{
    exceptions::{BatchProcessingError, ExceptionInfo},
    types::{PartialItemFailureResponse, RawEvent},
};

enum EventType {
    SQS,
}

enum BatchEventTypes<T> {
    SQSEvent(SQSEvent<T>),
}

type SuccessResponse<T> = (String, Box<dyn Any>, BatchEventTypes<T>);
type FailureResponse<T> = (String, String, BatchEventTypes<T>);

trait ContextManager {
    fn prepare_context_manager(&mut self);
    fn clean_context_manager(&self) -> Result<(), BatchProcessingError>;
}

#[async_trait]
trait BatchProcessor<T> {
    async fn async_process_record(&self, record: &RawEvent) -> T;
    fn process_record(&self, record: &RawEvent) -> T;

    fn process(&self, records: Vec<RawEvent>) -> Vec<T> {
        records
            .iter()
            .map(|item| self.process_record(item))
            .collect()
    }

    async fn async_process(&self, records: Vec<RawEvent>) -> Vec<T>
    where
        T: Future + Send + 'static,
    {
        future::join_all(records.iter().map(|item| self.async_process_record(item))).await
    }

    fn success_handler(
        &self,
        record: &RawEvent,
        result: Box<dyn Any>,
    ) -> SuccessResponse<EventType>;

    fn failure_handler(
        &self,
        record: &RawEvent,
        exception: ExceptionInfo,
    ) -> FailureResponse<EventType>;
}

struct BasePartialProcessor<T> {
    success_messages: Vec<T>,
    fail_messages: Vec<T>,
    exceptions: Vec<ExceptionInfo>,
    lambda_context: Option<LambdaContext>,
    batch_response: Option<PartialItemFailureResponse>,
    records: Vec<T>,
}

impl<T> BasePartialProcessor<T> {
    fn new() -> BasePartialProcessor<T> {
        BasePartialProcessor {
            success_messages: vec![],
            fail_messages: vec![],
            exceptions: vec![],
            lambda_context: None,
            batch_response: None,
            records: vec![],
        }
    }

    fn response(&self) -> &Option<PartialItemFailureResponse> {
        &self.batch_response
    }

    fn has_messages_to_report(&self) -> bool {
        if self.fail_messages.len() > 0 {
            return true;
        }

        log::debug!(
            "All {} records successfully processed",
            self.success_messages.len()
        );

        return false;
    }

    fn entire_batch_failed(&self) -> bool {
        self.exceptions.len() == self.records.len()
    }
}

impl<T> ContextManager for BasePartialProcessor<T> {
    fn prepare_context_manager(&mut self) {
        self.success_messages.clear();
        self.fail_messages.clear();
        self.exceptions.clear();
        self.batch_response = None;
    }

    fn clean_context_manager(&self) -> Result<(), BatchProcessingError> {
        if !self.has_messages_to_report() {
            return Ok(());
        }

        if self.entire_batch_failed() {
            return Err(BatchProcessingError::new(
                format!(
                    "All records failed processing. {:?} individual errors logged",
                    self.exceptions.len()
                ),
                &self.exceptions,
            ));
        }

        return Ok(());
    }
}
