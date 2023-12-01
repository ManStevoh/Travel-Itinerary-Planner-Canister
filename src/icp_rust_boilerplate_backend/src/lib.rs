#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

/// Struct representing a travel plan.
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TravelPlan {
    id: u64,
    destination: String,
    start_date: u64,
    end_date: u64,
    transportation: String,
    accommodation: String,
    activities: Vec<String>,
}

// Implement Storable and BoundedStorable for TravelPlan similar to Message
impl Storable for TravelPlan {
    /// Convert the travel plan to bytes for storage.
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    /// Convert bytes to a travel plan for retrieval.
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => {
                // Handle deserialization error
                panic!("Error decoding TravelPlan: {:?}", err);
            }
        }
    }
}

impl BoundedStorable for TravelPlan {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static TRAVEL_PLANS: RefCell<StableBTreeMap<u64, TravelPlan, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

/// Struct representing payload for creating a new travel plan.
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TravelPlanPayload {
    destination: String,
    start_date: u64,
    end_date: u64,
    transportation: String,
    accommodation: String,
    activities: Vec<String>,
}

/// Enum representing possible errors in the travel plan operations.
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    DecodeError { msg: String },
}

/// Retrieve a travel plan by ID.
#[ic_cdk::query]
fn get_travel_plan(id: u64) -> Result<TravelPlan, Error> {
    match _get_travel_plan(&id) {
        Some(plan) => Ok(plan),
        None => Err(Error::NotFound {
            msg: format!("a travel plan with id={} not found", id),
        }),
    }
}

/// Add a new travel plan.
#[ic_cdk::update]
fn add_travel_plan(plan: TravelPlanPayload) -> Option<TravelPlan> {
    // Validate that start_date is before end_date
    if plan.start_date >= plan.end_date {
        return None;
    }

    // Validate that essential string fields are not empty
    if plan.destination.is_empty()
        || plan.transportation.is_empty()
        || plan.accommodation.is_empty()
    {
        return None;
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    // Use time to resolve the warning
    let _current_time = time();

    let travel_plan = TravelPlan {
        id,
        destination: plan.destination,
        start_date: plan.start_date,
        end_date: plan.end_date,
        transportation: plan.transportation,
        accommodation: plan.accommodation,
        activities: plan.activities,
    };

    do_insert_travel_plan(&travel_plan);
    Some(travel_plan)
}

/// Update an existing travel plan.
#[ic_cdk::update]
fn update_travel_plan(id: u64, payload: TravelPlanPayload) -> Result<TravelPlan, Error> {
    match TRAVEL_PLANS.with(|service| service.borrow().get(&id)) {
        Some(mut plan) => {
            plan.destination = payload.destination;
            plan.start_date = payload.start_date;
            plan.end_date = payload.end_date;
            plan.transportation = payload.transportation;
            plan.accommodation = payload.accommodation;
            plan.activities = payload.activities;
            do_insert_travel_plan(&plan);
            Ok(plan)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a travel plan with id={}. plan not found", id),
        }),
    }
}

/// Insert a travel plan into the storage.
fn do_insert_travel_plan(plan: &TravelPlan) {
    TRAVEL_PLANS.with(|service| service.borrow_mut().insert(plan.id, plan.clone()));
}

/// Delete a travel plan by ID.
#[ic_cdk::update]
fn delete_travel_plan(id: u64) -> Result<TravelPlan, Error> {
    match TRAVEL_PLANS.with(|service| service.borrow_mut().remove(&id)) {
        Some(plan) => Ok(plan),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a travel plan with id={}. plan not found.", id),
        }),
    }
}

/// Retrieve a travel plan by ID (internal function).
fn _get_travel_plan(id: &u64) -> Option<TravelPlan> {
    TRAVEL_PLANS.with(|service| service.borrow().get(id))
}

/// Add multiple travel plans in a single call.
#[ic_cdk::update]
fn add_multiple_travel_plans(plans: Vec<TravelPlanPayload>) -> Vec<Option<TravelPlan>> {
    plans
        .into_iter()
        .map(|plan| add_travel_plan(plan))
        .collect()
}

/// Update multiple travel plans in a single call.
#[ic_cdk::update]
fn update_all_travel_plans(payloads: Vec<(u64, TravelPlanPayload)>) -> Vec<Result<TravelPlan, Error>> {
    payloads
        .into_iter()
        .map(|(id, payload)| update_travel_plan(id, payload))
        .collect()
}

