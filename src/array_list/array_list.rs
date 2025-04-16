use std::mem::MaybeUninit;



#[derive(Debug)]
pub struct ArrayListVer2<T> {
    items: Box<[MaybeUninit<T>]>,
    len: usize,
    capacity: usize,
}

impl<T: Copy> ArrayListVer2<T> {
    pub fn new() -> Self {

        let init:Box<[MaybeUninit<T>]> = unsafe {
            Box::<[MaybeUninit<T>]>::new_uninit_slice(2).assume_init()
        };

        ArrayListVer2 {
            items: init,
            len: 0,
            capacity: 2,
        }
    }

    pub fn append(&mut self, value: T){

        if self.len == self.capacity {
            self.grow();
        }

        self.items[self.len].write(value);
        self.len += 1;
    }

    fn grow(&mut self){
        let new_capacity = self.capacity*2;
        let mut new_items = unsafe {
            Box::<[MaybeUninit<T>]>::new_uninit_slice(new_capacity).assume_init()
        };

        for (i, item) in self.items.iter().enumerate() {
            unsafe {
                new_items[i].write(item.assume_init());
            }
        }

        for i in 0..self.len {
            unsafe {
                std::ptr::drop_in_place(self.items[i].as_mut_ptr());
            }
        }

        self.items = new_items;
        self.capacity = new_capacity;
    }

    pub fn display(&self) -> Vec<Option<&T>>{
            self.items.iter().take(self.len).map(|item| unsafe {
                Some(item.assume_init_ref())
        }).collect()
    }
}