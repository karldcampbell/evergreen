use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Holding {
    name: String,
    qty: f32,
    price: f32,
    target: f32
}

impl Holding {
    fn value(&self) -> f32 {
        self.qty * self.price
    }

    fn perc(&self, p_total: f32) -> f32 {
        self.value() / p_total
    }

    fn drift(&self, p_total: f32) -> f32 {
        self.perc(p_total) - self.target
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: evergreen <portfolio file> <value to add>");
        std::process::exit(1);
    }


    let portfolio = &args[1];
    let deposit = args[2].parse().unwrap();

    println!("reading file: {}", portfolio);
    let portfolio : Vec<Holding> = fs::read_to_string(portfolio).unwrap().trim().lines()
        .map(|l| l.trim().split(',').collect())
        .map(|l: Vec<&str> | Holding{
            name: l[0].trim().to_string(),
            target: l[1].trim().parse().unwrap(),
            qty: l[2].trim().parse().unwrap(),
            price: l[3].trim().parse().unwrap()
        }).collect();

    calculate_trades(&portfolio, deposit);
}

fn calculate_trades(old_port: &Vec<Holding>, deposit: f32) {
    let mut port = (*old_port).clone();

    let mut remaining = deposit.clone();

    loop {
        let next_to_buy = get_next_to_buy(&mut port);
        let can_buy_next = remaining >= next_to_buy.price;
        if can_buy_next {
            next_to_buy.qty += 1.0;
            remaining -= next_to_buy.price;
        } else {
            break;
        }
    }

    let p_total = port.iter()
        .map(|v| v.value())
        .fold(0.0, |acc, v| acc + v);
    
    println!("=== new portfolio ===");
    for v in port.iter() {
        println!("{:6} {:4} @ {:5} -- ${:8.2} {:.2}/{}", 
                 v.name, v.qty, v.price, v.value(), v.perc(p_total), v.target);
    }

    println!("\n=== trades ===");
    for (old, new) in old_port.iter().zip(port.iter()) {
        assert!(old.name == new.name);
        if new.qty > old.qty {
            println!("buy {} {}", new.qty - old.qty, old.name);
        }
    }
    println!("remaining: ${:.2}", remaining);
}

fn rebalance(

fn get_next_to_buy(port: &mut Vec<Holding>) -> &mut Holding {
    let p_total = port.iter()
        .map(|v| v.value())
        .fold(0.0, |acc, v| acc + v);

    port.iter_mut().min_by(
        |a, b| a.drift(p_total).partial_cmp(&b.drift(p_total)).unwrap_or(Ordering::Equal)
    ).unwrap()
}
