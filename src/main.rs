
fn main() {
    let num_1 = [1, 2, 3, 4, 5];
    let num_2 = [6, 7, 8, 9, 10];
    //num_1 and num_2を結合してnum_listに格納
    let num_list = [&num_1[..], &num_2[..]].concat();
    println!("{:?}", num_list);

    // //list_numの中身のインデックスと値を別の配列に二次元配列[usize,isize]で格納
    // let mut num_list_index: Vec<(usize, i32)> = Vec::new();
    // for (i, &num) in num_list.iter().enumerate() {
    //     num_list_index.push((i, num));
    // }
    // let num_list_even: Vec<_> = num_list.iter().filter(|&x| x % 2 == 0).collect();
    // println!("{:?}", num_list_even);
    // println!(
    //     "{:?}",
    //     num_list.iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>()
    // );
    // println!("{:?}", num_list_index.iter().collect::<Vec<_>>());

    let string = num_1.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    //stringに格納された値を連続した文字列に変換
    let string = string.join(" ");
    println!("{}", string);

    let mut num_list_hash = HashMap::new();
    for (i, &num) in num_list.iter().enumerate() {
        num_list_hash.insert(i, num);
    }
    for (key, value) in &num_list_hash {
        println!("{}: {}", key, value);
    }
    // num_list_hashをソートして表示
    let mut num_list_hash: Vec<_> = num_list_hash.iter().collect();
    num_list_hash.sort();
    println!("{:?}", num_list_hash);
}
