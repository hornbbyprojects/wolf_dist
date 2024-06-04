use crate::game::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Timer {
    callback: Box<dyn FnOnce(&mut Game)>,
    proc_at: u32,
}

struct Processing {
    callback: Rc<RefCell<Box<dyn Fn(&mut Game)>>>,
}

pub struct TimerSystem {
    timers: IdMap<TimerId, Timer>,
    processing: IdMap<TimerId, Processing>,
}

impl TimerSystem {
    pub fn new() -> Self {
        TimerSystem {
            timers: IdMap::new(),
            processing: IdMap::new(),
        }
    }
    pub fn add_processing<T: 'static + Fn(&mut Game)>(
        game: &mut Game,
        callback: Box<T>,
    ) -> TimerId {
        let id = game.get_id();
        let processing = Processing {
            callback: Rc::new(RefCell::new(callback)),
        };
        game.timer_system.processing.insert(id, processing);
        id
    }
    pub fn add_timer(
        game: &mut Game,
        callback: Box<dyn FnOnce(&mut Game)>,
        time_left: u32,
    ) -> TimerId {
        let id = game.get_id();
        let timer = Timer {
            callback,
            proc_at: game.tick_counter + time_left,
        };
        game.timer_system.timers.insert(id, timer);
        id
    }
    pub fn remove_timer(game: &mut Game, timer_id: TimerId) {
        game.timer_system.timers.remove(timer_id);
    }
    pub fn activate_immediately(game: &mut Game, timer_id: TimerId) {
        if let Some(timer) = game.timer_system.timers.remove(timer_id) {
            (timer.callback)(game);
        }
    }
    pub fn step(game: &mut Game) {
        let mut processings = Vec::new();
        for (_id, processing) in game.timer_system.processing.iter() {
            processings.push(processing.callback.clone());
        }
        for processing_callback in processings {
            let callback = processing_callback.borrow();
            callback(game);
        }
        let mut activating = Vec::new();
        for (id, timer) in game.timer_system.timers.iter() {
            if timer.proc_at <= game.tick_counter {
                activating.push(id);
            }
        }
        for timer_id in activating {
            let timer = game.timer_system.timers.remove(timer_id).unwrap();
            (timer.callback)(game);
        }
    }
}
