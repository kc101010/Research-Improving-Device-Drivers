#include <linux/init.h>
#include <linux/module.h>

MODULE_LICENSE("GPL");

int Exprt(void);

int HelloKernel_init(void){
	printk(KERN_ALERT "This function: %s in this file %s \n", __FUNCTION__, __FILE__);
	Exprt();
	return 0;
}

void HelloKernel_exit(void){
	printk(KERN_ALERT "This function: %s in this file %s \n", __FUNCTION__, __FILE__);
}

module_init(HelloKernel_init);
module_exit(HelloKernel_exit);
