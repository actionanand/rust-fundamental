fn add(num_one: i32, num_two: i32) -> i32 {
  return num_one + num_two;
}

fn main() {
  let mut added_value = add(13, 10);
  let free_shipping = false;

  if added_value > 50 {
    print!("Congrats! You're eligible for free shipping.");
  } else if added_value > 20 {
    print!("You'll be eligible for free shipping, if you add some more items.");
  } else {
    print!("You're not eligible for free shipping.");
  }

  added_value = match free_shipping {
    true => added_value + 0,
    false => added_value + 10
  };

  match added_value {
    1 => println!("This is one"),
    2 => println!("This is two"),
    _ => println!("No match found!")
  }

  println!("Toatl : {:?}", added_value);

  let items: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", items);

  let vector_items = vec![1, 2, 3, 4, 5];
  println!("{:?}", vector_items);

  let mut vector_items_2 = Vec::new();
  vector_items_2.push(1);
  vector_items_2.push(2);
  vector_items_2.push(3);
  vector_items_2.push(4);
  vector_items_2.push(5);
  println!("{:?}", vector_items_2);
  
}