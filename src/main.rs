use std::io;
use std::io::prelude::*;
use text_io::read;
mod potions;
use crate::potions::hp_potion;
use crate::potions::med_hp_potion;
use crate::potions::large_hp_potion;
  //I want a title screen
  //I want to start the story 
  //I want multiple rooms to be able to go between
  //I want items to be a prominent part of the game 
  //I want multiple routes to go through to finish the game meaning I want to be able to have unique playthoughs
  //I want an inventory system
  //I want a fighting system
  //I want some kind of mana/health/stamina system
#[derive(Debug)]
struct Rooms {
    north_room: String,
    south_room: String,
    west_room: String,
    east_room: String
}

fn main() {
  let mut are_items_in_closet = false;
  let mut player_inv = vec!("".to_string());
  //player_inv.push("Health Potion".to_string())
  player_inv.remove(0);
  let mut player_hp = 100;
  let mut player_max_hp  = 100;
  let mut prev_player_hp  = 100;
  
  let mut player_mp = 100;
  let mut player_max_mp  = 100;
  let mut prev_player_mp  = 100;
  //title_screen();
  //let name: String = scene_0();
  (player_inv, player_hp, player_max_hp, prev_player_hp, player_mp, player_max_mp, prev_player_mp) 
    = 
    scene_1(player_inv.clone(), player_hp.clone(), player_max_hp.clone(), prev_player_hp.clone(), player_mp.clone(), player_max_mp.clone(), prev_player_mp.clone());
  //scene_2();
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

fn check_inv(player_inv: Vec<String>) {
  if player_inv.is_empty() == false {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    let mut a = 1;
    for i in player_inv {
      println!("{}: {}", a, i);
      a += 1;
    }
    println!("");
  }
  else {
    println!("\x1B[2J\x1B[1;1H"); //clears the screen
    println!("Inventory:");
    println!("There's nothing here!");
    println!("");
    return
  }
}

fn room_change_w_items(chosen_rooms:Rooms, items:Vec<String>, mut player_inv:Vec<String>) -> (Vec<String>, String, bool) {
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
        player_inv.push(i.to_string());
      }
      items_looted = true;
      println!("");
    }
  loop {  
    if north_room != "" {println!("North: {}", north_room);}
    if south_room != "" {println!("South: {}", south_room);}
    if west_room != "" {println!("West: {}", west_room);}
    if east_room != "" {println!("East: {}", east_room);}

    println!("Check: Check Inventory");
    println!("");
    println!("Which direction will you go?");
    
    direction = read!();
    direction = direction.to_lowercase();
    //checks if the direction chosen is locked, blocked, or empty
    if direction == "Check" || direction == "check" {
      check_inv(player_inv.clone());
    }
    if direction == "north" || direction == "n" {
      if north_room != "" && north_room != "locked" && north_room != "Locked" && north_room != "blocked" && north_room != "Blocked"{
        room_title = north_room.clone();
        println!("");
        return (player_inv, room_title, items_looted);
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
        return (player_inv, room_title, items_looted);
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
        return (player_inv, room_title, items_looted);
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
        return (player_inv, room_title, items_looted);
      }else {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      println!("");
      println!("You Can't Go That Way\n");
      }
    }
  }
}

