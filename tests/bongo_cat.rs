use bongo_cat::bongo_cat;

#[bongo_cat(width = 64, height = 64, binary)]
mod bongo_cat_img {}

#[test]
fn bongo_cat_test() {
    println!("bongo_cat_default: {:#?}", bongo_cat_img::DEFAULT);
}
