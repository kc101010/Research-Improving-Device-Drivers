#include <linux/init.h>
#include <linux/module.h>

void MyExit_exit(void){
	printk(KERN_ALERT "Exiting multifile driver");
}

module_exit(MyExit_exit);
