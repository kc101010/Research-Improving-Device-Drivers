/*
    Written by Kyle Christie
    From https://deepu.tech/memory-management-in-rust/
*/

struct Employee <'a>{
    name: &'a str,  //'a sets lifetime of struct, ref of name must outlive Employee struct
    salary: i32,    //setting lifetime is required within Rust code, the comma (') is known as the lifetime specifier
    sales: i32,
    bonus: i32,
}

const BONUS_PERCENTAGE: i32 = 10;

fn get_bonus_percentage(salary: &i32) -> i32{
    let percentage = (salary * BONUS_PERCENTAGE) / 100;
    return percentage;
}

fn find_employee_bonus(salary: &i32, no_of_sales: i32) -> i32{
    let bonus_perecentage = get_bonus_percentage(salary);
    let bonus = bonus_perecentage * no_of_sales;
    return bonus;
}

fn main(){
    let mut john = Employee{
        name: &format!("{}", "John"), //tried using dynamic keyword but it didn't work - look into this.
        salary: 5000,
        sales: 5,
        bonus: 0,
    };

    john.bonus = find_employee_bonus(&john.salary, john.sales);
    println!("Bonus for {} is {}", john.name, john.bonus);

}