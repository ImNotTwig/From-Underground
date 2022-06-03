use std::io;
use std::io::prelude::*;
use text_io::read;
use rand::Rng;
mod potions;
use crate::potions::hp_potion;
use crate::potions::med_hp_potion;
use crate::potions::large_hp_potion;
use crate::potions::mp_potion;
use crate::potions::med_mp_potion;
use crate::potions::large_mp_potion;
  //I want a title screen [x]
  //I want to start the story  [x]
  //I want multiple rooms to be able to go between
  //I want items to be a prominent part of the game 
  //I want multiple routes to go through to finish the game meaning I want to be able to have unique playthoughs
  //I want an inventory system [x]
  //I want a fighting system 
  //I want some kind of mana/health system [x]
#[derive(Debug)]
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
  magic_atks: Vec<String>
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
  player.player_inv.remove(0);
  title_screen();
  loop {
    if level == 0 {
      player = scene_0(player);
      level = 1
    }
    if level == 1 {
      (player, level) = scene_1(player);
    }
    if level == 2 {
      (player, level) = scene_2(player);
      return
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
      println!("{}: {}", a, i);
      a += 1;
    }
    println!("");
    println!("Would you like to use an item? [y/n]");
    let choice: String = read!();
    let choice = choice.to_lowercase();
    if choice == "yes" || choice == "y" {
      println!("Which item?");
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
  let used_item = false;
  if player.player_inv.is_empty() == false {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    let mut a = 1;
    for i in player.player_inv.clone() {
      println!("{}: {}", a, i);
      a += 1;
    }
    println!("");
    println!("Would you like to use an item? [y/n]");
    let choice: String = read!();
    let choice = choice.to_lowercase();
    if choice == "yes" || choice == "y" {
      println!("Which item?");
      let mut item_choice: usize = read!();
      item_choice -= 1;
      let item_use_input: String = player.player_inv[item_choice].clone();
      player = item_use(player, item_use_input);
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
      magic_atks: vec!("".to_string())
    };
    loop {
      if turn == "opponent" {
        let mut random_atk = rand::thread_rng();
        let rat_atk: i32 = random_atk.gen_range(mutant_rat.min_atk..(mutant_rat.max_atk + 1));
        println!("The mutant rat attacks you dealing {} damage", rat_atk);
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp - rat_atk; 
        if player.player_hp < 0 || player.player_hp == 0 {
          println!("You have 0hp/50hp");
          println!("You died!");
          return (player, false);
        }
        println!("You have {}hp/{}hp", player.player_hp, player.player_max_hp);
        println!("");
        turn = "player".to_string();
      }
      if turn == "player" {
        println!("Do you want to: 
1) Attack
2) Use an Item
3) Run Away");
        let choice: i32 = read!();
        println!("");
        if choice == 1 {
          let mut random_atk = rand::thread_rng();
          let player_atk: i32 = random_atk.gen_range(player.player_min_atk..(player.player_max_atk + 1));
          println!("You attack the mutant rat dealing {} damage", player_atk);
          mutant_rat.hp = mutant_rat.hp - player_atk;
          if mutant_rat.hp < 0 || mutant_rat.hp == 0 {
            println!("The mutant rat has 0hp/25hp");
            println!("You win!");
            return (player, true);
          }
          println!("The mutant rat has {}hp/25hp", mutant_rat.hp);
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
          println!("You can't run away from this fight!");
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
    magic_atks: vec!("".to_string())
  };
  if opponent == "mutant_rat" {
    opponent_stats = Opponent {
      hp: 25,
      mp: 0,
      min_atk: 4,
      max_atk: 7,
      magic_atks: vec!("".to_string())
    };
    opp_max_hp = opponent_stats.hp;
    attacker = "mutant rat";
  } 
    loop {
      if turn == "opponent" {
        let mut random_atk = rand::thread_rng();
        let opp_atk: i32 = random_atk.gen_range(opponent_stats.min_atk..(opponent_stats.max_atk + 1));
        println!("The {} attacks you dealing {} damage",attacker, opp_atk);
        player.prev_player_hp = player.player_hp;
        player.player_hp = player.player_hp - opp_atk; 
        if player.player_hp < 0 || player.player_hp == 0 {
          println!("You have 0hp/{}hp", player.player_max_hp);
          println!("You died!");
          return (player, false)
        }
        println!("You have {}hp/{}hp", player.player_hp, player.player_max_hp);
        println!("");
        turn = "player".to_string();
      }
      if turn == "player" {
        println!("Do you want to: 
1) Attack
2) Use an Item
3) Run Away");
        let choice: i32 = read!();
        println!("");
        if choice == 1 {
          let mut random_atk = rand::thread_rng();
          let player_atk: i32 = random_atk.gen_range(player.player_min_atk..(player.player_max_atk + 1));
          println!("You attack the {} dealing {} damage",attacker , player_atk);
          opponent_stats.hp = opponent_stats.hp - player_atk;
          if opponent_stats.hp < 0 || opponent_stats.hp == 0 {
            println!("The {} has 0hp/{}hp", attacker, opp_max_hp);
            println!("You win!");
            return (player, true);
          }
          println!("The {} has {}hp/{}hp", attacker, opponent_stats.hp, opp_max_hp);
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
            println!("You successfully ran away!");
            return (player, true)
          }
          else {
            let strength_difference = opponent_stats.max_atk - player.player_max_atk;
            let mut rng = rand::thread_rng();
            let escape_chance: i32 = rng.gen_range(1..(strength_difference + 1));
            if escape_chance > strength_difference / 2 {
              println!("You successfully ran away!");
              return (player, true)
            }
            else {
              println!("You failed to run away!");
              turn = "opponent".to_string();
            }
          }
        }
      }
    }
}

