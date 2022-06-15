use std::io;
use std::io::prelude::*;
use text_io::read;
use rand::Rng;
use dialoguer::{theme::ColorfulTheme, Input};
use ansi_term::Style;
mod potions;
use crate::potions::*;
mod food;
use crate::food::*;
  //I want a title screen [x]
  //I want to start the story  [x]
  //I want multiple rooms to be able to go between [x]
  //I want items to be a prominent part of the game [x]
  //I want multiple routes to go through to finish the game meaning I want to be able to have unique playthoughs
  //I want an inventory system [x]
  //I want a fighting system [x]
  //I want some kind of mana/health system [x]
struct Rooms {
  north_room: String,
  south_room: String,
  west_room: String,
  east_room: String
}
struct Opponent {
  hp: i32,
  mp: i32,
  min_atk: i32,
  max_atk: i32,
  magic_atks: Vec<String>,
  inv: Vec<String>
}
#[derive(Clone)]
pub struct Player {
  player_inv: Vec<String>,
  player_name: String,

  player_hp: i32,
  player_max_hp: i32,
  prev_player_hp: i32,

  player_mp: i32,
  player_max_mp: i32,
  prev_player_mp: i32,

  player_min_atk: i32,
  player_max_atk: i32
}
#[derive(Clone)]
struct RoomsData {
  room_title: String,
  picked_up_item_from_closet: bool,
  are_items_in_closet: bool,
  picked_up_item_from_kitchen: bool,
  are_items_in_kitchen: bool,
  picked_up_item_from_library: bool,
  are_items_in_library: bool
}

