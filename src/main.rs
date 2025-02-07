use rand::Rng;
use std::collections::HashSet;

fn main() { 
    let thing = stuff();
    let message: u64 = 2_u64.pow(thing.e as u32) % thing.n;
    println!("thing={:?}\nEn_message={}", thing, message);
    let de = message.pow(thing.d as u32) % thing.n;
    println!("decrypted={}", de);
}

#[derive(Debug)]
struct privateKey {
    factors: Vec<u64>,
    totient: u64,
    d: u64,
    e: u64,
    n: u64
}

#[derive(Debug)]
struct publicKey {
    power: Vec<u64>,
    encrypt_key: u64,
}

//1 >< n with no common factors
fn totient(p: u64, q: u64) -> u64 {
    (p - 1) * (q - 1)
}

// 1 < e < t(n) coprime != t(n), n
fn e_key(totient: u64, n: u64) -> u64{
    let mut temp_e = totient - 1;
    let mut possible_e = Vec::new();
    let mut t_divisors: HashSet<u64> = HashSet::new();
    divisors(totient, &mut t_divisors);
    divisors(n, &mut t_divisors);
        //println!("temp_e={} divisors={:?} orig_num{}", temp_e, t_divisors, totient);
    for num in 2..=temp_e {
        if gcd(&num, &t_divisors) {
            possible_e.push(num);
        }
    }
    //panic!("possible e's={:?}\nt_divs={:?}", possible_e, t_divisors);

    let idx = rand::thread_rng().gen_range(0..possible_e.len());
    possible_e[0]
}

fn gcd(number: &u64, t_divisors: &HashSet<u64>) -> bool {
    for val in t_divisors.iter() {
        if number % val == 0 {
            return false;
        }
    }
    true
}

fn divisors(val: u64, divs: &mut HashSet<u64>) -> &HashSet<u64> {
    let mut some_num: u64 = val / 2;
    divs.insert(val);
    for number in 2..=some_num {
        if val % number == 0 {
            match divs.get(&number) {
                Some(_) => continue,
                None =>  {
                    divs.insert(number);
                }
            }
        }
    }
    divs
}

fn priv_primes() -> Vec<u64> {
    let mut p1: Option<u64> = generate_prime(7);
    let mut p2: Option<u64> = generate_prime(7);

    while Some(p2) == Some(p1) {
        //println!("{:?} {:?}", p1, p2);
        p2 = generate_prime(7 as u64);
    }
    let p1: u64 = p1.expect("message");
    let p2: u64 = p2.expect("message");
    vec![p1, p2]
}

//de(mod t(n)) = 1
fn d_key(totient: u64, e: u64) -> u64 {
    let mut possible_d = Vec::new();
    for num in e..totient {
        if e*num % totient == 1 {
            println!("pattern={:?}", num);
            possible_d.push(num);
        }
    }
    println!("ppossible_d={:?}", possible_d);
    let idx = rand::thread_rng().gen_range(0..possible_d.len());
    possible_d[0]
}

fn stuff() -> privateKey {
    let v = priv_primes();
    let p = v[0];
    let q = v[1];
    let n = p*q;
    println!("p={} q={} -- n={}", p, q, n);
    let t: u64 = totient(p, q);
    //println!("totient={}", t);
    let e: u64 = e_key(t, n);
    let  d: u64 = d_key(t, e);

    let s = privateKey {
        factors: vec![p, q],
        totient: t,
        d: d,
        e: e,
        n: p * q
    };
    println!("{:?}", s);
    s
    
}

fn generate_prime(limit: u64) -> Option<u64> {
    let start: u64 = 0;
    let mut count: u64 = 0;
    let mut prime_list: Vec<u64> = Vec::new();

    for num in 3..=limit {
        let mut divisor_count: usize= 0;
        for i in 1..=num {
            if i > num {
                break;
            }

            if num % i == 0 {
                divisor_count += 1;
            }
            if divisor_count > 2 {
                break;
            }
        }
        //println!("ad count ={} num={}", divisor_count, num);
        if divisor_count <= 2 {
            count += 1;
            prime_list.push(num);
            //println!("{}", num);
        }
    }
    //println!("prime list -> {:?}", prime_list);
    let idx: usize = rand::thread_rng().gen_range(0..prime_list.len());

    Some(prime_list[idx])
}
