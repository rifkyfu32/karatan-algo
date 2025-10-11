use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

fn main() {
    println!("Hello, world!");
}
/*
Melakukan pencarian biner pada sebuah slice yang sudah terurut.
 # Argumen
 * `list` - Slice yang sudah terurut untuk dicari.
 * `item` - Item yang ingin dicari di dalam slice.

 # Mengembalikan
 * `Some(usize)` - Jika item ditemukan, mengembalikan indeks dari item tersebut.
 * `None` - Jika item tidak ditemukan.
*/
fn binary_search<T: Ord + Display>(list: &[T], item: &T) -> Option<usize> {
    // Menentukan batas bawah (low) dan batas atas (high) dari area pencarian.
    let mut low = 0;
    let mut high = list.len();
    // Terus mencari selama area pencarian masih valid (low < high).
    while low < high {
        /*
        Menghitung indeks tengah untuk menghindari overflow.
        Ini adalah cara yang aman untuk menghitung indeks tengah.
        Cara ini menghindari potensi integer overflow yang bisa terjadi
        jika kita menggunakan (low + high) / 2 pada list yang sangat besar.
        */
        let mid = low + (high - low) / 2;
        println!("low before: {}", low);
        println!("high before: {}", high);
        println!("mid: {}", mid);
        println!("list[mid]: {}", list[mid]);
        /*
        Membandingkan nilai pada indeks tengah (list[mid]) dengan item yang dicari
        menggunakan metode .cmp(). Metode ini mengembalikan enum Ordering yang memiliki
        tiga varian: Less, Greater, atau Equal.
        */
        match list[mid].cmp(item) {
            /*
            Jika nilai tengah lebih kecil, maka item pasti ada di sebelah kanan.
            Jadi, perbarui batas bawah low menjadi mid + 1
            */
            Ordering::Less => low = mid + 1,
            /*
            Jika nilai tengah lebih besar, maka item pasti ada di sebelah kiri.
            perbarui batas atas high menjadi mid
            */
            Ordering::Greater => high = mid,
            // Jika sama, item ditemukan! Kembalikan indeksnya.
            Ordering::Equal => return Some(mid),
        }
        println!("low after: {}", low);
        println!("high after: {}", high);
    }
    // Jika loop selesai tanpa menemukan item, berarti item tidak ada di dalam list.
    None
}
/*
Menemukan indeks dari elemen terkecil dalam sebuah slice.
Mengembalikan `None` jika slice kosong.
 */
fn find_smallest<T: Ord + Display>(arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut smallest_index = 0;
    for i in 1..arr.len() {
        // Jika elemen saat ini lebih kecil dari elemen terkecil yang sudah tercatat
        if arr[i] < arr[smallest_index] {
            smallest_index = i;
        }
        println!("find smallest: {}", &arr[smallest_index]);
    }
    return Some(smallest_index);
}
/*
Mengurutkan sebuah Vector menggunakan algoritma selection sort.
Fungsi ini akan mengonsumsi vector asli dan mengembalikan vector baru yang sudah terurut.
 */
