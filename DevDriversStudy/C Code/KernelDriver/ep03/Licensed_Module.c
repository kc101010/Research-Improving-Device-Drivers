#include <linux/init.h>
#include <linux/module.h>

MODULE_LICENSE("GPL");

//__init = module function can be removed as it is not necessary

__init int mod_init(void){
	printk(KERN_ALERT "Inside Licensed Kernel Module %s ",__FILE__);
	return 0;
}

void mod_exit(void){
	printk(KERN_ALERT "Exiting Licensed Kernel Module");
}

module_init(mod_init);
module_exit(mod_exit);
