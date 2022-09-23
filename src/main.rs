#![recursion_limit = "256"]
#[derive(Debug, PartialEq, Eq)]
enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
    NoRuleApplies
}

fn main() {
    let term = Term::If(Box::new(Term::True), Box::new(Term::Zero), Box::new(Term::False));
    println!("{:?}", eval1(term));
}

fn isnumericval(term: &Term) -> bool {
    match &*term {
        Term::Zero => true,
        Term::Succ(t) => isnumericval(&*t),
        _ => false
    }
}

fn isval(term: Term) -> bool {
    match term {
        Term::True => true,
        Term::False => true,
        _ => isnumericval(&term)
    }
}

fn eval1(term: Term) -> Term {
    match term {
        Term::If(t1, t2, _) if *t1 == Term::True => *t2,
        Term::If(t1, _, t3) if *t1 == Term::False => *t3,
        Term::If(t1, t2, t3) => Term::If(Box::new(eval1(*t1)), t2, t3),
        Term::Succ(t) => Term::Succ(Box::new(eval1(*t))),
        Term::Pred(t) => {
            match *t {
                Term::Zero => Term::Zero,
                Term::Succ(nv) if isnumericval(&*nv) => *nv,
                _ => eval1(*t)
            }
        },
        Term::IsZero(t) => {
            match *t {
                Term::Zero => Term::True,
                Term::Succ(nv) if isnumericval(&*nv) => *nv,
                _ => Term::IsZero(Box::new(eval1(*t)))
            }
        },
        _ => Term::NoRuleApplies
    }
}

