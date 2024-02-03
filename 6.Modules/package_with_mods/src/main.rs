mod external_mod;
mod external_parent_mod;

mod inline_mod {
    pub fn print_one(){
        println!("one");
    }
}

fn main() {
    crate::inline_mod::print_one();
    crate::external_mod::print_two();
    crate::external_parent_mod::sub_mod::print_three();
}
