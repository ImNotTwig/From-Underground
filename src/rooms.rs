pub fn entrance() -> String {
    println!("Entrance");
    println!("");
    let enterance_rooms = Rooms {
        north_room: "Follow".to_string(),
        south_room: "Locked".to_string(),
        west_room: "West Hallway".to_string(),
        east_room: "East Hallway".to_string()
      };
      let room_title = room_change(enterance_rooms);
      return room_title;
}

pub fn east_hallway() -> String {
    println!("East Hallway");
    println!("");
    let east_hallway_rooms = Rooms {
        north_room: "Library".to_string(),
        south_room: "Kitchen".to_string(),
        west_room: "Return to Entrance".to_string(),
        east_room: "Locked".to_string()
    };
    let room_title = room_change(east_hallway_rooms);
    return room_title;
}

pub fn west_hallway() -> String {
    println!("West Hallway");
    println!("");
    let west_hallway_rooms = Rooms {
        north_room: "Supply Closet".to_string(),
        south_room: "Locked".to_string(),
        west_room: "Locked".to_string(),
        east_room: "Return to Entrance".to_string()
    };
    let room_title = room_change(west_hallway_rooms);
    return room_title;
}

pub fn supply_closet(mut items_in_room: i32, mut player_inv: Vec<String>, player_hp: i32, player_max_hp: i32, prev_player_hp: i32,player_mp: i32, player_max_mp: i32, prev_player_mp: i32) -> (Vec<String>, i32, i32, i32, i32, i32, i32, String, i32) {
  println!("Supply Closet");
  println!("");
  let mut items = vec!("");
  let supply_closet_rooms = Rooms {
        north_room: "".to_string(),
        south_room: "Return to West Hallway".to_string(),
        west_room: "".to_string(),
        east_room: "".to_string()
    };
  if items_in_room != 0 {
    if items_in_room == 1 {
      items = vec!("Rope");
    }
    if items_in_room == 2 {
      items = vec!("Gemstone");
    }
    if items_in_room == 3 {
      items = vec!("Rope", "Gemstone");
    }
  let mut counter = 1;
  println!("Items in the room:");
  for i in items {
    println!("Item {}: {}",counter,i);
    counter += 1;
  }
  let item_selection: i32 = read!();

  if item_selection != 0 {
    if items_in_room == 1 { //if only gemstone is showing
      println!("");
      println!("You picked up some rope");
      player_inv.push("Rope".to_string());
      println!("");
      items_in_room = 0;
      }
    else if items_in_room == 2 { //if only rope is showing
      println!("");
      println!("You picked up a gemstone");
      player_inv.push("Gemstone".to_string());
      println!("");
      items_in_room = 0;
      }
    if items_in_room != 2 {
      if items_in_room != 1 {  
        if items_in_room == 3 { //if all items are in room
          if item_selection == 2 { //Selected Gemstone
            println!("");
            println!("You picked up a gemstone");
            println!("");
            player_inv.push("Gemstone".to_string());
            items_in_room = 1; //makes the items vector only show Rope
            }
          if item_selection == 1 { //Selected Rope
            println!("");
            println!("You picked up some rope");
            println!("");
            player_inv.push("Rope".to_string());
            items_in_room = 2; //makes the items vector only show Gemstone 
            }
          }
        }
      }
    }
  } 
  let room_title = room_change(supply_closet_rooms);
  return (player_inv, player_hp, player_max_hp, prev_player_hp, player_mp, player_max_mp, prev_player_mp, room_title, items_in_room);
}
