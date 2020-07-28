use queue::Queue;
use std::thread;
use std::time::Duration;

struct Drone
{
    sp:                 serialport,
    pub mut q:              Queue,
    pub mut dataArray:      vec,

}

