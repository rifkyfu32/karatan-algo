use std::cmp::Ordering;

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
fn binary_search<T: Ord + std::fmt::Display>(list: &[T], item: &T) -> Option<usize> {
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
fn find_smallest<T: Ord>(arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut smallest_index = 0;
    for i in 1..arr.len() {
        // Jika elemen saat ini lebih kecil dari elemen terkecil yang sudah tercatat
        if arr[i] < arr[smallest_index] {
            smallest_index = i;
        }
    }
    return Some(smallest_index);
}
/*
Mengurutkan sebuah Vector menggunakan algoritma selection sort.
Fungsi ini akan mengonsumsi vector asli dan mengembalikan vector baru yang sudah terurut.
 */
fn selection_sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    // Membuat vector baru dengan kapasitas yang sama untuk efisiensi
    let mut new_arr = Vec::with_capacity(arr.len());
    // Ulangi selama vector asli masih memiliki elemen
    while !arr.is_empty() {
        // Temukan indeks elemen terkecil di sisa vector asli
        let smallest_index = find_smallest(&arr).unwrap();
        // Pindahkan elemen terkecil dari vector asli ke vector baru
        new_arr.push(arr.remove(smallest_index));
    }
    new_arr
}

mod tests {
    use crate::{binary_search, selection_sort};

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
        let list=vec![3,5,2,4,1,7,9,6];
        let sort_list=selection_sort(list);
        println!("{:?}", sort_list);
    }
}
