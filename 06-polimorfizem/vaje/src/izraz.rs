use std::ops::*;
use std::fmt::*;

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz <T> {
    Konstanta(T),
    Spremenljivka(String),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

impl<T> Izraz<T> {
    fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Spremenljivka(_) => todo!();
            Izraz::Operacija(
                l,
                _,
                r, ) => l.collect() + r.collect(),
                }
        }
    }
impl<T: Display> Izraz<T> {
    fn izpis(&self) -> String {
            match self {
                Izraz::Konstanta(v) => v.to_string(),
                Izraz::Operacija(
                    l,
                    bin_operacija,
                    r) => {
                        let li = l.izpis();
                        let ri = r.izpis();
                        match bin_operacija {
                            BinOperacija::Plus => format!("({} + {})",li, ri),
                            BinOperacija::Minus => format!("({} - {})",li, ri),
                            BinOperacija::Times => format!("({} * {})",li, ri),
                        }
                    }
            }
        }

}

impl<T> Izraz<T> 
where
        T: add<output = T> + Sub<Output = T> + Mul<Output = T> + Clone
{
    fn konst(v: T) -> Self {
        Izraz::Konstanta(v)
    }
    
    fn spr(ime: &str) -> Self{
        Izraz::Spremenljivka(ime.to.string())
    }

    fn eval(&self) -> T {
        match self {
            Izraz::Konstanta(v) => v.clone(),
            Izraz::Spremenljivka(_) => todo!(),
            Izraz::Operacija(
                l,
                bin_operacija,
                r) => {
                    let lv = l.eval();
                    let rv = r.eval();
                    match bin_operacija {
                        BinOperacija::Plus
                        BinOperacija::Minus
                        BinOperacija::Times
                    }

                }
        }
    }
    

    
}

impl<T: Display> Display for Izraz<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Izraz: {}", self.izpis())
    }
}