fn room_change(chosen_rooms:Rooms, player_inv:Vec<String>) -> String {
  let mut direction:String = "".to_string();
  let mut room_title: String = "".to_string(); 
  let north_room = chosen_rooms.north_room;
  let south_room = chosen_rooms.south_room;
  let west_room = chosen_rooms.west_room;
  let east_room = chosen_rooms.east_room;
  
  loop {  
    if north_room != "" {println!("North: {}\n", north_room);}
    if south_room != "" {println!("South: {}\n", south_room);}
    if west_room != "" {println!("West: {}\n", west_room);}
    if east_room != "" {println!("East: {}\n", east_room);}
    println!("");
    println!("Check: Check Inventory");
    println!("");
    println!("Which direction will you go?");
    
    direction = read!();
    direction = direction.to_lowercase();
    //checks if the direction chosen is locked, blocked, or empty
    if direction == "Check" || direction == "check" {
      check_inv(player_inv.clone());
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

fn entrance(player_inv: Vec<String>) -> String {
  println!("Entrance");
  println!("");
  let enterance_rooms = Rooms {
      north_room: "Follow".to_string(),
      south_room: "Locked".to_string(),
      west_room: "West Hallway".to_string(),
      east_room: "East Hallway".to_string()
    };
    let room_title = room_change(enterance_rooms, player_inv);
    return room_title;
}

fn east_hallway(player_inv: Vec<String>) -> String {
  println!("East Hallway");
  println!("");
  let east_hallway_rooms = Rooms {
      north_room: "Library".to_string(),
      south_room: "Kitchen".to_string(),
      west_room: "Return to Entrance".to_string(),
      east_room: "Locked".to_string()
  };
  let room_title = room_change(east_hallway_rooms, player_inv);
  return room_title;
}

fn west_hallway(player_inv: Vec<String>) -> String {
  println!("West Hallway");
  println!("");
  let west_hallway_rooms = Rooms {
      north_room: "Supply Closet".to_string(),
      south_room: "Locked".to_string(),
      west_room: "Locked".to_string(),
      east_room: "Return to Entrance".to_string()
  };
  let room_title = room_change(west_hallway_rooms, player_inv);
  return room_title;
}

fn supply_closet(mut items_in_closet: bool, mut player_inv: Vec<String>, player_hp: i32, player_max_hp: i32, prev_player_hp: i32,player_mp: i32, player_max_mp: i32, prev_player_mp: i32) -> (Vec<String>, i32, i32, i32, i32, i32, i32, String, bool) {
  println!("Supply Closet");
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
      (player_inv, room_title, items_in_closet) = room_change_w_items(supply_closet_rooms, items, player_inv);
    }
    else {
      room_title = room_change(supply_closet_rooms, player_inv.clone());
    }
  return (player_inv, player_hp, player_max_hp, prev_player_hp, player_mp, player_max_mp, prev_player_mp, room_title, items_in_closet);
}

fn scene_0() -> String {
  println!("What is your name?");
  let name: String = read!();
  println!("Hello {}, press enter to continue", name);
  
  let mut stdin = io::stdin();
  // Read a single byte and discard
  let _ = stdin.read(&mut [0u8]).unwrap();
  
  println!("\x1B[2J\x1B[1;1H");
  
  println!(r#""You were knocked out cold, dude."
"{} Are you okay?""#, name);
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
  }
  else if print_text == 2 {
    println!("");
    println!("He helps you up and you look around.
You seem to be in a cave");
  }
  return name.to_string();
}


fn scene_1(mut player_inv: Vec<String>, mut player_hp: i32, mut player_max_hp: i32, mut prev_player_hp: i32, mut player_mp: i32, mut player_max_mp: i32, mut prev_player_mp: i32) -> (Vec<String>, i32, i32, i32, i32, i32, i32) {
  println!("\x1B[2J\x1B[1;1H"); //clears the screen
  println!(r#""Follow me I'll show you to operations room.""#);
  println!("");
  let mut room_title = entrance(player_inv.clone());
  let mut picked_up_item_from_closet = false;
  let mut closet_room_title: String = "".to_string();
  let mut are_items_in_closet = true;
  loop {
    if room_title == "Follow" {
      return (player_inv.clone(), player_hp, player_max_hp, prev_player_hp, player_mp, player_max_mp, prev_player_mp);
    }

    if room_title == "West Hallway" || room_title == "Return to West Hallway" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      room_title = west_hallway(player_inv.clone());
    }
    if room_title == "Supply Closet" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      if picked_up_item_from_closet == false {
        are_items_in_closet = true;
      }

      (player_inv, 
           player_hp,
           player_max_hp,
           prev_player_hp, 
           player_mp, 
           player_max_mp, 
           prev_player_mp, 
           closet_room_title,
           picked_up_item_from_closet)
        = 
        supply_closet(are_items_in_closet,
                      player_inv.clone(), 
                      player_hp, 
                      player_max_hp, 
                      prev_player_hp, 
                      player_mp, 
                      player_max_mp, 
                      prev_player_mp);
      
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
      room_title = east_hallway(player_inv.clone());
    }
    if room_title == "Return to Entrance" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen
      room_title = entrance(player_inv.clone());
    }
    if room_title == "Kitchen" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen

    }
    if room_title == "Library" {
      println!("\x1B[2J\x1B[1;1H"); //clears the screen

    }
  }
}
