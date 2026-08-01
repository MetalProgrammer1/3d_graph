use chumsky::prelude::*;

#[derive(Debug)]
pub enum Expr<'src> {
    Num(f32),
    Neg(Box<Expr<'src>>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),
    Var(&'src str),
    Pow(Box<Expr<'src>>, Box<Expr<'src>>),
    Call {
        func: &'src str,
        arg: Box<Expr<'src>>,
    },
}

// pub fn to_parse() {
//     //let src = "0.5*sin(x^y)+cos(x)";

//     match parser().parse(&src).into_result() {
//         Ok(ast) => match eval(&ast, 2.0, 4.0) {
//             output => println!("{}", output),
//         },
//         Err(parse_errs) => parse_errs
//             .into_iter()
//             .for_each(|e| println!("Parse error: {}", e)),
//     }
// }

pub fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    let ident = text::ascii::ident().padded();
    let expr = recursive(|expr| {
        let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));

        let var = ident.map(Expr::Var);
        let call = ident
            .then_ignore(just('('))
            .then(expr.clone())
            .then_ignore(just(')'))
            .map(|(func, arg)| Expr::Call {
                func,
                arg: Box::new(arg),
            });
        let atom = call
            .or(int)
            .or(var)
            .or(expr.clone().delimited_by(just('('), just(')')))
            .padded();
        let op = |c| just(c).padded();

        let unary = op("-")
            .repeated()
            .foldr(atom.clone(), |_op, rhs| Expr::Neg(Box::new(rhs)));

        let pow = unary.clone().foldl(
            op("^")
                .to(Expr::Pow as fn(_, _) -> _)
                .then(unary)
                .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let product = pow.clone().foldl(
            choice((
                op("*").to(Expr::Mul as fn(_, _) -> _),
                op("/").to(Expr::Div as fn(_, _) -> _),
            ))
            .then(pow)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                op("+").to(Expr::Add as fn(_, _) -> _),
                op("-").to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );
        sum
    });
    expr
}

pub fn eval<'src>(expr: &'src Expr<'src>, x: f32, y: f32) -> f32 {
    match expr {
        Expr::Num(x) => *x,
        Expr::Neg(a) => -eval(a, x, y),
        Expr::Add(a, b) => eval(a, x, y) + eval(b, x, y),
        Expr::Sub(a, b) => eval(a, x, y) - eval(b, x, y),
        Expr::Mul(a, b) => eval(a, x, y) * eval(b, x, y),
        Expr::Div(a, b) => eval(a, x, y) / eval(b, x, y),
        Expr::Var("x") => x,
        Expr::Var("y") => y,
        Expr::Pow(a, b) => eval(a, x, y).powf(eval(b, x, y)),
        Expr::Call { func, arg } => {
            let v = eval(arg, x, y);
            match *func {
                "sin" => v.sin(),
                "cos" => v.cos(),
                "ln" => v.ln(),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
