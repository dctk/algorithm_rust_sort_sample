
/// バブルソート
#[allow(dead_code)]
pub fn bubble_sort<T>(data :&mut Vec<T>)
	where 
		T:Ord 
{
	let len = data.len() - 1;
	let mut count = 0;
	let mut continue_flg = true;

	while continue_flg{
		continue_flg = false;
		for i in 0..(len-count) {
			if data[i]>data[i+1]{
				continue_flg=true;
				data.swap(i,i+1);
			}
		}
		count+=1;
	}
}

#[test]
fn test_bubble_sort()
{
	let mut data = vec![1,4,62,2,3,4,5,6,55,8,11,233];
	bubble_sort(&mut data);
	assert_eq!(data,vec![1,2,3,4,4,5,6,8,11,55,62,233]);
}

// クイックソート
#[allow(dead_code)]
pub fn quick_sort<T>(data:&mut Vec<T>,left:usize,right:usize)
	where
		T: Ord{

	let mut l = left;
	let mut r = right;
	let mut div_idx =left;
	//let div = data[l];   // here           : cannot move out of indexed content
	//let div = &data[l];  // data.swap(...) : cannot borrow `data` as mutable because it is also borrowed as immutable

	while l<r {
		while l<=r && data[l]<=data[div_idx]{
			l+=1;
		}
		while l<=r && data[r]>data[div_idx]{
			r-=1;	
		}

		if l<r {
			if div_idx == l{
				div_idx = r;
			}
			else if div_idx == r{
				div_idx = l;
			}
			data.swap(l,r);
		}
	}

	// 分割の基準にしたデータを左右の境界に移動する、
	data.swap(div_idx,r);

	if r>0 && left<(r-1){
		quick_sort(data,left,r-1);
	}	
	if (r+1)<right{
		quick_sort(data,r+1,right);
	}
}

#[test]
fn test_quick_sort()
{
	let mut data = vec![1,4,62,2,3,4,5,6,55,8,11,6,233];

	let len = data.len();
	//quick_sort(&mut data,0,data.len()-1); // cannot borrow `data` as immutable because it is also borrowed as mutable
	quick_sort(&mut data,0,len-1);

	assert_eq!(data,vec![1,2,3,4,4,5,6,6,8,11,55,62,233]);
}

// マージソート
#[allow(dead_code)]
pub fn merge_sort<T>(data : &mut Vec<T>, left: usize, right:usize) 
	where
		T : Ord+std::fmt::Display
{
	if left==right {
		return;
	}

	let mid = (right-left)/2;
	if left<left+mid{
		merge_sort(data,left,left+mid);
	}
	if left+mid+1<right{
		merge_sort(data,left+mid+1,right);
	}
	
	let mut left_idx = left;
	let mut right_idx = left+mid+1;

    // 削除/挿入のコストが高いので何とかしたい
	while left_idx<right_idx && right_idx<=right {
		if data[left_idx]>data[right_idx]{
			let tmp = data.remove(right_idx);
			data.insert(left_idx,tmp);
			right_idx+=1;
		}
		left_idx+=1;
	}
}

#[test]
fn test_merge_sort()
{
	let mut data = vec![1,4,62,2,3,4,5,6,55,8,11,233,];
	let len = data.len()-1;
	merge_sort(&mut data,0,len);
	assert_eq!(data,vec![1,2,3,4,4,5,6,8,11,55,62,233]);
}

// コームソート
pub fn comb_sort<T>(data: &mut Vec<T>)
    where
    T:Ord{

    let mut flg = false;
    let rate = 13;
    let len = data.len();
    let mut gap = len;
    
    loop{
        gap = (gap*10)/rate;
        if gap == 0{
            gap = 1;
        }

        flg = false;
        for i in 0..(len-gap){
            if data[i]>data[i+gap]{
                flg=true;
                data.swap(i,i+gap);
            }
        }

        if !flg && gap==1 {
            break;
        }
    }
}

#[test]
fn test_comb_sort()
{
	let mut data = vec![1,4,62,2,3,4,5,6,55,8,11,233,];
	comb_sort(&mut data);
	assert_eq!(data,vec![1,2,3,4,4,5,6,8,11,55,62,233]);
}

pub fn insert_sort<T>(data: &mut Vec<T>)
    where
    T : Ord{

    let len = data.len();
    for i in 1..len{
        for j in 0..i{
            if data[j]>data[i]{                
                let tmp = data.remove(i);
                data.insert(j,tmp);
                break;
            }
        }
    }
}

#[test]
fn test_insert_sort()
{
	let mut data = vec![1,4,62,2,3,4,5,6,55,8,11,233,];
	insert_sort(&mut data);
	assert_eq!(data,vec![1,2,3,4,4,5,6,8,11,55,62,233]);
}
