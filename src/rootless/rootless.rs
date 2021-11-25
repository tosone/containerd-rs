use nix::unistd;

pub const CONTAINERD_SOCKET: &str = "/run/containerd/containerd.sock";

pub fn is_rootless() -> bool {
    unistd::getuid().as_raw() == 0
}
