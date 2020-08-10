extern crate libc;

use std::io;
use std::io::Error;
use std::os::unix::io::RawFd;

/// Describes an epoll event
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))]
#[derive(Clone, Copy, Debug)]
pub struct EpollEvent {
    ///  a bit mask composed by ORing together zero or more event types
    pub events: i32,
    /// user data variable
    pub data: u64,
}

/// Create an epoll instance
///
/// # Arguments
///
/// `flags`:  flags to pass to `libc::epoll_create1`. Set flags to 0 for behavior equivalent to
/// `epoll_create()`. Include `libc::EPOLL_CLOEXEC` in flags to set close-on-exec on the new file
/// descriptor.
///
/// # Return Value
///
/// Returns file descriptor for the created epoll instance
///
/// # Notes
///
/// See https://man7.org/linux/man-pages/man2/epoll_create1.2.html for complete documentation of the
/// underlying C function that is called.
///
pub fn epoll_create1(flags: i32) -> io::Result<RawFd> {
    // On success, returns a file descriptor (a nonnegative integer).  On error, -1 is returned,
    // and errno is set to indicate the error
    unsafe {
        let result = libc::epoll_create1(flags);
        if result < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(result)
        }
    }
}

/// Add, remove, or change entries in the interests list of the epoll instance
///
/// # Arguments
///
/// `epfd`: file descriptor to the epoll instance.
///
/// `op`: operation to be performed on the target file descriptor. Valid values include
/// `libc::EPOLL_CTL_ADD`, libc::EPOLL_CTL_DEL`, and `libc::EPOLL_CTL_MOD`.
///
/// `fd`: target file descriptor.
///
/// `event`: epoll event argument to describe the object linked to the target file descriptor
///
/// # Notes
///
/// See https://man7.org/linux/man-pages/man2/epoll_ctl.2.html for complete documentation of the
/// underlying C function that is called.
pub fn epoll_ctl(epfd: RawFd, op: i32, fd: RawFd, mut event: EpollEvent) -> io::Result<()> {
    // cast event to mut pointer to libc::epoll_event
    let e = &mut event as *mut _ as *mut libc::epoll_event;

    // When successful, epoll_ctl() returns zero.  When an error occurs,
    // epoll_ctl() returns -1 and errno is set appropriately.
    unsafe {
        let result = libc::epoll_ctl(epfd, op as i32, fd, e);
        if result < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Wait for I/O events on the epoll instance
///
/// # Arguments
///
/// `epfd`: file descriptor to the epoll instance.
///
/// `events`: used to return information from the ready list about file descriptors in the interest
/// list that have some events available
///
/// `max_events`: maximum number of events
///
/// `timeout`: number of milliseconds that `epoll_wait()` will block. Set to 0 to return
/// immediately, even if no events are available. Set to -1 to block indefinitely until an event
/// occurs.
///
/// # Return Value
///
/// returns the number of file descriptors ready for the requested I/O, or zero if no file
/// descriptor became ready during the requested timeout milliseconds.
///
/// # Notes
///
/// See https://man7.org/linux/man-pages/man2/epoll_wait.2.html for complete documentation of the
/// underlying C function that is called.
pub fn epoll_wait(
    epfd: RawFd,
    events: &mut [EpollEvent],
    max_events: i32,
    timeout: i32,
) -> io::Result<usize> {
    // When successful, epoll_wait() returns the number of file descriptors ready for the requested
    // I/O, or zero if no file descriptor became ready during the requested timeout milliseconds.
    // When an error occurs, epoll_wait() returns -1 and errno is set appropriately.
    unsafe {
        let result = libc::epoll_wait(
            epfd,
            events.as_mut_ptr() as *mut libc::epoll_event,
            max_events,
            timeout,
        );
        if result < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(result as usize)
        }
    }
}

/// Close a file descriptor
///
/// # Arguments
///
/// `fd`: file descriptor to close
///
/// # Notes
/// See https://man7.org/linux/man-pages/man2/close.2.html for complete documentation of the
/// underlying C function that is called.
pub fn close(fd: RawFd) -> io::Result<()> {
    unsafe {
        let result = libc::close(fd);
        if result < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
