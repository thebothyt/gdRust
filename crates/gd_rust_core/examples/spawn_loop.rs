use gd_rust::prelude::*;

fn main() {
    let mut level = Level::new("gdrust demo");
    level.settings.speed = Speed::Double;

    let mover = level.next_group();
    let looper = level.next_group();
    let hits = level.next_item();

    level.add(Block::default(5.0, 0.0).add_groups(&[mover]));
    level.add(Orb::yellow(8.0, 2.0));
    level.add(Spike::regular(12.0, 0.0));

    level.add(MoveTrigger::new(mover, 4.0, 0.0, 1.0, Easing::EaseInOut).at(2.0, 8.0).touch_triggered(true));

    // spawn loop so this actually repeats
    level.add(MoveTrigger::new(mover, -4.0, 0.0, 1.0, Easing::EaseInOut).at(0.0, 12.0).add_groups(&[looper]).spawn_triggered(false));
    level.add(SpawnTrigger::new(looper, 1.0, 0.0).at(1.0, 12.0).add_groups(&[looper]).spawn_triggered(false));

    level.add(CountTrigger::new(hits, 3, looper, false, false).at(3.0, 12.0));

    level.export_gmd(&std::env::var("GDRUST_OUT").unwrap_or("out.gmd".into())).unwrap();
}