/// Get the next available ID for a new travel plan.
#[ic_cdk::query]
fn get_next_available_id() -> u64 {
    ID_COUNTER.with(|counter| *counter.borrow().get() + 1)
}

/// Query to get a page of travel plans based on offset and limit.
#[ic_cdk::query]
fn get_travel_plans_page(offset: u64, limit: u64) -> Vec<TravelPlan> {
    TRAVEL_PLANS
        .with(|service| {
            let plans = service
                .borrow()
                .range(offset..offset + limit)
                .map(|(_, plan)| plan.clone())
                .collect();
            plans
        })
}

/// Query to get the total number of travel plans.
#[ic_cdk::query]
fn count_travel_plans() -> u64 {
    TRAVEL_PLANS.with(|service| service.borrow().len() as u64)
}

/// Calculate the total duration of a travel plan.
#[ic_cdk::query]
fn calculate_travel_plan_duration(id: u64) -> Option<u64> {
    match TRAVEL_PLANS.with(|service| service.borrow().get(&id)) {
        Some(plan) => Some(plan.end_date - plan.start_date),
        None => None,
    }
}

/// Generate a shareable link for an available travel plan.
#[ic_cdk::query]
fn generate_shareable_link(id: u64) -> Option<String> {
    // Check if the travel plan with the given ID exists
    if _get_travel_plan(&id).is_some() {
        // Assume a simple format for the link, such as "/travel_plan/{id}"
        Some(format!("/travel_plan/{}", id))
    } else {
        None // Return None if the travel plan doesn't exist
    }
}

/// Retrieve a travel plan using a shareable link.
#[ic_cdk::query]
fn get_travel_plan_by_link(link: String) -> Result<TravelPlan, Error> {
    // Extract the travel plan ID from the link
    let id: u64 = link
        .rsplit('/')
        .next()
        .and_then(|id_str| id_str.parse().ok())
        .ok_or(Error::DecodeError {
            msg: "Invalid link format".to_string(),
        })?;

    // Retrieve the travel plan using the ID
    get_travel_plan(id)
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct Budget {
    total_budget: f64,
    remaining_budget: f64,
}

impl Storable for Budget {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => panic!("Error decoding Budget: {:?}", err),
        }
    }
}

impl BoundedStorable for Budget {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static BUDGET: RefCell<Budget> = RefCell::new(
        Budget {
            total_budget: 0.0, // Set an initial value
            remaining_budget: 0.0, // Set an initial value
        }
    );
}

/// Set the budget for the trip.
#[ic_cdk::update]
fn set_budget(total_budget: f64) -> f64 {
    BUDGET.with(|budget| {
        let mut budget = budget.borrow_mut();
        budget.total_budget = total_budget;
        budget.remaining_budget = total_budget;
        budget.remaining_budget
    })
}

/// Get the remaining budget.
#[ic_cdk::query]
fn get_remaining_budget() -> f64 {
    BUDGET.with(|budget| budget.borrow().remaining_budget)
}

/// Record an expense and update the remaining budget.
#[ic_cdk::update]
fn record_expense(expense_amount: f64) -> Result<f64, Error> {
    if expense_amount < 0.0 {
        return Err(Error::DecodeError {
            msg: "Expense amount must be non-negative".to_string(),
        });
    }

    BUDGET.with(|budget| {
        let mut budget = budget.borrow_mut();
        if expense_amount > budget.remaining_budget {
            return Err(Error::DecodeError {
                msg: "Expense exceeds remaining budget".to_string(),
            });
        }

        budget.remaining_budget -= expense_amount;
        Ok(budget.remaining_budget)
    })
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Accommodation {
    id: u64,
    name: String,
    location: String,
    check_in_date: u64,
    check_out_date: u64,
    cost_per_night: f64,
}

impl Storable for Accommodation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => panic!("Error decoding Accommodation: {:?}", err),
        }
    }
}

impl BoundedStorable for Accommodation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static ACCOMMODATIONS: RefCell<StableBTreeMap<u64, Accommodation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

