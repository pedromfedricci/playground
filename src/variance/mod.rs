#[cfg(test)]
mod test;

trait Animal {
    fn snuggle(&self) -> &'static str {
        let shout = "animal snuggle";
        println!("{}", shout);
        shout
    }

    fn eat(&mut self) {
        println!("animal eat");
    }
}

trait Cat: Animal {
    fn meow(&self) -> &'static str {
        let shout = "cat meow";
        println!("{}", shout);
        shout
    }
}

trait Dog: Animal {
    fn bark(&self) -> &'static str {
        let shout = "dog bark";
        println!("{}", shout);
        shout
    }
}

#[allow(dead_code)]
fn get_animal_fake_rnd(fake_rand: u16) -> Box<dyn Animal> {
    if fake_rand % 2 == 0 {
        Box::new(MyDog())
    } else {
        Box::new(MyCat())
    }
}

#[allow(dead_code)]
fn get_animal_but_actually_cat() -> impl Animal {
    MyCat()
}

#[allow(dead_code)]
fn get_animal_but_actually_dog() -> impl Animal {
    MyDog()
}

#[allow(dead_code)]
fn handle_animal(animal: impl Animal) -> &'static str {
    animal.snuggle()
}

#[allow(dead_code)]
fn handle_cat(cat: impl Cat) -> &'static str {
    cat.snuggle()
}

#[allow(dead_code)]
fn love(pet: &impl Animal) -> &'static str {
    pet.snuggle()
}

#[allow(dead_code)]
fn evil_feeder(pet: &mut impl Animal) {
    let _spike = MyDog();
    // imagine you could do something like this:
    //
    // *pet = _spike;
    //
    // you would change the reference value to
    // some other subtype of Animal, different from
    // the input (pet) subtype of Animal:
    // you could pass a mutable reference to a Cat,
    // and inside this function you would be able to
    // set the value of a Dog, a complete different
    // type from Cat.
    //
    pet.snuggle();
}

#[derive(Debug)]
struct MyCat();

#[derive(Debug)]
struct MyDog();

impl Animal for MyCat {
    fn snuggle(&self) -> &'static str {
        let shout = "mycat snuggle";
        println!("{}", shout);
        shout
    }
}
impl Cat for MyCat {}

impl Animal for MyDog {
    fn snuggle(&self) -> &'static str {
        let shout = "mydog snuggle";
        println!("{}", shout);
        shout
    }
}
impl Dog for MyDog {}
