#![feature(entry_insert)]

use bit_set::BitSet;
use nanoid::nanoid;
use num_rational::Rational32 as Ratio;

use std::collections::{ HashSet, HashMap };
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EventPossibilities {
    possibilities: BitSet,
    id: String,
}
impl EventPossibilities {
    fn new(possibilities: BitSet) -> EventPossibilities {
        EventPossibilities { id: nanoid!(), possibilities }
    }
}

/// immutability gang
/// has lifetime 's, which is the lifetime of the situation. every probablitiy will be owned by the
/// situation it occupies.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Probability<'s> {
    UNIT(&'s EventPossibilities),                       // P(X)
    NOT (&'s Probability<'s>),                          // P(!X)
    COND(&'s Probability<'s>, &'s Probability<'s>),     // P(A | B)
    OR  (&'s Probability<'s>, &'s Probability<'s>),     // P(A u B)
    AND (&'s Probability<'s>, &'s Probability<'s>),     // P(A n B)     not MUL: doesn't assume independence
    ADD (&'s Probability<'s>, &'s Probability<'s>),     // P(A) + P(B)  not OR: doesn't assume disjointness
    MUL (&'s Probability<'s>, &'s Probability<'s>),     // P(A) * P(B)  only for algebraic purposes
    SUB (&'s Probability<'s>, &'s Probability<'s>),     // P(A) - P(B)  only for algebraic purposes
    DIV (&'s Probability<'s>, &'s Probability<'s>),     // P(A) / P(B)  only for algebraic purposes
}

#[derive(Debug)]
pub struct Situation<'s> {
    events: HashSet<EventPossibilities>,
    memo: RefCell<HashMap<Probability<'s>, Option<Ratio>>>,
}
impl<'s> Situation<'s> {
    fn generate_expresison_pair(&self, notation: &str,
                                p1: Probability<'s>,
                                p2: Probability<'s>,
                                ) -> &Probability<'s> {
        //let mut ret: Probability;
        let mut idx: usize = 0;
        // stack machine to parse polish notation
        let mut stack: Vec<&Probability>;

        for char in notation.chars() {
            stack.push(match char {
                '1' => &p1,
                '2' => &p2,
                '!' => {
                    let x = stack.pop().expect("RPN empty `!`");
                    self.memo.borrow_mut().entry(Probability::NOT(x)).insert(None).key()
                },
                c => {
                    let x2 = stack.pop().expect("RPN fricked");
                    let x1 = stack.pop().expect("RPN fricked");
                    match c {
                        '|' => self.memo.borrow_mut().entry(Probability::COND(x1, x2)).insert(None).key(),
                        'u' => self.memo.borrow_mut().entry(Probability::OR  (x1, x2)).insert(None).key(),
                        'n' => self.memo.borrow_mut().entry(Probability::AND (x1, x2)).insert(None).key(),
                        '+' => self.memo.borrow_mut().entry(Probability::ADD (x1, x2)).insert(None).key(),
                        '*' => self.memo.borrow_mut().entry(Probability::MUL (x1, x2)).insert(None).key(),
                        '-' => self.memo.borrow_mut().entry(Probability::SUB (x1, x2)).insert(None).key(),
                        '/' => self.memo.borrow_mut().entry(Probability::DIV (x1, x2)).insert(None).key(),
                        c => panic!("Unknown RPN character {}", c)
                    }
                }
            });
        }
        stack.pop().expect("generate_expression_pair RPN invalid")
    }
    fn generate_neighbors(&self, cur: &Probability<'s>) -> Vec<&Probability<'s>> {
        // TODO: so like, this actually doesn't work because i need to :fist_right: :crab:
        match cur {
            &Probability::UNIT(ev) => /* TODO: no clue what the first & on this line does */ {
                self.events.iter().filter_map(|alt| if ev == alt { None } else {
                    //Some(self.generate_expresison_pair("+*|122*|1!2!2",     // polish
                    Some(self.generate_expresison_pair("12|2*12!|2!*+",     // reverse polish
                        Probability::<'s>::UNIT(ev),
                        Probability::<'s>::UNIT(alt),
                   ))


                    //    Probability::<'s>::ADD(
                    //        Probability::<'s>::COND(
                    //            Probability::<'s>::UNIT(ev),
                    //            Probability::<'s>::UNIT(alt),
                    //        ),
                    //
                    //))
                }).collect()
            },
            _ => Vec::new(),
        }
    }
    fn search() {
        // TODO: implement
        // start by adding the goal to the queue
        // repeatedly expand it, and add anything we don't know yet into the queue or smt
    }
}
