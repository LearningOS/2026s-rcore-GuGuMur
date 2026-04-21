//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Total accumulated running time
    pub total_time: usize,
    /// Last time this task was scheduled in
    pub last_start_time: usize,
    /// System call counts indexed by syscall number
    pub syscall_count: [usize; MAX_SYSCALL_NUM],
}

/// The status of a task
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Information about a single syscall performed by a task.
pub struct SyscallInfo {
    /// The syscall number.
    pub id: usize,
    /// The number of times this syscall has been invoked.
    pub times: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Snapshot information for a task, suitable for reporting or tracing.
pub struct TaskInfo {
    /// The task identifier.
    pub id: usize,
    /// The current status of the task.
    pub status: TaskStatus,
    /// Per-syscall statistics for this task.
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    /// Total accumulated running time for the task.
    pub time: usize,
}
