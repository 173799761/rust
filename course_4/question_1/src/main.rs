struct Cat ;

impl Cat {
    fn catTalk(&self) {
        println!("catTalk:miao~");
    }
}


struct Dog;

impl Dog {
    fn dogTalk(&self) {
        println!("dogTalk:wang!");
    }
}

struct Cattle ;
impl Cattle {
    fn cattleTalk(&self) {
        println!("cattleTalk:mou~");
    }
}

enum EnumWrappedType {
    One(Cat),
    Two(Dog),
    Three(Cattle),
}

fn enumMethodDispatchTest() {
    let vec: Vec<EnumWrappedType> = vec![
        EnumWrappedType::One(Cat),
        EnumWrappedType::Two(Dog),
        EnumWrappedType::Three(Cattle),
    ];

    for enumWrappedType in vec.iter() {
        match enumWrappedType {
            EnumWrappedType::One(enum1) => enum1.catTalk(),
            EnumWrappedType::Two(enum2) => enum2.dogTalk(),
            EnumWrappedType::Three(enum3) => enum3.cattleTalk(),
        }
    }
}

//****************************Trait ***************************************************************** */
trait TraitMethodAnimal {
    fn talk(&self);
}

impl TraitMethodAnimal for Cat {
    fn talk(&self) {
        println!("TraitMethodAnimal cat talk : miao ~~~~~~~~~");
    }
}

impl TraitMethodAnimal for Dog {
    fn talk(&self) {
        println!("TraitMethodAnimal Dog talk : wang ~~~~~~~~~");
    }
}

impl TraitMethodAnimal for Cattle {
    fn talk(&self) {
        println!("TraitMethodAnimal Cattle talk : mou ~~~~~~~~~");
    }
}

fn traitTest() {
    let vec: Vec<Box<dyn TraitMethodAnimal>> = vec![
        Box::new(Cat),
        Box::new(Dog),
        Box::new(Cattle),
    ];

    for item in vec.iter() {
        item.talk();
    }
}

fn main() {
    println!("***************EnumMethodDispatchTest *******************************");
    enumMethodDispatchTest();

    println!("");
    println!("***************traitTest *******************************");
    traitTest();
}
