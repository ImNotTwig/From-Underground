use crate::remove_first;
use crate::Player;

pub fn leftover_steak(mut player:Player) -> Player {
    if player.player_inv.contains(&"Leftover Steak".to_string()) {
      if player.player_hp != player.player_max_hp {
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp + 15;
        remove_first(&mut player.player_inv, |r| r == "Leftover Steak");
        if player.player_hp > player.player_max_hp {
          player.player_hp = player.player_max_hp 
        }
        println!("You healed yourself for 15hp");
        return player;
      }
      else {
        println!("You have full health!");
        return player;
      }
    }
    else {
      println!("You don't have any leftover steak!");
      return player;
    }
  }

  pub fn granny_apple(mut player:Player) -> Player {
    if player.player_inv.contains(&"Granny Apple".to_string()) {
      if player.player_hp != player.player_max_hp {
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp + 10;
        remove_first(&mut player.player_inv, |r| r == "Granny Apple");
        if player.player_hp > player.player_max_hp {
          player.player_hp = player.player_max_hp 
        }
        println!("You healed yourself for 10hp");
        return player;
      }
      else {
        println!("You have full health!");
        return player;
      }
    }
    else {
      println!("You don't have any granny apples!");
      return player;
    }
  }