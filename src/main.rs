use std::time::Instant;
use rand::Rng;

//MARK: Bubble Sort
fn bubble_sort<T>(arr: &mut [T]) where T: Ord {
	// Loop through the array n-1 times
	for i in 0..(arr.len() - 1) {
		let mut swapped = false;
		// Loop through the array from 0 to n-i-1
		// This is because elements after n-i-1 are already in their position

		for j in 0..(arr.len() - i - 1) {
			// Compare elements and swap if they are in the wrong order
			if arr[j] > arr[j + 1] {
				arr.swap(j, j + 1);
				swapped = true;
			}
		}

		// If we haven't made any swaps in this iteration, the array is sorted and we can break
		if !swapped {
			break;
		}
	}
}

//MARK: Insertion Sort
fn insertion_sort<T>(arr: &mut [T]) where T: Ord + Copy {
	// Loop through each element starting from the second element
	for i in 1..arr.len() {
		let mut current = arr[i];
		let mut j = i;
		// Shift elements to the right while they are greater than current
		while j > 0 && arr[j - 1] > current {
			arr[j] = arr[j - 1];
			j -= 1;
		}
		// Insert current element at the correct position
		arr[j] = current;
	}
}

fn merge_sort(arr: &mut [i32]) {
	let len = arr.len();
	if len <= 1 {
		return;
	}

	let mid = len / 2;

	let mut left = arr[..mid].to_vec();
	let mut right = arr[mid..].to_vec();

	merge_sort(&mut left);
	merge_sort(&mut right);
	merge(&left, &right, arr);
}

fn merge(left: &[i32], right: &[i32], arr: &mut [i32]) {
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;

	while i < left.len() && j < right.len() {
		if left[i] <= right[j] {
			arr[k] = left[i];
			i += 1;
		} else {
			arr[k] = right[j];
			j += 1;
		}
		k += 1;
	}

	while i < left.len() {
		arr[k] = left[i];
		i += 1;
		k += 1;
	}

	while j < right.len() {
		arr[k] = right[j];
		j += 1;
		k += 1;
	}
}

fn main() {
	let mut numbers: Vec<i32> = Vec::new();
	for n in 0..100000 {
		numbers.push(rand::thread_rng().gen_range(1..=100));
	}

	println!("Numbers created.\nStarting to sort...");

	let start_timer_1 = Instant::now();
	bubble_sort(&mut numbers);
	println!("[Bubble]:\tSorted array: {:.2?}", start_timer_1.elapsed());

	let start_timer_2 = Instant::now();
	insertion_sort(&mut numbers);
	println!("[Insertion]:\tSorted array: {:.2?}", start_timer_2.elapsed());

	let start_timer_3 = Instant::now();
	merge_sort(&mut numbers);
	println!("[Merge]:\tSorted array: {:.2?}", start_timer_3.elapsed());
}