fn selection_sort<T: Ord + Debug + Display>(mut arr: Vec<T>) -> Vec<T> {
    // Membuat vector baru dengan kapasitas yang sama untuk efisiensi
    let mut new_arr = Vec::with_capacity(arr.len());
    // Ulangi selama vector asli masih memiliki elemen
    while !arr.is_empty() {
        // Temukan indeks elemen terkecil di sisa vector asli
        println!("while: {:?}", &arr);
        let smallest_index = find_smallest(&arr).unwrap();
        println!("smallest: {}", &arr[smallest_index]);
        // Pindahkan elemen terkecil dari vector asli ke vector baru
        new_arr.push(arr.remove(smallest_index));
    }
    new_arr
}
/// Menghitung jumlah total elemen dalam sebuah slice i32 secara rekursif.
fn sum_recursive(arr: &[i32]) -> i32 {
    match arr {
        // Base Case: Jika slice kosong ([]), hasilnya adalah 0.
        [] => 0,
        /*
        Recursive Case: Jika slice tidak kosong...
        'head' akan berisi elemen pertama (misal: 1).
        'tail' akan berisi sisa slice (misal: [2, 3, 4]).
        '@ ..' adalah sintaks untuk "sisa dari slice".
         */
        [head, tail @ ..] => head + sum_recursive(tail),
    }
}
fn sum_recursive_simple(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    } else {
        let head = arr[0];
        let tail = &arr[1..];
        println!("{}", &arr.len());
        println!("{:?}", &arr[1..]);
        println!("{:?}", &arr[..arr.len()]);
        return head + sum_recursive_simple(tail);
    }
}
fn sum_recursive_idiom(arr: &[i32]) -> i32 {
    //Jika arr.split_first() mengembalikan Some, bongkar isinya ke dalam variabel head dan tail
    if let Some((head, tail)) = arr.split_first() {
        return head + sum_recursive_idiom(tail);
    } else {
        return 0;
    }
}
fn quicksort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() < 2 {
        // Base Case: Array dengan 0 atau 1 elemen sudah pasti terurut.
        return arr.to_vec();
    } else {
        // Recursive Case
        // 1. Pilih pivot
        let pivot = &arr[0];
        //list comprehension
        // 2. Buat sub-array 'less' (elemen yang lebih kecil atau sama dengan pivot).
        let less: Vec<T> = arr[1..].iter().filter(|&x| x <= pivot).cloned().collect();
        // 3. Buat sub-array 'greater' (elemen yang lebih besar atau sama dengan pivot).
        let greater: Vec<T> = arr[1..].iter().filter(|&x| x >= pivot).cloned().collect();
        let mut result = quicksort(&less);
        result.push(pivot.clone());
        result.extend(quicksort(&greater));
        result
    }
}
fn quicksort_3way<T: Ord + Clone + Debug + Display>(arr: &[T]) -> Vec<T> {
    println!("array: {:?}", arr);
    if arr.len() < 2 {
        println!("array lenght < 2 {:?}", arr);
        return arr.to_vec();
    } else {
        let pivot = &arr[arr.len() / 2];
        println!("pivot: {}", pivot);
        let less: Vec<T> = arr
            .iter()
            .filter(|&x| x.cmp(pivot) == Ordering::Less)
            .cloned()
            .collect();
        println!("less: {:?}", less);
        let equal: Vec<T> = arr
            .iter()
            .filter(|&x| x.cmp(pivot) == Ordering::Equal)
            .cloned()
            .collect();
        println!("equal: {:?}", equal);
        let greater: Vec<T> = arr
            .iter()
            .filter(|&x| x.cmp(pivot) == Ordering::Greater)
            .cloned()
            .collect();
        println!("greater: {:?}", greater);
        let mut result = quicksort_3way(&less);
        result.extend(equal);
        result.extend(quicksort_3way(&greater));
        println!("result: {:?}", result);
        return result;
    }
}
fn factorial(x: u64) -> u64 {
    println!("x: {}", x);
    if x == 1 {
        return 1;
    }
    let result = x * factorial(x - 1);
    println!("result: {} * factorial({}) = {}", x, x - 1, result);
    result
}
fn multiplication_table(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut table = Vec::new();
    for row in arr.iter() {
        let mut current = Vec::new();
        for col in arr.iter() {
            current.push(row * col);
        }
        table.push(current);
    }
    table
}
fn multiplication_table_function(arr: &[i32]) -> Vec<Vec<i32>> {
    arr.iter()
        .map(|row| arr.iter().map(|col| row * col).collect())
        .collect()
}
fn is_seller(name: &str) -> bool {
    name.ends_with('m')
}
fn breadth_first_search(graph: &HashMap<String, Vec<String>>, start_node: &str) -> bool {
    println!("Graph: {:?}", graph);
    // 1. Gunakan VecDeque sebagai antrian (queue),
    // efisien untuk operasi pop_front (mengambil dari depan) dan push_back (menambah ke belakang) yang merupakan inti dari BFS
    let mut search_queue = VecDeque::new();
    // Inisialisasi antrian dengan tetangga dari titik awal.
    if let Some(neighbors) = graph.get(start_node) {
        for neighbor in neighbors {
            println!("starting add person: {}", neighbor);
            search_queue.push_back(neighbor.clone());
        }
    }
    println!("Search queue: {:?}", search_queue);
    // 2. Gunakan HashSet untuk melacak node yang sudah diperiksa.
    // Ini jauh lebih efisien daripada list/vector untuk pengecekan.
    let mut searched = HashSet::new();

    // 3. Loop selama antrian tidak kosong.
    while let Some(person) = search_queue.pop_front() {
        // Hanya proses orang ini jika belum pernah diperiksa sebelumnya.
        if !searched.contains(&person) {
            // Cek apakah orang ini adalah penjual.
            let is_seller = is_seller(&person);
            println!("is {} is seller? {}", &person, is_seller);
            if is_seller {
                println!("{} is seller. We found it!", person);
                return true;
            } else {
                // Jika bukan, tambahkan semua tetangganya ke dalam antrian.
                println!("because {} not seller, add next person", &person);
                if let Some(neighbors) = graph.get(&person) {
                    for neighbor in neighbors {
                        println!("new person: {}", neighbor);
                        search_queue.push_back(neighbor.clone());
                    }
                }
                // Tandai orang ini sebagai sudah diperiksa.
                searched.insert(person);
            }
        } else {
            println!("Skip, because {} has been serached before.", &person);
        }
        println!("Searched: {:?}", searched);
        println!("Search queue: {:?}", search_queue);
    }
    // Jika loop selesai dan tidak ada penjual yang ditemukan.
    println!("Tidak ada penjual yang ditemukan.");
    false
}

