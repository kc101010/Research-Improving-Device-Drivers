#include <linux/init.h>
#include <linux/module.h>
#include <linux/fs.h>

int chdev_open(struct inode *pinode, struct file *pfile){
	printk(KERN_ALERT "Now inside %s function\n", __FUNCTION__);
	return 0;
}

ssize_t chdev_read(struct file *pfile, char __user *buffer, size_t length, loff_t *offset){
	printk(KERN_ALERT "Now inside %s function\n", __FUNCTION__);
	return 0;
}

ssize_t chdev_write(struct file *pfile, const char __user *buffer, size_t length, loff_t *offset){
	printk(KERN_ALERT "Now inside %s function\n", __FUNCTION__);
	return length;
}

int chdev_close(struct inode *pinode, struct file *pfile){
	printk(KERN_ALERT "Now inside %s function\n", __FUNCTION__);
	return 0;
}

struct file_operations ch_drv_operations = {
	.owner = THIS_MODULE,
	.open = chdev_open,
	.read = chdev_read,
	.write = chdev_write,
	.release = chdev_close,
};


int ch_drv_init(void){

	printk(KERN_ALERT "Hello");
	register_chrdev(240, "Simple Char Drv", &ch_drv_operations);

	return 0;
}

void ch_drv_exit(void){
	unregister_chrdev(240, "Simple Char Drv");
	printk(KERN_ALERT "Bye");
}

module_init(ch_drv_init);
module_exit(ch_drv_exit);
