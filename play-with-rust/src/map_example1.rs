
pub fn find_total_price_of_each_dep(){
    let csv_report=[
        "Name,Dep, Salary",
        "A,IT,1000.00",
        "B,IT,2000.00",
        "C,ADMIN,1000.00",
    ];
    // find the total salary of each department
    // note: department not known in advance
    use std::collections::HashMap;
    let mut department_salary: HashMap<&str, f64> = HashMap::new();
    for(i, line) in csv_report.iter().enumerate(){
        if i==0{continue;}
        let fields: Vec<&str> = line.split(",").collect();
        let dep = fields[1];
        let salary: f64 = fields[2].parse().unwrap();
        let total = department_salary.entry(dep).or_insert(0.0);
        *total += salary;
    }
    println!("{:?}", department_salary);
}