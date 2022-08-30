## Necessary equipment for developing and testing a device driver

I think I'll mainly focus on Linux drivers as I have the facilities to do so. I think there is too much overhead for Windows for me to think of using it.

### Essentials
+ Some kind of Raspberry Pi computer for Linux development.
+ If needed, peripherals to use said Pi computer
+ If needed, a smaller/cheap monitor for displaying Pi computer.



[While Reddit isn't the best place](https://www.reddit.com/r/embedded/comments/v1jvwr/linux_device_driver_project_ideas/) I did look at this post for some inspiration in what I could do for a practical device driver project. Here's a list of what's caught my eye
+ Keyboard driver for BBQ20KBD with I2C and USB interface.
+ Driver that works through queue's and interrupts
+ "Learn about debugging your driver through pr_ messages, dev_ messages, logging levels, and a few more."
+ "For my case, I've made drivers for the following
	1.  I2C based keyboards, trackballs, microcontrollers with custom firmware.
	2.  SPI displays. I'm learning DRM for the moment.
	3.  A few UART devices.
	4.  Motor drivers."
+ [Pi and sensors, develop tools, use tools to communicate with app that displays readings?](https://www.reddit.com/r/embedded/comments/v1jvwr/comment/iaok9g9/?utm_source=share&utm_medium=web2x&context=3)

The BBQ20KBD is essentially a blackberry phone keyboard attached to a board with GPIO pins. This could be better than plain writing a driver for a standard keyboard though I could still leave that option open.

I had always though of creating my own electronic device and controlling that through a driver but I don't really think it would be feasible. 

[Another reddit post](https://www.reddit.com/r/embedded/comments/esl2ig/simple_device_to_practice_writing_linux_drivers/) which has a good few comments that I find interesting

[I was led to this site](https://bootlin.com/training/kernel/) which is training for Embedded Linux kernel and driver development training. All their materials are free to access. I COULD EVEN WRITE A DRIVER FOR GAME HARDWARE SUCH AS A NINTENDO WII NUNCHUCK

[Course Materials](https://bootlin.com/doc/training/linux-kernel/)

A lot of the [comments](https://www.reddit.com/r/embeddedlinux/comments/s659rk/comment/ht6ul5m/?utm_source=share&utm_medium=web2x&context=3) I'm seeing say to hook a low-level system component into a higher-level app component. 