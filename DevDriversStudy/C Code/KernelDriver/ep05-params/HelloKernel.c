#include <linux/init.h>
#include <linux/module.h>
#include <linux/moduleparam.h>

int count = 1;

module_param(count, int, 0644); //module param def

int HelloKernel_init(void){
	int index = 0;
	printk(KERN_ALERT "This function: %s in this file %s \n", __FUNCTION__, __FILE__);
	for(index = 0; index < count; index++){
		printk(KERN_ALERT "Hi, my index is: %d\n", index );
	}

return 0;
}

void HelloKernel_exit(void){
	int index = 0;
	printk(KERN_ALERT "This function: %s in this file %s \n", __FUNCTION__, __FILE__);
	for(index = 0; index < count; index++){
		printk(KERN_ALERT "Goodbye: %d", index);
	}
}

module_init(HelloKernel_init);
module_exit(HelloKernel_exit);
