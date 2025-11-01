//! Transaction support

use crate::connection::Connection;
use crate::error::Result;

/// Transaction isolation level
#[derive(Debug, Clone, Copy)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
    Snapshot,
}

/// SQL Server transaction
pub struct Transaction<'conn> {
    _conn: &'conn mut Connection,
    committed: bool,
}

impl<'conn> Transaction<'conn> {
    /// Begin a new transaction
    pub async fn begin(conn: &'conn mut Connection) -> Result<Self> {
        // TODO: Execute BEGIN TRANSACTION
        log::warn!("Transaction::begin() not yet implemented");
        Ok(Self {
            _conn: conn,
            committed: false,
        })
    }

    /// Commit the transaction
    pub async fn commit(mut self) -> Result<()> {
        // TODO: Execute COMMIT TRANSACTION
        self.committed = true;
        log::warn!("Transaction::commit() not yet implemented");
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(mut self) -> Result<()> {
        // TODO: Execute ROLLBACK TRANSACTION
        self.committed = true;
        log::warn!("Transaction::rollback() not yet implemented");
        Ok(())
    }
}

impl<'conn> Drop for Transaction<'conn> {
    fn drop(&mut self) {
        if !self.committed {
            log::warn!("Transaction dropped without commit/rollback - will auto-rollback");
            // TODO: Auto-rollback on drop
        }
    }
}