/// Add a new accommodation.
#[ic_cdk::update]
fn add_accommodation(accommodation: Accommodation) -> Option<Accommodation> {
    // Validate that check_in_date is before check_out_date
    if accommodation.check_in_date >= accommodation.check_out_date {
        return None;
    }

    // Validate that essential string fields are not empty
    if accommodation.name.is_empty() || accommodation.location.is_empty() {
        return None;
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let accommodation = Accommodation { id, ..accommodation };

    do_insert_accommodation(&accommodation);
    Some(accommodation)
}

/// Update an existing accommodation.
#[ic_cdk::update]
fn update_accommodation(id: u64, updated_accommodation: Accommodation) -> Result<Accommodation, Error> {
    match ACCOMMODATIONS.with(|service| service.borrow().get(&id)) {
        Some(mut accommodation) => {
            accommodation.name = updated_accommodation.name;
            accommodation.location = updated_accommodation.location;
            accommodation.check_in_date = updated_accommodation.check_in_date;
            accommodation.check_out_date = updated_accommodation.check_out_date;
            accommodation.cost_per_night = updated_accommodation.cost_per_night;
            do_insert_accommodation(&accommodation);
            Ok(accommodation)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update an accommodation with id={}. accommodation not found", id),
        }),
    }
}

/// Insert an accommodation into the storage.
fn do_insert_accommodation(accommodation: &Accommodation) {
    ACCOMMODATIONS.with(|service| service.borrow_mut().insert(accommodation.id, accommodation.clone()));
}

/// Delete an accommodation by ID.
#[ic_cdk::update]
fn delete_accommodation(id: u64) -> Result<Accommodation, Error> {
    match ACCOMMODATIONS.with(|service| service.borrow_mut().remove(&id)) {
        Some(accommodation) => Ok(accommodation),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete an accommodation with id={}. accommodation not found.", id),
        }),
    }
}

/// Retrieve an accommodation by ID.
#[ic_cdk::query]
fn get_accommodation(id: u64) -> Result<Accommodation, Error> {
    match ACCOMMODATIONS.with(|service| service.borrow().get(&id)) {
        Some(accommodation) => Ok(accommodation.clone()),
        None => Err(Error::NotFound {
            msg: format!("an accommodation with id={} not found", id),
        }),
    }
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Transportation {
    id: u64,
    mode: String,
    departure_date: u64,
    arrival_date: u64,
    cost: f64,
}

impl Storable for Transportation {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        match Decode!(bytes.as_ref(), Self) {
            Ok(decoded) => decoded,
            Err(err) => panic!("Error decoding Transportation: {:?}", err),
        }
    }
}

impl BoundedStorable for Transportation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static TRANSPORTATIONS: RefCell<StableBTreeMap<u64, Transportation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

/// Add a new transportation booking.
#[ic_cdk::update]
fn add_transportation(transportation: Transportation) -> Option<Transportation> {
    // Validate that departure_date is before arrival_date
    if transportation.departure_date >= transportation.arrival_date {
        return None;
    }

    // Validate that essential string fields are not empty
    if transportation.mode.is_empty() {
        return None;
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let transportation = Transportation { id, ..transportation };

    do_insert_transportation(&transportation);
    Some(transportation)
}

/// Update an existing transportation booking.
#[ic_cdk::update]
fn update_transportation(id: u64, updated_transportation: Transportation) -> Result<Transportation, Error> {
    match TRANSPORTATIONS.with(|service| service.borrow().get(&id)) {
        Some(mut transportation) => {
            transportation.mode = updated_transportation.mode;
            transportation.departure_date = updated_transportation.departure_date;
            transportation.arrival_date = updated_transportation.arrival_date;
            transportation.cost = updated_transportation.cost;
            do_insert_transportation(&transportation);
            Ok(transportation)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a transportation booking with id={}. transportation not found", id),
        }),
    }
}

/// Insert a transportation booking into the storage.
fn do_insert_transportation(transportation: &Transportation) {
    TRANSPORTATIONS.with(|service| service.borrow_mut().insert(transportation.id, transportation.clone()));
}

/// Delete a transportation booking by ID.
#[ic_cdk::update]
fn delete_transportation(id: u64) -> Result<Transportation, Error> {
    match TRANSPORTATIONS.with(|service| service.borrow_mut().remove(&id)) {
        Some(transportation) => Ok(transportation),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a transportation booking with id={}. transportation not found.", id),
        }),
    }
}

/// Retrieve a transportation booking by ID.
#[ic_cdk::query]
fn get_transportation(id: u64) -> Result<Transportation, Error> {
    match TRANSPORTATIONS.with(|service| service.borrow().get(&id)) {
        Some(transportation) => Ok(transportation.clone()),
        None => Err(Error::NotFound {
            msg: format!("a transportation booking with id={} not found", id),
        }),
    }
}

// Export Candid for the Travel Itinerary Planner
ic_cdk::export_candid!();