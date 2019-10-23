use libc;

pub enum Fork {
    Parent(libc::pid_t),
    Child,
}

// Upon successful completion, fork() returns a value of 0 to the child process
// and returns the process ID of the child process to the parent process.
// Otherwise, a value of -1 is returned to the parent process, no child process
// is created.
pub fn fork() -> Result<Fork, libc::pid_t> {
    let res = unsafe { libc::fork() };
    match res {
        -1 => Err(res),
        0 => Ok(Fork::Child),
        res => Ok(Fork::Parent(res)),
    }
}

// Upon successful completion, the setsid() system call returns the value of the
// process group ID of the new process group, which is the same as the process ID
// of the calling process. If an error occurs, setsid() returns -1
pub fn setsid() -> Result<libc::pid_t, libc::pid_t> {
    let res = unsafe { libc::setsid() };
    match res {
        -1 => Err(res),
        res => Ok(res),
    }
}

/*
 * The parent forks the child
 * The parent exits
 * The child calls setsid() to start a new session with no controlling terminals
 * The child forks a grandchild
 * The child exits
 * The grandchild is now the daemon
 * ps -axo ppid,pid,pgid,sess,tty,tpgid,stat,uid,user,command | egrep "fork|sleep|PID"
 */
pub fn daemon() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fork() {
        match fork() {
            Ok(Fork::Parent(child)) => assert!(child > 0),
            Ok(Fork::Child) => println!("child process"),
            Err(_) => assert!(false),
        }
    }
}
