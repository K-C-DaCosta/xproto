# An X protocol client
For making a window on unix systems.

## Why?
After doing a bit of research on windowing I learned that there is a server process that sits on all unix machines that you can communicate with in order to open a window,manipulate it,and listen to events on that window. So,for mostly for educational purposes, I decided to talk to this server using my own code/API without using libX11. 

## Resources
- I followed the spec outlined here: https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html 
- I figured out where the unix socket was located here: https://www.x.org/releases/X11R7.7/doc/man/man1/Xserver.1.xhtml
    - I tried to connect to it via TCP socket, but AFAIK, that is actually disabled by default and you must enable it in config files somewhere 


## How to run an example
There's currenly only one example.
clone the repo then do:
```
cargo run --example=create_window
```