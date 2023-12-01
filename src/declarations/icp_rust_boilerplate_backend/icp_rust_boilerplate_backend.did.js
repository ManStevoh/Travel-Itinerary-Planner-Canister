export const idlFactory = ({ IDL }) => {
  const Accommodation = IDL.Record({
    'id' : IDL.Nat64,
    'check_out_date' : IDL.Nat64,
    'check_in_date' : IDL.Nat64,
    'name' : IDL.Text,
    'cost_per_night' : IDL.Float64,
    'location' : IDL.Text,
  });
  const TravelPlanPayload = IDL.Record({
    'destination' : IDL.Text,
    'transportation' : IDL.Text,
    'activities' : IDL.Vec(IDL.Text),
    'end_date' : IDL.Nat64,
    'accommodation' : IDL.Text,
    'start_date' : IDL.Nat64,
  });
  const TravelPlan = IDL.Record({
    'id' : IDL.Nat64,
    'destination' : IDL.Text,
    'transportation' : IDL.Text,
    'activities' : IDL.Vec(IDL.Text),
    'end_date' : IDL.Nat64,
    'accommodation' : IDL.Text,
    'start_date' : IDL.Nat64,
  });
  const Transportation = IDL.Record({
    'id' : IDL.Nat64,
    'cost' : IDL.Float64,
    'mode' : IDL.Text,
    'departure_date' : IDL.Nat64,
    'arrival_date' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'DecodeError' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Accommodation, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : Transportation, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : TravelPlan, 'Err' : Error });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Float64, 'Err' : Error });
  return IDL.Service({
    'add_accommodation' : IDL.Func(
        [Accommodation],
        [IDL.Opt(Accommodation)],
        [],
      ),
    'add_multiple_travel_plans' : IDL.Func(
        [IDL.Vec(TravelPlanPayload)],
        [IDL.Vec(IDL.Opt(TravelPlan))],
        [],
      ),
    'add_transportation' : IDL.Func(
        [Transportation],
        [IDL.Opt(Transportation)],
        [],
      ),
    'add_travel_plan' : IDL.Func(
        [TravelPlanPayload],
        [IDL.Opt(TravelPlan)],
        [],
      ),
    'calculate_travel_plan_duration' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Nat64)],
        ['query'],
      ),
    'count_travel_plans' : IDL.Func([], [IDL.Nat64], ['query']),
    'delete_accommodation' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_transportation' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_travel_plan' : IDL.Func([IDL.Nat64], [Result_2], []),
    'generate_shareable_link' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(IDL.Text)],
        ['query'],
      ),
    'get_accommodation' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_next_available_id' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_remaining_budget' : IDL.Func([], [IDL.Float64], ['query']),
    'get_transportation' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_travel_plan' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_travel_plan_by_link' : IDL.Func([IDL.Text], [Result_2], ['query']),
    'get_travel_plans_page' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Vec(TravelPlan)],
        ['query'],
      ),
    'record_expense' : IDL.Func([IDL.Float64], [Result_3], []),
    'set_budget' : IDL.Func([IDL.Float64], [IDL.Float64], []),
    'update_accommodation' : IDL.Func([IDL.Nat64, Accommodation], [Result], []),
    'update_all_travel_plans' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat64, TravelPlanPayload))],
        [IDL.Vec(Result_2)],
        [],
      ),
    'update_transportation' : IDL.Func(
        [IDL.Nat64, Transportation],
        [Result_1],
        [],
      ),
    'update_travel_plan' : IDL.Func(
        [IDL.Nat64, TravelPlanPayload],
        [Result_2],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
