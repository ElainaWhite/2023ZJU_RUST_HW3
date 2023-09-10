use std::collections::HashMap;
mod MyHashMap;
mod MyStack;
mod MyRc;


fn main() {
    
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{my_hash_map, MyStack, MyRc::MyRc};

    #[test]
    fn test_hash_map() {
        let  mymap:HashMap<i32,&str> =  my_hash_map!(1 => "a", 2 => "b", 3=> "c");
        assert_eq!(mymap.get(&1),Some(&"a"));
        assert_eq!(mymap.get(&2),Some(&"b"));
        assert_eq!(mymap.get(&3),Some(&"c"));
    }

    #[test]
    fn test_stack() {
        let stack = MyStack::MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        assert_eq!(stack.pop(),Some(4));
        assert_eq!(stack.pop(),Some(3));
        assert_eq!(stack.pop(),Some(2));
        assert_eq!(stack.pop(),Some(1));
        assert_eq!(stack.pop(),None);
    }

    #[test]
    fn test_my_rc() {
        let mut _rc1 = MyRc::new(5);
        assert_eq!(*_rc1,5);
        assert_eq!(_rc1.get_count(),1);

        let mut _rc2 = _rc1.clone();
        assert_eq!(*_rc2,5);
        assert_eq!(_rc1.get_count(),2);
        assert_eq!(_rc2.get_count(),2);

    }
}
