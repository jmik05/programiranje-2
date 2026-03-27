struct Aritmeticnozaporedje {
    a0: i64,
    d: i64,
    index: u64
}

impl Aritmeticnozaporedje {
    fn new(a0: i64, d: i64) -> Self{
        Aritmeticnozaporedje {a0, d, index: 0}
    }

    fn next(&mut self) -> i64 {
        self.index += 1;
        self.a0 + (self.index as i64 - 1) * self.d
    }

    fn n_th(&mut self, n: u64) -> i64 {
        self.a0 + (n as i64) * self.d
    }


    fn reset(&mut self) -> () {
        self.index = 0
    }    

    fn current(&self) -> i64 {
        self.a0 + (self.index as i64) * self.d
    }

    fn sum(&mut self, n: u64) -> i64 {
        let tmp_index = self.index;
        let mut vsota = 0;
        for _ in 0..n{
            vsota+= self.next()
        }
        self.index =tmp_index;
        vsota

    }

    fn vsota(&self, other: &Self) -> Self {
        Self::new(self.a0 + other.a0, self.d + other.d)
    }

}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}

/* trenutni clen
vsota prvih nekaj
nasledji clen
prejsnji clen
 */


fn collect(&self) -> u32 {
    match self {
        Izraz::Konstanta(_) => 1,
        Izraz::Operacija(
            l,
            _,
            r, ) => l.collect() + r.collect(),
            }
    }
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


fn main() {
    let mut an= Aritmeticnozaporedje::new (1, 1);
    let mut bn = Aritmeticnozaporedje:: new (5, -4);
    let mut cn = Aritmeticnozaporedje:: new (5, -4);
    let mut cn = an.vsota(&bn);
    println!("({},{}",an.next(), an.next())
}
/*fn main() {
    let Izraz:: Operacija (
        Box::new(Izraz:: Konstanta(1),
        BinOperacija::Plus,
        Box::new(Izraz::Operacija(
           Izraz::Konstanta(2),
            BinOperacija::Times,
            Izraz::Konstanta(3))))   
    );
    }*/
