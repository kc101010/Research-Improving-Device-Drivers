#include <linux/init.h>
#include <linux/module.h>

int MyInit_init(void){
	printk(KERN_ALERT "Start of multifile Driver");
	return 0;
}

module_init(MyInit_init);
