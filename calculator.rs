enum Operation
{
    Add(i64, i64),
    Subtract(i64, i64),
    Multiply(i64 , i64),
    Divide(i64 , i64)
}

fn Calculate(Op : Operation) -> f64 {
    match op
    {
        Operation::Add(x,y) => x + y,
        Operation::Subtract(x,y) => x - y,
        Operation::Multiply(x,y) => x * y,
        Operation::Divide(x,y) => x / y,
    }
}

fn main()
{
    let mut op : String = String::new();
    let mut x : String = String::new();
    let mut y : String = String::new();
    
    println("Enter Number 1 : ");
    std::io::stdin().read_line(&mut x).unwrap();

    println("Enter Number 2 : ");
    std::io::stdin().read_line(&mut y).unwrap();

    println("Enter Operation. (+,-,*,/) ");
    std::io::stdin().read_line(&mut x).unwrap();
    
    

    let op : char = op.trim().parse().unwrap();
    let num1 : f64 = x.trim().parse().unwrap();
    let num2 : f64 = y.trim().parse().unwrap();
    
    let operation = match op{
        '+' => Operation::Add(num1,num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1 , num2),
        _ => println("Invalid Operation Entered."),
    };

    let result = Calculate(operation);
    println("{}",result);
}