fn room_change_w_items(chosen_rooms:Rooms, items:Vec<String>, mut player:Player) -> (Player, String, bool) {
  let mut direction:String = "".to_string();
  let mut room_title: String = "".to_string(); 
  let north_room = chosen_rooms.north_room;
  let south_room = chosen_rooms.south_room;
  let west_room = chosen_rooms.west_room;
  let east_room = chosen_rooms.east_room;
  let mut items_looted = false;
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
  loop {  
    if north_room != "" {println!("[North]: {}", north_room);}
    if south_room != "" {println!("[South]: {}", south_room);}
    if west_room != "" {println!("[West]: {}", west_room);}
    if east_room != "" {println!("[East]: {}", east_room);}

    println!("[Check]: Check Inventory");
    println!("");
    println!("Enter a direction or Check:");
    
    direction = read!();
    direction = direction.to_lowercase();
    //checks if the direction chosen is locked, blocked, or empty
    if direction == "Check" || direction == "check" {
      check_inv(player.clone());
    }
    if direction == "north" || direction == "n" {
      if north_room != "" && north_room != "locked" && north_room != "Locked" && north_room != "blocked" && north_room != "Blocked"{
        room_title = north_room.clone();
        println!("");
        return (player, room_title, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "south" || direction == "s" {
      if south_room != "" && south_room != "locked" && south_room != "Locked" && south_room != "blocked" && south_room != "Blocked"{
        room_title = south_room.clone();
        println!("");
        return (player, room_title, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "west" || direction == "w" {
      if west_room != "" && west_room != "locked" && west_room != "Locked" && west_room != "blocked" && west_room != "Blocked"{
        room_title = west_room.clone();
        println!("");
        return (player, room_title, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "east" || direction == "e" {
      if east_room != "" && east_room != "locked" && east_room != "Locked" && east_room != "blocked" && east_room != "Blocked"{
        room_title = east_room.clone();
        println!("");
        return (player, room_title, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
  }
}

fn room_change(chosen_rooms:Rooms, mut player:Player) -> String {
  let mut direction:String = "".to_string();
  let mut room_title: String = "".to_string(); 
  let north_room = chosen_rooms.north_room;
  let south_room = chosen_rooms.south_room;
  let west_room = chosen_rooms.west_room;
  let east_room = chosen_rooms.east_room;
  
  loop {  
    if north_room != "" {println!("[North]: {}", north_room);}
    if south_room != "" {println!("[South]: {}", south_room);}
    if west_room != "" {println!("[West]: {}", west_room);}
    if east_room != "" {println!("[East]: {}", east_room);}
    println!("");
    println!("[Check]: Check Inventory");
    println!("");
    println!("Enter a direction or check:");
    
    direction = read!();
    direction = direction.to_lowercase();
    //checks if the direction chosen is locked, blocked, or empty
    if direction == "Check" || direction == "check" {
      check_inv(player.clone());
    }
    if direction == "north" || direction == "n" {
      if north_room != "" && north_room != "locked" && north_room != "Locked" && north_room != "blocked" && north_room != "Blocked"{
        room_title = north_room.clone();
        println!("");
        return room_title
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "south" || direction == "s" {
      if south_room != "" && south_room != "locked" && south_room != "Locked" && south_room != "blocked" && south_room != "Blocked"{
        room_title = south_room.clone();
        println!("");
        return room_title
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "west" || direction == "w" {
      if west_room != "" && west_room != "locked" && west_room != "Locked" && west_room != "blocked" && west_room != "Blocked"{
        room_title = west_room.clone();
        println!("");
        return room_title
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
    
    if direction == "east" || direction == "e" {
      if east_room != "" && east_room != "locked" && east_room != "Locked" && east_room != "blocked" && east_room != "Blocked"{
        room_title = east_room.clone();
        println!("");
        return room_title
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
  }
}

fn title_screen() {
  println!("\x1B[2J\x1B[1;1H"); //clears the screen
  println!("Welcome to: From Underground, a text adventure");
}

fn entrance(mut player:Player) -> String {
  println!("Room: Entrance");
  println!("");
  let enterance_rooms = Rooms {
      north_room: "Follow".to_string(),
      south_room: "Locked".to_string(),
      west_room: "West Hallway".to_string(),
      east_room: "East Hallway".to_string()
    };
    let room_title = room_change(enterance_rooms, player.clone());
    return room_title;
}

fn east_hallway(mut player:Player) -> String {
  println!("Room: East Hallway");
  println!("");
  let east_hallway_rooms = Rooms {
      north_room: "Library".to_string(),
      south_room: "Kitchen".to_string(),
      west_room: "Return to Entrance".to_string(),
      east_room: "Locked".to_string()
  };
  let room_title = room_change(east_hallway_rooms, player.clone());
  return room_title;
}

fn west_hallway(mut player:Player) -> String {
  println!("Room: West Hallway");
  println!("");
  let west_hallway_rooms = Rooms {
      north_room: "Supply Closet".to_string(),
      south_room: "Locked".to_string(),
      west_room: "Locked".to_string(),
      east_room: "Return to Entrance".to_string()
  };
  let room_title = room_change(west_hallway_rooms, player.clone());
  return room_title;
}

fn supply_closet(mut items_in_closet: bool, mut player:Player) -> (Player, String, bool) {
  println!("Room: Supply Closet");
  println!("");
  let mut room_title = "".to_string();
  let supply_closet_rooms = Rooms {
        north_room: "".to_string(),
        south_room: "Return to West Hallway".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
    };
    if items_in_closet == true {
      let items = vec!["Rope".to_string(), "Gemstone".to_string()];
      (player, room_title, items_in_closet) = room_change_w_items(supply_closet_rooms, items, player.clone());
    }
    else {
      room_title = room_change(supply_closet_rooms, player.clone());
    }
  return (player.clone(), room_title, items_in_closet);
}

fn kitchen(mut items_in_kitchen: bool, mut player:Player) -> (Player, String, bool) {
  println!("Room: Kitchen");
  println!("");
  let mut room_title = "".to_string();
  let kitchen_rooms = Rooms {
        north_room: "Return to East Hallway".to_string(),
        south_room: "".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
    };
    if items_in_kitchen == true {
      let items = vec!["Granny Apple".to_string(), "Leftover Steak".to_string()];
      (player, room_title, items_in_kitchen) = room_change_w_items(kitchen_rooms, items, player.clone());
    }
    else {
      room_title = room_change(kitchen_rooms, player.clone());
    }
  return (player.clone(), room_title, items_in_kitchen);
}

fn library(mut items_in_library: bool, mut player:Player) -> (Player, String, bool) {
  println!("Room: Library");
  println!("");
  let mut room_title = "".to_string();
  let kitchen_rooms = Rooms {
        north_room: "".to_string(),
        south_room: "Return to East Hallway".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
    };
    if items_in_library == true {
      let items = vec!["Health Potion".to_string(), "Mana Potion".to_string()];
      (player, room_title, items_in_library) = room_change_w_items(kitchen_rooms, items, player.clone());
    }
    else {
      room_title = room_change(kitchen_rooms, player.clone());
    }
  return (player.clone(), room_title, items_in_library);
}

fn scene_0(mut player:Player) -> Player {
  println!("What is your name?");
  let player_name: String = read!();
  player.player_name = player_name;
  println!("Hello {}, press enter to continue", player.player_name);
  
  let mut stdin = io::stdin();
  // Read a single byte and discard
  let _ = stdin.read(&mut [0u8]).unwrap();
  
  println!("\x1B[2J\x1B[1;1H");
  
  println!(r#""You were knocked out cold, dude."
"{} Are you okay?""#, player.player_name);
  let response: String = read!();
  
  let mut response: String = response.to_lowercase();
  let mut print_text = 0;

  while print_text == 0 {
    if response == "y" || response == "yes" {
      println!(r#""Alright, I'll help you up.""#);
      print_text = 1;
      break
    }
    else if response == "n" || response =="no" {
      println!(r#""Are you sure? Do you need help getting up?""#);
      print_text = 2;
      break
    }
    if response != "y" && response != "yes" && response != "n" && response != "no" {
      println!(r#""What?""#);
      print_text = 0;
    }
    if print_text == 0 {
      response = read!();
    }
  }

  if print_text == 1 {
    println!("");
    println!("You stand up and look around. 
it seems like you're in an underground cave.");
    println!("");
  }
  else {
    println!("");
    println!("He helps you up and you look around.
You seem to be in a cave");
    println!("");
  }
  return player;
}

fn scene_1(mut player:Player) -> (Player, i32) {
  println!(r#""Follow me I'll show you to operations room.""#);
  println!("");
  let mut room_title = entrance(player.clone());
  let mut picked_up_item_from_closet = false;
  let mut closet_room_title: String = "".to_string();
  let mut are_items_in_closet = true;
  let mut kitchen_room_title: String = "".to_string();
  let mut picked_up_item_from_kitchen = false;
  let mut are_items_in_kitchen = true;
  let mut library_room_title: String = "".to_string();
  let mut picked_up_item_from_library = false;
  let mut are_items_in_library = true;
  loop {
    if room_title == "Follow" {
      return (player.clone(), 2);
    }

    if room_title == "West Hallway" || room_title == "Return to West Hallway" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      room_title = west_hallway(player.clone());
    }
    if room_title == "Supply Closet" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if picked_up_item_from_closet == false {
        are_items_in_closet = true;
      }

      (player, closet_room_title, picked_up_item_from_closet) = supply_closet(are_items_in_closet, player.clone());
      
      if closet_room_title == "Return to West Hallway" {
        room_title = "Return to West Hallway".to_string();
      }
      if picked_up_item_from_closet == true {
        are_items_in_closet = false;
      }
      if picked_up_item_from_closet == false {
        are_items_in_closet = true;
      }
    }
  
    if room_title == "East Hallway" || room_title == "Return to East Hallway" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      room_title = east_hallway(player.clone());
    }
    if room_title == "Return to Entrance" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      room_title = entrance(player.clone());
    }
    if room_title == "Kitchen" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if picked_up_item_from_kitchen == false {
        are_items_in_kitchen = true;
      }

      (player, kitchen_room_title, picked_up_item_from_kitchen) = kitchen(are_items_in_kitchen, player.clone());
      
      if kitchen_room_title == "Return to East Hallway" {
        room_title = "Return to East Hallway".to_string();
      }
      if picked_up_item_from_kitchen == true {
        are_items_in_kitchen = false;
      }
      if picked_up_item_from_kitchen == false {
        are_items_in_kitchen = true;
      }
    }
    if room_title == "Library" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if picked_up_item_from_library == false {
        are_items_in_library = true;
      }

      (player, library_room_title, picked_up_item_from_library) = library(are_items_in_library, player.clone());
      
      if library_room_title == "Return to East Hallway" {
        room_title = "Return to East Hallway".to_string();
      }
      if picked_up_item_from_library == true {
        are_items_in_library = false;
      }
      if picked_up_item_from_library == false {
        are_items_in_library = true;
      }
    }
  }
}

fn scene_2(mut player:Player) -> (Player, i32) {
  let mut win_or_lose = false;
  println!("\x1B[2J\x1B[1;1H"); //clears the screen
  println!(r#""Here we are, the operations room.""#);
  println!("You hear the door behind you close.");
  println!(r#""What is that?""#);
  println!("He points at a weird animal coming out of the darkness towards the other side of the room.");
  println!("It charges at you");
  println!("");
  (player, win_or_lose) = tutorial_fight("mutant_rat".to_string(), player.clone());
  if win_or_lose == false {
    return (player.clone(), 2)
  }
  println!("\x1B[2J\x1B[1;1H"); //clears the screen
  println!(r#""Phew, I was almost worried we were gonna get hurt there.""#);
  println!(r#""Here, one second, I'm going to turn on the operations screen""#);
  println!("He turns on the screen and it says: ");
println!("=====================================
=                                   =
=                                   =
=      Thank you for playing!       =
=       This is only a demo,        =
=    but I am still developing it.  =
=    I hope you enjoyed it though.  =
=                                   =
=                                   =
=                                   =
=                                   =
=                                   =
=                                   =
=                                   =
=====================================");
  return (player.clone(), 3)
}