



#[deprecated]
pub fn add_system(){

}



pub trait GameSystem{
    fn init(){}
    fn _update(){}
    fn _physics_update(){}
    fn _render_update(){}
    fn _exit(){}
}





#[cfg(test)]
mod tests{
    use std::alloc;

    use hecs::{Entity, World};
    use pyo3::{prelude::*, types::PyDict};


    #[test]
    fn test1()
    {
        Python::attach(|py|{
            py.import("sys");
            let time_module = py.import("time").unwrap();            let locals = PyDict::new(py);
            locals.set_item("time", time_module);
            py.run(cr#"
time.sleep(2)
print('sa')
                "#, None, Some(&locals)).unwrap();
            py.run(cr#"
time.sleep(2)
print('sa')
                "#, None, Some(&locals)).unwrap();
        });
        println!("end")
    }

    #[test]
    fn hecs_test(){
        let mut world = World::new();
        // Nearly any type can be used as a component with zero boilerplate
        let a: hecs::Entity = world.spawn((123, true, "abc"));
        println!("{:?}",a);
        let b = world.spawn((42, false));
        println!("{:?}",b);
        let b = world.spawn((42, false));
        println!("{:?}",b);
        let b = world.spawn((42, false));
        println!("{:?}",b);
        let b = world.spawn((42, false));
        println!("{:?}",b);
        // Systems can be simple for loops
        for (number, &flag,) in world.query_mut::<(&mut i32, &bool)>() {
            println!("sa");
            if !flag { *number *= 2; }
        }
        //world.insert(entity, components)
        ;
        // Random access is simple and safe
        println!("size of entity:{}",size_of::<Entity>());
        println!("{:?}",world.get::<&i32>(a).unwrap());
        println!("{:?}",world.get::<&i32>(b).unwrap());
    }



    struct ScriptComponent{

    }
    impl ScriptComponent {
        pub fn new() -> Self {

            todo!();

        }
    }

    #[test]
    fn script_test(){
        let world  = hecs::World::new();
        
    }
}