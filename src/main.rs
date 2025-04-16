mod array_list;
use array_list::fail_array_list::{ListTrait, ArrayList};
use array_list::array_list::ArrayListVer2;

fn main() {
    // let mut fail_array_list:ArrayList<u32> = ArrayList::new();

    // println!("this init array_list : {:?}" , fail_array_list.display());
    // fail_array_list.append(500);
    // fail_array_list.append(501);
    // fail_array_list.append(502);
    // fail_array_list.append(503);

    // println!("this array_list : {:?}" , fail_array_list.display());

    let mut array_list_ver2:ArrayListVer2<u32> = ArrayListVer2::new();
    array_list_ver2.append(301);
    array_list_ver2.append(302);
    array_list_ver2.append(303);
    // array_list_ver2.append(304);
    println!("this raw : {:?}" , array_list_ver2);
    println!("this : {:?}", array_list_ver2.display());
}
