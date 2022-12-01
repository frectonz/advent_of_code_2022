open Base

let data = In_channel.with_open_text "../data" In_channel.input_all

type elfCalories = {calories: int list}

let parseElfCalories data =
  String.split ~on:'\n' data
  |> List.group ~break:(fun _ a -> String.equal a "")
  |> List.map ~f:(fun x ->
         let x = List.filter ~f:(fun x -> not (String.equal x "")) x in
         let calories = List.map ~f:(fun x -> Int.of_string x) x in
         {calories} )

let findLargestElfCalories elfCalories =
  let totaledElfCalories =
    List.map ~f:(fun x -> List.fold ~init:0 ~f:(+) x.calories) elfCalories
  in
  List.max_elt ~compare:compare totaledElfCalories

let filterWithSum elfCalories sum =
  List.filter ~f:(fun x -> not (List.fold ~init:0 ~f:(+) x.calories = sum)) elfCalories

let findTopThreeLargestElfCalories elfCalories =
  let top = findLargestElfCalories elfCalories |> Option.value_exn in
  let filtered = filterWithSum elfCalories top in
  let second = findLargestElfCalories filtered |> Option.value_exn in
  let filtered = filterWithSum filtered second in
  let third = findLargestElfCalories filtered |> Option.value_exn in
  top, second, third

let () =
  let allElfCalories = parseElfCalories data in
  let top, second, third = findTopThreeLargestElfCalories allElfCalories in
  let _ = Stdio.print_endline ("top = " ^(top |> Int.to_string)) in
  let _ = Stdio.print_endline ("second = " ^(second |> Int.to_string)) in
  let _ = Stdio.print_endline ("third = " ^(third |> Int.to_string)) in
  let _ = Stdio.print_endline ("top + second + third = " ^((top + second + third) |> Int.to_string)) in
  ()
