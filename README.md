# An X protocol client
For making a window on unix systems.

## Why?
After doing a bit of research on windowing I learned that there is a server process that sits on all unix machines that you can communicate with in order to open,manipulate,and listen to events.  For mostly for educational purposes I decided to talk to this server using my own code/API without using libX11. I followed the spec outlined here: https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html 