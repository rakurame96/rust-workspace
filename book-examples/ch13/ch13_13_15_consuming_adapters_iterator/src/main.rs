fn main() {
    let v1 = vec![1, 3, 5, 7, 9];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 4, 6, 8, 10]);
}
