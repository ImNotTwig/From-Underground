use crate::remove_first;

pub fn hp_potion(mut player_inv: Vec<String>, player_hp: i32, player_max_hp: i32, prev_player_hp: i32) -> (/*player_inv:*/ Vec<String>, /*player_hp:*/ i32, /*prev_player_hp:*/ i32) {
  if player_inv.contains(&"Health Potion".to_string()) {
    if player_hp != player_max_hp {
      let prev_player_hp = player_hp;
      let player_hp = player_hp + 25;
      remove_first(&mut player_inv, |r| r == "Health Potion");
      return (player_inv, player_hp, prev_player_hp);
    }
    else {
      println!("You have full health!");
      return (player_inv, player_hp, prev_player_hp);
    }
  }
  else {
    println!("You don't have any health potions!");
    return (player_inv, player_hp, prev_player_hp);
  }
}

//this is how to use this function, it's the same for every varient
// let (player_inv, player_hp, prev_player_hp) = hp_potion(player_inv, player_hp, player_max_hp, prev_player_hp);

pub fn med_hp_potion(mut player_inv: Vec<String>, player_hp: i32, player_max_hp: i32, prev_player_hp: i32) -> (/*player_inv:*/ Vec<String>, /*player_hp:*/ i32, /*prev_player_hp:*/ i32) {
  if player_inv.contains(&"Medium Health Potion".to_string()) {
    if player_hp != player_max_hp {
      let prev_player_hp = player_hp;
      let player_hp = player_hp + 25;
      remove_first(&mut player_inv, |r| r == "Medium Health Potion");
      return (player_inv, player_hp, prev_player_hp);
    }
    else {
      println!("You have full health!");
      return (player_inv, player_hp, prev_player_hp);
    }
  }
  else {
    println!("You don't have any medium health potions!");
    return (player_inv, player_hp, prev_player_hp);
  }
}

pub fn large_hp_potion(mut player_inv: Vec<String>, player_hp: i32, player_max_hp: i32, prev_player_hp: i32) -> (/*player_inv:*/ Vec<String>, /*player_hp:*/ i32, /*prev_player_hp:*/ i32) {
  if player_inv.contains(&"Large Health Potion".to_string()) {
    if player_hp != player_max_hp {
      let prev_player_hp = player_hp;
      let player_hp = player_hp + 25;
      remove_first(&mut player_inv, |r| r == "Large Health Potion");
      return (player_inv, player_hp, prev_player_hp);
    }
    else {
      println!("You have full health!");
      return (player_inv, player_hp, prev_player_hp);
    }
  }
  else {
    println!("You don't have any large health potions!");
    return (player_inv, player_hp, prev_player_hp);
  }
}