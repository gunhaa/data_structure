

use std::mem::MaybeUninit;

// Rust의 정적 배열은 java와 다르게 stack에 저장되며, 컴파일타임에 자명한 len이 밝혀져야 한다

const SIZE_LIST: [usize; 10] = [2,4,8,16,32,64,128,256,512,1024];

#[derive(Debug)]
pub struct ArrayList<T> {
    pub items: [MaybeUninit<T>; SIZE_LIST[0]],
    pub capacity: usize,
    pub num_items: usize,
}

pub trait ListTrait<T> {
    fn new() -> Self;
    fn append(&mut self, item: T);
    fn display(&self)-> Vec<Option<&T>>;
    fn grow(&mut self);
    // fn index_of(&self, element: T);
    // fn set(index: u32, element: T);
    // fn len();
    // fn clear();
    // fn is_empty();
}

impl<T> ListTrait<T> for ArrayList<T> {
    fn new() -> Self {

        let arr: [MaybeUninit<T>; SIZE_LIST[0]] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        Self {
            items : arr,
            capacity : SIZE_LIST[0],
            num_items : 0,
        }
    }

    fn grow(&mut self) {

        let next_capacity = SIZE_LIST.iter()
                            .find(|&&item| item > self.capacity)
                            .copied()
                            .unwrap();

        // let mut grow_item: [MaybeUninit<T>; next_capacity] = unsafe {
        //     MaybeUninit::uninit().assume_init()
        // };


        self.capacity = next_capacity;
        // self.items = grow_item;
    }

    fn append(&mut self, item: T) {

        if !self.num_items < self.capacity {
            // 새로운 배열로 교체, 혹은 크기를 늘려야함
            self.grow();
        }

        unsafe {
            self.items[self.num_items].as_mut_ptr().write(item);
        }

        self.num_items += 1;
    }


    fn display(&self) -> Vec<Option<&T>>{
        self.items.iter().take(self.num_items).map(|item| unsafe {
            Some(item.assume_init_ref())
        }).collect()
    }
}

