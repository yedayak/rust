#![feature(negative_bounds, associated_type_bounds)]

trait Trait {
    type Assoc;
}

fn test<T: !Trait<Assoc = i32>>() {}
//~^ ERROR associated type constraints not allowed on negative bounds

fn test2<T>() where T: !Trait<Assoc = i32> {}
//~^ ERROR associated type constraints not allowed on negative bounds

fn test3<T: !Trait<Assoc: Send>>() {}
//~^ ERROR associated type constraints not allowed on negative bounds

fn test4<T>() where T: !Trait<Assoc: Send> {}
//~^ ERROR associated type constraints not allowed on negative bounds

fn main() {}
