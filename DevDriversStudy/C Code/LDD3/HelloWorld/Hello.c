#include <linux/init.h>
#include <linux/module.h>
//macro tells kernel that mod has free license, kernel would complain otherwise
MODULE_LICENSE("Dual BSD/GPL");

//invoke when module is loaded
static int hello_init(void){
	//similar to printf but declared in Kernel
	//Kernel has doesn't have C std lib so must declare necessary base funcs
	printk(KERN_ALERT "Hello, World\n");
	return 0;
}

//invoke when module is removed
static void hello_exit(void){
	printk(KERN_ALERT "Goodbye, cruel world!\n");
}

//macros indicate which function is which
module_init(hello_init);
module_exit(hello_exit);
