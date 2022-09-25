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
    let term = Term::If(Box::new(Term::True), Box::new(Term::Succ(Box::new(Term::Zero))), Box::new(Term::False));
    println!("{:?}", large_step_eval(term));
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

fn large_step_eval(term: Term) -> Term {
    match term {
        Term::True => Term::True,
        Term::False => Term::False,
        Term::Zero => Term::Zero,
        Term::Succ(t) => large_step_eval(*t),
        Term::Pred(t) => {
            match large_step_eval(*t) {
                Term::Zero => Term::Zero,
                Term::Succ(t_) => large_step_eval(*t_),
                _ => Term::NoRuleApplies
            }
        },
        Term::IsZero(t) => {
            match *t {
                Term::Succ(_) => Term::False,
                Term::Zero => Term::True,
                _ => Term::NoRuleApplies
            }
        }
        Term::If(t1, t2, t3) => {
            match large_step_eval(*t1) {
                Term::True => large_step_eval(*t2),
                Term::False => large_step_eval(*t3),
                _ => Term::NoRuleApplies
            }
        },
        Term::NoRuleApplies => Term::NoRuleApplies
    }
}