fn main() {
  let mut level: i32 = 0;
  let mut player = Player {
    player_inv: vec!("".to_string()),
    player_name: "".to_string(),

    player_hp: 50,
    player_max_hp: 50,
    prev_player_hp: 50,

    player_mp: 50,
    player_max_mp: 50,
    prev_player_mp: 50,

    player_min_atk: 5,
    player_max_atk: 10
  };
  let mut rooms_data = RoomsData {
    room_title: "".to_string(),

    picked_up_item_from_closet: false,
    are_items_in_closet:true,

    picked_up_item_from_kitchen:false,
    are_items_in_kitchen:true,

    picked_up_item_from_library:false,
    are_items_in_library:true

  };
  
  player.player_inv.remove(0);

  println!("\x1B[2J\x1B[1;1H"); //clears the screen
  println!("Welcome to: From Underground, a text adventure");

  loop {
    if level == 0 {
      player = scene_0(player);
      level = 1
    }
    if level == 1 {
      println!("{}", Style::new().bold().paint(r#""Follow me I'll show you to the operations room.""#));
      println!("");
      (player, rooms_data, level) = scene_1(player, rooms_data.clone());
    }
    if level == 2 {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("{}", Style::new().bold().paint(r#""Here we are, the operations room.""#));
      println!("{}", Style::new().bold().paint("You hear the door behind you close."));
      println!("{}", Style::new().bold().paint(r#""What is that?""#));
      println!("{}", Style::new().bold().paint("He points at a weird animal coming out of the darkness towards the other side of the room."));
      println!("{}", Style::new().bold().paint("It charges at you"));
      println!("");
      (player, rooms_data, level) = scene_2(player, rooms_data.clone());
    }
    if level == 3 {
      println!("{}", Style::new().bold().paint(r#""That rat must've chewed up some of the hardware.""#));
      println!("{}", Style::new().bold().paint(r#""You look for the hard drive and I'll fix the power.""#));
      (player, rooms_data, level) = scene_3(player, rooms_data.clone());
    }
  }
}


fn remove_first<T, F>(vec: &mut Vec<T>, mut filter: F)
where
    F: for<'a> FnMut(&'a T) -> bool,
{
    let mut removed = false;
    vec.retain(move |item| {
        if removed || !filter(item) {
            true
        } else {
            removed = true;
            false
        }
    });
}

fn check_inv(mut player: Player) -> Player {
  if player.player_inv.is_empty() == false {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    let mut a = 1;
    for i in player.player_inv.clone() {
      println!("{}: {}", Style::new().italic().paint(a.to_string()), Style::new().italic().paint(i.to_string()));
      a += 1;
    }
    println!("");
    println!("{}", Style::new().bold().paint("Would you like to use an item? [y/n]"));
    let choice: String = read!();
    let choice = choice.to_lowercase();
    if choice == "yes" || choice == "y" {
      println!("{}", Style::new().bold().paint("Which item?"));
      let mut item_choice: usize = read!();
      item_choice -= 1;
      let item_use_input: String = player.player_inv[item_choice].clone();
      player = item_use(player, item_use_input);
      return player
    }
    else {
      return player
    }
  }
  else {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    println!("There's nothing here!");
    println!("");
    return player
  }
}

fn check_inv_fight(mut player: Player) -> (Player, bool) {
  let mut used_item = false;
  if player.player_inv.is_empty() == false {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    let mut a = 1;
    for i in player.player_inv.clone() {
      println!("{}: {}", Style::new().italic().paint(a.to_string()), Style::new().italic().paint(i.to_string()));
      a += 1;
    }
    println!("");
    println!("{}", Style::new().bold().paint("Would you like to use an item? [y/n]"));
    let choice: String = read!();
    let choice = choice.to_lowercase();
    if choice == "yes" || choice == "y" {
      println!("{}", Style::new().bold().paint("Which item?"));
      let mut item_choice: usize = read!();
      item_choice -= 1;
      let item_use_input: String = player.player_inv[item_choice].clone();
      player = item_use(player, item_use_input);
      used_item = true;
      return (player, used_item)
    }
    else {
      return (player, used_item)
    }
  }
  else {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    println!("There's nothing here!");
    println!("");
    return (player, used_item)
  }
}

fn item_use(mut player: Player, item: String) -> Player {
  if item == "Health Potion" {
    player = hp_potion(player);
  }
  if item == "Medium Health Potion" {
    player = med_hp_potion(player);
  }
  if item == "Large Health Potion" {
    player = large_hp_potion(player);
  }
  if item == "Mana Potion" {
    player = mp_potion(player);
  }
  if item == "Medium Mana Potion" {
    player = med_mp_potion(player);
  }
  if item == "Large Health Potion" {
    player = large_mp_potion(player);
  }
  if item == "Granny Apple" {
    player = granny_apple(player);
  }
  if item == "Leftover Steak" {
    player = leftover_steak(player); 
  }
  return player;
}

fn tutorial_fight(opponent: String, mut player: Player) -> (Player, bool) {
  let mut used_item = false;
  let mut turn = "player".to_string();
  if opponent == "mutant_rat" {
    let mut mutant_rat = Opponent {
      hp: 25,
      mp: 0,
      min_atk: 4,
      max_atk: 7,
      magic_atks: vec!("".to_string()),
      inv: vec!("".to_string())
    };
    loop {
      if turn == "opponent" {
        let mut random_atk = rand::thread_rng();
        let rat_atk: i32 = random_atk.gen_range(mutant_rat.min_atk..(mutant_rat.max_atk + 1));
        println!("{}", Style::new().bold().paint(format!("The mutant rat attacks you dealing {} damage", rat_atk)));
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp - rat_atk; 
        if player.player_hp < 0 || player.player_hp == 0 {
          println!("{}", Style::new().bold().paint("You have 0hp/50hp"));
          println!("{}", Style::new().bold().paint("You died!"));
          return (player, false);
        }
        println!("You have {}hp/{}hp", player.player_hp, player.player_max_hp);
        println!("");
        turn = "player".to_string();
      }
      if turn == "player" {
        let choice: i32 = Input::with_theme(&ColorfulTheme::default())
      .with_prompt("Do you want to:
1) Attack
2) Use an Item
3) Run Away
")
      .interact_text()
      .unwrap();
        println!("");
        if choice == 1 {
          let mut random_atk = rand::thread_rng();
          let player_atk: i32 = random_atk.gen_range(player.player_min_atk..(player.player_max_atk + 1));
          println!("\x1B[2J\x1B[1;1H"); //clears the screen
          println!("{}", Style::new().bold().paint(format!("You attack the mutant rat dealing {} damage", player_atk)));
          mutant_rat.hp = mutant_rat.hp - player_atk;
          if mutant_rat.hp < 0 || mutant_rat.hp == 0 {
            println!("{}", Style::new().bold().paint("The mutant rat has 0hp/25hp"));
            println!("{}", Style::new().bold().paint("You win!"));
            return (player, true);
          }
          println!("{}", Style::new().bold().paint(format!("The mutant rat has {}hp/25hp", mutant_rat.hp)));
          println!("");
          turn = "opponent".to_string();
        }
        if choice == 2 {
          (player, used_item) = check_inv_fight(player);
          if used_item == true {
            turn = "opponent".to_string();
          }  
        }
        if choice == 3 {
          println!("{}", Style::new().bold().paint("You can't run away from this fight!"));
        }
      }
    }
  }
  return (player, true);
}

fn fight(opponent: String, mut player: Player) -> (Player, bool) {
  let mut used_item = false;
  let mut opp_max_hp = 0;
  let mut turn = "player".to_string();
  let mut attacker = "";
  let mut opponent_stats = Opponent {
    hp: 0,
    mp: 0,
    min_atk: 0,
    max_atk: 0,
    magic_atks: vec!("".to_string()),
    inv: vec!("".to_string())
  };
  if opponent == "mutant_rat" {
    opponent_stats = Opponent {
      hp: 25,
      mp: 0,
      min_atk: 4,
      max_atk: 7,
      magic_atks: vec!("".to_string()),
      inv: vec!("".to_string())
    };
    opp_max_hp = opponent_stats.hp;
    attacker = "mutant rat";
  } 
    loop {
      if turn == "opponent" {
        let mut random_atk = rand::thread_rng();
        let opp_atk: i32 = random_atk.gen_range(opponent_stats.min_atk..(opponent_stats.max_atk + 1));
        println!("{}", Style::new().bold().paint(format!("The {} attacks you dealing {} damage",attacker, opp_atk)));
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp - opp_atk; 
        if player.player_hp < 0 || player.player_hp == 0 {
          println!("{}", Style::new().bold().paint(format!("You have 0hp/{}hp", player.player_max_hp)));
          println!("{}", Style::new().bold().paint("You died!"));
          return (player, false)
        }
        println!("{}", Style::new().bold().paint(format!("You have {}hp/{}hp", player.player_hp, player.player_max_hp)));
        println!("");
        turn = "player".to_string();
      }
      if turn == "player" {
        let choice: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to:
1) Attack
2) Use an Item
3) Run Away
")
        .interact_text()
        .unwrap();

        println!("");
        if choice == 1 {
          let mut random_atk = rand::thread_rng();
          let player_atk: i32 = random_atk.gen_range(player.player_min_atk..(player.player_max_atk + 1));
          println!("\x1B[2J\x1B[1;1H"); //clears the screen
          println!("{}", Style::new().bold().paint(format!("You attack the {} dealing {} damage",attacker , player_atk)));
          opponent_stats.hp = opponent_stats.hp - player_atk;
          if opponent_stats.hp < 0 || opponent_stats.hp == 0 {
            println!("{}", Style::new().bold().paint(format!("The {} has 0hp/{}hp", attacker, opp_max_hp)));
            println!("{}", Style::new().bold().paint("You win!"));
            return (player, true);
          }
          println!("{}", Style::new().bold().paint(format!("The {} has {}hp/{}hp", attacker, opponent_stats.hp, opp_max_hp)));
          println!("");
          turn = "opponent".to_string();
        }
        if choice == 2 {
          (player, used_item) = check_inv_fight(player);
          if used_item == true {
            turn = "opponent".to_string();
          }  
        }
        if choice == 3 {
          if player.player_max_atk > opponent_stats.max_atk {
            println!("{}", Style::new().bold().paint("You successfully ran away!"));
            return (player, true)
          }
          else {
            let strength_difference = opponent_stats.max_atk - player.player_max_atk;
            let mut rng = rand::thread_rng();
            let escape_chance: i32 = rng.gen_range(1..(strength_difference + 1));
            if escape_chance > strength_difference / 2 {
              println!("{}", Style::new().bold().paint("You successfully ran away!"));
              return (player, true)
            }
            else {
              println!("{}", Style::new().bold().paint("You failed to run away!"));
              turn = "opponent".to_string();
            }
          }
        }
      }
    }
}

fn room_change(chosen_rooms:Rooms, items:Vec<String>, mut player:Player, mut rooms_data:RoomsData, has_items: bool) -> (Player, RoomsData, bool) {
  let mut direction:String = "".to_string();
  let north_room = chosen_rooms.north_room;
  let south_room = chosen_rooms.south_room;
  let west_room = chosen_rooms.west_room;
  let east_room = chosen_rooms.east_room;
  let mut items_looted = false;
  let north_room_prompt = Style::new().bold().paint(format!("[North]: {}", north_room));
  let south_room_prompt = Style::new().bold().paint(format!("[South]: {}", south_room));
  let west_room_prompt = Style::new().bold().paint(format!("[West]: {}", west_room));
  let east_room_prompt = Style::new().bold().paint(format!("[East]: {}", east_room));
  if has_items == true  {
    println!("Would you like to: 
1) loot the room? 
or 
2) leave?");
    let choice: i32 = read!();
    println!("");

    if choice == 1 {
      let mut a = 1;
      println!("You Looted: ");
      for i in items.clone() {
        println!("{}: {}", a, i);
        a += 1;
        player.player_inv.push(i.to_string());
      }
      items_looted = true;
      println!("");
    }
  }
  loop {  
    if north_room != "" {println!("{}", north_room_prompt);}
    if south_room != "" {println!("{}", south_room_prompt);}
    if west_room != "" {println!("{}", west_room_prompt);}
    if east_room != "" {println!("{}", east_room_prompt);}
    
    direction = Input::with_theme(&ColorfulTheme::default())
      .with_prompt("
[Check]: Check Inventory

Enter a direction or Check
")
      .interact_text()
      .unwrap();
    direction = direction.to_lowercase();
    //checks if the direction chosen is locked, blocked, or empty
    if direction == "Check" || direction == "check" || direction == "c" || direction == "C" {
      check_inv(player.clone());
    }
    if direction == "north" || direction == "n" {
      if north_room != "" && north_room != "locked" && north_room != "Locked" && north_room != "blocked" && north_room != "Blocked"{
        rooms_data.room_title = north_room.clone();
        println!("");
        return (player, rooms_data, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "south" || direction == "s" {
      if south_room != "" && south_room != "locked" && south_room != "Locked" && south_room != "blocked" && south_room != "Blocked"{
        rooms_data.room_title = south_room.clone();
        println!("");
        return (player, rooms_data, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "west" || direction == "w" {
      if west_room != "" && west_room != "locked" && west_room != "Locked" && west_room != "blocked" && west_room != "Blocked"{
        rooms_data.room_title = west_room.clone();
        println!("");
        return (player, rooms_data, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "east" || direction == "e" {
      if east_room != "" && east_room != "locked" && east_room != "Locked" && east_room != "blocked" && east_room != "Blocked"{
        rooms_data.room_title = east_room.clone();
        println!("");
        return (player, rooms_data, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
  }
}

fn entrance(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Entrance"));
  println!("");
  let items = vec!("".to_string());
  let enterance_rooms = Rooms {
      north_room: "Follow".to_string(),
      south_room: "Locked".to_string(),
      west_room: "West Hallway".to_string(),
      east_room: "East Hallway".to_string()
    };
    (player, rooms_data, _) = room_change(enterance_rooms, items, player.clone(), rooms_data, false);
    return (player, rooms_data);
}

fn entrance_scene_two(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Entrance"));
  println!("");
  let items = vec!("".to_string());
  let enterance_rooms = Rooms {
      north_room: "Operations Room".to_string(),
      south_room: "Locked".to_string(),
      west_room: "West Hallway".to_string(),
      east_room: "East Hallway".to_string()
  };
  (player, rooms_data, _) = room_change(enterance_rooms, items, player.clone(), rooms_data, false);
  return (player, rooms_data);
}

fn east_hallway(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: East Hallway"));
  println!("");
  let items = vec!("".to_string());
  let east_hallway_rooms = Rooms {
      north_room: "Library".to_string(),
      south_room: "Kitchen".to_string(),
      west_room: "Return to Entrance".to_string(),
      east_room: "Locked".to_string()
  };
  (player, rooms_data, _) = room_change(east_hallway_rooms, items, player.clone(), rooms_data, false);
  return (player, rooms_data);
}

fn west_hallway(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: West Hallway"));
  println!("");
  let items = vec!("".to_string());
  let west_hallway_rooms = Rooms {
      north_room: "Supply Closet".to_string(),
      south_room: "Locked".to_string(),
      west_room: "Locked".to_string(),
      east_room: "Return to Entrance".to_string()
  };
  (player, rooms_data, _) = room_change(west_hallway_rooms, items, player.clone(), rooms_data, false);
  return (player, rooms_data);
}

fn supply_closet(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Supply Closet"));
  println!("");
  let supply_closet_rooms = Rooms {
        north_room: "".to_string(),
        south_room: "Return to West Hallway".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
  };
  if rooms_data.are_items_in_closet == true {
    let items = vec!["Rope".to_string(), "Gemstone".to_string()];
    (player, rooms_data, rooms_data.picked_up_item_from_closet) = room_change(supply_closet_rooms, items, player.clone(), rooms_data, true);
  }
  else {
    let items = vec!("".to_string());
    (player, rooms_data, rooms_data.picked_up_item_from_closet) = room_change(supply_closet_rooms, items, player.clone(), rooms_data, false);
  }
  return (player.clone(), rooms_data);
}

fn kitchen(mut player:Player, mut rooms_data:RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Kitchen"));
  println!("");
  let kitchen_rooms = Rooms {
        north_room: "Return to East Hallway".to_string(),
        south_room: "".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
    };
    if rooms_data.are_items_in_kitchen == true {
      let items = vec!["Granny Apple".to_string(), "Leftover Steak".to_string()];
      (player, rooms_data, rooms_data.picked_up_item_from_kitchen) = room_change(kitchen_rooms, items, player.clone(), rooms_data, true);
    }
    else {
      let items = vec!("".to_string());
      (player, rooms_data, rooms_data.picked_up_item_from_kitchen) = room_change(kitchen_rooms, items, player.clone(), rooms_data, false);
    }
  return (player.clone(), rooms_data);
}

fn library(mut player:Player, mut rooms_data:RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Library"));
  println!("");
  let library_rooms = Rooms {
        north_room: "".to_string(),
        south_room: "Return to East Hallway".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
  };
  if rooms_data.are_items_in_library == true {
    let items = vec!["Health Potion".to_string(), "Mana Potion".to_string()];
    (player, rooms_data, rooms_data.picked_up_item_from_library) = room_change(library_rooms, items, player.clone(), rooms_data, true);
  }
  else {
    let items = vec!("".to_string());
    (player, rooms_data, rooms_data.picked_up_item_from_library) = room_change(library_rooms, items, player.clone(), rooms_data, false);
  }
  return (player.clone(), rooms_data);
}
//finish this
fn operations_room(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData) {
  println!("{}", Style::new().underline().paint("Room: Operations Room"));
  println!("");
  let items = vec!("".to_string());
  let operation_room_rooms = Rooms {
    north_room: "".to_string(),
    south_room: "Entrance".to_string(),
    west_room: "".to_string(),
    east_room: "".to_string()
  };
  (player, rooms_data, _) = room_change(operation_room_rooms, items, player.clone(), rooms_data, false);
  return(player.clone(), rooms_data);
}

fn rooms_loop(mut player:Player, mut rooms_data:RoomsData) -> (Player, RoomsData) {
  loop {
		if rooms_data.room_title == "Operations Room" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			(player, rooms_data) = operations_room(player.clone(), rooms_data);
		}

		if rooms_data.room_title == "West Hallway" || rooms_data.room_title == "Return to West Hallway" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			(player, rooms_data) = west_hallway(player.clone(), rooms_data);
		}

		if rooms_data.room_title == "Supply Closet" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			if rooms_data.picked_up_item_from_closet == false {
				rooms_data.are_items_in_closet = true;
			}
			(player, rooms_data) = supply_closet(player.clone(), rooms_data);
			if rooms_data.picked_up_item_from_closet == true {
				rooms_data.are_items_in_closet = false;
			}
		}

		if rooms_data.room_title == "East Hallway" || rooms_data.room_title == "Return to East Hallway" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			(player, rooms_data) = east_hallway(player.clone(), rooms_data);
		}

		if rooms_data.room_title == "Entrance" || rooms_data.room_title == "Return to Entrance" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			(player, rooms_data) = entrance_scene_two(player.clone(), rooms_data);
		}

		if rooms_data.room_title == "Kitchen" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			if rooms_data.picked_up_item_from_kitchen == false {
				rooms_data.are_items_in_kitchen = true;
			}
			(player, rooms_data) = kitchen(player.clone(), rooms_data);
			if rooms_data.picked_up_item_from_kitchen == true {
				rooms_data.are_items_in_kitchen = false;
			}
		}
	
		if rooms_data.room_title == "Library" {
			println!("\x1B[2J\x1B[1;1H"); //clears the screen
			if rooms_data.picked_up_item_from_library == false {
				rooms_data.are_items_in_library = true;
			}
			(player, rooms_data) = library(player.clone(), rooms_data);
			if rooms_data.picked_up_item_from_library == true {
				rooms_data.are_items_in_library = false;
			}
		}
	}
}

fn scene_0(mut player:Player) -> Player {
  let player_name: String = Input::with_theme(&ColorfulTheme::default())
      .with_prompt("What is your name?")
      .interact_text()
      .unwrap();
  player.player_name = player_name;
  println!("{}", Style::new().bold().paint("Press enter to continue"));
  
  let mut stdin = io::stdin();
  // Read a single byte and discard
  let _ = stdin.read(&mut [0u8]).unwrap();
  
  println!("\x1B[2J\x1B[1;1H");
  let response: String = Input::with_theme(&ColorfulTheme::default())
      .with_prompt(format!(r#""You were knocked out cold {}, are you okay?""#, player.player_name))
      .interact_text()
      .unwrap();

  let mut response: String = response.to_lowercase();
  let mut print_text = 0;

  while print_text == 0 {
    if response == "y" || response == "yes" {
      println!("{}", Style::new().bold().paint(r#""Alright, I'll help you up.""#));
      print_text = 1;
      break
    }
    else if response == "n" || response =="no" {
      println!("{}", Style::new().bold().paint(r#""Are you sure? Do you need help getting up?""#));
      print_text = 2;
      break
    }
    if response != "y" && response != "yes" && response != "n" && response != "no" {
      println!("{}", Style::new().bold().paint(r#""What?""#));
      print_text = 0;
    }
    if print_text == 0 {
      response = read!();
    }
  }

  if print_text == 1 {
    println!("");
    println!("{}", Style::new().bold().paint("You stand up and look around. 
it seems like you're in an underground cave."));
    println!("");
  }
  else {
    println!("");
    println!("{}", Style::new().bold().paint("He helps you up and you look around.
You seem to be in a cave"));
    println!("");
  }
  return player;
}

fn scene_1(mut player:Player, mut rooms_data: RoomsData) -> (Player, RoomsData, i32) {
  (player, rooms_data) = entrance(player.clone(), rooms_data);
  // this loops has to be written here because of the if room_title == "Follow" statement, this is because it returns to go to the next scene
  loop {
      if rooms_data.room_title == "Follow" {
        return(player.clone(), rooms_data, 2);
      }

      if rooms_data.room_title == "Operations Room" {
        (player, rooms_data) = operations_room(player.clone(), rooms_data);
      }

    if rooms_data.room_title == "West Hallway" || rooms_data.room_title == "Return to West Hallway" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      (player, rooms_data) = west_hallway(player.clone(), rooms_data);
    }

    if rooms_data.room_title == "Supply Closet" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if rooms_data.picked_up_item_from_closet == false {
        rooms_data.are_items_in_closet = true;
      }
      (player, rooms_data) = supply_closet(player.clone(), rooms_data);
      if rooms_data.picked_up_item_from_closet == true {
        rooms_data.are_items_in_closet = false;
      }
    }
  
    if rooms_data.room_title == "East Hallway" || rooms_data.room_title == "Return to East Hallway" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      (player, rooms_data) = east_hallway(player.clone(), rooms_data);
    }

    if rooms_data.room_title == "Return to Entrance" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      (player, rooms_data) = entrance(player.clone(), rooms_data);
    }

    if rooms_data.room_title == "Kitchen" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if rooms_data.picked_up_item_from_kitchen == false {
        rooms_data.are_items_in_kitchen = true;
      }
      (player, rooms_data) = kitchen(player.clone(), rooms_data);
      if rooms_data.picked_up_item_from_kitchen == true {
        rooms_data.are_items_in_kitchen = false;
      }
    }
   
    if rooms_data.room_title == "Library" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if rooms_data.picked_up_item_from_library == false {
        rooms_data.are_items_in_library = true;
      }
      (player, rooms_data) = library(player.clone(), rooms_data);
      if rooms_data.picked_up_item_from_library == true {
        rooms_data.are_items_in_library = false;
      }
    }
  }
}

fn scene_2(mut player:Player, rooms_data:RoomsData) -> (Player, RoomsData, i32) {
  let mut did_win = false;
  (player, did_win) = tutorial_fight("mutant_rat".to_string(), player.clone());
  if did_win == false {
    return (player.clone(), rooms_data, 2)
  }
  println!("{}", Style::new().bold().paint(r#""Phew, I was almost worried we were gonna get hurt there.""#));
  println!("{}", Style::new().bold().paint(r#""Here, one second, I'm going to turn on the operations screen""#));
  println!("{}", Style::new().bold().paint("He turns on the screen and it says: "));
  println!("|===================================|
|                                   |
|            Operations:            |
|     ERROR: Power not connected.   |
|       Running on backup power.    |
|          Please reconnect.        |
|                                   |
|     ERROR: No storage drive.      |
|           Please insert.          |
|                                   |
|                                   |
|                                   |
|                                   |
|                                   |
|===================================|");
return (player, rooms_data, 3)
}

fn scene_3(mut player: Player, mut rooms_data:RoomsData) -> (Player, RoomsData, i32) {
  rooms_data.room_title = "Operations Room".to_string();
  (player, rooms_data) = rooms_loop(player, rooms_data);
  return (player, rooms_data, 4);
}