use good_lp::*;
use std::fs;
use std::io;

fn eror(file: &str, kode: &str, line: &str) -> String {
    let buffer = format!("Terjadi kesalahan, mohon periksa kembali file {}.txt\nkode eror: {}\n{:?}", file, kode, line);
    return buffer;
}

struct Var {
    value: Vec<Variable>,
    name: Vec<String>,
    cost: Vec<f64>
}

impl Var {
    fn init(problem_variable: &mut ProblemVariables) -> Var {
        let file = fs::read_to_string("./fungsi_tujuan.txt").expect(&eror("fungsi_tujuan", "1", "fungsi_tujuan.txt"));
        let mut name_buffer: Vec<String> = Vec::new();
        let mut cost_buffer: Vec<f64> = Vec::new();
        let mut temp;
        for line in file.lines() {
            temp = line.split_once('*').expect(&eror("fungsi_tujuan", "2", line));
            cost_buffer.push(temp.0.trim().parse().expect(&eror("fungsi_kendala", "3", line)));
            name_buffer.push(String::from(temp.1));
        };
        return Var {
            value: problem_variable.add_vector(variable().integer().min(0), file.lines().count()),
            name: name_buffer,
            cost: cost_buffer
        };
    }
    fn objective(&self) -> Expression {
        let mut buffer = self.cost[0]*self.value[0];
        for i in 1..self.value.len() {
            buffer += self.cost[i]*self.value[i];
        }
        return buffer;
    }
    fn less(&self, string: &str) -> Constraint {
        let split = string.split_once('<').expect(&eror("hambatan", "4", string));
        let lhs: Vec<&str> = split.0.split('+').collect();
        let rhs: f64 = split.1.trim().parse().expect(&eror("hambatan", "5", string));
        let mut koef: f64 = lhs[0].split_once('*').expect(&eror("hambatan", "6", string)).0.trim().parse().expect(&eror("hambatan", "7", string));
        let mut var = self.value[self.name.iter().position(|r| r == &String::from(lhs[0].split_once('*').expect(&eror("hambatan", "8", string)).1.trim())).expect(&eror("hambatan", "9", string))];
        let mut expression = Expression::from(koef*var);
        for i in 1..lhs.len() {
            koef = lhs[i].split_once('*').expect(&eror("hambatan", "6",string)).0.trim().parse().expect(&eror("hambatan", "7",string));
            var = self.value[self.name.iter().position(|r| r == &String::from(lhs[i].split_once('*').expect(&eror("hambatan", "8",string)).1.trim())).expect(&eror("hambatan", "9",string))];
            expression += koef*var;
        }
        return expression.leq(Expression::from(rhs));
    }
    fn equal(&self, string: &str) -> Constraint {
        let split = string.split_once('=').expect(&eror("hambatan", "4", string));
        let lhs: Vec<&str> = split.0.split('+').collect();
        let rhs: f64 = split.1.trim().parse().expect(&eror("hambatan", "5", string));
        let mut koef: f64 = lhs[0].split_once('*').expect(&eror("hambatan", "6", string)).0.trim().parse().expect(&eror("hambatan", "7", string));
        let mut var = self.value[self.name.iter().position(|r| r == &String::from(lhs[0].split_once('*').expect(&eror("hambatan", "8", string)).1.trim())).expect(&eror("hambatan", "9", string))];
        let mut expression = Expression::from(koef*var);
        for i in 1..lhs.len() {
            koef = lhs[i].split_once('*').expect(&eror("hambatan", "6", string)).0.trim().parse().expect(&eror("hambatan", "7", string));
            var = self.value[self.name.iter().position(|r| r == &String::from(lhs[i].split_once('*').expect(&eror("hambatan", "8", string)).1.trim())).expect(&eror("hambatan", "9", string))];
            expression += koef*var;
        }
        return expression.eq(Expression::from(rhs));
    }
    fn greater(&self, string: &str) -> Constraint {
        let split = string.split_once('>').expect(&eror("hambatan", "4", string));
        let lhs: Vec<&str> = split.0.split('+').collect();
        let rhs: f64 = split.1.trim().parse().expect(&eror("hambatan", "5", string));
        let mut koef: f64 = lhs[0].split_once('*').expect(&eror("hambatan", "6", string)).0.trim().parse().expect(&eror("hambatan", "7", string));
        let mut var = self.value[self.name.iter().position(|r| r == &String::from(lhs[0].split_once('*').expect(&eror("hambatan", "8", string)).1.trim())).expect(&eror("hambatan", "9", string))];
        let mut expression = Expression::from(koef*var);
        for i in 1..lhs.len() {
            koef = lhs[i].split_once('*').expect(&eror("hambatan", "6", string)).0.trim().parse().expect(&eror("hambatan", "7", string));
            var = self.value[self.name.iter().position(|r| r == &String::from(lhs[i].split_once('*').expect(&eror("hambatan", "8", string)).1.trim())).expect(&eror("hambatan", "9", string))];
            expression += koef*var;
        }
        return expression.geq(Expression::from(rhs));
    }
    fn constraint_init(&self) -> Vec<Constraint> {
        let mut buffer: Vec<Constraint> = Vec::new();
        let file = fs::read_to_string("./hambatan.txt").expect(&eror("hambatan", "10", "hambatan.txt"));
        for line in file.lines() {
            match line {
                x if x.contains('<') => buffer.push(self.less(line)),
                x if x.contains('=') => buffer.push(self.equal(line)),
                x if x.contains('>') => buffer.push(self.greater(line)),
                _ => panic!("{}", eror("hambatan", "11", line))
            }
        }
        return buffer;
    }
    fn print(&self, solution: &solvers::lpsolve::LpSolveSolution) {
        println!("\n\nHasil perhitungan:");
        for i in 0..self.value.len() {
            println!("{} = {}", self.name[i], solution.value(self.value[i]))
        }
    }
}

fn main() {
    let mut problem_variables = variables!();
    let variable = Var::init(&mut problem_variables);

    println!("\nModel (tanpa nama variabel, hanya untuk memastikan pembacaan data benar):");
    println!("\nFungsi tujuan:\nmax z = {}\n\nTerhadap:", problem_variables.display(&variable.objective()));
    for constraint in variable.constraint_init() {
        println!("{}", problem_variables.display(&constraint))
    }

    let mut model = problem_variables.minimise(variable.objective()).using(lp_solve);

    let constraints = variable.constraint_init();
    for constraint in constraints {
        model = model.with(constraint)
    }
    let solution = model.solve().unwrap();

    variable.print(&solution);
    println!("\nDengan\nz = {}", solution.eval(variable.objective()));
    println!("\nTekan 'Enter' untuk keluar");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit);
}