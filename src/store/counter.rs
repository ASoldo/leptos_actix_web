use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct CounterStore {
    pub count: RwSignal<i32>,
}

impl CounterStore {
    pub fn increment(&self) {
        self.count.update(|c| *c += 1);
    }
}

pub fn provide_counter_store() {
    provide_context(CounterStore {
        count: RwSignal::new(0),
    });
}

pub fn use_counter_store() -> CounterStore {
    expect_context::<CounterStore>()
}
