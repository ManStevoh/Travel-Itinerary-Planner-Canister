import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Accommodation {
  'id' : bigint,
  'check_out_date' : bigint,
  'check_in_date' : bigint,
  'name' : string,
  'cost_per_night' : number,
  'location' : string,
}
export type Error = { 'DecodeError' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : Accommodation } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Transportation } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : TravelPlan } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : number } |
  { 'Err' : Error };
export interface Transportation {
  'id' : bigint,
  'cost' : number,
  'mode' : string,
  'departure_date' : bigint,
  'arrival_date' : bigint,
}
export interface TravelPlan {
  'id' : bigint,
  'destination' : string,
  'transportation' : string,
  'activities' : Array<string>,
  'end_date' : bigint,
  'accommodation' : string,
  'start_date' : bigint,
}
export interface TravelPlanPayload {
  'destination' : string,
  'transportation' : string,
  'activities' : Array<string>,
  'end_date' : bigint,
  'accommodation' : string,
  'start_date' : bigint,
}
export interface _SERVICE {
  'add_accommodation' : ActorMethod<[Accommodation], [] | [Accommodation]>,
  'add_multiple_travel_plans' : ActorMethod<
    [Array<TravelPlanPayload>],
    Array<[] | [TravelPlan]>
  >,
  'add_transportation' : ActorMethod<[Transportation], [] | [Transportation]>,
  'add_travel_plan' : ActorMethod<[TravelPlanPayload], [] | [TravelPlan]>,
  'calculate_travel_plan_duration' : ActorMethod<[bigint], [] | [bigint]>,
  'count_travel_plans' : ActorMethod<[], bigint>,
  'delete_accommodation' : ActorMethod<[bigint], Result>,
  'delete_transportation' : ActorMethod<[bigint], Result_1>,
  'delete_travel_plan' : ActorMethod<[bigint], Result_2>,
  'generate_shareable_link' : ActorMethod<[bigint], [] | [string]>,
  'get_accommodation' : ActorMethod<[bigint], Result>,
  'get_next_available_id' : ActorMethod<[], bigint>,
  'get_remaining_budget' : ActorMethod<[], number>,
  'get_transportation' : ActorMethod<[bigint], Result_1>,
  'get_travel_plan' : ActorMethod<[bigint], Result_2>,
  'get_travel_plan_by_link' : ActorMethod<[string], Result_2>,
  'get_travel_plans_page' : ActorMethod<[bigint, bigint], Array<TravelPlan>>,
  'record_expense' : ActorMethod<[number], Result_3>,
  'set_budget' : ActorMethod<[number], number>,
  'update_accommodation' : ActorMethod<[bigint, Accommodation], Result>,
  'update_all_travel_plans' : ActorMethod<
    [Array<[bigint, TravelPlanPayload]>],
    Array<Result_2>
  >,
  'update_transportation' : ActorMethod<[bigint, Transportation], Result_1>,
  'update_travel_plan' : ActorMethod<[bigint, TravelPlanPayload], Result_2>,
}
