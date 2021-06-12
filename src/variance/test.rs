use super::*;

// &T is covariant over T, since you can only
// look at the value behind a shared reference.
// since you can't modify the value, there is no way
// you could break invariances and make the type
// behave incorrectly.
#[test]
fn love_cat_and_dog() {
    let cat = MyCat();
    assert_eq!(<MyCat as Animal>::snuggle(&cat), love(&cat));

    let dog = MyDog();
    assert_eq!(<MyDog as Animal>::snuggle(&dog), love(&dog));
}

// &mut T is invariant over T, which means that
// even though Cat and Dog are subtypes of Animal,
// &mut Cat and &mut Dog won't be be subtypes of &mut Animal.
// &mut T must be invariant over T or else we would be able
// to modify the value of T to some other type and break the
// type invariances and behave incorrectly.
//
// #[test]
// fn test_evil_feeder() {
//     let mut cat = MyCat;
//     evil_feeder(&mut cat);
// }

#[test]
fn fn_pointer_output_covariance() {
    let cat = MyCat();
    assert_eq!(cat.snuggle(), get_animal_but_actually_cat().snuggle());

    let dog = MyDog();
    assert_eq!(dog.snuggle(), get_animal_but_actually_dog().snuggle());
}

#[test]
fn fn_pointer_input_contravariance() {
    let cat = MyCat();
    let dog = MyDog();

    assert_eq!(dog.snuggle(), handle_animal(dog));
    assert_eq!(cat.snuggle(), handle_animal(cat));
}

#[test]
fn owned_values_covariance() {
    let cat = Box::new(MyCat());
    let dog = Box::new(MyDog());

    let mut pet: Box<dyn Animal>;

    pet = cat;
    assert_eq!("mycat snuggle", pet.snuggle());

    pet = dog;
    assert_eq!("mydog snuggle", pet.snuggle());
}
