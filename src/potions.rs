use crate::remove_first;
use crate::Player;

pub fn hp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Health Potion".to_string()) {
    if player.player_hp != player.player_max_hp {
      player.prev_player_hp = player.player_hp;
      player.player_hp = player.player_hp + 25;
      remove_first(&mut player.player_inv, |r| r == "Health Potion");
      if player.player_hp > player.player_max_hp {
        player.player_hp = player.player_max_hp 
      }
      return player;
    }
    else {
      println!("You have full health!");
      return player;
    }
  }
  else {
    println!("You don't have any health potions!");
    return player;
  }
}

//this is how to use this function, it's the same for every varient
// player = hp_potion(player);

pub fn med_hp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Medium Health Potion".to_string()) {
    if player.player_hp != player.player_max_hp {
      player.prev_player_hp = player.player_hp;
      player.player_hp = player.player_hp + 50;
      remove_first(&mut player.player_inv, |r| r == "Medium Health Potion");
      if player.player_hp > player.player_max_hp {
        player.player_hp = player.player_max_hp 
      }
      return player;
    }
    else {
      println!("You have full health!");
      return player;
    }
  }
  else {
    println!("You don't have any medium health potions!");
    return player;
  }
}

pub fn large_hp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Large Health Potion".to_string()) {
    if player.player_hp != player.player_max_hp {
      player.prev_player_hp = player.player_hp;
      player.player_hp = player.player_hp + 75;
      remove_first(&mut player.player_inv, |r| r == "Large Health Potion");
      if player.player_hp > player.player_max_hp {
        player.player_hp = player.player_max_hp 
      }
      return player;
    }
    else {
      println!("You have full health!");
      return player;
    }
  }
  else {
    println!("You don't have any large health potions!");
    return player;
  }
}


pub fn mp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Mana Potion".to_string()) {
    if player.player_mp != player.player_max_mp {
      player.prev_player_mp = player.player_mp;
      player.player_mp = player.player_mp + 25;
      remove_first(&mut player.player_inv, |r| r == "Mana Potion");
      if player.player_mp > player.player_max_mp {
        player.player_mp = player.player_max_mp 
      }
      return player;
    }
    else {
      println!("You have full mana!");
      return player;
    }
  }
  else {
    println!("You don't have any mana potions!");
    return player;
  }
}

//this is how to use this function, it's the same for every varient
// player = mp_potion(player);

pub fn med_mp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Medium Mana Potion".to_string()) {
    if player.player_mp != player.player_max_mp {
      player.prev_player_mp = player.player_mp;
      player.player_mp = player.player_mp + 50;
      remove_first(&mut player.player_inv, |r| r == "Medium Mana Potion");
      if player.player_mp > player.player_max_mp {
        player.player_mp = player.player_max_mp 
      }
      return player;
    }
    else {
      println!("You have full mana!");
      return player;
    }
  }
  else {
    println!("You don't have any medium mana potions!");
    return player;
  }
}

pub fn large_mp_potion(mut player:Player) -> Player {
  if player.player_inv.contains(&"Large Mana Potion".to_string()) {
    if player.player_mp != player.player_max_mp {
      player.prev_player_mp = player.player_mp;
      player.player_mp = player.player_mp + 75;
      remove_first(&mut player.player_inv, |r| r == "Large Mana Potion");
      if player.player_mp > player.player_max_mp {
        player.player_mp = player.player_max_mp 
      }
      return player;
    }
    else {
      println!("You have full mana!");
      return player;
    }
  }
  else {
    println!("You don't have any large mana potions!");
    return player;
  }
}