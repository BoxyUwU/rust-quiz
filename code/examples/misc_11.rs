#![feature(try_as_dyn)]
#![allow(warnings)]

fn main() {
    let [a, b, c, d, e, f, g, h, i] = std::array::from_fn(|_| static_mut_ref());
    
    print_closure_info("a", move || { 
        let _a = a; 
    });
    print_closure_info("b", move || { 
        match b { _b => {} }
    });
    print_closure_info("c", move || { 
        match c { _ => {} } 
    });
    print_closure_info("d", move || { 
        match d { &mut _d => {} } 
    });
    print_closure_info("e", move || { 
        let _e: &mut _ = e; 
    });
    print_closure_info("f", move || { 
        f == &mut 0; 
    });
    print_closure_info("g", move || { 
        PartialEq::eq(g, &mut 0); 
    });
    print_closure_info("h", move || { 
        println!("{:?}", h); 
    });
    print_closure_info("i", move || { 
        dbg!(i); 
    });
}

fn static_mut_ref() -> &'static mut u8 {
    Box::leak(Box::new(0))
}

/// Uses `try_as_dyn`, a nightly feature, just as a method to query the closure kind.
fn print_closure_info(name: &str, a: impl Sized + 'static) {
    if std::any::try_as_dyn::<_, dyn Fn()>(&a).is_some() {
        println!("{name} fn!");
    } else if std::any::try_as_dyn::<_, dyn FnMut()>(&a).is_some() {
        println!("{name} mut!");
    } else if std::any::try_as_dyn::<_, dyn FnOnce()>(&a).is_some() {
        println!("{name} once!");
    } else {
        println!("{name} ???");
    }
}


