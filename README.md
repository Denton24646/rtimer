# rtimer
A simple timer CLI tool that is helpful for day to day development tasks. rtimer is not meant to be super accurate but instead give you a rough estimate of time that can be helpful in certain situations. For example, say you are spinning up a cloud instance and want to see how long it takes to provision. Or you start a preemptible instance and want to get a sense of how long it stays up before being preempted. rtimer is accurate on a per second time interval. 

### Install
rtimer uses the Rust 2018 compiler 

`git clone https://github.com/Denton24646/rtimer`

`cargo build`

### Usage 
`cargo run 10 s true`
