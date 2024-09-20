// Быстрая сортировка массива
fn quicksort_t<T: Ord + Copy>(nums: &mut [T]) {
    if nums.len() <= 1 {
        // если массив состоит из 1 элемента, осуществляем возврат из функции
        return;
    }

    let mut i = 0; // левая граница
    let mut j = nums.len() - 1; // правая граница
    let pivot = nums[nums.len() / 2]; // опорный элемент, выбираем в центре массива
    loop {
        while nums[i] < pivot {
            // двигаемся с левой стороны до тех пор пока не найдем элемент меньше опорного
            i += 1;
        }
        while nums[j] > pivot {
            // двигаемся с правой стороны до тех пор пока не найдем элемент больше опорного
            j -= 1;
        }

        if i <= j {
            // меняем местами элементы
            let temp: T = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;

            i = i + 1;
            j = j - 1;
        }
        if i > j {
            // если достигли середины выходим из цикла
            break;
        }
    }
    // на данном этапе элементы слева от опорного меньше а справа - больше
    if j > 0 {
        //
        quicksort_t(&mut nums[..=j]);
    }

    if nums.len() > i {
        quicksort_t(&mut nums[i..]);
    }
}

fn main() {
    let mut numbers = [4, 9, 7, 6, 2, 3, 8];
    quicksort_t(&mut numbers);
    println!("{:?}", numbers);
}
