use std::cell::RefCell;
use std::fmt::Pointer;
use std::ops::Deref;
use std::ptr::NonNull;
use std::ptr;
pub struct MyRc<T> {
    pointer : NonNull<MyRcBox<T>>,
    
}

struct MyRcBox<T> {
    count : usize,
    value : T,
}

impl<T> MyRc<T>  {
    pub fn new(value_ : T) -> MyRc<T> {
        unsafe {
            let ptr = Box::leak (Box::new( MyRcBox{ count : 1 ,value : value_ } ) ).into();
            
            MyRc{pointer : ptr}

        }
    }

    pub fn drop(&mut self) {
            unsafe {
                let now_count = self.pointer.as_ref().count - 1;
                self.pointer.as_mut().count = now_count;
                println!("Drop! Current count = {}", now_count);
                if now_count == 0 {
                    let local_box  = Box::from_raw(self.pointer.as_mut());
                    println!("Free pointer!");
                }
            }
    }

    pub fn get_count(& self) -> usize {
        unsafe {
            self.pointer.as_ref().count
        }
    }

    pub fn clone (&mut self) -> MyRc<T> {
        unsafe {
            let new_count = self.pointer.as_ref().count + 1;
            self.pointer.as_mut().count = new_count;
            println!("Cloned! Now count = {}",new_count);
            MyRc { pointer: (self.pointer) }
        }

    } 

}

impl<T> Deref for MyRc<T> {
    type Target = T;
     fn deref(&self) -> &Self::Target {
        unsafe {
            &self.pointer.as_ref().value
        }
    }
}

