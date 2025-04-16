use data_structure::{ListTrait , ArrayList};


fn main() {
    let mut array_list:ArrayList<u32> = ArrayList::new();

    println!("this init array_list : {:?}" , array_list.display());
    array_list.append(500);
    array_list.append(501);
    array_list.append(502);
    array_list.append(503);

    println!("this array_list : {:?}" , array_list.display());
}
