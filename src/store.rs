use std::time::{Instant, SystemTime};

use crate::{ActionId, ActionWithId, Effects, Reducer, TimeService};

/// Wraps around State and allows only immutable borrow,
/// Through `StateWrapper::get` method.
///
/// Mutable borrow of state can only happen in reducer.
pub struct StateWrapper<State> {
    inner: State,
}

impl<State> StateWrapper<State> {
    /// Get immutable reference to State.
    #[inline(always)]
    pub fn get(&self) -> &State {
        &self.inner
    }

    /// Get mutable reference to State.
    ///
    /// Only should be used in reducer and is not `pub`
    /// so it can't be accessed from lib users.
    #[inline(always)]
    fn get_mut(&mut self) -> &mut State {
        &mut self.inner
    }
}

impl<T: Clone> Clone for StateWrapper<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// A container holding a state and providing the possibility to dispatch actions.
///
/// A store is defined by the state is holds and the actions it can dispatch.
pub struct Store<State, Service, Action> {
    reducer: Reducer<State, Action>,
    effects: Effects<State, Service, Action>,

    /// Current State.
    ///
    /// Immutable access can be gained using `store.state.get()`.
    /// Mutation can only happen inside reducer.
    pub state: StateWrapper<State>,
    pub service: Service,

    monotonic_time: Instant,
    last_action_id: ActionId,
}

impl<State, Service, Action> Store<State, Service, Action>
where
    Service: TimeService,
{
    /// Creates a new store.
    pub fn new(
        reducer: Reducer<State, Action>,
        effects: Effects<State, Service, Action>,
        mut service: Service,
        initial_time: SystemTime,
        initial_state: State,
    ) -> Self {
        let initial_monotonic_time = service.monotonic_time();
        let initial_time_nanos = initial_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|x| x.as_nanos())
            .unwrap_or(0);

        Self {
            reducer,
            effects,
            service,
            state: StateWrapper {
                inner: initial_state,
            },

            monotonic_time: initial_monotonic_time,
            last_action_id: ActionId::new_unchecked(initial_time_nanos as u64),
        }
    }

    /// Returns the current state.
    #[inline(always)]
    pub fn state(&self) -> &State {
        self.state.get()
    }

    #[inline(always)]
    pub fn service(&mut self) -> &mut Service {
        &mut self.service
    }

    pub fn dispatch(&mut self, action: Action) {
        let monotonic_time = self.service.monotonic_time();
        let time_passed = monotonic_time
            .duration_since(self.monotonic_time)
            .as_nanos();

        self.monotonic_time = monotonic_time;
        self.last_action_id = self.last_action_id.next(time_passed as u64);

        let action_with_id = ActionWithId {
            id: self.last_action_id,
            action,
        };

        self.dispatch_reducer(&action_with_id);
        self.dispatch_effects(&action_with_id);
    }

    /// Runs the reducer.
    #[inline(always)]
    fn dispatch_reducer(&mut self, action_with_id: &ActionWithId<Action>) {
        (&self.reducer)(self.state.get_mut(), action_with_id);
    }

    /// Runs the effects.
    #[inline(always)]
    fn dispatch_effects(&mut self, action_with_id: &ActionWithId<Action>) {
        (&self.effects)(self, action_with_id);
    }
}

impl<State, Service, Action> Clone for Store<State, Service, Action>
where
    State: Clone,
    Service: Clone,
    Action: Clone,
{
    fn clone(&self) -> Self {
        Self {
            reducer: self.reducer,
            effects: self.effects,
            service: self.service.clone(),
            state: self.state.clone(),

            monotonic_time: self.monotonic_time.clone(),
            last_action_id: self.last_action_id.clone(),
        }
    }
}
