fn sum(one: i32, two: i32) -> i32 {
    one + two
}

#[test]
fn test_positive_sum(){
    let res = sum(2,2);
    assert_eq!(res, 4);
}

#[test]
fn test_negative_sum(){
    let res = sum(-2,-2);
    assert_eq!(res, -4);
}