mod tests {
    use std::collections::HashMap;

    use crate::{
        binary_search, breadth_first_search, factorial, multiplication_table,
        multiplication_table_function, quicksort, quicksort_3way, selection_sort, sum_recursive,
        sum_recursive_idiom, sum_recursive_simple,
    };

    #[test]
    fn test_binary_search() {
        let list = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ];
        println!("len list: {}", list.len());
        let item_search = 12;
        match binary_search(&list, &item_search) {
            Some(index) => println!("Item {} ditemukan pada indeks {}.", item_search, index),
            None => println!("Item {} tidak ditemukan.", item_search),
        }
        // Contoh dengan kata (String)
        let words = vec![
            "apple",
            "banana",
            "cherry",
            "durian",
            "grape",
            "jackfruit",
            "kiwi",
            "orange",
            "pineapple",
        ];
        let target_word = "pineapple";

        match binary_search(&words, &&target_word) {
            Some(index) => println!("Kata '{}' ditemukan pada indeks {}.", target_word, index),
            None => println!("Kata '{}' tidak ditemukan.", target_word),
        }
    }
    #[test]
    fn test_selection_sort() {
        let list = vec![3, 5, 2, 4, 1, 7, 9, 6];
        let sort_list = selection_sort(list);
        println!("{:?}", sort_list);
    }
    #[test]
    fn test_sum_recursive() {
        let list = vec![3, 2, 5, 7, 2, 5];
        let sum = sum_recursive(&list);
        println!("Jumlah dari {:?} adalah {}", list, sum);
    }
    #[test]
    fn test_sum_recursive_simple() {
        let arr = vec![2, 3, 1, 2, 5, 4];
        let sum = sum_recursive_simple(&arr);
        println!("Jumlah dari {:?} adalah {}", arr, sum)
    }
    #[test]
    fn test_sum_recursive_idiom() {
        let list = vec![2, 3, 4, 1];
        let sum = sum_recursive_idiom(&list);
        println!("Jumlah dari {:?} adalah {}", list, sum);
    }
    #[test]
    fn test_quicksort() {
        let list = vec![2, 3, 1, 6, 4, 5];
        let sort = quicksort(&list);
        println!("{:?}", sort);
    }
    #[test]
    fn test_quicksort_3way() {
        let list = vec![2, 4, 1, 9, 4, 1, 5];
        let sort = quicksort_3way(&list);
        println!("{:?}", sort);
    }
    #[test]
    fn test_factorial() {
        let salesman_traveling = factorial(10) as f64;
        let estimate = 10.0; // 10 operation per second
        let days = 86_000.0;
        let operation = salesman_traveling / estimate / days;
        println!("salesman travelling = {}", salesman_traveling);
        println!("operation = {} day", operation);
        println!("big o notation O(n!)");
    }
    #[test]
    fn test_multiplication_table() {
        let list = vec![2, 3, 7, 8, 10];
        let result = multiplication_table(&list);
        for item in result {
            println!("{:?}", item);
        }
    }
    #[test]
    fn test_multiplication_table_function() {
        let list = vec![1, 2, 3, 4, 5];
        let result = multiplication_table_function(&list);
        for item in result {
            println!("{:?}", item);
        }
    }
    #[test]
    fn test_breadth_first_search() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert(
            "you".to_string(),
            vec!["alice".to_string(), "bob".to_string(), "claire".to_string()],
        );
        graph.insert("alice".to_string(), vec!["peggy".to_string()]);
        graph.insert(
            "bob".to_string(),
            vec!["anuj".to_string(), "peggy".to_string()],
        );
        graph.insert(
            "claire".to_string(),
            vec!["thom".to_string(), "jhonny".to_string()],
        );
        graph.insert("peggy".to_string(), vec![]);
        graph.insert("anuj".to_string(), vec![]);
        graph.insert("thom".to_string(), vec![]);
        graph.insert("jhonny".to_string(), vec![]);
        breadth_first_search(&graph, "you");
    }
}
