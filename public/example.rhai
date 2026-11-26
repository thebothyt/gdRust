set_name("gdrust playground");
set_speed(Speed::Double);

let mover = next_group();
let looper = next_group();
let hits = next_item();

add(start_pos(0, 0));
add(Block::default(5, 0).add_groups([mover]));
add(Orb::yellow(8, 2));
add(Spike::regular(12, 0));

add(MoveTrigger::new(mover, 4, 0, 1, Easing::EaseInOut).at(2, 8).touch_triggered(true));

// spawn loop so this actually repeats
add(MoveTrigger::new(mover, -4, 0, 1, Easing::EaseInOut).at(0, 12).add_groups([looper]).spawn_triggered(false));
add(SpawnTrigger::new(looper, 1, 0).at(1, 12).add_groups([looper]).spawn_triggered(false));

add(CountTrigger::new(hits, 3, looper, false, false).at(3, 12));
