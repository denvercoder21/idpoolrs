use idpoolrs::id_pool::{IdPool, st::IdPoolSt};

fn main() {
    let mut pool = IdPoolSt::new();

    let a = pool.next().unwrap();
    let b = pool.next().unwrap();

    println!("id: {}", &a);
    println!("id: {}", &b);
}
