#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]

pub mod config;
pub mod handlers;
pub mod state_manager;
pub mod types;

pub use config::AtomaStateManagerConfig;
use sqlx::Postgres;
pub use state_manager::{AtomaState, AtomaStateManager, AtomaStateManagerError};

/// Builds a query with an IN clause and optional additional conditions
///
/// # Arguments
/// * `base_query` - The base SQL query to build upon
/// * `column` - The column name to use in the IN clause
/// * `values` - The array of values to include in the IN clause
/// * `additional_conditions` - Optional additional WHERE conditions to add after the IN clause
///
/// # Returns
/// A QueryBuilder configured with the IN clause and ready for additional bindings
pub(crate) fn build_query_with_in<'a, T: sqlx::Type<Postgres> + sqlx::Encode<'a, Postgres>>(
    base_query: &str,
    column: &str,
    values: &'a [T],
    additional_conditions: Option<&str>,
) -> sqlx::QueryBuilder<'a, Postgres> {
    let mut builder = sqlx::QueryBuilder::new(base_query);

    if values.is_empty() {
        builder.push(" WHERE 1=0");
        return builder;
    }

    builder.push(" WHERE ");
    builder.push(column);
    builder.push(" IN (");

    // Create placeholders for the IN clause
    let mut separated = builder.separated(", ");
    for value in values {
        separated.push_bind(value);
    }
    separated.push_unseparated(")");

    // Add additional conditions if present
    if let Some(conditions) = additional_conditions {
        builder.push(" AND ");
        builder.push(conditions);
    }

    builder
}
